use core::ptr;

use crate::checked::{checked_add_usize, checked_mul_usize};
use crate::rust_alloc::vec::Vec;
use webp_abi::SharpYuvConversionMatrix;

use super::dsp;
use super::gamma::{gamma_to_linear, linear_to_gamma};

const K_NUM_ITERATIONS: usize = 4;
const YUV_FIX: i32 = 16;
const K_YUV_HALF: i64 = 1_i64 << (YUV_FIX - 1);
const K_MAX_BIT_DEPTH: i32 = 14;

type Fixed = i16;
type FixedY = u16;

#[inline]
fn get_precision_shift(rgb_bit_depth: i32) -> i32 {
    if rgb_bit_depth + 2 <= K_MAX_BIT_DEPTH {
        2
    } else {
        K_MAX_BIT_DEPTH - rgb_bit_depth
    }
}

#[inline]
fn clip_8b(value: i32) -> u8 {
    if (0..=255).contains(&value) {
        value as u8
    } else if value < 0 {
        0
    } else {
        255
    }
}

#[inline]
fn clip(value: i32, max: i32) -> u16 {
    if value < 0 {
        0
    } else if value > max {
        max as u16
    } else {
        value as u16
    }
}

#[inline]
fn clip_bit_depth(value: i32, bit_depth: i32) -> FixedY {
    let max = (1_i32 << bit_depth) - 1;
    if (0..=max).contains(&value) {
        value as FixedY
    } else if value < 0 {
        0
    } else {
        max as FixedY
    }
}

#[inline]
fn rgb_to_gray(r: i64, g: i64, b: i64) -> i32 {
    ((13_933_i64 * r + 46_871_i64 * g + 4_732_i64 * b + K_YUV_HALF) >> YUV_FIX) as i32
}

fn scale_down(a: u16, b: u16, c: u16, d: u16, rgb_bit_depth: i32) -> u32 {
    let bit_depth = rgb_bit_depth + get_precision_shift(rgb_bit_depth);
    let a = gamma_to_linear(a, bit_depth);
    let b = gamma_to_linear(b, bit_depth);
    let c = gamma_to_linear(c, bit_depth);
    let d = gamma_to_linear(d, bit_depth);
    linear_to_gamma(((a + b + c + d + 2) >> 2) as u32, bit_depth) as u32
}

fn update_w(src: &[FixedY], dst: &mut [FixedY], width: usize, rgb_bit_depth: i32) {
    let bit_depth = rgb_bit_depth + get_precision_shift(rgb_bit_depth);
    for i in 0..width {
        let r = gamma_to_linear(src[i], bit_depth) as i64;
        let g = gamma_to_linear(src[width + i], bit_depth) as i64;
        let b = gamma_to_linear(src[2 * width + i], bit_depth) as i64;
        let y = rgb_to_gray(r, g, b);
        dst[i] = linear_to_gamma(y as u32, bit_depth);
    }
}

fn update_chroma(
    src1: &[FixedY],
    src2: &[FixedY],
    dst: &mut [Fixed],
    uv_w: usize,
    rgb_bit_depth: i32,
) {
    for i in 0..uv_w {
        let base = 2 * i;
        let r = scale_down(
            src1[base],
            src1[base + 1],
            src2[base],
            src2[base + 1],
            rgb_bit_depth,
        );
        let g = scale_down(
            src1[2 * uv_w + base],
            src1[2 * uv_w + base + 1],
            src2[2 * uv_w + base],
            src2[2 * uv_w + base + 1],
            rgb_bit_depth,
        );
        let b = scale_down(
            src1[4 * uv_w + base],
            src1[4 * uv_w + base + 1],
            src2[4 * uv_w + base],
            src2[4 * uv_w + base + 1],
            rgb_bit_depth,
        );
        let w = rgb_to_gray(r as i64, g as i64, b as i64);
        dst[i] = (r as i32 - w) as Fixed;
        dst[uv_w + i] = (g as i32 - w) as Fixed;
        dst[2 * uv_w + i] = (b as i32 - w) as Fixed;
    }
}

fn store_gray(rgb: &[FixedY], y: &mut [FixedY], width: usize) {
    for i in 0..width {
        y[i] = rgb_to_gray(
            rgb[i] as i64,
            rgb[width + i] as i64,
            rgb[2 * width + i] as i64,
        ) as FixedY;
    }
}

