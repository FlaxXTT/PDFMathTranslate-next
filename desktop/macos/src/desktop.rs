//! macOS window, WebKit navigation, and native file panels.
//!
//! AppKit and WebKit perform all delegate calls on the main thread. Objective-C
//! selectors and their argument types below use the system framework headers.

use crate::APP_NAME;
use crate::runtime::{Backend, BackendStatus, RuntimePaths};
use std::cell::RefCell;
use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::io;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::time::{Duration, Instant};

type Id = *mut c_void;
type Sel = *mut c_void;
type Imp = unsafe extern "C" fn();
type WebKitPolicy = isize;

const NIL: Id = null_mut();
const NAVIGATION_CANCEL: WebKitPolicy = 0;
const NAVIGATION_ALLOW: WebKitPolicy = 1;
const NAVIGATION_DOWNLOAD: WebKitPolicy = 2;
const MODAL_OK: isize = 1;
const LINK_ACTIVATED: isize = 0;
const ACTIVATION_POLICY_REGULAR: isize = 0;
const FULL_SIZE_CONTENT_VIEW: usize = 1 << 15;
const WINDOW_STYLE: usize = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | FULL_SIZE_CONTENT_VIEW;
const WINDOW_TITLE_HIDDEN: isize = 1;
const RIGHT_TO_LEFT: isize = 1;
const AT_DOCUMENT_START: isize = 0;
const BACKING_BUFFERED: usize = 2;
const VIEW_RESIZE_WIDTH_AND_HEIGHT: usize = (1 << 1) | (1 << 4);

#[cfg(target_arch = "aarch64")]
type ObjcBool = bool;
#[cfg(target_arch = "x86_64")]
type ObjcBool = i8;

fn objc_bool(value: bool) -> ObjcBool {
    #[cfg(target_arch = "aarch64")]
    {
        value
    }
    #[cfg(target_arch = "x86_64")]
    {
        i8::from(value)
    }
}

fn yes(value: ObjcBool) -> bool {
    value != objc_bool(false)
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Size {
    width: f64,
    height: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Rect {
    origin: Point,
    size: Size,
}

const CONTENT_RECT: Rect = Rect {
    origin: Point { x: 0.0, y: 0.0 },
    size: Size {
        width: 980.0,
        height: 638.0,
    },
};

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "OBJC_CLASS_$_NSApplication"]
    static NS_APPLICATION_CLASS: u8;
}

#[link(name = "WebKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "OBJC_CLASS_$_WKWebView"]
    static WK_WEB_VIEW_CLASS: u8;
}

#[link(name = "objc")]
unsafe extern "C" {
    fn objc_msgSend();
    fn objc_getClass(name: *const c_char) -> Id;
    fn objc_getProtocol(name: *const c_char) -> Id;
    fn objc_allocateClassPair(superclass: Id, name: *const c_char, extra: usize) -> Id;
    fn objc_disposeClassPair(class: Id);
    fn objc_registerClassPair(class: Id);
    fn class_addProtocol(class: Id, protocol: Id) -> ObjcBool;
    fn class_addMethod(class: Id, name: Sel, implementation: Imp, types: *const c_char)
    -> ObjcBool;
    fn sel_registerName(name: *const c_char) -> Sel;
    fn objc_retain(object: Id) -> Id;
    fn objc_release(object: Id);
}

// The caller names the exact Objective-C method argument and return types.
// No message in this bridge returns a structure.
macro_rules! send {
    ($object:expr, $selector:expr => $result:ty $(, $value:expr => $arg:ty)*) => {{
        let implementation: unsafe extern "C" fn(Id, Sel $(, $arg)*) -> $result =
            std::mem::transmute(objc_msgSend as *const ());
        implementation($object, sel_registerName($selector.as_ptr()) $(, $value)*)
    }};
}

unsafe fn class(name: &CStr) -> Id {
    unsafe { objc_getClass(name.as_ptr()) }
}

unsafe fn string(value: &str) -> Id {
    let value = CString::new(value.replace('\0', "�")).expect("NULs replaced");
    unsafe {
        send!(class(c"NSString"), c"stringWithUTF8String:" => Id, value.as_ptr() => *const c_char)
    }
}

