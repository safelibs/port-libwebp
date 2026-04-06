extern "C" {
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type WEBP_FILTER_TYPE = ::core::ffi::c_uint;
pub const WEBP_FILTER_FAST: WEBP_FILTER_TYPE = 6;
pub const WEBP_FILTER_BEST: WEBP_FILTER_TYPE = 5;
pub const WEBP_FILTER_LAST: WEBP_FILTER_TYPE = 4;
pub const WEBP_FILTER_GRADIENT: WEBP_FILTER_TYPE = 3;
pub const WEBP_FILTER_VERTICAL: WEBP_FILTER_TYPE = 2;
pub const WEBP_FILTER_HORIZONTAL: WEBP_FILTER_TYPE = 1;
pub const WEBP_FILTER_NONE: WEBP_FILTER_TYPE = 0;
pub const SMAX: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn GradientPredictor(
    mut a: uint8_t,
    mut b: uint8_t,
    mut c: uint8_t,
) -> ::core::ffi::c_int {
    let g: ::core::ffi::c_int =
        a as ::core::ffi::c_int + b as ::core::ffi::c_int - c as ::core::ffi::c_int;
    return if g & !(0xff as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        g
    } else if g < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        255 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPEstimateBestFilter(
    mut data: *const uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
) -> WEBP_FILTER_TYPE {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut bins: [[::core::ffi::c_int; 16]; 4] = [[0; 16]; 4];
    memset(
        &raw mut bins as *mut [::core::ffi::c_int; 16] as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[[::core::ffi::c_int; 16]; 4]>() as size_t,
    );
    j = 2 as ::core::ffi::c_int;
    while j < height - 1 as ::core::ffi::c_int {
        let p: *const uint8_t = data.offset((j * stride) as isize);
        let mut mean: ::core::ffi::c_int =
            *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        i = 2 as ::core::ffi::c_int;
        while i < width - 1 as ::core::ffi::c_int {
            let diff0: ::core::ffi::c_int = abs(*p.offset(i as isize) as ::core::ffi::c_int - mean)
                as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int;
            let diff1: ::core::ffi::c_int = abs(*p.offset(i as isize) as ::core::ffi::c_int
                - *p.offset((i - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int;
            let diff2: ::core::ffi::c_int = abs(*p.offset(i as isize) as ::core::ffi::c_int
                - *p.offset((i - width) as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int;
            let grad_pred: ::core::ffi::c_int = GradientPredictor(
                *p.offset((i - 1 as ::core::ffi::c_int) as isize),
                *p.offset((i - width) as isize),
                *p.offset((i - width - 1 as ::core::ffi::c_int) as isize),
            ) as ::core::ffi::c_int;
            let diff3: ::core::ffi::c_int =
                abs(*p.offset(i as isize) as ::core::ffi::c_int - grad_pred) as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int;
            bins[WEBP_FILTER_NONE as ::core::ffi::c_int as usize][diff0 as usize] =
                1 as ::core::ffi::c_int;
            bins[WEBP_FILTER_HORIZONTAL as ::core::ffi::c_int as usize][diff1 as usize] =
                1 as ::core::ffi::c_int;
            bins[WEBP_FILTER_VERTICAL as ::core::ffi::c_int as usize][diff2 as usize] =
                1 as ::core::ffi::c_int;
            bins[WEBP_FILTER_GRADIENT as ::core::ffi::c_int as usize][diff3 as usize] =
                1 as ::core::ffi::c_int;
            mean = 3 as ::core::ffi::c_int * mean
                + *p.offset(i as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int;
            i += 2 as ::core::ffi::c_int;
        }
        j += 2 as ::core::ffi::c_int;
    }
    let mut filter: ::core::ffi::c_int = 0;
    let mut best_filter: WEBP_FILTER_TYPE = WEBP_FILTER_NONE;
    let mut best_score: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
    filter = WEBP_FILTER_NONE as ::core::ffi::c_int;
    while filter < WEBP_FILTER_LAST as ::core::ffi::c_int {
        let mut score: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < SMAX {
            if bins[filter as usize][i as usize] > 0 as ::core::ffi::c_int {
                score += i;
            }
            i += 1;
        }
        if score < best_score {
            best_score = score;
            best_filter = filter as WEBP_FILTER_TYPE;
        }
        filter += 1;
    }
    return best_filter;
}