#[inline]
fn filter2(a: i32, b: i32, w0: i32, bit_depth: i32) -> FixedY {
    clip_bit_depth(((a * 3 + b + 2) >> 2) + w0, bit_depth)
}

#[inline]
fn shift(value: i32, amount: i32) -> i32 {
    if amount >= 0 {
        value << amount
    } else {
        value >> -amount
    }
}

unsafe fn read_sample(ptr: *const u8, bit_depth: i32, index: usize) -> u16 {
    if bit_depth == 8 {
        // SAFETY: the caller bounds-checks `index` against the provided row width.
        unsafe { *ptr.add(index) as u16 }
    } else {
        let offset = index * core::mem::size_of::<u16>();
        // SAFETY: the caller bounds-checks `offset` and unaligned reads mirror the C implementation.
        unsafe { ptr::read_unaligned(ptr.add(offset).cast::<u16>()) }
    }
}

unsafe fn write_yuv_sample(dst: *mut u8, bit_depth: i32, index: usize, value: u16) {
    if bit_depth <= 8 {
        // SAFETY: the caller bounds-checks `index` against the output plane width.
        unsafe { *dst.add(index) = value as u8 };
    } else {
        let offset = index * core::mem::size_of::<u16>();
        // SAFETY: the caller bounds-checks `offset` and unaligned writes mirror the C implementation.
        unsafe { ptr::write_unaligned(dst.add(offset).cast::<u16>(), value) };
    }
}

fn import_one_row(
    r_ptr: *const u8,
    g_ptr: *const u8,
    b_ptr: *const u8,
    rgb_step: i32,
    rgb_bit_depth: i32,
    pic_width: usize,
    dst: &mut [FixedY],
) {
    let step = if rgb_bit_depth > 8 {
        rgb_step as usize / 2
    } else {
        rgb_step as usize
    };
    let width = (pic_width + 1) & !1;
    let shift_amount = get_precision_shift(rgb_bit_depth);
    for i in 0..pic_width {
        let off = i * step;
        // SAFETY: the row buffers are caller-provided and indexed within the padded row width.
        let r = unsafe { read_sample(r_ptr, rgb_bit_depth, off) as i32 };
        // SAFETY: same reasoning as the read above for the green plane.
        let g = unsafe { read_sample(g_ptr, rgb_bit_depth, off) as i32 };
        // SAFETY: same reasoning as the read above for the blue plane.
        let b = unsafe { read_sample(b_ptr, rgb_bit_depth, off) as i32 };
        dst[i] = shift(r, shift_amount) as FixedY;
        dst[width + i] = shift(g, shift_amount) as FixedY;
        dst[2 * width + i] = shift(b, shift_amount) as FixedY;
    }
    if pic_width & 1 == 1 {
        dst[pic_width] = dst[pic_width - 1];
        dst[width + pic_width] = dst[width + pic_width - 1];
        dst[2 * width + pic_width] = dst[2 * width + pic_width - 1];
    }
}

fn interpolate_two_rows(
    best_y: &[FixedY],
    prev_uv: &[Fixed],
    cur_uv: &[Fixed],
    next_uv: &[Fixed],
    width: usize,
    out1: &mut [FixedY],
    out2: &mut [FixedY],
    rgb_bit_depth: i32,
) {
    let uv_w = width >> 1;
    let len = (width - 1) >> 1;
    let bit_depth = rgb_bit_depth + get_precision_shift(rgb_bit_depth);

    for segment in 0..3 {
        let prev = &prev_uv[segment * uv_w..(segment + 1) * uv_w];
        let cur = &cur_uv[segment * uv_w..(segment + 1) * uv_w];
        let next = &next_uv[segment * uv_w..(segment + 1) * uv_w];
        let row1 = &mut out1[segment * width..(segment + 1) * width];
        let row2 = &mut out2[segment * width..(segment + 1) * width];
        let best_row1 = &best_y[..width];
        let best_row2 = &best_y[width..2 * width];

        row1[0] = filter2(
            cur[0] as i32,
            prev[0] as i32,
            best_row1[0] as i32,
            bit_depth,
        );
        row2[0] = filter2(
            cur[0] as i32,
            next[0] as i32,
            best_row2[0] as i32,
            bit_depth,
        );

        if len > 0 {
            dsp::filter_row(
                cur,
                prev,
                &best_row1[1..1 + 2 * len],
                &mut row1[1..1 + 2 * len],
                bit_depth,
            );
            dsp::filter_row(
                cur,
                next,
                &best_row2[1..1 + 2 * len],
                &mut row2[1..1 + 2 * len],
                bit_depth,
            );
        }

        if width & 1 == 0 {
            row1[width - 1] = filter2(
                cur[uv_w - 1] as i32,
                prev[uv_w - 1] as i32,
                best_row1[width - 1] as i32,
                bit_depth,
            );
            row2[width - 1] = filter2(
                cur[uv_w - 1] as i32,
                next[uv_w - 1] as i32,
                best_row2[width - 1] as i32,
                bit_depth,
            );
        }
    }
}