struct Owned(Id);

impl Drop for Owned {
    fn drop(&mut self) {
        // All owners in this module originate from Objective-C alloc/new or retain.
        unsafe { objc_release(self.0) };
    }
}

struct AutoreleasePool(Id);

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        unsafe { send!(self.0, c"drain" => ()) };
    }
}

struct State {
    backend: Option<Backend>,
    failure: Option<String>,
    log: PathBuf,
    deadline: Instant,
    window: Option<Owned>,
    web_view: Id,
    startup_label: Id,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

fn startup_error(message: &str, log: &std::path::Path) -> String {
    format!("{message}\nLog file {}", log.display())
}

fn state_window() -> Id {
    STATE.with_borrow(|slot| {
        slot.as_ref()
            .and_then(|state| state.window.as_ref())
            .map_or(NIL, |window| window.0)
    })
}

unsafe fn make_menu(app: Id) {
    // alloc/new yields owned references, while the menu hierarchy retains its children.
    unsafe {
        let menu = Owned(send!(class(c"NSMenu"), c"new" => Id));
        let item = Owned(send!(class(c"NSMenuItem"), c"new" => Id));
        let submenu = Owned(send!(class(c"NSMenu"), c"new" => Id));
        send!(menu.0, c"addItem:" => (), item.0 => Id);
        send!(item.0, c"setSubmenu:" => (), submenu.0 => Id);
        send!(submenu.0, c"addItemWithTitle:action:keyEquivalent:" => Id,
            string(&format!("Quit {APP_NAME}")) => Id, sel_registerName(c"terminate:".as_ptr()) => Sel,
            string("q") => Id);
        send!(app, c"setMainMenu:" => (), menu.0 => Id);
    }
}

extern "C" fn did_finish_launching(delegate: Id, _: Sel, notification: Id) {
    // Objects created here stay on the main thread. The window retains the WebView
    // and label so their borrowed pointers remain valid in State during startup.
    unsafe {
        let app = send!(notification, c"object" => Id);
        make_menu(app);
        let window = Owned(send!(
            send!(class(c"NSWindow"), c"alloc" => Id),
            c"initWithContentRect:styleMask:backing:defer:" => Id,
            CONTENT_RECT => Rect, WINDOW_STYLE => usize, BACKING_BUFFERED => usize,
            objc_bool(false) => ObjcBool
        ));
        send!(window.0, c"setReleasedWhenClosed:" => (), objc_bool(false) => ObjcBool);
        send!(window.0, c"setTitle:" => (), string(APP_NAME) => Id);
        send!(window.0, c"setTitleVisibility:" => (), WINDOW_TITLE_HIDDEN => isize);
        send!(window.0, c"setTitlebarAppearsTransparent:" => (), objc_bool(true) => ObjcBool);
        send!(window.0, c"setMinSize:" => (), Size { width: 560.0, height: 440.0 } => Size);
        send!(window.0, c"center" => ());

        let config = Owned(send!(class(c"WKWebViewConfiguration"), c"new" => Id));
        let controls = if send!(window.0, c"windowTitlebarLayoutDirection" => isize) == RIGHT_TO_LEFT {
            "document.documentElement.dataset.windowControls = 'right'"
        } else {
            "document.documentElement.dataset.windowControls = 'left'"
        };
        let script = Owned(send!(
            send!(class(c"WKUserScript"), c"alloc" => Id),
            c"initWithSource:injectionTime:forMainFrameOnly:" => Id,
            string(controls) => Id, AT_DOCUMENT_START => isize, objc_bool(true) => ObjcBool
        ));
        let content_controller = send!(config.0, c"userContentController" => Id);
        send!(content_controller, c"addUserScript:" => (), script.0 => Id);
        let web_view_class = (&raw const WK_WEB_VIEW_CLASS).cast_mut().cast();
        let web_view = Owned(send!(
            send!(web_view_class, c"alloc" => Id), c"initWithFrame:configuration:" => Id,
            CONTENT_RECT => Rect, config.0 => Id
        ));
        send!(web_view.0, c"setNavigationDelegate:" => (), delegate => Id);
        send!(web_view.0, c"setUIDelegate:" => (), delegate => Id);
        send!(web_view.0, c"setAutoresizingMask:" => (), VIEW_RESIZE_WIDTH_AND_HEIGHT => usize);
        send!(window.0, c"setContentView:" => (), web_view.0 => Id);

        let message = STATE.with_borrow(|slot| {
            slot.as_ref()
                .and_then(|state| state.failure.clone())
                .unwrap_or_else(|| format!("Opening {APP_NAME}…"))
        });
        let label = send!(class(c"NSTextField"), c"labelWithString:" => Id, string(&message) => Id);
        send!(label, c"setAlignment:" => (), 2_isize => isize);
        send!(label, c"setTextColor:" => (), send!(class(c"NSColor"), c"secondaryLabelColor" => Id) => Id);
        send!(label, c"setFont:" => (), send!(class(c"NSFont"), c"systemFontOfSize:" => Id, 14.0_f64 => f64) => Id);
        send!(label, c"setFrame:" => (), CONTENT_RECT => Rect);
        send!(label, c"setAutoresizingMask:" => (), VIEW_RESIZE_WIDTH_AND_HEIGHT => usize);
        send!(web_view.0, c"addSubview:" => (), label => Id);

        let window_id = window.0;
        let waiting = STATE.with_borrow_mut(|slot| {
            let state = slot.as_mut().expect("desktop state before launch");
            state.window = Some(window);
            state.web_view = web_view.0;
            state.startup_label = label;
            state.backend.is_some()
        });
        send!(window_id, c"makeKeyAndOrderFront:" => (), NIL => Id);
        send!(app, c"activateIgnoringOtherApps:" => (), objc_bool(true) => ObjcBool);

        if waiting {
            send!(
                class(c"NSTimer"), c"scheduledTimerWithTimeInterval:target:selector:userInfo:repeats:" => Id,
                0.25_f64 => f64, delegate => Id, sel_registerName(c"backendTimer:".as_ptr()) => Sel,
                NIL => Id, objc_bool(true) => ObjcBool
            );
        }
    }
}

enum StartupUpdate {
    Wait,
    Load {
        web_view: Id,
        label: Id,
        url: String,
    },
    Error {
        label: Id,
        message: String,
    },
    Stop,
}

extern "C" fn backend_timer(_: Id, _: Sel, timer: Id) {
    let update = STATE.with_borrow_mut(|slot| {
        let Some(state) = slot.as_mut() else {
            return StartupUpdate::Stop;
        };
        let Some(backend) = state.backend.as_mut() else {
            return StartupUpdate::Stop;
        };
        match backend.poll() {
            Ok(BackendStatus::Ready) => StartupUpdate::Load {
                web_view: state.web_view,
                label: std::mem::replace(&mut state.startup_label, NIL),
                url: backend.url().to_owned(),
            },
            Ok(BackendStatus::Starting) if Instant::now() < state.deadline => StartupUpdate::Wait,
            status => {
                let message = match status {
                    Ok(BackendStatus::Starting) => {
                        "Startup timed out. Check your connection and reopen the app.".to_owned()
                    }
                    Ok(BackendStatus::Exited(exit)) => format!("The background service exited ({exit})."),
                    Err(error) => format!("Could not read the background service status ({error})."),
                    Ok(BackendStatus::Ready) => unreachable!(),
                };
                StartupUpdate::Error {
                    label: state.startup_label,
                    message: startup_error(&message, &state.log),
                }
            }
        }
    });

    // Release the state borrow before messaging WebKit or AppKit, which can
    // immediately call delegates again.
    unsafe {
        match update {
            StartupUpdate::Wait => {}
            StartupUpdate::Stop => send!(timer, c"invalidate" => ()),
            StartupUpdate::Load {
                web_view,
                label,
                url,
            } => {
                send!(timer, c"invalidate" => ());
                send!(label, c"removeFromSuperview" => ());
                let url = send!(class(c"NSURL"), c"URLWithString:" => Id, string(&url) => Id);
                let request = send!(class(c"NSURLRequest"), c"requestWithURL:" => Id, url => Id);
                send!(web_view, c"loadRequest:" => Id, request => Id);
            }
            StartupUpdate::Error { label, message } => {
                send!(timer, c"invalidate" => ());
                send!(label, c"setStringValue:" => (), string(&message) => Id);
                let backend = STATE
                    .with_borrow_mut(|slot| slot.as_mut().and_then(|state| state.backend.take()));
                drop(backend);
            }
        }
    }
}

extern "C" fn last_window_closed(_: Id, _: Sel, _: Id) -> ObjcBool {
    objc_bool(true)
}

extern "C" fn will_terminate(_: Id, _: Sel, _: Id) {
    let backend =
        STATE.with_borrow_mut(|slot| slot.as_mut().and_then(|state| state.backend.take()));
    drop(backend);
}

// Apple's Blocks ABI. AppKit copies the stack block when beginning a sheet.
// The copy helper retains the native panel and copies WebKit's completion so
// both remain alive until the sheet has closed.
#[repr(C)]
struct BlockHeader {
    isa: *const c_void,
    flags: c_int,
    reserved: c_int,
    invoke: *const c_void,
    descriptor: *const c_void,
}

#[repr(C)]
struct PanelBlock {
    header: BlockHeader,
    panel: Id,
    completion: Id,
    selection: Sel,
}

#[repr(C)]
struct NativePanelDescriptor {
    reserved: usize,
    size: usize,
    copy: unsafe extern "C" fn(*mut PanelBlock, *const PanelBlock),
    dispose: unsafe extern "C" fn(*mut PanelBlock),
    signature: *const c_char,
}

// The pointer always refers to the process-lifetime immutable signature below.
unsafe impl Sync for NativePanelDescriptor {}

unsafe extern "C" {
    static _NSConcreteStackBlock: u8;
    fn _Block_copy(block: Id) -> Id;
    fn _Block_release(block: Id);
}

unsafe extern "C" fn panel_copy(dst: *mut PanelBlock, src: *const PanelBlock) {
    unsafe {
        (*dst).panel = objc_retain((*src).panel);
        (*dst).completion = _Block_copy((*src).completion);
    }
}

unsafe extern "C" fn panel_dispose(block: *mut PanelBlock) {
    unsafe {
        objc_release((*block).panel);
        _Block_release((*block).completion);
    }
}

unsafe extern "C" fn panel_finished(block: *const PanelBlock, result: isize) {
    if result == MODAL_OK {
        // NSOpenPanel.URLs or NSSavePanel.URL, both return an Objective-C object.
        unsafe {
            let get_selection: unsafe extern "C" fn(Id, Sel) -> Id =
                std::mem::transmute(objc_msgSend as *const ());
            let selection = get_selection((*block).panel, (*block).selection);
            complete_selection((*block).completion, selection);
        }
    } else {
        unsafe { complete_selection((*block).completion, NIL) };
    }
}

static PANEL_DESCRIPTOR: NativePanelDescriptor = NativePanelDescriptor {
    reserved: 0,
    size: std::mem::size_of::<PanelBlock>(),
    copy: panel_copy,
    dispose: panel_dispose,
    signature: c"v16@?0q8".as_ptr(),
};

unsafe fn complete_selection(completion: Id, selection: Id) {
    unsafe {
        let header = &*completion.cast::<BlockHeader>();
        let call: unsafe extern "C" fn(Id, Id) = std::mem::transmute(header.invoke);
        call(completion, selection);
    }
}

unsafe fn complete_policy(completion: Id, policy: WebKitPolicy) {
    unsafe {
        let header = &*completion.cast::<BlockHeader>();
        let call: unsafe extern "C" fn(Id, WebKitPolicy) = std::mem::transmute(header.invoke);
        call(completion, policy);
    }
}

unsafe fn begin_panel(panel: Id, completion: Id, selection: &CStr) {
    let window = state_window();
    if window.is_null() {
        unsafe { complete_selection(completion, NIL) };
        return;
    }
    unsafe {
        let mut block = PanelBlock {
            header: BlockHeader {
                isa: (&raw const _NSConcreteStackBlock).cast(),
                flags: (1 << 25) | (1 << 30),
                reserved: 0,
                invoke: panel_finished as *const c_void,
                descriptor: (&raw const PANEL_DESCRIPTOR).cast(),
            },
            panel,
            completion,
            selection: sel_registerName(selection.as_ptr()),
        };
        send!(panel, c"beginSheetModalForWindow:completionHandler:" => (),
            window => Id, (&mut block as *mut PanelBlock).cast() => Id);
    }
}

extern "C" fn open_panel(_: Id, _: Sel, _: Id, parameters: Id, _: Id, completion: Id) {
    unsafe {
        let panel = send!(class(c"NSOpenPanel"), c"openPanel" => Id);
        let multiple = send!(parameters, c"allowsMultipleSelection" => ObjcBool);
        let directories = send!(parameters, c"allowsDirectories" => ObjcBool);
        send!(panel, c"setAllowsMultipleSelection:" => (), multiple => ObjcBool);
        send!(panel, c"setCanChooseDirectories:" => (), directories => ObjcBool);
        send!(panel, c"setCanChooseFiles:" => (), objc_bool(true) => ObjcBool);
        begin_panel(panel, completion, c"URLs");
    }
}

extern "C" fn save_panel(_: Id, _: Sel, _: Id, _: Id, filename: Id, completion: Id) {
    unsafe {
        let panel = send!(class(c"NSSavePanel"), c"savePanel" => Id);
        send!(panel, c"setNameFieldStringValue:" => (), filename => Id);
        begin_panel(panel, completion, c"URL");
    }
}

extern "C" fn decide_navigation(_: Id, _: Sel, _: Id, action: Id, completion: Id) {
    unsafe {
        if yes(send!(action, c"shouldPerformDownload" => ObjcBool)) {
            complete_policy(completion, NAVIGATION_DOWNLOAD);
            return;
        }
        if send!(action, c"navigationType" => isize) == LINK_ACTIVATED {
            let request = send!(action, c"request" => Id);
            let url = send!(request, c"URL" => Id);
            let host = send!(url, c"host" => Id);
            if !host.is_null()
                && !yes(send!(host, c"isEqualToString:" => ObjcBool, string("127.0.0.1") => Id))
                && !yes(send!(host, c"isEqualToString:" => ObjcBool, string("localhost") => Id))
            {
                let workspace = send!(class(c"NSWorkspace"), c"sharedWorkspace" => Id);
                send!(workspace, c"openURL:" => ObjcBool, url => Id);
                complete_policy(completion, NAVIGATION_CANCEL);
                return;
            }
        }
        complete_policy(completion, NAVIGATION_ALLOW);
    }
}

extern "C" fn decide_response(_: Id, _: Sel, _: Id, navigation_response: Id, completion: Id) {
    unsafe {
        let response = send!(navigation_response, c"response" => Id);
        let attachment = if yes(
            send!(response, c"isKindOfClass:" => ObjcBool, class(c"NSHTTPURLResponse") => Id),
        ) {
            let header = send!(
                response, c"valueForHTTPHeaderField:" => Id, string("Content-Disposition") => Id
            );
            let lowercase = send!(header, c"lowercaseString" => Id);
            yes(send!(lowercase, c"containsString:" => ObjcBool, string("attachment") => Id))
        } else {
            false
        };
        let can_show = yes(send!(navigation_response, c"canShowMIMEType" => ObjcBool));
        let policy = if attachment || !can_show {
            NAVIGATION_DOWNLOAD
        } else {
            NAVIGATION_ALLOW
        };
        complete_policy(completion, policy);
    }
}

extern "C" fn became_download(delegate: Id, _: Sel, _: Id, _: Id, download: Id) {
    unsafe { send!(download, c"setDelegate:" => (), delegate => Id) }
}

unsafe fn delegate_class() -> io::Result<Id> {
    unsafe {
        let class_name = c"HermeneueinRustDesktopDelegate";
        let delegate = objc_allocateClassPair(class(c"NSObject"), class_name.as_ptr(), 0);
        if delegate.is_null() {
            return Err(io::Error::other("Could not create the macOS application delegate."));
        }

        let protocols = [
            c"NSApplicationDelegate",
            c"WKUIDelegate",
            c"WKNavigationDelegate",
            c"WKDownloadDelegate",
        ];
        for name in protocols {
            let protocol = objc_getProtocol(name.as_ptr());
            // WebKit discovers these delegate methods by selector. Some SDK
            // protocols have no registered runtime object until first use.
            if !protocol.is_null() {
                class_addProtocol(delegate, protocol);
            }
        }

        #[cfg(target_arch = "aarch64")]
        let bool_callback = c"B@:@";
        #[cfg(target_arch = "x86_64")]
        let bool_callback = c"c@:@";
        let methods = [
            (
                c"applicationDidFinishLaunching:",
                did_finish_launching as *const (),
                c"v@:@",
            ),
            (
                c"applicationShouldTerminateAfterLastWindowClosed:",
                last_window_closed as *const (),
                bool_callback,
            ),
            (
                c"applicationWillTerminate:",
                will_terminate as *const (),
                c"v@:@",
            ),
            (c"backendTimer:", backend_timer as *const (), c"v@:@"),
            (
                c"webView:runOpenPanelWithParameters:initiatedByFrame:completionHandler:",
                open_panel as *const (),
                c"v@:@@@@?",
            ),
            (
                c"webView:decidePolicyForNavigationAction:decisionHandler:",
                decide_navigation as *const (),
                c"v@:@@@?",
            ),
            (
                c"webView:decidePolicyForNavigationResponse:decisionHandler:",
                decide_response as *const (),
                c"v@:@@@?",
            ),
            (
                c"webView:navigationAction:didBecomeDownload:",
                became_download as *const (),
                c"v@:@@@",
            ),
            (
                c"webView:navigationResponse:didBecomeDownload:",
                became_download as *const (),
                c"v@:@@@",
            ),
            (
                c"download:decideDestinationUsingResponse:suggestedFilename:completionHandler:",
                save_panel as *const (),
                c"v@:@@@@?",
            ),
        ];
        for (name, method, types) in methods {
            let selector = sel_registerName(name.as_ptr());
            let implementation: Imp = std::mem::transmute(method);
            if !yes(class_addMethod(
                delegate,
                selector,
                implementation,
                types.as_ptr(),
            )) {
                objc_disposeClassPair(delegate);
                return Err(io::Error::other(format!(
                    "Could not register the macOS callback {}",
                    name.to_string_lossy()
                )));
            }
        }
        objc_registerClassPair(delegate);
        Ok(delegate)
    }
}

pub fn run(paths: &RuntimePaths) -> Result<(), Box<dyn std::error::Error>> {
    // AppKit ownership and the run loop are confined to the calling main thread.
    unsafe {
        if !yes(send!(class(c"NSThread"), c"isMainThread" => ObjcBool)) {
            return Err(io::Error::other("The window must open on the main thread.").into());
        }
        let _pool = AutoreleasePool(send!(class(c"NSAutoreleasePool"), c"new" => Id));
        let app_class = (&raw const NS_APPLICATION_CLASS).cast_mut().cast();
        let app = send!(app_class, c"sharedApplication" => Id);
        let delegate = Owned(send!(delegate_class()?, c"new" => Id));
        let (backend, failure) = match Backend::start(paths, false) {
            Ok(backend) => (Some(backend), None),
            Err(error) => (
                None,
                Some(startup_error(
                    &format!("Could not start the background service ({error})."),
                    &paths.log,
                )),
            ),
        };
        STATE.with_borrow_mut(|slot| {
            *slot = Some(State {
                backend,
                failure,
                log: paths.log.clone(),
                deadline: Instant::now() + Duration::from_secs(300),
                window: None,
                web_view: NIL,
                startup_label: NIL,
            });
        });
        send!(app, c"setActivationPolicy:" => ObjcBool, ACTIVATION_POLICY_REGULAR => isize);
        send!(app, c"setDelegate:" => (), delegate.0 => Id);
        send!(app, c"run" => ());
        send!(app, c"setDelegate:" => (), NIL => Id);
        drop(STATE.with_borrow_mut(Option::take));
    }
    Ok(())
}
