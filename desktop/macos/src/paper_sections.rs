use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Default)]
pub(super) struct Sections {
    pages: BTreeSet<usize>,
    regions: BTreeMap<usize, Vec<Region>>,
}

impl Sections {
    pub(super) fn whole_page(&self, page: usize) -> bool {
        self.pages.contains(&page)
    }

    pub(super) fn contains(&self, page: usize, bounds: Option<[f64; 4]>, text: &str) -> bool {
        if self.whole_page(page) {
            return true;
        }
        let Some(regions) = self.regions.get(&page) else {
            return false;
        };
        if let Some(bounds) = bounds.filter(|bounds| bounds.iter().all(|value| value.is_finite())) {
            let rect = Rect::from(bounds);
            regions.iter().any(|region| region.bounds.covers(rect))
        } else {
            let text_key = words_key(text);
            regions.iter().any(|region| {
                front_label(text) == Some(region.kind)
                    || (region.kind == Kind::References && is_reference_heading(text))
                    || (text_key.len() >= 12 && region.text_key.contains(&text_key))
            })
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Abstract,
    Keywords,
    References,
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

impl From<[f64; 4]> for Rect {
    fn from([x0, y0, x1, y1]: [f64; 4]) -> Self {
        Self { x0: x0.min(x1), y0: y0.min(y1), x1: x1.max(x0), y1: y1.max(y0) }
    }
}

impl Rect {
    fn midpoint_y(self) -> f64 { (self.y0 + self.y1) / 2.0 }

    fn union(self, other: Self) -> Self {
        Self { x0: self.x0.min(other.x0), y0: self.y0.min(other.y0),
               x1: self.x1.max(other.x1), y1: self.y1.max(other.y1) }
    }

    fn covers(self, other: Self) -> bool {
        let x = (self.x1.min(other.x1) + 3.0 - (self.x0.max(other.x0) - 3.0))
            .max(0.0).min(other.x1 - other.x0);
        let y = (self.y1.min(other.y1) + 4.0 - (self.y0.max(other.y0) - 4.0))
            .max(0.0).min(other.y1 - other.y0);
        let width = other.x1 - other.x0;
        let height = other.y1 - other.y0;
        width > 0.0 && height > 0.0 && x / width >= 0.66 && y / height >= 0.62
    }
}

struct Region {
    kind: Kind,
    bounds: Rect,
    text_key: String,
}

struct Page {
    width: f64,
    height: f64,
    lines: Vec<Line>,
}

struct Line {
    text: String,
    bounds: Rect,
}

fn words_key(text: &str) -> String {
    text.chars()
        .filter(|ch| ch.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn without_number(text: &str) -> &str {
    let trimmed = text.trim();
    if let Some((prefix, rest)) = trimmed.split_once(char::is_whitespace) {
        let has_digit = prefix.chars().any(|ch| ch.is_ascii_digit());
        if has_digit && prefix.chars().all(|ch| ch.is_ascii_digit() || matches!(ch, '.' | ')' | '(' | '．' | '、')) {
            return rest.trim_start();
        }
    }
    trimmed
}

fn heading_key(text: &str) -> String {
    let text = without_number(text).trim_end_matches([':', '：', '.', '．']).trim();
    if text.chars().all(|ch| ch.is_alphabetic() || ch.is_whitespace() || matches!(ch, 'é' | 'É')) {
        words_key(text)
    } else {
        String::new()
    }
}

fn front_label(text: &str) -> Option<Kind> {
    let label = text.split_once([':', '：', '—', '–']).map_or(text, |(before, _)| before);
    match heading_key(label).as_str() {
        "abstract" | "摘要" | "内容摘要" | "resumen" | "résumé" | "zusammenfassung" => Some(Kind::Abstract),
        "keywords" | "关键词" | "关键字" | "關鍵詞" | "關鍵字"
        | "palabrasclave" | "motsclés" | "schlüsselwörter" => Some(Kind::Keywords),
        _ => None,
    }
}

fn is_reference_heading(text: &str) -> bool {
    matches!(heading_key(text).as_str(),
        "references" | "bibliography" | "workscited" | "literaturecited"
        | "参考文献" | "參考文獻" | "引用文献" | "参考资料" | "參考資料" | "referencias" | "bibliografía"
        | "références" | "literaturverzeichnis")
}

fn is_contents_heading(text: &str) -> bool {
    matches!(heading_key(text).as_str(), "contents" | "tableofcontents" | "目录" | "目次")
}

fn is_intro_heading(text: &str) -> bool {
    let key = heading_key(text);
    matches!(key.as_str(), "introduction" | "maintext" | "引言" | "绪论" | "前言" | "正文")
        || (without_number(text).len() < text.trim().len() && key.len() <= 42
            && matches!(key.as_str(), "background" | "methods" | "materialsandmethods" | "results" | "研究方法" | "研究背景"))
}

fn is_post_references_heading(text: &str) -> bool {
    let key = heading_key(text);
    if matches!(key.as_str(), "appendix" | "appendices" | "supplementarymaterial"
        | "supplementaryinformation" | "supportinginformation" | "authorcontributions"
        | "acknowledgments" | "acknowledgements" | "附录" | "补充材料" | "作者贡献" | "致谢") {
        return true;
    }
    let text = without_number(text).trim();
    if let Some((word, tail)) = text.split_once(char::is_whitespace) {
        if word.eq_ignore_ascii_case("appendix") {
            let code = tail.split(['.', ' ', ':', '—', '–']).next().unwrap_or_default();
            return code.len() == 1 && code.chars().all(|ch| ch.is_ascii_alphanumeric());
        }
    }
    text.strip_prefix("附录").is_some_and(|suffix| suffix.trim().starts_with(|ch: char| ch.is_ascii_alphanumeric()))
}

fn has_bibliography_evidence(lines: impl Iterator<Item = impl AsRef<str>>) -> bool {
    let mut years = 0;
    for line in lines.take(90) {
        let line = line.as_ref();
        let lower = line.to_ascii_lowercase();
        if lower.contains("doi.org/") || lower.contains("doi:") {
            return true;
        }
        let first = line.split_whitespace().next().unwrap_or_default();
        if first.len() >= 3 && first.starts_with('[') && first.ends_with(']')
            && first[1..first.len()-1].bytes().all(|byte| byte.is_ascii_digit()) {
            return true;
        }
        years += line.split(|ch: char| !ch.is_ascii_digit()).filter(|token| {
            token.len() == 4 && (token.starts_with("19") || token.starts_with("20"))
        }).count();
        if years >= 2 {
            return true;
        }
    }
    false
}

fn add_region(regions: &mut Vec<Region>, kind: Kind, lines: &[&Line]) {
    let Some(first) = lines.first() else { return; };
    let bounds = lines.iter().skip(1).fold(first.bounds, |bounds, line| bounds.union(line.bounds));
    let text_key = lines.iter().map(|line| words_key(&line.text)).collect::<String>();
    regions.push(Region { kind, bounds, text_key });
}

#[derive(Clone, Copy)]
struct AbstractTail { column: Rect, near_foot: bool }

fn front_order(page: &Page) -> Vec<&Line> {
    let mut ordered: Vec<_> = page.lines.iter().collect();
    ordered.sort_by(|left, right| right.bounds.midpoint_y().total_cmp(&left.bounds.midpoint_y())
        .then(left.bounds.x0.total_cmp(&right.bounds.x0)));
    ordered
}

fn front_regions(page: &Page, regions: &mut Vec<Region>) -> Option<AbstractTail> {
    let ordered = front_order(page);
    let introduction = ordered.iter().find(|line| is_intro_heading(&line.text))
        .map_or(0.0, |line| line.bounds.y1);
    let mut abstract_tail = None;
    for anchor in &ordered {
        let Some(kind) = front_label(&anchor.text) else { continue; };
        let center_y = anchor.bounds.midpoint_y();
        if center_y <= introduction || center_y < page.height * 0.14 || center_y > page.height * 0.94 {
            continue;
        }
        let center_x = (anchor.bounds.x0 + anchor.bounds.x1) / 2.0;
        let tolerance = (page.width * 0.05).max(14.0);
        let mut seen = vec![*anchor];
        let mut base_x = None::<f64>;
        let mut previous_y = center_y;
        let mut previous_step = 12.0;
        let mut heading_ends_abstract = introduction > 0.0;
        let max_lines = if kind == Kind::Keywords { 13 } else { 65 };
        for line in &ordered {
            let y = line.bounds.midpoint_y();
            if y >= previous_y - 2.5 || y <= introduction || y < page.height * 0.055 {
                continue;
            }
            let same_column = if let Some(x) = base_x {
                (line.bounds.x0 - x).abs() <= tolerance
            } else {
                (line.bounds.x0 - anchor.bounds.x0).abs() <= tolerance
                    || (kind == Kind::Abstract && line.bounds.x0 <= center_x && line.bounds.x1 >= center_x)
            };
            if !same_column {
                continue;
            }
            if front_label(&line.text).is_some() || is_intro_heading(&line.text) || is_reference_heading(&line.text) {
                heading_ends_abstract = true;
                break;
            }
            let gap = previous_y - y;
            let minimum_gap = match (kind, seen.len()) {
                (Kind::Abstract, 1) => 46.0,
                (Kind::Keywords, _) => 20.0,
                _ => 25.0,
            };
            if gap > f64::max(minimum_gap, previous_step * 2.2) {
                break;
            }
            seen.push(*line);
            base_x.get_or_insert(line.bounds.x0);
            previous_y = y;
            previous_step = gap.clamp(7.0, 15.0);
            if seen.len() >= max_lines {
                break;
            }
        }
        add_region(regions, kind, &seen);
        if kind == Kind::Abstract && seen.len() > 1 && !heading_ends_abstract {
            let column = seen.iter().skip(2).fold(seen[1].bounds, |bounds, line| bounds.union(line.bounds));
            abstract_tail = Some(AbstractTail { column, near_foot: column.y0 < page.height * 0.17 });
        }
    }
    abstract_tail
}

fn continue_abstract(page: &Page, tail: AbstractTail, regions: &mut Vec<Region>) -> Option<AbstractTail> {
    let ordered = front_order(page);
    let tolerance = (page.width * 0.05).max(14.0);
    let column = |line: &Line| (line.bounds.x0 - tail.column.x0).abs() <= tolerance;
    let end = ordered.iter().copied().find(|line| {
        is_intro_heading(&line.text) || (column(line) && front_label(&line.text).is_some())
    });
    let until = end.map_or(0.0, |line| line.bounds.y1);
    let keywords_follow = end.is_some_and(|line| front_label(&line.text) == Some(Kind::Keywords));
    if !tail.near_foot && !keywords_follow {
        return None;
    }
    let mut taken = Vec::new();
    let mut previous_y = None::<f64>;
    for line in ordered {
        let y = line.bounds.midpoint_y();
        if y < page.height * 0.055 || y > page.height * 0.95 || y <= until || !column(line) {
            continue;
        }
        if let Some(previous) = previous_y {
            if previous - y > 27.0 { break; }
        } else if y < page.height * 0.64 || line.bounds.x1 - line.bounds.x0 < 75.0 {
            continue;
        }
        taken.push(line);
        previous_y = Some(y);
    }
    if taken.is_empty() { return None; }
    add_region(regions, Kind::Abstract, &taken);
    let last_y = taken.last()?.bounds.y0;
    (end.is_none() && last_y < page.height * 0.17).then_some(AbstractTail { near_foot: true, ..tail })
}

fn reference_region(page: &Page, from: usize, to: usize, regions: &mut Vec<Region>) {
    let lines: Vec<_> = page.lines[from..to].iter().collect();
    if lines.is_empty() { return; }
    let split = page.width * 0.48;
    let wide = lines.iter().any(|line| line.bounds.x0 < split && line.bounds.x1 > split + 35.0);
    if wide {
        add_region(regions, Kind::References, &lines);
    } else {
        let left: Vec<_> = lines.iter().copied().filter(|line| line.bounds.x0 < split).collect();
        let right: Vec<_> = lines.iter().copied().filter(|line| line.bounds.x0 >= split).collect();
        add_region(regions, Kind::References, &left);
        add_region(regions, Kind::References, &right);
    }
}

fn locate(pages: &[Page]) -> Sections {
    let mut result = Sections::default();
    let mut previous_abstract = None;
    for (index, page) in pages.iter().take(3).enumerate() {
        let mut regions = Vec::new();
        let continuation = previous_abstract.and_then(|tail| continue_abstract(page, tail, &mut regions));
        previous_abstract = front_regions(page, &mut regions).or(continuation);
        if !regions.is_empty() {
            result.regions.insert(index, regions);
        }
    }
    let mut active = false;
    for (index, page) in pages.iter().enumerate() {
        if active {
            if let Some(end) = page.lines.iter().position(|line| is_post_references_heading(&line.text)) {
                reference_region(page, 0, end, result.regions.entry(index).or_default());
                active = false;
            } else if !page.lines.is_empty() {
                result.pages.insert(index);
            }
            continue;
        }
        if page.lines.iter().any(|line| is_contents_heading(&line.text)) {
            continue;
        }
        let Some(start) = page.lines.iter().position(|line| is_reference_heading(&line.text)) else {
            continue;
        };
        let end = page.lines[start+1..].iter().position(|line| is_post_references_heading(&line.text))
            .map_or(page.lines.len(), |offset| start + 1 + offset);
        let tail = page.lines[start+1..end].iter().map(|line| line.text.as_str());
        let next = pages.get(index+1).into_iter().flat_map(|next| {
            next.lines.iter().take_while(|line| !is_post_references_heading(&line.text)).map(|line| line.text.as_str())
        });
        if end == start + 1 && end < page.lines.len() {
            continue;
        }
        if index < pages.len() / 2 && !has_bibliography_evidence(tail.clone().chain(next)) {
            continue;
        }
        reference_region(page, start, end, result.regions.entry(index).or_default());
        active = end == page.lines.len();
    }
    result
}

#[cfg(target_os = "macos")]
mod pdfkit {
    use super::{Line, Page, Rect};
    use std::ffi::{CStr, CString, c_char, c_void};
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;
    use std::ptr::null_mut;

    type Id = *mut c_void;
    type Sel = *mut c_void;

    #[cfg(target_arch = "aarch64")]
    type ObjcBool = bool;
    #[cfg(target_arch = "x86_64")]
    type ObjcBool = i8;

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct Rectangle { x: f64, y: f64, width: f64, height: f64 }

    #[link(name = "PDFKit", kind = "framework")]
    unsafe extern "C" {
        #[link_name = "OBJC_CLASS_$_PDFDocument"]
        static PDF_DOCUMENT_CLASS: u8;
    }

    #[link(name = "Foundation", kind = "framework")]
    unsafe extern "C" {}

    #[link(name = "objc")]
    unsafe extern "C" {
        fn objc_msgSend();
        #[cfg(target_arch = "x86_64")]
        fn objc_msgSend_stret();
        fn objc_getClass(name: *const c_char) -> Id;
        fn sel_registerName(name: *const c_char) -> Sel;
        fn objc_release(object: Id);
    }

    // The selector's exact signature determines the Objective-C calling convention.
    macro_rules! send {
        ($object:expr, $selector:expr => $result:ty $(, $argument:expr => $argument_type:ty)*) => {{
            let method: unsafe extern "C" fn(Id, Sel $(, $argument_type)*) -> $result =
                std::mem::transmute(objc_msgSend as *const ());
            method($object, sel_registerName($selector.as_ptr()) $(, $argument)*)
        }};
    }

    macro_rules! rect {
        ($object:expr, $selector:expr, $argument:expr => $argument_type:ty) => {{
            #[cfg(target_arch = "aarch64")]
            let bounds = send!($object, $selector => Rectangle, $argument => $argument_type);
            #[cfg(target_arch = "x86_64")]
            let bounds = {
                let mut bounds = Rectangle::default();
                let method: unsafe extern "C" fn(*mut Rectangle, Id, Sel, $argument_type) =
                    std::mem::transmute(objc_msgSend_stret as *const ());
                method(&mut bounds, $object, sel_registerName($selector.as_ptr()), $argument);
                bounds
            };
            bounds
        }};
    }

    struct Pool(Id);
    impl Drop for Pool {
        fn drop(&mut self) { unsafe { send!(self.0, c"drain" => ()) }; }
    }

    struct Owned(Id);
    impl Drop for Owned {
        fn drop(&mut self) { unsafe { objc_release(self.0) }; }
    }

    fn pool() -> Pool {
        // PDFKit temporary selections live only through the page in which they are read.
        unsafe { Pool(send!(objc_getClass(c"NSAutoreleasePool".as_ptr()), c"new" => Id)) }
    }

    fn page_lines(page: Id, bounds: Rectangle) -> Vec<Line> {
        // PDFKit's line order avoids combining the left keyword column with the right abstract.
        unsafe {
            let selection = send!(page, c"selectionForRect:" => Id, bounds => Rectangle);
            let lines = send!(selection, c"selectionsByLine" => Id);
            let count = send!(lines, c"count" => usize);
            (0..count).filter_map(|index| {
                let line = send!(lines, c"objectAtIndex:" => Id, index => usize);
                let string = send!(line, c"string" => Id);
                let utf8 = send!(string, c"UTF8String" => *const c_char);
                if utf8.is_null() { return None; }
                let text = CStr::from_ptr(utf8).to_string_lossy().trim().to_owned();
                if text.is_empty() { return None; }
                let geometry = rect!(line, c"boundsForPage:", page => Id);
                let x0 = geometry.x - bounds.x;
                let y0 = geometry.y - bounds.y;
                Some(Line { text, bounds: Rect::from([x0, y0, x0 + geometry.width, y0 + geometry.height]) })
            }).collect()
        }
    }

    pub(super) fn read(path: &Path, check_cancel: &mut impl FnMut() -> Result<(), String>) -> Result<Vec<Page>, String> {
        let Some(path) = CString::new(path.as_os_str().as_bytes()).ok() else { return Ok(Vec::new()); };
        let _outer = pool();
        // Use a read-only PDFDocument detached from any window. The outer pool owns the URL.
        let document = unsafe {
            let url_class = objc_getClass(c"NSURL".as_ptr());
            let url = send!(url_class, c"fileURLWithFileSystemRepresentation:isDirectory:relativeToURL:" => Id,
                path.as_ptr() => *const c_char, false as ObjcBool => ObjcBool, null_mut() => Id);
            let class = (&raw const PDF_DOCUMENT_CLASS).cast_mut().cast();
            let allocated = send!(class, c"alloc" => Id);
            Owned(send!(allocated, c"initWithURL:" => Id, url => Id))
        };
        if document.0.is_null() { return Ok(Vec::new()); }
        let count = unsafe { send!(document.0, c"pageCount" => usize) };
        let mut pages = Vec::with_capacity(count);
        for index in 0..count {
            check_cancel()?;
            let _inner = pool();
            let page = unsafe { send!(document.0, c"pageAtIndex:" => Id, index => usize) };
            let bounds = unsafe { rect!(page, c"boundsForBox:", 0 => isize) };
            pages.push(Page { width: bounds.width, height: bounds.height, lines: page_lines(page, bounds) });
        }
        Ok(pages)
    }
}

pub(super) fn inspect(path: &Path, mut check_cancel: impl FnMut() -> Result<(), String>) -> Result<Sections, String> {
    check_cancel()?;
    #[cfg(target_os = "macos")]
    { Ok(locate(&pdfkit::read(path, &mut check_cancel)?)) }
    #[cfg(not(target_os = "macos"))]
    { let _ = path; Ok(Sections::default()) }
}
