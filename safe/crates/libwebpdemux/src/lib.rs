#![no_std]

use webp_abi::{
    WebPAnimDecoder, WebPAnimDecoderOptions, WebPAnimInfo, WebPChunkIterator, WebPData,
    WebPDemuxState, WebPDemuxer, WebPFormatFeature, WebPIterator,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // SAFETY: aborting is the only valid panic strategy for these no_std cdylibs.
    unsafe { libc::abort() }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub extern "C" fn WebPGetDemuxVersion() -> i32 {
    unsafe { webp_core::demux::demux::WebPGetDemuxVersion() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxInternal(
    data: *const WebPData,
    allow_partial: i32,
    state: *mut WebPDemuxState,
    version: i32,
) -> *mut WebPDemuxer {
    unsafe { webp_core::demux::demux::WebPDemuxInternal(data, allow_partial, state, version) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxDelete(dmux: *mut WebPDemuxer) {
    unsafe { webp_core::demux::demux::WebPDemuxDelete(dmux) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxGetI(
    dmux: *const WebPDemuxer,
    feature: WebPFormatFeature,
) -> u32 {
    unsafe { webp_core::demux::demux::WebPDemuxGetI(dmux, feature) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxGetFrame(
    dmux: *const WebPDemuxer,
    frame_number: i32,
    iter: *mut WebPIterator,
) -> i32 {
    unsafe { webp_core::demux::demux::WebPDemuxGetFrame(dmux, frame_number, iter) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxNextFrame(iter: *mut WebPIterator) -> i32 {
    unsafe { webp_core::demux::demux::WebPDemuxNextFrame(iter) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxPrevFrame(iter: *mut WebPIterator) -> i32 {
    unsafe { webp_core::demux::demux::WebPDemuxPrevFrame(iter) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxReleaseIterator(iter: *mut WebPIterator) {
    unsafe { webp_core::demux::demux::WebPDemuxReleaseIterator(iter) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxGetChunk(
    dmux: *const WebPDemuxer,
    fourcc: *const i8,
    chunk_number: i32,
    iter: *mut WebPChunkIterator,
) -> i32 {
    unsafe { webp_core::demux::demux::WebPDemuxGetChunk(dmux, fourcc, chunk_number, iter) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxNextChunk(iter: *mut WebPChunkIterator) -> i32 {
    unsafe { webp_core::demux::demux::WebPDemuxNextChunk(iter) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxPrevChunk(iter: *mut WebPChunkIterator) -> i32 {
    unsafe { webp_core::demux::demux::WebPDemuxPrevChunk(iter) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPDemuxReleaseChunkIterator(iter: *mut WebPChunkIterator) {
    unsafe { webp_core::demux::demux::WebPDemuxReleaseChunkIterator(iter) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPAnimDecoderOptionsInitInternal(
    dec_options: *mut WebPAnimDecoderOptions,
    abi_version: i32,
) -> i32 {
    unsafe {
        webp_core::demux::anim_decode::WebPAnimDecoderOptionsInitInternal(dec_options, abi_version)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPAnimDecoderNewInternal(
    webp_data: *const WebPData,
    dec_options: *const WebPAnimDecoderOptions,
    abi_version: i32,
) -> *mut WebPAnimDecoder {
    unsafe {
        webp_core::demux::anim_decode::WebPAnimDecoderNewInternal(
            webp_data,
            dec_options,
            abi_version,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPAnimDecoderGetInfo(
    dec: *const WebPAnimDecoder,
    info: *mut WebPAnimInfo,
) -> i32 {
    unsafe { webp_core::demux::anim_decode::WebPAnimDecoderGetInfo(dec, info) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPAnimDecoderGetNext(
    dec: *mut WebPAnimDecoder,
    buf_ptr: *mut *mut u8,
    timestamp_ptr: *mut i32,
) -> i32 {
    unsafe { webp_core::demux::anim_decode::WebPAnimDecoderGetNext(dec, buf_ptr, timestamp_ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPAnimDecoderHasMoreFrames(dec: *const WebPAnimDecoder) -> i32 {
    unsafe { webp_core::demux::anim_decode::WebPAnimDecoderHasMoreFrames(dec) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPAnimDecoderReset(dec: *mut WebPAnimDecoder) {
    unsafe { webp_core::demux::anim_decode::WebPAnimDecoderReset(dec) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPAnimDecoderGetDemuxer(
    dec: *const WebPAnimDecoder,
) -> *const WebPDemuxer {
    unsafe { webp_core::demux::anim_decode::WebPAnimDecoderGetDemuxer(dec) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WebPAnimDecoderDelete(dec: *mut WebPAnimDecoder) {
    unsafe { webp_core::demux::anim_decode::WebPAnimDecoderDelete(dec) }
}
