extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SmoothParams {
    pub width_: ::core::ffi::c_int,
    pub height_: ::core::ffi::c_int,
    pub stride_: ::core::ffi::c_int,
    pub row_: ::core::ffi::c_int,
    pub src_: *mut uint8_t,
    pub dst_: *mut uint8_t,
    pub radius_: ::core::ffi::c_int,
    pub scale_: ::core::ffi::c_int,
    pub mem_: *mut ::core::ffi::c_void,
    pub start_: *mut uint16_t,
    pub cur_: *mut uint16_t,
    pub end_: *mut uint16_t,
    pub top_: *mut uint16_t,
    pub average_: *mut uint16_t,
    pub num_levels_: ::core::ffi::c_int,
    pub min_: ::core::ffi::c_int,
    pub max_: ::core::ffi::c_int,
    pub min_level_dist_: ::core::ffi::c_int,
    pub correction_: *mut int16_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FIX: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const LFIX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LUT_SIZE: ::core::ffi::c_int =
    ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int + LFIX) - 1 as ::core::ffi::c_int;
pub const DFIX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLIP_8b_MASK: ::core::ffi::c_int =
    (!(0 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int + DFIX) as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn clip_8b(mut v: ::core::ffi::c_int) -> uint8_t {
    return (if v & CLIP_8b_MASK == 0 {
        (v >> DFIX) as uint8_t as ::core::ffi::c_uint
    } else if v < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_uint
    } else {
        255 as ::core::ffi::c_uint
    }) as uint8_t;
}
unsafe extern "C" fn VFilter(p: *mut SmoothParams) {
    let mut src: *const uint8_t = (*p).src_;
    let w: ::core::ffi::c_int = (*p).width_;
    let cur: *mut uint16_t = (*p).cur_;
    let top: *const uint16_t = (*p).top_;
    let out: *mut uint16_t = (*p).end_;
    let mut sum: uint16_t = 0 as uint16_t;
    let mut x: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    while x < w {
        let mut new_value: uint16_t = 0;
        sum =
            (sum as ::core::ffi::c_int + *src.offset(x as isize) as ::core::ffi::c_int) as uint16_t;
        new_value =
            (*top.offset(x as isize) as ::core::ffi::c_int + sum as ::core::ffi::c_int) as uint16_t;
        *out.offset(x as isize) = (new_value as ::core::ffi::c_int
            - *cur.offset(x as isize) as ::core::ffi::c_int)
            as uint16_t;
        *cur.offset(x as isize) = new_value;
        x += 1;
    }
    (*p).top_ = (*p).cur_;
    (*p).cur_ = (*p).cur_.offset(w as isize);
    if (*p).cur_ == (*p).end_ {
        (*p).cur_ = (*p).start_;
    }
    if (*p).row_ >= 0 as ::core::ffi::c_int && (*p).row_ < (*p).height_ - 1 as ::core::ffi::c_int {
        (*p).src_ = (*p).src_.offset((*p).stride_ as isize);
    }
}
unsafe extern "C" fn HFilter(p: *mut SmoothParams) {
    let in_0: *const uint16_t = (*p).end_;
    let out: *mut uint16_t = (*p).average_;
    let scale: uint32_t = (*p).scale_ as uint32_t;
    let w: ::core::ffi::c_int = (*p).width_;
    let r: ::core::ffi::c_int = (*p).radius_;
    let mut x: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    while x <= r {
        let delta: uint16_t =
            (*in_0.offset((x + r - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *in_0.offset((r - x) as isize) as ::core::ffi::c_int) as uint16_t;
        *out.offset(x as isize) = ((delta as uint32_t).wrapping_mul(scale) >> FIX) as uint16_t;
        x += 1;
    }
    while x < w - r {
        let delta_0: uint16_t = (*in_0.offset((x + r) as isize) as ::core::ffi::c_int
            - *in_0.offset((x - r - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as uint16_t;
        *out.offset(x as isize) = ((delta_0 as uint32_t).wrapping_mul(scale) >> FIX) as uint16_t;
        x += 1;
    }
    while x < w {
        let delta_1: uint16_t = (2 as ::core::ffi::c_int
            * *in_0.offset((w - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            - *in_0.offset((2 as ::core::ffi::c_int * w - 2 as ::core::ffi::c_int - r - x) as isize)
                as ::core::ffi::c_int
            - *in_0.offset((x - r - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as uint16_t;
        *out.offset(x as isize) = ((delta_1 as uint32_t).wrapping_mul(scale) >> FIX) as uint16_t;
        x += 1;
    }
}
unsafe extern "C" fn ApplyFilter(p: *mut SmoothParams) {
    let average: *const uint16_t = (*p).average_;
    let w: ::core::ffi::c_int = (*p).width_;
    let correction: *const int16_t = (*p).correction_;
    let dst: *mut uint8_t = (*p).dst_;
    let mut x: ::core::ffi::c_int = 0;
    x = 0 as ::core::ffi::c_int;
    while x < w {
        let v: ::core::ffi::c_int = *dst.offset(x as isize) as ::core::ffi::c_int;
        if v < (*p).max_ && v > (*p).min_ {
            let c: ::core::ffi::c_int = (v << DFIX)
                + *correction.offset(
                    (*average.offset(x as isize) as ::core::ffi::c_int - (v << LFIX)) as isize,
                ) as ::core::ffi::c_int;
            *dst.offset(x as isize) = clip_8b(c);
        }
        x += 1;
    }
    (*p).dst_ = (*p).dst_.offset((*p).stride_ as isize);
}
unsafe extern "C" fn InitCorrectionLUT(lut: *mut int16_t, mut min_dist: ::core::ffi::c_int) {
    let threshold1: ::core::ffi::c_int = min_dist << LFIX;
    let threshold2: ::core::ffi::c_int =
        3 as ::core::ffi::c_int * threshold1 >> 2 as ::core::ffi::c_int;
    let max_threshold: ::core::ffi::c_int = threshold2 << DFIX;
    let delta: ::core::ffi::c_int = threshold1 - threshold2;
    let mut i: ::core::ffi::c_int = 0;
    i = 1 as ::core::ffi::c_int;
    while i <= LUT_SIZE {
        let mut c: ::core::ffi::c_int = if i <= threshold2 {
            i << DFIX
        } else if i < threshold1 {
            max_threshold * (threshold1 - i) / delta
        } else {
            0 as ::core::ffi::c_int
        };
        c >>= LFIX;
        *lut.offset(i as isize) = c as int16_t;
        *lut.offset(-i as isize) = -c as int16_t;
        i += 1;
    }
    *lut.offset(0 as ::core::ffi::c_int as isize) = 0 as int16_t;
}
unsafe extern "C" fn CountLevels(p: *mut SmoothParams) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut last_level: ::core::ffi::c_int = 0;
    let mut used_levels: [uint8_t; 256] = [
        0 as ::core::ffi::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut data: *const uint8_t = (*p).src_;
    (*p).min_ = 255 as ::core::ffi::c_int;
    (*p).max_ = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < (*p).height_ {
        i = 0 as ::core::ffi::c_int;
        while i < (*p).width_ {
            let v: ::core::ffi::c_int = *data.offset(i as isize) as ::core::ffi::c_int;
            if v < (*p).min_ {
                (*p).min_ = v;
            }
            if v > (*p).max_ {
                (*p).max_ = v;
            }
            used_levels[v as usize] = 1 as uint8_t;
            i += 1;
        }
        data = data.offset((*p).stride_ as isize);
        j += 1;
    }
    (*p).min_level_dist_ = (*p).max_ - (*p).min_;
    last_level = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        if used_levels[i as usize] != 0 {
            (*p).num_levels_ += 1;
            if last_level >= 0 as ::core::ffi::c_int {
                let level_dist: ::core::ffi::c_int = i - last_level;
                if level_dist < (*p).min_level_dist_ {
                    (*p).min_level_dist_ = level_dist;
                }
            }
            last_level = i;
        }
        i += 1;
    }
}
unsafe extern "C" fn InitParams(
    data: *mut uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut radius: ::core::ffi::c_int,
    p: *mut SmoothParams,
) -> ::core::ffi::c_int {
    let R: ::core::ffi::c_int = 2 as ::core::ffi::c_int * radius + 1 as ::core::ffi::c_int;
    let size_scratch_m: size_t = (((R + 1 as ::core::ffi::c_int) * width) as size_t)
        .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t);
    let size_m: size_t =
        (width as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t);
    let size_lut: size_t = ((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * LUT_SIZE)
        as size_t)
        .wrapping_mul(::core::mem::size_of::<int16_t>() as size_t);
    let total_size: size_t = size_scratch_m.wrapping_add(size_m).wrapping_add(size_lut);
    let mut mem: *mut uint8_t = WebPSafeMalloc(1 as uint64_t, total_size) as *mut uint8_t;
    if mem.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*p).mem_ = mem as *mut ::core::ffi::c_void;
    (*p).start_ = mem as *mut uint16_t;
    (*p).cur_ = (*p).start_;
    (*p).end_ = (*p).start_.offset((R * width) as isize);
    (*p).top_ = (*p).end_.offset(-(width as isize));
    memset(
        (*p).top_ as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (width as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    );
    mem = mem.offset(size_scratch_m as isize);
    (*p).average_ = mem as *mut uint16_t;
    mem = mem.offset(size_m as isize);
    (*p).width_ = width;
    (*p).height_ = height;
    (*p).stride_ = stride;
    (*p).src_ = data;
    (*p).dst_ = data;
    (*p).radius_ = radius;
    (*p).scale_ = ((1 as ::core::ffi::c_int) << FIX + LFIX) / (R * R);
    (*p).row_ = -radius;
    CountLevels(p);
    (*p).correction_ = (mem as *mut int16_t).offset(LUT_SIZE as isize);
    InitCorrectionLUT((*p).correction_, (*p).min_level_dist_);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn CleanupParams(p: *mut SmoothParams) {
    WebPSafeFree((*p).mem_);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDequantizeLevels(
    data: *mut uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut strength: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut radius: ::core::ffi::c_int =
        4 as ::core::ffi::c_int * strength / 100 as ::core::ffi::c_int;
    if strength < 0 as ::core::ffi::c_int || strength > 100 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if data.is_null() || width <= 0 as ::core::ffi::c_int || height <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if 2 as ::core::ffi::c_int * radius + 1 as ::core::ffi::c_int > width {
        radius = width - 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    }
    if 2 as ::core::ffi::c_int * radius + 1 as ::core::ffi::c_int > height {
        radius = height - 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
    }
    if radius > 0 as ::core::ffi::c_int {
        let mut p: SmoothParams = SmoothParams {
            width_: 0,
            height_: 0,
            stride_: 0,
            row_: 0,
            src_: ::core::ptr::null_mut::<uint8_t>(),
            dst_: ::core::ptr::null_mut::<uint8_t>(),
            radius_: 0,
            scale_: 0,
            mem_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            start_: ::core::ptr::null_mut::<uint16_t>(),
            cur_: ::core::ptr::null_mut::<uint16_t>(),
            end_: ::core::ptr::null_mut::<uint16_t>(),
            top_: ::core::ptr::null_mut::<uint16_t>(),
            average_: ::core::ptr::null_mut::<uint16_t>(),
            num_levels_: 0,
            min_: 0,
            max_: 0,
            min_level_dist_: 0,
            correction_: ::core::ptr::null_mut::<int16_t>(),
        };
        memset(
            &raw mut p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SmoothParams>() as size_t,
        );
        if InitParams(data, width, height, stride, radius, &raw mut p) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if p.num_levels_ > 2 as ::core::ffi::c_int {
            while p.row_ < p.height_ {
                VFilter(&raw mut p);
                if p.row_ >= p.radius_ {
                    HFilter(&raw mut p);
                    ApplyFilter(&raw mut p);
                }
                p.row_ += 1;
            }
        }
        CleanupParams(&raw mut p);
    }
    return 1 as ::core::ffi::c_int;
}
