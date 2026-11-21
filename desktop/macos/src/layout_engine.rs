//! Local DocLayout-YOLO inference and the page geometry it consumes.

use std::ffi::{CString, c_char, c_void};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr::NonNull;
use std::sync::OnceLock;

use babeldoc_p2_stages::stages::layout_parser::{Coordinate, ModelBox};
use ort::{session::Session, value::Tensor};

const MODEL_EDGE: usize = 1024;
const BOX_WIDTH: usize = 6;
const CLASS_NAMES: [&str; 10] = [
    "title",
    "plain text",
    "abandon",
    "figure",
    "figure_caption",
    "table",
    "table_caption",
    "table_footnote",
    "isolate_formula",
    "formula_caption",
];

pub struct LayoutModel(Session);

impl LayoutModel {
    pub fn new(model_path: &Path, runtime_path: &Path) -> Result<Self, String> {
        static RUNTIME: OnceLock<Result<(), String>> = OnceLock::new();
        if !runtime_path.is_file() {
            return Err("The app is missing its ONNX Runtime files.".into());
        }
        if let Err(error) = RUNTIME.get_or_init(|| {
            let environment = ort::init_from(runtime_path)
                .map_err(|error| format!("Could not load ONNX Runtime ({error})."))?;
            environment
                .with_name("Hermeneuein")
                .with_telemetry(false)
                .commit();
            Ok(())
        }) {
            return Err(error.clone());
        }
        let threads = std::thread::available_parallelism().map_or(1, |count| count.get().min(4));
        let environment = ort::environment::Environment::current()
            .map_err(|error| format!("Could not access the layout detection runtime ({error})."))?;
        let cpu_device = environment
            .devices()
            .find(|device| matches!(device.ep(), Ok("CPUExecutionProvider")))
            .ok_or("Layout detection requires a CPU execution provider.")?;
        let session = Session::builder()
            .map_err(|error| format!("Could not start layout detection ({error})."))?
            .with_devices([cpu_device], None)
            .map_err(|error| format!("Could not set the CPU execution provider for layout detection ({error})."))?
            .with_intra_threads(threads)
            .map_err(|error| format!("Could not configure layout detection ({error})."))?
            .commit_from_file(model_path)
            .map_err(|error| format!("Could not load the layout detection model ({error})."))?;
        Ok(Self(session))
    }

    pub fn infer(
        &mut self,
        source: &Path,
        page_number: usize,
    ) -> Result<([f64; 2], Vec<ModelBox>), String> {
        let raster = rasterize(source, page_number)?;
        let (input, scale, padding) = model_input(&raster);
        let tensor = Tensor::from_array(([1, 3, MODEL_EDGE, MODEL_EDGE], input))
            .map_err(|error| format!("Could not prepare the page image for layout detection ({error})."))?;
        let output = self
            .0
            .run(ort::inputs!["images" => tensor])
            .map_err(|error| format!("Layout detection failed ({error})."))?;
        let (shape, rows) = output[0]
            .try_extract_tensor::<f32>()
            .map_err(|error| format!("Could not read the layout detection result ({error})."))?;
        if shape.len() != 3 || shape[0] != 1 || shape[2] != BOX_WIDTH as i64 {
            return Err(format!("Unexpected layout detection output shape ({shape:?})."));
        }
        let full_size = [raster.width, raster.height];
        let mut boxes = Vec::new();
        for row in rows.chunks_exact(BOX_WIDTH) {
            if row[4] <= 0.25 || !row.iter().all(|value| value.is_finite()) || row[5] < 0.0 {
                continue;
            }
            let Some(class_name) = CLASS_NAMES.get(row[5] as usize) else {
                continue;
            };
            let xyxy = std::array::from_fn(|index| {
                let axis = index % 2;
                let point = (row[index] - padding[axis]) / scale;
                Coordinate::F32(point * (raster.point_size[axis] / full_size[axis] as f64) as f32)
            });
            boxes.push(ModelBox {
                xyxy,
                conf: f64::from(row[4]),
                class_name: (*class_name).into(),
            });
        }
        boxes.sort_by(|a, b| b.conf.total_cmp(&a.conf));
        Ok((raster.point_size, boxes))
    }
}

