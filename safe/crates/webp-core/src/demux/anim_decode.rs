use crate::checked::check_size_overflow;
use crate::decode::webp_dec::{
    MODE_bgrA, MODE_rgbA, WebPBitstreamFeatures, WebPDecode, WebPDecoderConfig,
    WebPInitDecoderConfigInternal, MODE_BGRA, MODE_RGBA, VP8_STATUS_OK,
};
use crate::demux::demux::{
    WebPDemuxDelete, WebPDemuxGetFrame, WebPDemuxGetI, WebPDemuxInternal, WebPDemuxReleaseIterator,
};
use crate::rust_alloc::boxed::Box;
use crate::{webp_safe_calloc, webp_safe_free};
use core::{mem, ptr, slice};
use webp_abi::{
    MODE_bgrA as ABI_MODE_bgrA, MODE_rgbA as ABI_MODE_rgbA, WebPAnimDecoder,
    WebPAnimDecoderOptions, WebPAnimInfo, WebPData, WebPDemuxer, WebPFormatFeature, WebPIterator,
    MODE_BGRA as ABI_MODE_BGRA, MODE_RGBA as ABI_MODE_RGBA, WEBP_ABI_IS_INCOMPATIBLE,
    WEBP_DECODER_ABI_VERSION, WEBP_DEMUX_ABI_VERSION, WEBP_MUX_BLEND, WEBP_MUX_DISPOSE_BACKGROUND,
    WEBP_MUX_DISPOSE_NONE, WEBP_MUX_NO_BLEND,
};

const NUM_CHANNELS: u32 = 4;

#[cfg(target_endian = "big")]
#[inline]
const fn channel_shift(index: u32) -> u32 {
    24 - index * 8
}

#[cfg(not(target_endian = "big"))]
#[inline]
const fn channel_shift(index: u32) -> u32 {
    index * 8
}

type BlendRowFunc = fn(*mut u32, *const u32, i32);

struct AnimDecoder {
    demux: *mut WebPDemuxer,
    config: WebPDecoderConfig,
    blend_func: BlendRowFunc,
    info: WebPAnimInfo,
    curr_frame: *mut u8,
    prev_frame_disposed: *mut u8,
    prev_frame_timestamp: i32,
    prev_iter: WebPIterator,
    prev_frame_was_keyframe: i32,
    next_frame: i32,
}

#[inline]
unsafe fn decoder_ref<'a>(dec: *const WebPAnimDecoder) -> Option<&'a AnimDecoder> {
    (!dec.is_null()).then(|| unsafe { &*dec.cast::<AnimDecoder>() })
}

#[inline]
unsafe fn decoder_mut<'a>(dec: *mut WebPAnimDecoder) -> Option<&'a mut AnimDecoder> {
    (!dec.is_null()).then(|| unsafe { &mut *dec.cast::<AnimDecoder>() })
}

fn default_decoder_options() -> WebPAnimDecoderOptions {
    WebPAnimDecoderOptions {
        color_mode: ABI_MODE_RGBA,
        use_threads: 0,
        padding: [0; 7],
    }
}

pub unsafe fn WebPAnimDecoderOptionsInitInternal(
    dec_options: *mut WebPAnimDecoderOptions,
    abi_version: i32,
) -> i32 {
    if dec_options.is_null() || WEBP_ABI_IS_INCOMPATIBLE(abi_version, WEBP_DEMUX_ABI_VERSION) {
        return 0;
    }
    // SAFETY: caller provides writable storage for the options struct.
    unsafe { *dec_options = default_decoder_options() };
    1
}