#[inline]
fn rgb_to_yuv_component(r: i32, g: i32, b: i32, coeffs: &[i32; 4], sfix: i32) -> i32 {
    let rounder = 1_i32 << (YUV_FIX + sfix - 1);
    (coeffs[0] * r + coeffs[1] * g + coeffs[2] * b + coeffs[3] + rounder) >> (YUV_FIX + sfix)
}

#[allow(clippy::too_many_arguments)]
fn convert_wrgb_to_yuv(
    best_y: &[FixedY],
    best_uv: &[Fixed],
    y_ptr: *mut u8,
    y_stride: usize,
    u_ptr: *mut u8,
    u_stride: usize,
    v_ptr: *mut u8,
    v_stride: usize,
    rgb_bit_depth: i32,
    yuv_bit_depth: i32,
    width: usize,
    height: usize,
    yuv_matrix: &SharpYuvConversionMatrix,
) -> bool {
    let width_padded = (width + 1) & !1;
    let height_padded = (height + 1) & !1;
    let uv_w = width_padded >> 1;
    let uv_h = height_padded >> 1;
    let sfix = get_precision_shift(rgb_bit_depth);
    let yuv_max = (1_i32 << yuv_bit_depth) - 1;

    for j in 0..height {
        let best_y_row = &best_y[j * width_padded..(j + 1) * width_padded];
        let uv_row_base = (j / 2) * 3 * uv_w;
        let best_uv_row = &best_uv[uv_row_base..uv_row_base + 3 * uv_w];
        // SAFETY: row pointers are advanced within the validated plane bounds.
        let y_row_ptr = unsafe { y_ptr.add(j * y_stride) };
        for i in 0..width {
            let off = i >> 1;
            let w = best_y_row[i] as i32;
            let r = best_uv_row[off] as i32 + w;
            let g = best_uv_row[uv_w + off] as i32 + w;
            let b = best_uv_row[2 * uv_w + off] as i32 + w;
            let y = rgb_to_yuv_component(r, g, b, &yuv_matrix.rgb_to_y, sfix);
            // SAFETY: each row write stays within the caller-provided plane width.
            unsafe {
                write_yuv_sample(
                    y_row_ptr,
                    yuv_bit_depth,
                    i,
                    if yuv_bit_depth <= 8 {
                        clip_8b(y) as u16
                    } else {
                        clip(y, yuv_max)
                    },
                );
            }
        }
    }

    for j in 0..uv_h {
        let uv_row = &best_uv[j * 3 * uv_w..(j + 1) * 3 * uv_w];
        // SAFETY: row pointers are advanced within the validated plane bounds.
        let u_row_ptr = unsafe { u_ptr.add(j * u_stride) };
        // SAFETY: row pointers are advanced within the validated plane bounds.
        let v_row_ptr = unsafe { v_ptr.add(j * v_stride) };
        for i in 0..uv_w {
            let r = uv_row[i] as i32;
            let g = uv_row[uv_w + i] as i32;
            let b = uv_row[2 * uv_w + i] as i32;
            let u = rgb_to_yuv_component(r, g, b, &yuv_matrix.rgb_to_u, sfix);
            let v = rgb_to_yuv_component(r, g, b, &yuv_matrix.rgb_to_v, sfix);
            let u_value = if yuv_bit_depth <= 8 {
                clip_8b(u) as u16
            } else {
                clip(u, yuv_max)
            };
            let v_value = if yuv_bit_depth <= 8 {
                clip_8b(v) as u16
            } else {
                clip(v, yuv_max)
            };
            // SAFETY: each row write stays within the caller-provided chroma width.
            unsafe { write_yuv_sample(u_row_ptr, yuv_bit_depth, i, u_value) };
            // SAFETY: each row write stays within the caller-provided chroma width.
            unsafe { write_yuv_sample(v_row_ptr, yuv_bit_depth, i, v_value) };
        }
    }

    true
}

