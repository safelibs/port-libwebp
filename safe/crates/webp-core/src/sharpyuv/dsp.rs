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

pub fn init_dsp() {}

pub fn update_y(ref_y: &[u16], src_y: &[u16], dst_y: &mut [u16], bit_depth: i32) -> u64 {
    let mut diff = 0_u64;
    let max_y = (1_i32 << bit_depth) - 1;
    for ((&reference, &source), dst) in ref_y.iter().zip(src_y).zip(dst_y.iter_mut()) {
        let diff_y = reference as i32 - source as i32;
        *dst = clip(*dst as i32 + diff_y, max_y);
        diff += diff_y.unsigned_abs() as u64;
    }
    diff
}

pub fn update_rgb(ref_rgb: &[i16], src_rgb: &[i16], dst_rgb: &mut [i16]) {
    for ((&reference, &source), dst) in ref_rgb.iter().zip(src_rgb).zip(dst_rgb.iter_mut()) {
        *dst = dst.wrapping_add(reference.wrapping_sub(source));
    }
}

pub fn filter_row(a: &[i16], b: &[i16], best_y: &[u16], out: &mut [u16], bit_depth: i32) {
    let max_y = (1_i32 << bit_depth) - 1;
    for i in 0..(out.len() / 2) {
        let v0 =
            (a[i] as i32 * 9 + a[i + 1] as i32 * 3 + b[i] as i32 * 3 + b[i + 1] as i32 + 8) >> 4;
        let v1 =
            (a[i + 1] as i32 * 9 + a[i] as i32 * 3 + b[i + 1] as i32 * 3 + b[i] as i32 + 8) >> 4;
        out[2 * i] = clip(best_y[2 * i] as i32 + v0, max_y);
        out[2 * i + 1] = clip(best_y[2 * i + 1] as i32 + v1, max_y);
    }
}