fn apply_decoder_options(dec_options: &WebPAnimDecoderOptions, dec: &mut AnimDecoder) -> bool {
    let mode = dec_options.color_mode;
    let core_mode = if mode == ABI_MODE_RGBA {
        MODE_RGBA
    } else if mode == ABI_MODE_BGRA {
        MODE_BGRA
    } else if mode == ABI_MODE_rgbA {
        MODE_rgbA
    } else if mode == ABI_MODE_bgrA {
        MODE_bgrA
    } else {
        return false;
    };

    dec.blend_func = if mode == ABI_MODE_RGBA || mode == ABI_MODE_BGRA {
        blend_pixel_row_non_premult
    } else {
        blend_pixel_row_premult
    };
    if unsafe { WebPInitDecoderConfigInternal(&mut dec.config, WEBP_DECODER_ABI_VERSION) } == 0 {
        return false;
    }
    dec.config.output.colorspace = core_mode;
    dec.config.output.is_external_memory = 1;
    dec.config.options.use_threads = dec_options.use_threads;
    true
}

fn alloc_canvas(width: u32, height: u32) -> *mut u8 {
    let Some(stride) = (width as u64).checked_mul(NUM_CHANNELS as u64) else {
        return ptr::null_mut();
    };
    if height == 0 {
        return ptr::null_mut();
    }
    webp_safe_calloc(stride, height as usize).cast()
}

fn cleanup_decoder(dec: &mut AnimDecoder) {
    unsafe { WebPDemuxReleaseIterator(&mut dec.prev_iter) };
    if !dec.demux.is_null() {
        unsafe { WebPDemuxDelete(dec.demux) };
        dec.demux = ptr::null_mut();
    }
    if !dec.curr_frame.is_null() {
        webp_safe_free(dec.curr_frame.cast());
        dec.curr_frame = ptr::null_mut();
    }
    if !dec.prev_frame_disposed.is_null() {
        webp_safe_free(dec.prev_frame_disposed.cast());
        dec.prev_frame_disposed = ptr::null_mut();
    }
}

pub unsafe fn WebPAnimDecoderNewInternal(
    webp_data: *const WebPData,
    dec_options: *const WebPAnimDecoderOptions,
    abi_version: i32,
) -> *mut WebPAnimDecoder {
    if webp_data.is_null() || WEBP_ABI_IS_INCOMPATIBLE(abi_version, WEBP_DEMUX_ABI_VERSION) {
        return ptr::null_mut();
    }
    let webp_data = unsafe { &*webp_data };

    let mut features = WebPBitstreamFeatures {
        width: 0,
        height: 0,
        has_alpha: 0,
        has_animation: 0,
        format: 0,
        pad: [0; 5],
    };
    if unsafe {
        crate::decode::webp_dec::WebPGetFeaturesInternal(
            webp_data.bytes,
            webp_data.size,
            &mut features,
            WEBP_DECODER_ABI_VERSION,
        )
    } != VP8_STATUS_OK
    {
        return ptr::null_mut();
    }

    let mut dec = Box::new(AnimDecoder {
        demux: ptr::null_mut(),
        config: unsafe { mem::zeroed() },
        blend_func: blend_pixel_row_non_premult,
        info: WebPAnimInfo::default(),
        curr_frame: ptr::null_mut(),
        prev_frame_disposed: ptr::null_mut(),
        prev_frame_timestamp: 0,
        prev_iter: WebPIterator::default(),
        prev_frame_was_keyframe: 0,
        next_frame: 1,
    });

    let options = if dec_options.is_null() {
        default_decoder_options()
    } else {
        unsafe { *dec_options }
    };
    if !apply_decoder_options(&options, &mut dec) {
        cleanup_decoder(&mut dec);
        return ptr::null_mut();
    }

    dec.demux = unsafe { WebPDemuxInternal(webp_data, 0, ptr::null_mut(), WEBP_DEMUX_ABI_VERSION) };
    if dec.demux.is_null() {
        cleanup_decoder(&mut dec);
        return ptr::null_mut();
    }

    dec.info.canvas_width =
        unsafe { WebPDemuxGetI(dec.demux, WebPFormatFeature::WEBP_FF_CANVAS_WIDTH) };
    dec.info.canvas_height =
        unsafe { WebPDemuxGetI(dec.demux, WebPFormatFeature::WEBP_FF_CANVAS_HEIGHT) };
    dec.info.loop_count =
        unsafe { WebPDemuxGetI(dec.demux, WebPFormatFeature::WEBP_FF_LOOP_COUNT) };
    dec.info.bgcolor =
        unsafe { WebPDemuxGetI(dec.demux, WebPFormatFeature::WEBP_FF_BACKGROUND_COLOR) };
    dec.info.frame_count =
        unsafe { WebPDemuxGetI(dec.demux, WebPFormatFeature::WEBP_FF_FRAME_COUNT) };

    dec.curr_frame = alloc_canvas(dec.info.canvas_width, dec.info.canvas_height);
    if dec.curr_frame.is_null() {
        cleanup_decoder(&mut dec);
        return ptr::null_mut();
    }
    dec.prev_frame_disposed = alloc_canvas(dec.info.canvas_width, dec.info.canvas_height);
    if dec.prev_frame_disposed.is_null() {
        cleanup_decoder(&mut dec);
        return ptr::null_mut();
    }

    dec.prev_frame_timestamp = 0;
    dec.prev_iter = WebPIterator::default();
    dec.prev_frame_was_keyframe = 0;
    dec.next_frame = 1;
    Box::into_raw(dec).cast()
}