struct PageRaster {
    point_size: [f64; 2],
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

fn model_input(raster: &PageRaster) -> (Vec<f32>, f32, [f32; 2]) {
    // v0.7's model fit preserves aspect ratio, centers grey padding and consumes BGR.
    let scale = MODEL_EDGE as f64 / raster.width.max(raster.height) as f64;
    let resized = [raster.width, raster.height].map(|length| {
        (length as f64 * scale)
            .round_ties_even()
            .clamp(1.0, MODEL_EDGE as f64) as usize
    });
    let offset = resized.map(|length| (MODEL_EDGE - length) / 2);
    let padding = [raster.width, raster.height].map(|length| {
        (((MODEL_EDGE as f64 - length as f64 * scale) / 2.0) - 0.1).round_ties_even() as f32
    });
    let mut tensor = vec![114.0 / 255.0; 3 * MODEL_EDGE * MODEL_EDGE];
    let columns = sample_positions(raster.width, resized[0]);
    let rows = sample_positions(raster.height, resized[1]);
    let area = MODEL_EDGE * MODEL_EDGE;
    for (y, &(y0, y1, dy)) in rows.iter().enumerate() {
        for (x, &(x0, x1, dx)) in columns.iter().enumerate() {
            let destination = (y + offset[1]) * MODEL_EDGE + x + offset[0];
            for channel in 0..3 {
                let sample = |row, col| {
                    f32::from(raster.pixels[(row * raster.width + col) * 4 + 2 - channel])
                };
                let top = sample(y0, x0) * (1.0 - dx) + sample(y0, x1) * dx;
                let bottom = sample(y1, x0) * (1.0 - dx) + sample(y1, x1) * dx;
                tensor[channel * area + destination] =
                    (top * (1.0 - dy) + bottom * dy).round() / 255.0;
            }
        }
    }
    (tensor, scale as f32, padding)
}

fn sample_positions(size: usize, resized: usize) -> Vec<(usize, usize, f32)> {
    let ratio = size as f64 / resized as f64;
    (0..resized)
        .map(|index| {
            let source = ((index as f64 + 0.5) * ratio - 0.5).clamp(0.0, (size - 1) as f64);
            let base = source.floor() as usize;
            (
                base,
                (base + 1).min(size - 1),
                (source - base as f64) as f32,
            )
        })
        .collect()
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

type Native = *mut c_void;

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    fn CGDataProviderCreateWithFilename(filename: *const c_char) -> Native;
    fn CGPDFDocumentCreateWithProvider(provider: Native) -> Native;
    fn CGPDFDocumentGetPage(document: Native, page_number: usize) -> Native;
    fn CGPDFPageGetBoxRect(page: Native, kind: i32) -> Rect;
    fn CGColorSpaceCreateDeviceRGB() -> Native;
    fn CGBitmapContextCreate(
        data: Native,
        width: usize,
        height: usize,
        bits_per_component: usize,
        bytes_per_row: usize,
        space: Native,
        bitmap_info: u32,
    ) -> Native;
    fn CGContextSetRGBFillColor(context: Native, red: f64, green: f64, blue: f64, alpha: f64);
    fn CGContextFillRect(context: Native, rect: Rect);
    fn CGContextScaleCTM(context: Native, x: f64, y: f64);
    fn CGContextDrawPDFPage(context: Native, page: Native);
    fn CGContextFlush(context: Native);
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFRelease(value: Native);
}

struct OwnedCf(NonNull<c_void>);

impl OwnedCf {
    fn new(value: Native, message: &str) -> Result<Self, String> {
        NonNull::new(value).map(Self).ok_or_else(|| message.into())
    }

    fn get(&self) -> Native {
        self.0.as_ptr()
    }
}

impl Drop for OwnedCf {
    fn drop(&mut self) {
        // This handle owns one retained Core Foundation object.
        unsafe { CFRelease(self.get()) };
    }
}

fn rasterize(source: &Path, page_number: usize) -> Result<PageRaster, String> {
    let path = CString::new(source.as_os_str().as_bytes()).map_err(|_| "The PDF path is invalid.")?;
    let number = page_number.checked_add(1).ok_or("The PDF page number is invalid.")?;
    // The provider, document, and context stay alive through the draw. CoreGraphics writes
    // to the caller-owned bitmap; the context is dropped before the bitmap is read or moved.
    unsafe {
        let provider = OwnedCf::new(
            CGDataProviderCreateWithFilename(path.as_ptr()),
            "Could not read the PDF.",
        )?;
        let document = OwnedCf::new(
            CGPDFDocumentCreateWithProvider(provider.get()),
            "Could not open the PDF.",
        )?;
        let page = CGPDFDocumentGetPage(document.get(), number);
        if page.is_null() {
            return Err("The PDF page number is invalid.".into());
        }
        // The BabelDOC parser normalizes the MediaBox origin to zero before layout inference.
        let media = CGPDFPageGetBoxRect(page, 0);
        let point_size = [media.x + media.width, media.y + media.height];
        if point_size
            .iter()
            .any(|edge| !edge.is_finite() || *edge <= 0.0)
        {
            return Err("The PDF page dimensions are invalid.".into());
        }
        let long_edge = point_size[0].max(point_size[1]);
        let target = MODEL_EDGE as f64;
        let scale = if long_edge <= 2.0 * target {
            1.0
        } else {
            ((target * 72.0 / long_edge).floor().max(1.0) / 72.0).min(2.0 * target / long_edge)
        };
        let [width, height] = point_size.map(|edge| (edge * scale).ceil().max(1.0) as usize);
        let mut pixels = vec![255_u8; width * height * 4];
        let space = OwnedCf::new(CGColorSpaceCreateDeviceRGB(), "Could not initialize the page image color space.")?;
        const RGBX_BITMAP: u32 = 5; // kCGImageAlphaNoneSkipLast
        let context = OwnedCf::new(
            CGBitmapContextCreate(
                pixels.as_mut_ptr().cast(),
                width,
                height,
                8,
                width * 4,
                space.get(),
                RGBX_BITMAP,
            ),
            "Could not initialize the page image bitmap.",
        )?;
        CGContextSetRGBFillColor(context.get(), 1.0, 1.0, 1.0, 1.0);
        CGContextFillRect(
            context.get(),
            Rect {
                x: 0.0,
                y: 0.0,
                width: width as f64,
                height: height as f64,
            },
        );
        CGContextScaleCTM(context.get(), scale, scale);
        CGContextDrawPDFPage(context.get(), page);
        CGContextFlush(context.get());
        drop(context);
        Ok(PageRaster {
            point_size,
            width,
            height,
            pixels,
        })
    }
}
