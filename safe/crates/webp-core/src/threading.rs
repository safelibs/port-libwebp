use core::ffi::c_void;
use core::ptr;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct WebPWorkerStatus(pub i32);

pub const NOT_OK: WebPWorkerStatus = WebPWorkerStatus(0);
pub const OK: WebPWorkerStatus = WebPWorkerStatus(1);
pub const WORK: WebPWorkerStatus = WebPWorkerStatus(2);

pub type WebPWorkerHook =
    Option<unsafe extern "C" fn(data1: *mut c_void, data2: *mut c_void) -> i32>;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WebPWorker {
    pub impl_: *mut c_void,
    pub status_: WebPWorkerStatus,
    pub hook: WebPWorkerHook,
    pub data1: *mut c_void,
    pub data2: *mut c_void,
    pub had_error: i32,
}

pub type WebPWorkerInit = Option<unsafe extern "C" fn(worker: *mut WebPWorker)>;
pub type WebPWorkerReset = Option<unsafe extern "C" fn(worker: *mut WebPWorker) -> i32>;
pub type WebPWorkerSync = Option<unsafe extern "C" fn(worker: *mut WebPWorker) -> i32>;
pub type WebPWorkerLaunch = Option<unsafe extern "C" fn(worker: *mut WebPWorker)>;
pub type WebPWorkerExecute = Option<unsafe extern "C" fn(worker: *mut WebPWorker)>;
pub type WebPWorkerEnd = Option<unsafe extern "C" fn(worker: *mut WebPWorker)>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WebPWorkerInterface {
    pub Init: WebPWorkerInit,
    pub Reset: WebPWorkerReset,
    pub Sync: WebPWorkerSync,
    pub Launch: WebPWorkerLaunch,
    pub Execute: WebPWorkerExecute,
    pub End: WebPWorkerEnd,
}

unsafe extern "C" fn default_init(worker: *mut WebPWorker) {
    if worker.is_null() {
        return;
    }
    // SAFETY: the null check above guarantees `worker` is valid for a plain reset.
    unsafe {
        ptr::write(
            worker,
            WebPWorker {
                impl_: ptr::null_mut(),
                status_: NOT_OK,
                hook: None,
                data1: ptr::null_mut(),
                data2: ptr::null_mut(),
                had_error: 0,
            },
        );
    }
}

unsafe extern "C" fn default_sync(worker: *mut WebPWorker) -> i32 {
    if worker.is_null() {
        return 0;
    }
    // SAFETY: the null check above guarantees the pointed worker can be read.
    unsafe { ((*worker).had_error == 0) as i32 }
}

unsafe extern "C" fn default_reset(worker: *mut WebPWorker) -> i32 {
    if worker.is_null() {
        return 0;
    }
    // SAFETY: the null check above guarantees the pointed worker can be updated in place.
    unsafe {
        (*worker).had_error = 0;
        if (*worker).status_.0 < OK.0 {
            (*worker).status_ = OK;
        } else if (*worker).status_.0 > OK.0 {
            (*worker).status_ = OK;
        }
    }
    1
}

unsafe extern "C" fn default_execute(worker: *mut WebPWorker) {
    if worker.is_null() {
        return;
    }
    // SAFETY: the hook is an FFI callback supplied by the caller and is invoked only when present.
    unsafe {
        if let Some(hook) = (*worker).hook {
            (*worker).had_error |= (hook((*worker).data1, (*worker).data2) == 0) as i32;
        }
    }
}

unsafe extern "C" fn default_launch(worker: *mut WebPWorker) {
    // SAFETY: launching the default worker is just the synchronous execution path.
    unsafe { default_execute(worker) }
}

unsafe extern "C" fn default_end(worker: *mut WebPWorker) {
    if worker.is_null() {
        return;
    }
    // SAFETY: the null check above guarantees the pointed worker can be updated in place.
    unsafe {
        (*worker).status_ = NOT_OK;
        (*worker).impl_ = ptr::null_mut();
    }
}

const DEFAULT_INTERFACE: WebPWorkerInterface = WebPWorkerInterface {
    Init: Some(default_init),
    Reset: Some(default_reset),
    Sync: Some(default_sync),
    Launch: Some(default_launch),
    Execute: Some(default_execute),
    End: Some(default_end),
};

static mut CURRENT_INTERFACE: WebPWorkerInterface = DEFAULT_INTERFACE;

pub fn webp_set_worker_interface(winterface: *const WebPWorkerInterface) -> i32 {
    if winterface.is_null() {
        return 0;
    }
    // SAFETY: the pointer was checked for null and is copied by value before storing.
    let interface = unsafe { ptr::read(winterface) };
    if interface.Init.is_none()
        || interface.Reset.is_none()
        || interface.Sync.is_none()
        || interface.Launch.is_none()
        || interface.Execute.is_none()
        || interface.End.is_none()
    {
        return 0;
    }
    // SAFETY: this mirrors libwebp's documented non-thread-safe global replacement semantics.
    unsafe {
        CURRENT_INTERFACE = interface;
    }
    1
}

pub fn webp_get_worker_interface() -> *const WebPWorkerInterface {
    ptr::addr_of!(CURRENT_INTERFACE)
}