pub unsafe fn WebPAnimDecoderGetInfo(dec: *const WebPAnimDecoder, info: *mut WebPAnimInfo) -> i32 {
    let Some(dec) = (unsafe { decoder_ref(dec) }) else {
        return 0;
    };
    if info.is_null() {
        return 0;
    }
    // SAFETY: `info` is a caller-provided writable output pointer.
    unsafe { *info = dec.info };
    1
}

#[inline]
fn is_full_frame(width: i32, height: i32, canvas_width: i32, canvas_height: i32) -> bool {
    width == canvas_width && height == canvas_height
}

fn zero_fill_canvas(buf: *mut u8, canvas_width: u32, canvas_height: u32) -> bool {
    let size = (canvas_width as u64)
        .saturating_mul(canvas_height as u64)
        .saturating_mul(NUM_CHANNELS as u64);
    if !check_size_overflow(size) {
        return false;
    }
    // SAFETY: `buf` points to a canvas allocation of `size` bytes.
    unsafe { ptr::write_bytes(buf, 0, size as usize) };
    true
}

fn zero_fill_frame_rect(
    buf: *mut u8,
    buf_stride: i32,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
) {
    let mut row = unsafe {
        buf.add(
            (y_offset as usize)
                .saturating_mul(buf_stride as usize)
                .saturating_add((x_offset as usize).saturating_mul(NUM_CHANNELS as usize)),
        )
    };
    for _ in 0..height {
        // SAFETY: row lies within the caller-provided canvas, and width is already validated by demux.
        unsafe {
            ptr::write_bytes(
                row,
                0,
                (width as usize).saturating_mul(NUM_CHANNELS as usize),
            )
        };
        row = unsafe { row.add(buf_stride as usize) };
    }
}

fn copy_canvas(src: *const u8, dst: *mut u8, width: u32, height: u32) -> bool {
    let size = (width as u64)
        .saturating_mul(height as u64)
        .saturating_mul(NUM_CHANNELS as u64);
    if !check_size_overflow(size) {
        return false;
    }
    // SAFETY: src and dst point to non-overlapping canvas buffers of `size` bytes.
    unsafe { ptr::copy_nonoverlapping(src, dst, size as usize) };
    true
}

fn is_key_frame(
    curr: &WebPIterator,
    prev: &WebPIterator,
    prev_frame_was_keyframe: bool,
    canvas_width: i32,
    canvas_height: i32,
) -> bool {
    if curr.frame_num == 1 {
        true
    } else if (curr.has_alpha == 0 || curr.blend_method == WEBP_MUX_NO_BLEND)
        && is_full_frame(curr.width, curr.height, canvas_width, canvas_height)
    {
        true
    } else {
        prev.dispose_method == WEBP_MUX_DISPOSE_BACKGROUND
            && (is_full_frame(prev.width, prev.height, canvas_width, canvas_height)
                || prev_frame_was_keyframe)
    }
}

