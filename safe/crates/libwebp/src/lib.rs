#![no_std]

use webp_core::{default_cpu_info, VP8CPUInfo, WebPWorkerInterface};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // SAFETY: aborting is the only valid panic strategy for these no_std cdylibs.
    unsafe { libc::abort() }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub extern "C" fn WebPSetWorkerInterface(winterface: *const WebPWorkerInterface) -> i32 {
    webp_core::webp_set_worker_interface(winterface)
}

#[unsafe(no_mangle)]
pub extern "C" fn WebPGetWorkerInterface() -> *const WebPWorkerInterface {
    webp_core::webp_get_worker_interface()
}

#[unsafe(no_mangle)]
pub static mut VP8GetCPUInfo: VP8CPUInfo = Some(default_cpu_info);

#[used]
static KEEP_BUFFER_DEC: unsafe extern "C" fn(
    *mut webp_core::decode::buffer_dec::WebPDecBuffer,
    i32,
) -> i32 = webp_core::decode::buffer_dec::WebPInitDecBufferInternal;

#[used]
static KEEP_IDEC_DEC: unsafe extern "C" fn(
    *mut webp_core::decode::idec_dec::WebPDecBuffer,
) -> *mut webp_core::decode::idec_dec::WebPIDecoder = webp_core::decode::idec_dec::WebPINewDecoder;

#[used]
static KEEP_VP8_DEC: unsafe extern "C" fn(*const u8, usize) -> i32 =
    webp_core::decode::vp8_dec::VP8CheckSignature;

#[used]
static KEEP_VP8L_DEC: unsafe extern "C" fn(*const u8, usize) -> i32 =
    webp_core::decode::vp8l_dec::VP8LCheckSignature;

#[used]
static KEEP_WEBP_DEC: unsafe extern "C" fn(
    *const u8,
    usize,
    *mut webp_core::decode::webp_dec::WebPDecoderConfig,
) -> webp_core::decode::webp_dec::VP8StatusCode = webp_core::decode::webp_dec::WebPDecode;