fn try_zeroed_vec<T: Default + Clone>(len: usize) -> Option<Vec<T>> {
    let mut values = Vec::new();
    if values.try_reserve_exact(len).is_err() {
        return None;
    }
    values.resize(len, T::default());
    Some(values)
}

#[allow(clippy::too_many_arguments)]
fn do_sharp_argb_to_yuv(
    r_ptr: *const u8,
    g_ptr: *const u8,
    b_ptr: *const u8,
    rgb_step: i32,
    rgb_stride: i32,
    rgb_bit_depth: i32,
    y_ptr: *mut u8,
    y_stride: usize,
    u_ptr: *mut u8,
    u_stride: usize,
    v_ptr: *mut u8,
    v_stride: usize,
    yuv_bit_depth: i32,
    width: usize,
    height: usize,
    yuv_matrix: &SharpYuvConversionMatrix,
) -> bool {
    let width_padded = (width + 1) & !1;
    let height_padded = (height + 1) & !1;
    let uv_w = width_padded >> 1;
    let uv_h = height_padded >> 1;
    let diff_y_threshold = (3 * width_padded * height_padded) as u64;

    let Some(tmp_len) = checked_mul_usize(width_padded * 3, 2) else {
        return false;
    };
    let Some(best_y_len) = checked_mul_usize(width_padded, height_padded) else {
        return false;
    };
    let target_y_len = best_y_len;
    let Some(best_rgb_y_len) = checked_mul_usize(width_padded, 2) else {
        return false;
    };
    let Some(best_uv_len) = checked_mul_usize(uv_w * 3, uv_h) else {
        return false;
    };
    let target_uv_len = best_uv_len;
    let best_rgb_uv_len = uv_w * 3;

    let mut tmp_buffer = match try_zeroed_vec::<FixedY>(tmp_len) {
        Some(values) => values,
        None => return false,
    };
    let mut best_y = match try_zeroed_vec::<FixedY>(best_y_len) {
        Some(values) => values,
        None => return false,
    };
    let mut target_y = match try_zeroed_vec::<FixedY>(target_y_len) {
        Some(values) => values,
        None => return false,
    };
    let mut best_rgb_y = match try_zeroed_vec::<FixedY>(best_rgb_y_len) {
        Some(values) => values,
        None => return false,
    };
    let mut best_uv = match try_zeroed_vec::<Fixed>(best_uv_len) {
        Some(values) => values,
        None => return false,
    };
    let mut target_uv = match try_zeroed_vec::<Fixed>(target_uv_len) {
        Some(values) => values,
        None => return false,
    };
    let mut best_rgb_uv = match try_zeroed_vec::<Fixed>(best_rgb_uv_len) {
        Some(values) => values,
        None => return false,
    };

    let mut prev_diff_y_sum = u64::MAX;

    for j in (0..height).step_by(2) {
        let is_last_row = j == height - 1;
        let (src1, src2) = tmp_buffer.split_at_mut(3 * width_padded);
        // SAFETY: source row pointers are offset by whole rows within the caller-provided image.
        let r_row_ptr = unsafe { r_ptr.add(j * rgb_stride as usize) };
        // SAFETY: same reasoning as the red row pointer above.
        let g_row_ptr = unsafe { g_ptr.add(j * rgb_stride as usize) };
        // SAFETY: same reasoning as the blue row pointer above.
        let b_row_ptr = unsafe { b_ptr.add(j * rgb_stride as usize) };
        import_one_row(
            r_row_ptr,
            g_row_ptr,
            b_row_ptr,
            rgb_step,
            rgb_bit_depth,
            width,
            src1,
        );
        if !is_last_row {
            // SAFETY: the next-row offsets stay within the caller-provided image.
            let r_next = unsafe { r_row_ptr.add(rgb_stride as usize) };
            // SAFETY: same reasoning as the red next-row pointer above.
            let g_next = unsafe { g_row_ptr.add(rgb_stride as usize) };
            // SAFETY: same reasoning as the blue next-row pointer above.
            let b_next = unsafe { b_row_ptr.add(rgb_stride as usize) };
            import_one_row(r_next, g_next, b_next, rgb_step, rgb_bit_depth, width, src2);
        } else {
            src2.copy_from_slice(src1);
        }

        let best_y_row = &mut best_y[j * width_padded..j * width_padded + 2 * width_padded];
        store_gray(src1, &mut best_y_row[0..width_padded], width_padded);
        store_gray(
            src2,
            &mut best_y_row[width_padded..2 * width_padded],
            width_padded,
        );

        let target_y_row = &mut target_y[j * width_padded..j * width_padded + 2 * width_padded];
        update_w(
            src1,
            &mut target_y_row[0..width_padded],
            width_padded,
            rgb_bit_depth,
        );
        update_w(
            src2,
            &mut target_y_row[width_padded..2 * width_padded],
            width_padded,
            rgb_bit_depth,
        );
        let uv_row = &mut target_uv[(j / 2) * 3 * uv_w..(j / 2 + 1) * 3 * uv_w];
        update_chroma(src1, src2, uv_row, uv_w, rgb_bit_depth);
        best_uv[(j / 2) * 3 * uv_w..(j / 2 + 1) * 3 * uv_w].copy_from_slice(uv_row);
    }

    for iter in 0..K_NUM_ITERATIONS {
        let mut diff_y_sum = 0_u64;
        let mut prev_uv_offset = 0;
        let mut cur_uv_offset = 0;
        for j in (0..height_padded).step_by(2) {
            let (src1, src2) = tmp_buffer.split_at_mut(3 * width_padded);
            let next_uv_offset = if j < height_padded - 2 {
                cur_uv_offset + 3 * uv_w
            } else {
                cur_uv_offset
            };

            let best_y_rows = &best_y[j * width_padded..j * width_padded + 2 * width_padded];
            let prev_uv_row = &best_uv[prev_uv_offset..prev_uv_offset + 3 * uv_w];
            let cur_uv_row = &best_uv[cur_uv_offset..cur_uv_offset + 3 * uv_w];
            let next_uv_row = &best_uv[next_uv_offset..next_uv_offset + 3 * uv_w];

            interpolate_two_rows(
                best_y_rows,
                prev_uv_row,
                cur_uv_row,
                next_uv_row,
                width_padded,
                src1,
                src2,
                rgb_bit_depth,
            );
            prev_uv_offset = cur_uv_offset;
            cur_uv_offset = next_uv_offset;

            update_w(
                src1,
                &mut best_rgb_y[0..width_padded],
                width_padded,
                rgb_bit_depth,
            );
            update_w(
                src2,
                &mut best_rgb_y[width_padded..2 * width_padded],
                width_padded,
                rgb_bit_depth,
            );
            update_chroma(src1, src2, &mut best_rgb_uv, uv_w, rgb_bit_depth);

            let target_y_rows = &target_y[j * width_padded..j * width_padded + 2 * width_padded];
            let best_y_rows_mut =
                &mut best_y[j * width_padded..j * width_padded + 2 * width_padded];
            diff_y_sum += dsp::update_y(
                target_y_rows,
                &best_rgb_y,
                best_y_rows_mut,
                rgb_bit_depth + get_precision_shift(rgb_bit_depth),
            );
            let target_uv_row = &target_uv[(j / 2) * 3 * uv_w..(j / 2 + 1) * 3 * uv_w];
            let best_uv_row = &mut best_uv[(j / 2) * 3 * uv_w..(j / 2 + 1) * 3 * uv_w];
            dsp::update_rgb(target_uv_row, &best_rgb_uv, best_uv_row);
        }

        if iter > 0 && (diff_y_sum < diff_y_threshold || diff_y_sum > prev_diff_y_sum) {
            break;
        }
        prev_diff_y_sum = diff_y_sum;
    }

    convert_wrgb_to_yuv(
        &best_y,
        &best_uv,
        y_ptr,
        y_stride,
        u_ptr,
        u_stride,
        v_ptr,
        v_stride,
        rgb_bit_depth,
        yuv_bit_depth,
        width,
        height,
        yuv_matrix,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn sharp_yuv_convert(
    r_ptr: *const u8,
    g_ptr: *const u8,
    b_ptr: *const u8,
    rgb_step: i32,
    rgb_stride: i32,
    rgb_bit_depth: i32,
    y_ptr: *mut u8,
    y_stride: i32,
    u_ptr: *mut u8,
    u_stride: i32,
    v_ptr: *mut u8,
    v_stride: i32,
    yuv_bit_depth: i32,
    width: i32,
    height: i32,
    yuv_matrix: &SharpYuvConversionMatrix,
) -> bool {
    if width < 1
        || height < 1
        || width == i32::MAX
        || height == i32::MAX
        || r_ptr.is_null()
        || g_ptr.is_null()
        || b_ptr.is_null()
        || y_ptr.is_null()
        || u_ptr.is_null()
        || v_ptr.is_null()
    {
        return false;
    }
    if !matches!(rgb_bit_depth, 8 | 10 | 12 | 16) || !matches!(yuv_bit_depth, 8 | 10 | 12) {
        return false;
    }
    if rgb_bit_depth > 8 && (rgb_step % 2 != 0 || rgb_stride % 2 != 0) {
        return false;
    }
    if yuv_bit_depth > 8 && (y_stride % 2 != 0 || u_stride % 2 != 0 || v_stride % 2 != 0) {
        return false;
    }

    let rgb_max = (1_i32 << rgb_bit_depth) - 1;
    let rgb_round = 1_i32 << (rgb_bit_depth - 1);
    let yuv_max = (1_i32 << yuv_bit_depth) - 1;
    let sfix = get_precision_shift(rgb_bit_depth);
    let mut scaled_matrix = *yuv_matrix;

    if rgb_bit_depth != yuv_bit_depth {
        for i in 0..3 {
            scaled_matrix.rgb_to_y[i] = (yuv_matrix.rgb_to_y[i] * yuv_max + rgb_round) / rgb_max;
            scaled_matrix.rgb_to_u[i] = (yuv_matrix.rgb_to_u[i] * yuv_max + rgb_round) / rgb_max;
            scaled_matrix.rgb_to_v[i] = (yuv_matrix.rgb_to_v[i] * yuv_max + rgb_round) / rgb_max;
        }
    }

    scaled_matrix.rgb_to_y[3] = shift(yuv_matrix.rgb_to_y[3], sfix);
    scaled_matrix.rgb_to_u[3] = shift(yuv_matrix.rgb_to_u[3], sfix);
    scaled_matrix.rgb_to_v[3] = shift(yuv_matrix.rgb_to_v[3], sfix);

    let Some(y_stride) = usize::try_from(y_stride).ok() else {
        return false;
    };
    let Some(u_stride) = usize::try_from(u_stride).ok() else {
        return false;
    };
    let Some(v_stride) = usize::try_from(v_stride).ok() else {
        return false;
    };
    let Some(width) = usize::try_from(width).ok() else {
        return false;
    };
    let Some(height) = usize::try_from(height).ok() else {
        return false;
    };
    let width_padded = (width + 1) & !1;
    let height_padded = (height + 1) & !1;
    let uv_w = width_padded >> 1;
    let uv_h = height_padded >> 1;
    let bytes_per_yuv = if yuv_bit_depth <= 8 { 1 } else { 2 };
    let min_y_stride = checked_mul_usize(width, bytes_per_yuv).unwrap_or(usize::MAX);
    let min_uv_stride = checked_mul_usize(uv_w, bytes_per_yuv).unwrap_or(usize::MAX);
    if y_stride < min_y_stride || u_stride < min_uv_stride || v_stride < min_uv_stride {
        return false;
    }
    let _ = checked_add_usize(
        checked_mul_usize(height.saturating_sub(1), y_stride).unwrap_or(usize::MAX),
        min_y_stride,
    )
    .unwrap_or(usize::MAX);
    let _ = checked_add_usize(
        checked_mul_usize(uv_h.saturating_sub(1), u_stride).unwrap_or(usize::MAX),
        min_uv_stride,
    )
    .unwrap_or(usize::MAX);

    do_sharp_argb_to_yuv(
        r_ptr,
        g_ptr,
        b_ptr,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr,
        y_stride,
        u_ptr,
        u_stride,
        v_ptr,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        &scaled_matrix,
    )
}