#[inline]
fn blend_channel_non_premult(
    src: u32,
    src_a: u8,
    dst: u32,
    dst_a: u8,
    scale: u32,
    shift: u32,
) -> u8 {
    let src_channel = ((src >> shift) & 0xff) as u8;
    let dst_channel = ((dst >> shift) & 0xff) as u8;
    let blend_unscaled =
        u32::from(src_channel) * u32::from(src_a) + u32::from(dst_channel) * u32::from(dst_a);
    ((blend_unscaled * scale) >> channel_shift(3)) as u8
}

#[inline]
fn blend_pixel_non_premult(src: u32, dst: u32) -> u32 {
    let src_a = ((src >> channel_shift(3)) & 0xff) as u8;
    if src_a == 0 {
        return dst;
    }

    let dst_a = ((dst >> channel_shift(3)) & 0xff) as u8;
    let dst_factor_a = ((u32::from(dst_a) * (256 - u32::from(src_a))) >> 8) as u8;
    let blend_a = src_a.wrapping_add(dst_factor_a);
    let scale = (1_u32 << 24) / u32::from(blend_a);
    let blend_r = blend_channel_non_premult(src, src_a, dst, dst_factor_a, scale, channel_shift(0));
    let blend_g = blend_channel_non_premult(src, src_a, dst, dst_factor_a, scale, channel_shift(1));
    let blend_b = blend_channel_non_premult(src, src_a, dst, dst_factor_a, scale, channel_shift(2));

    (u32::from(blend_r) << channel_shift(0))
        | (u32::from(blend_g) << channel_shift(1))
        | (u32::from(blend_b) << channel_shift(2))
        | (u32::from(blend_a) << channel_shift(3))
}

fn blend_pixel_row_non_premult(src: *mut u32, dst: *const u32, num_pixels: i32) {
    if num_pixels <= 0 {
        return;
    }
    // SAFETY: callers provide row pointers with at least `num_pixels` pixels.
    let src = unsafe { slice::from_raw_parts_mut(src, num_pixels as usize) };
    let dst = unsafe { slice::from_raw_parts(dst, num_pixels as usize) };
    for (src_pixel, dst_pixel) in src.iter_mut().zip(dst.iter()) {
        let src_alpha = ((*src_pixel >> channel_shift(3)) & 0xff) as u8;
        if src_alpha != 0xff {
            *src_pixel = blend_pixel_non_premult(*src_pixel, *dst_pixel);
        }
    }
}

#[inline]
fn channelwise_multiply(pixel: u32, scale: u32) -> u32 {
    let mask = 0x00ff_00ff;
    let rb = ((pixel & mask) * scale) >> 8;
    let ag = ((pixel >> 8) & mask) * scale;
    (rb & mask) | (ag & !mask)
}

#[inline]
fn blend_pixel_premult(src: u32, dst: u32) -> u32 {
    let src_a = ((src >> channel_shift(3)) & 0xff) as u8;
    src.wrapping_add(channelwise_multiply(dst, 256 - u32::from(src_a)))
}

fn blend_pixel_row_premult(src: *mut u32, dst: *const u32, num_pixels: i32) {
    if num_pixels <= 0 {
        return;
    }
    // SAFETY: callers provide row pointers with at least `num_pixels` pixels.
    let src = unsafe { slice::from_raw_parts_mut(src, num_pixels as usize) };
    let dst = unsafe { slice::from_raw_parts(dst, num_pixels as usize) };
    for (src_pixel, dst_pixel) in src.iter_mut().zip(dst.iter()) {
        let src_alpha = ((*src_pixel >> channel_shift(3)) & 0xff) as u8;
        if src_alpha != 0xff {
            *src_pixel = blend_pixel_premult(*src_pixel, *dst_pixel);
        }
    }
}

fn find_blend_range_at_row(
    src: &WebPIterator,
    dst: &WebPIterator,
    canvas_y: i32,
    left1: &mut i32,
    width1: &mut i32,
    left2: &mut i32,
    width2: &mut i32,
) {
    let src_max_x = src.x_offset + src.width;
    let dst_max_x = dst.x_offset + dst.width;
    let dst_max_y = dst.y_offset + dst.height;
    *left1 = -1;
    *width1 = 0;
    *left2 = -1;
    *width2 = 0;

    if canvas_y < dst.y_offset
        || canvas_y >= dst_max_y
        || src.x_offset >= dst_max_x
        || src_max_x <= dst.x_offset
    {
        *left1 = src.x_offset;
        *width1 = src.width;
        return;
    }
    if src.x_offset < dst.x_offset {
        *left1 = src.x_offset;
        *width1 = dst.x_offset - src.x_offset;
    }
    if src_max_x > dst_max_x {
        *left2 = dst_max_x;
        *width2 = src_max_x - dst_max_x;
    }
}

pub unsafe fn WebPAnimDecoderGetNext(
    dec: *mut WebPAnimDecoder,
    buf_ptr: *mut *mut u8,
    timestamp_ptr: *mut i32,
) -> i32 {
    let Some(dec) = (unsafe { decoder_mut(dec) }) else {
        return 0;
    };
    if buf_ptr.is_null() || timestamp_ptr.is_null() {
        return 0;
    }
    if unsafe { WebPAnimDecoderHasMoreFrames(dec as *mut AnimDecoder as *const WebPAnimDecoder) }
        == 0
    {
        return 0;
    }

    let width = dec.info.canvas_width;
    let height = dec.info.canvas_height;
    let mut iter = WebPIterator::default();
    if unsafe { WebPDemuxGetFrame(dec.demux, dec.next_frame, &mut iter) } == 0 {
        return 0;
    }
    let timestamp = dec.prev_frame_timestamp + iter.duration;
    let is_key = is_key_frame(
        &iter,
        &dec.prev_iter,
        dec.prev_frame_was_keyframe != 0,
        width as i32,
        height as i32,
    );

    if is_key {
        if !zero_fill_canvas(dec.curr_frame, width, height) {
            unsafe { WebPDemuxReleaseIterator(&mut iter) };
            return 0;
        }
    } else if !copy_canvas(dec.prev_frame_disposed, dec.curr_frame, width, height) {
        unsafe { WebPDemuxReleaseIterator(&mut iter) };
        return 0;
    }

    let stride = (width as u64).saturating_mul(NUM_CHANNELS as u64);
    let size = (iter.height as u64).saturating_mul(stride);
    let out_offset = (iter.y_offset as u64)
        .saturating_mul(stride)
        .saturating_add((iter.x_offset as u64).saturating_mul(NUM_CHANNELS as u64));
    let total_size = (width as u64)
        .saturating_mul(height as u64)
        .saturating_mul(NUM_CHANNELS as u64);
    if !check_size_overflow(size)
        || !check_size_overflow(out_offset)
        || !check_size_overflow(total_size)
        || out_offset > total_size
        || out_offset.saturating_add(size) > total_size
    {
        unsafe { WebPDemuxReleaseIterator(&mut iter) };
        return 0;
    }

    unsafe {
        let rgba = &mut dec.config.output.u.RGBA;
        rgba.stride = stride as i32;
        rgba.size = size as usize;
        rgba.rgba = dec.curr_frame.add(out_offset as usize);
    }
    if unsafe { WebPDecode(iter.fragment.bytes, iter.fragment.size, &mut dec.config) }
        != VP8_STATUS_OK
    {
        unsafe { WebPDemuxReleaseIterator(&mut iter) };
        return 0;
    }

    if iter.frame_num > 1 && iter.blend_method == WEBP_MUX_BLEND && !is_key {
        if dec.prev_iter.dispose_method == WEBP_MUX_DISPOSE_NONE {
            for y in 0..iter.height {
                let offset = ((iter.y_offset + y) as usize)
                    .saturating_mul(width as usize)
                    .saturating_add(iter.x_offset as usize);
                (dec.blend_func)(
                    unsafe { dec.curr_frame.cast::<u32>().add(offset) },
                    unsafe { dec.prev_frame_disposed.cast::<u32>().add(offset) },
                    iter.width,
                );
            }
        } else {
            for y in 0..iter.height {
                let canvas_y = iter.y_offset + y;
                let (mut left1, mut width1, mut left2, mut width2) = (-1, 0, -1, 0);
                find_blend_range_at_row(
                    &iter,
                    &dec.prev_iter,
                    canvas_y,
                    &mut left1,
                    &mut width1,
                    &mut left2,
                    &mut width2,
                );
                if width1 > 0 {
                    let offset1 = (canvas_y as usize)
                        .saturating_mul(width as usize)
                        .saturating_add(left1 as usize);
                    (dec.blend_func)(
                        unsafe { dec.curr_frame.cast::<u32>().add(offset1) },
                        unsafe { dec.prev_frame_disposed.cast::<u32>().add(offset1) },
                        width1,
                    );
                }
                if width2 > 0 {
                    let offset2 = (canvas_y as usize)
                        .saturating_mul(width as usize)
                        .saturating_add(left2 as usize);
                    (dec.blend_func)(
                        unsafe { dec.curr_frame.cast::<u32>().add(offset2) },
                        unsafe { dec.prev_frame_disposed.cast::<u32>().add(offset2) },
                        width2,
                    );
                }
            }
        }
    }

    dec.prev_frame_timestamp = timestamp;
    unsafe { WebPDemuxReleaseIterator(&mut dec.prev_iter) };
    dec.prev_iter = iter;
    dec.prev_frame_was_keyframe = i32::from(is_key);
    if !copy_canvas(dec.curr_frame, dec.prev_frame_disposed, width, height) {
        return 0;
    }
    if dec.prev_iter.dispose_method == WEBP_MUX_DISPOSE_BACKGROUND {
        zero_fill_frame_rect(
            dec.prev_frame_disposed,
            (width * NUM_CHANNELS) as i32,
            dec.prev_iter.x_offset,
            dec.prev_iter.y_offset,
            dec.prev_iter.width,
            dec.prev_iter.height,
        );
    }
    dec.next_frame += 1;

    // SAFETY: caller provided valid output pointers.
    unsafe {
        *buf_ptr = dec.curr_frame;
        *timestamp_ptr = timestamp;
    }
    1
}

pub unsafe fn WebPAnimDecoderHasMoreFrames(dec: *const WebPAnimDecoder) -> i32 {
    let Some(dec) = (unsafe { decoder_ref(dec) }) else {
        return 0;
    };
    i32::from(dec.next_frame <= dec.info.frame_count as i32)
}

pub unsafe fn WebPAnimDecoderReset(dec: *mut WebPAnimDecoder) {
    let Some(dec) = (unsafe { decoder_mut(dec) }) else {
        return;
    };
    dec.prev_frame_timestamp = 0;
    unsafe { WebPDemuxReleaseIterator(&mut dec.prev_iter) };
    dec.prev_iter = WebPIterator::default();
    dec.prev_frame_was_keyframe = 0;
    dec.next_frame = 1;
}

pub unsafe fn WebPAnimDecoderGetDemuxer(dec: *const WebPAnimDecoder) -> *const WebPDemuxer {
    let Some(dec) = (unsafe { decoder_ref(dec) }) else {
        return ptr::null();
    };
    dec.demux
}

pub unsafe fn WebPAnimDecoderDelete(dec: *mut WebPAnimDecoder) {
    if dec.is_null() {
        return;
    }
    // SAFETY: `dec` was allocated with `Box::into_raw` in this module.
    let mut dec = unsafe { Box::from_raw(dec.cast::<AnimDecoder>()) };
    cleanup_decoder(&mut dec);
}
