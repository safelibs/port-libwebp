extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type bit_t = uint64_t;
pub type range_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8BitReader {
    pub value_: bit_t,
    pub range_: range_t,
    pub bits_: ::core::ffi::c_int,
    pub buf_: *const uint8_t,
    pub buf_end_: *const uint8_t,
    pub buf_max_: *const uint8_t,
    pub eof_: ::core::ffi::c_int,
}
pub type lbit_t = uint64_t;
pub type vp8l_val_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBitReader {
    pub val_: vp8l_val_t,
    pub buf_: *const uint8_t,
    pub len_: size_t,
    pub pos_: size_t,
    pub bit_pos_: ::core::ffi::c_int,
    pub eos_: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __ASSERT_FUNCTION_0: [::core::ffi::c_char; 50] = unsafe {
    ::core::mem::transmute::<[u8; 50], [::core::ffi::c_char; 50]>(
        *b"int VP8LIsEndOfStream(const VP8LBitReader *const)\0",
    )
};
pub const BITS: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const VP8L_MAX_NUM_BIT_READ: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const VP8L_LBITS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const VP8L_WBITS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8LPrefetchBits(br: *mut VP8LBitReader) -> uint32_t {
    return ((*br).val_ >> ((*br).bit_pos_ & VP8L_LBITS - 1 as ::core::ffi::c_int)) as uint32_t;
}
#[inline]
unsafe extern "C" fn VP8LIsEndOfStream(br: *const VP8LBitReader) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if (*br).pos_ <= (*br).len_ {
        } else {
            __assert_fail(
                b"br->pos_ <= br->len_\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/utils/bit_reader_utils.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                172 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION_0.as_ptr(),
            );
        }
    };
    return ((*br).eos_ != 0 || (*br).pos_ == (*br).len_ && (*br).bit_pos_ > VP8L_LBITS)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn BSwap64(mut x: uint64_t) -> uint64_t {
    return x.swap_bytes();
}
#[inline]
unsafe extern "C" fn WebPMemToUint32(ptr: *const uint8_t) -> uint32_t {
    let mut A: uint32_t = 0;
    memcpy(
        &raw mut A as *mut ::core::ffi::c_void,
        ptr as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    return A;
}
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> ::core::ffi::c_int {
    return 31 as ::core::ffi::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn VP8LoadNewBytes(br: *mut VP8BitReader) {
    '_c2rust_label: {
        if !br.is_null() && !(*br).buf_.is_null() {
        } else {
            __assert_fail(
                b"br != NULL && br->buf_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/utils/bit_reader_inl_utils.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_uint,
                b"void VP8LoadNewBytes(VP8BitReader *const restrict)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*br).buf_ < (*br).buf_max_ {
        let mut bits: bit_t = 0;
        let mut in_bits: lbit_t = 0;
        memcpy(
            &raw mut in_bits as *mut ::core::ffi::c_void,
            (*br).buf_ as *const ::core::ffi::c_void,
            ::core::mem::size_of::<lbit_t>() as size_t,
        );
        (*br).buf_ = (*br)
            .buf_
            .offset((BITS >> 3 as ::core::ffi::c_int) as isize);
        bits = BSwap64(in_bits as uint64_t) as bit_t;
        bits >>= 64 as ::core::ffi::c_int - BITS;
        (*br).value_ = bits | (*br).value_ << BITS;
        (*br).bits_ += BITS;
    } else {
        VP8LoadFinalBytes(br);
    };
}
#[inline]
unsafe extern "C" fn VP8GetBit(
    br: *mut VP8BitReader,
    mut prob: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut range: range_t = (*br).range_;
    if (*br).bits_ < 0 as ::core::ffi::c_int {
        VP8LoadNewBytes(br);
    }
    let pos: ::core::ffi::c_int = (*br).bits_;
    let split: range_t = range.wrapping_mul(prob as range_t) >> 8 as ::core::ffi::c_int;
    let value: range_t = ((*br).value_ >> pos) as range_t;
    let bit: ::core::ffi::c_int = (value > split) as ::core::ffi::c_int;
    if bit != 0 {
        range = range.wrapping_sub(split);
        (*br).value_ = (*br)
            .value_
            .wrapping_sub((split.wrapping_add(1 as range_t) as bit_t) << pos);
    } else {
        range = split.wrapping_add(1 as range_t);
    }
    let shift: ::core::ffi::c_int =
        7 as ::core::ffi::c_int ^ BitsLog2Floor(range as uint32_t) as ::core::ffi::c_int;
    range <<= shift;
    (*br).bits_ -= shift;
    (*br).range_ = range.wrapping_sub(1 as range_t);
    return bit;
}
#[no_mangle]
pub unsafe extern "C" fn VP8BitReaderSetBuffer(
    br: *mut VP8BitReader,
    start: *const uint8_t,
    mut size: size_t,
) {
    (*br).buf_ = start;
    (*br).buf_end_ = start.offset(size as isize);
    (*br).buf_max_ = if size >= ::core::mem::size_of::<lbit_t>() as usize {
        start
            .offset(size as isize)
            .offset(-(::core::mem::size_of::<lbit_t>() as usize as isize))
            .offset(1 as ::core::ffi::c_int as isize)
    } else {
        start
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8InitBitReader(
    br: *mut VP8BitReader,
    start: *const uint8_t,
    mut size: size_t,
) {
    '_c2rust_label: {
        if !br.is_null() {
        } else {
            __assert_fail(
                b"br != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                37 as ::core::ffi::c_uint,
                b"void VP8InitBitReader(VP8BitReader *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !start.is_null() {
        } else {
            __assert_fail(
                b"start != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                38 as ::core::ffi::c_uint,
                b"void VP8InitBitReader(VP8BitReader *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if size < ((1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int) as size_t {
        } else {
            __assert_fail(
                b"size < (1u << 31)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                39 as ::core::ffi::c_uint,
                b"void VP8InitBitReader(VP8BitReader *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*br).range_ = (255 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as range_t;
    (*br).value_ = 0 as bit_t;
    (*br).bits_ = -(8 as ::core::ffi::c_int);
    (*br).eof_ = 0 as ::core::ffi::c_int;
    VP8BitReaderSetBuffer(br, start, size);
    VP8LoadNewBytes(br);
}
#[no_mangle]
pub unsafe extern "C" fn VP8RemapBitReader(br: *mut VP8BitReader, mut offset: ptrdiff_t) {
    if !(*br).buf_.is_null() {
        (*br).buf_ = (*br).buf_.offset(offset as isize);
        (*br).buf_end_ = (*br).buf_end_.offset(offset as isize);
        (*br).buf_max_ = (*br).buf_max_.offset(offset as isize);
    }
}
#[no_mangle]
pub static mut kVP8Log2Range: [uint8_t; 128] = [
    7 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut kVP8NewRange: [uint8_t; 128] = [
    127 as ::core::ffi::c_int as uint8_t,
    127 as ::core::ffi::c_int as uint8_t,
    191 as ::core::ffi::c_int as uint8_t,
    127 as ::core::ffi::c_int as uint8_t,
    159 as ::core::ffi::c_int as uint8_t,
    191 as ::core::ffi::c_int as uint8_t,
    223 as ::core::ffi::c_int as uint8_t,
    127 as ::core::ffi::c_int as uint8_t,
    143 as ::core::ffi::c_int as uint8_t,
    159 as ::core::ffi::c_int as uint8_t,
    175 as ::core::ffi::c_int as uint8_t,
    191 as ::core::ffi::c_int as uint8_t,
    207 as ::core::ffi::c_int as uint8_t,
    223 as ::core::ffi::c_int as uint8_t,
    239 as ::core::ffi::c_int as uint8_t,
    127 as ::core::ffi::c_int as uint8_t,
    135 as ::core::ffi::c_int as uint8_t,
    143 as ::core::ffi::c_int as uint8_t,
    151 as ::core::ffi::c_int as uint8_t,
    159 as ::core::ffi::c_int as uint8_t,
    167 as ::core::ffi::c_int as uint8_t,
    175 as ::core::ffi::c_int as uint8_t,
    183 as ::core::ffi::c_int as uint8_t,
    191 as ::core::ffi::c_int as uint8_t,
    199 as ::core::ffi::c_int as uint8_t,
    207 as ::core::ffi::c_int as uint8_t,
    215 as ::core::ffi::c_int as uint8_t,
    223 as ::core::ffi::c_int as uint8_t,
    231 as ::core::ffi::c_int as uint8_t,
    239 as ::core::ffi::c_int as uint8_t,
    247 as ::core::ffi::c_int as uint8_t,
    127 as ::core::ffi::c_int as uint8_t,
    131 as ::core::ffi::c_int as uint8_t,
    135 as ::core::ffi::c_int as uint8_t,
    139 as ::core::ffi::c_int as uint8_t,
    143 as ::core::ffi::c_int as uint8_t,
    147 as ::core::ffi::c_int as uint8_t,
    151 as ::core::ffi::c_int as uint8_t,
    155 as ::core::ffi::c_int as uint8_t,
    159 as ::core::ffi::c_int as uint8_t,
    163 as ::core::ffi::c_int as uint8_t,
    167 as ::core::ffi::c_int as uint8_t,
    171 as ::core::ffi::c_int as uint8_t,
    175 as ::core::ffi::c_int as uint8_t,
    179 as ::core::ffi::c_int as uint8_t,
    183 as ::core::ffi::c_int as uint8_t,
    187 as ::core::ffi::c_int as uint8_t,
    191 as ::core::ffi::c_int as uint8_t,
    195 as ::core::ffi::c_int as uint8_t,
    199 as ::core::ffi::c_int as uint8_t,
    203 as ::core::ffi::c_int as uint8_t,
    207 as ::core::ffi::c_int as uint8_t,
    211 as ::core::ffi::c_int as uint8_t,
    215 as ::core::ffi::c_int as uint8_t,
    219 as ::core::ffi::c_int as uint8_t,
    223 as ::core::ffi::c_int as uint8_t,
    227 as ::core::ffi::c_int as uint8_t,
    231 as ::core::ffi::c_int as uint8_t,
    235 as ::core::ffi::c_int as uint8_t,
    239 as ::core::ffi::c_int as uint8_t,
    243 as ::core::ffi::c_int as uint8_t,
    247 as ::core::ffi::c_int as uint8_t,
    251 as ::core::ffi::c_int as uint8_t,
    127 as ::core::ffi::c_int as uint8_t,
    129 as ::core::ffi::c_int as uint8_t,
    131 as ::core::ffi::c_int as uint8_t,
    133 as ::core::ffi::c_int as uint8_t,
    135 as ::core::ffi::c_int as uint8_t,
    137 as ::core::ffi::c_int as uint8_t,
    139 as ::core::ffi::c_int as uint8_t,
    141 as ::core::ffi::c_int as uint8_t,
    143 as ::core::ffi::c_int as uint8_t,
    145 as ::core::ffi::c_int as uint8_t,
    147 as ::core::ffi::c_int as uint8_t,
    149 as ::core::ffi::c_int as uint8_t,
    151 as ::core::ffi::c_int as uint8_t,
    153 as ::core::ffi::c_int as uint8_t,
    155 as ::core::ffi::c_int as uint8_t,
    157 as ::core::ffi::c_int as uint8_t,
    159 as ::core::ffi::c_int as uint8_t,
    161 as ::core::ffi::c_int as uint8_t,
    163 as ::core::ffi::c_int as uint8_t,
    165 as ::core::ffi::c_int as uint8_t,
    167 as ::core::ffi::c_int as uint8_t,
    169 as ::core::ffi::c_int as uint8_t,
    171 as ::core::ffi::c_int as uint8_t,
    173 as ::core::ffi::c_int as uint8_t,
    175 as ::core::ffi::c_int as uint8_t,
    177 as ::core::ffi::c_int as uint8_t,
    179 as ::core::ffi::c_int as uint8_t,
    181 as ::core::ffi::c_int as uint8_t,
    183 as ::core::ffi::c_int as uint8_t,
    185 as ::core::ffi::c_int as uint8_t,
    187 as ::core::ffi::c_int as uint8_t,
    189 as ::core::ffi::c_int as uint8_t,
    191 as ::core::ffi::c_int as uint8_t,
    193 as ::core::ffi::c_int as uint8_t,
    195 as ::core::ffi::c_int as uint8_t,
    197 as ::core::ffi::c_int as uint8_t,
    199 as ::core::ffi::c_int as uint8_t,
    201 as ::core::ffi::c_int as uint8_t,
    203 as ::core::ffi::c_int as uint8_t,
    205 as ::core::ffi::c_int as uint8_t,
    207 as ::core::ffi::c_int as uint8_t,
    209 as ::core::ffi::c_int as uint8_t,
    211 as ::core::ffi::c_int as uint8_t,
    213 as ::core::ffi::c_int as uint8_t,
    215 as ::core::ffi::c_int as uint8_t,
    217 as ::core::ffi::c_int as uint8_t,
    219 as ::core::ffi::c_int as uint8_t,
    221 as ::core::ffi::c_int as uint8_t,
    223 as ::core::ffi::c_int as uint8_t,
    225 as ::core::ffi::c_int as uint8_t,
    227 as ::core::ffi::c_int as uint8_t,
    229 as ::core::ffi::c_int as uint8_t,
    231 as ::core::ffi::c_int as uint8_t,
    233 as ::core::ffi::c_int as uint8_t,
    235 as ::core::ffi::c_int as uint8_t,
    237 as ::core::ffi::c_int as uint8_t,
    239 as ::core::ffi::c_int as uint8_t,
    241 as ::core::ffi::c_int as uint8_t,
    243 as ::core::ffi::c_int as uint8_t,
    245 as ::core::ffi::c_int as uint8_t,
    247 as ::core::ffi::c_int as uint8_t,
    249 as ::core::ffi::c_int as uint8_t,
    251 as ::core::ffi::c_int as uint8_t,
    253 as ::core::ffi::c_int as uint8_t,
    127 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8LoadFinalBytes(br: *mut VP8BitReader) {
    '_c2rust_label: {
        if !br.is_null() && !(*br).buf_.is_null() {
        } else {
            __assert_fail(
                b"br != NULL && br->buf_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                b"void VP8LoadFinalBytes(VP8BitReader *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*br).buf_ < (*br).buf_end_ {
        (*br).bits_ += 8 as ::core::ffi::c_int;
        let fresh0 = (*br).buf_;
        (*br).buf_ = (*br).buf_.offset(1);
        (*br).value_ = *fresh0 as bit_t | (*br).value_ << 8 as ::core::ffi::c_int;
    } else if (*br).eof_ == 0 {
        (*br).value_ <<= 8 as ::core::ffi::c_int;
        (*br).bits_ += 8 as ::core::ffi::c_int;
        (*br).eof_ = 1 as ::core::ffi::c_int;
    } else {
        (*br).bits_ = 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetValue(
    br: *mut VP8BitReader,
    mut bits: ::core::ffi::c_int,
) -> uint32_t {
    let mut v: uint32_t = 0 as uint32_t;
    loop {
        let fresh1 = bits;
        bits = bits - 1;
        if !(fresh1 > 0 as ::core::ffi::c_int) {
            break;
        }
        v |= (VP8GetBit(br, 0x80 as ::core::ffi::c_int) << bits) as uint32_t;
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetSignedValue(
    br: *mut VP8BitReader,
    mut bits: ::core::ffi::c_int,
) -> int32_t {
    let value: ::core::ffi::c_int = VP8GetValue(br, bits) as ::core::ffi::c_int;
    return if VP8GetValue(br, 1 as ::core::ffi::c_int) != 0 {
        -(value as int32_t)
    } else {
        value as int32_t
    };
}
pub const VP8L_LOG8_WBITS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut kBitMask: [uint32_t; 25] = [
    0 as ::core::ffi::c_int as uint32_t,
    0x1 as ::core::ffi::c_int as uint32_t,
    0x3 as ::core::ffi::c_int as uint32_t,
    0x7 as ::core::ffi::c_int as uint32_t,
    0xf as ::core::ffi::c_int as uint32_t,
    0x1f as ::core::ffi::c_int as uint32_t,
    0x3f as ::core::ffi::c_int as uint32_t,
    0x7f as ::core::ffi::c_int as uint32_t,
    0xff as ::core::ffi::c_int as uint32_t,
    0x1ff as ::core::ffi::c_int as uint32_t,
    0x3ff as ::core::ffi::c_int as uint32_t,
    0x7ff as ::core::ffi::c_int as uint32_t,
    0xfff as ::core::ffi::c_int as uint32_t,
    0x1fff as ::core::ffi::c_int as uint32_t,
    0x3fff as ::core::ffi::c_int as uint32_t,
    0x7fff as ::core::ffi::c_int as uint32_t,
    0xffff as ::core::ffi::c_int as uint32_t,
    0x1ffff as ::core::ffi::c_int as uint32_t,
    0x3ffff as ::core::ffi::c_int as uint32_t,
    0x7ffff as ::core::ffi::c_int as uint32_t,
    0xfffff as ::core::ffi::c_int as uint32_t,
    0x1fffff as ::core::ffi::c_int as uint32_t,
    0x3fffff as ::core::ffi::c_int as uint32_t,
    0x7fffff as ::core::ffi::c_int as uint32_t,
    0xffffff as ::core::ffi::c_int as uint32_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8LInitBitReader(
    br: *mut VP8LBitReader,
    start: *const uint8_t,
    mut length: size_t,
) {
    let mut i: size_t = 0;
    let mut value: vp8l_val_t = 0 as vp8l_val_t;
    '_c2rust_label: {
        if !br.is_null() {
        } else {
            __assert_fail(
                b"br != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                145 as ::core::ffi::c_uint,
                b"void VP8LInitBitReader(VP8LBitReader *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !start.is_null() {
        } else {
            __assert_fail(
                b"start != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                146 as ::core::ffi::c_uint,
                b"void VP8LInitBitReader(VP8LBitReader *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if length < 0xfffffff8 as ::core::ffi::c_uint as size_t {
        } else {
            __assert_fail(
                b"length < 0xfffffff8u\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_uint,
                b"void VP8LInitBitReader(VP8LBitReader *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*br).len_ = length;
    (*br).val_ = 0 as vp8l_val_t;
    (*br).bit_pos_ = 0 as ::core::ffi::c_int;
    (*br).eos_ = 0 as ::core::ffi::c_int;
    if length > ::core::mem::size_of::<vp8l_val_t>() as usize {
        length = ::core::mem::size_of::<vp8l_val_t>() as usize as size_t;
    }
    i = 0 as size_t;
    while i < length {
        value |= (*start.offset(i as isize) as vp8l_val_t) << (8 as size_t).wrapping_mul(i);
        i = i.wrapping_add(1);
    }
    (*br).val_ = value;
    (*br).pos_ = length;
    (*br).buf_ = start;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitReaderSetBuffer(
    br: *mut VP8LBitReader,
    buf: *const uint8_t,
    mut len: size_t,
) {
    '_c2rust_label: {
        if !br.is_null() {
        } else {
            __assert_fail(
                b"br != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                167 as ::core::ffi::c_uint,
                b"void VP8LBitReaderSetBuffer(VP8LBitReader *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !buf.is_null() {
        } else {
            __assert_fail(
                b"buf != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                168 as ::core::ffi::c_uint,
                b"void VP8LBitReaderSetBuffer(VP8LBitReader *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if len < 0xfffffff8 as ::core::ffi::c_uint as size_t {
        } else {
            __assert_fail(
                b"len < 0xfffffff8u\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                169 as ::core::ffi::c_uint,
                b"void VP8LBitReaderSetBuffer(VP8LBitReader *const, const uint8_t *const, size_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*br).buf_ = buf;
    (*br).len_ = len;
    (*br).eos_ = ((*br).pos_ > (*br).len_ || VP8LIsEndOfStream(br) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn VP8LSetEndOfStream(br: *mut VP8LBitReader) {
    (*br).eos_ = 1 as ::core::ffi::c_int;
    (*br).bit_pos_ = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ShiftBytes(br: *mut VP8LBitReader) {
    while (*br).bit_pos_ >= 8 as ::core::ffi::c_int && (*br).pos_ < (*br).len_ {
        (*br).val_ >>= 8 as ::core::ffi::c_int;
        (*br).val_ |= (*(*br).buf_.offset((*br).pos_ as isize) as vp8l_val_t)
            << VP8L_LBITS - 8 as ::core::ffi::c_int;
        (*br).pos_ = (*br).pos_.wrapping_add(1);
        (*br).bit_pos_ -= 8 as ::core::ffi::c_int;
    }
    if VP8LIsEndOfStream(br) != 0 {
        VP8LSetEndOfStream(br);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDoFillBitWindow(br: *mut VP8LBitReader) {
    '_c2rust_label: {
        if (*br).bit_pos_ >= 32 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"br->bit_pos_ >= VP8L_WBITS\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                195 as ::core::ffi::c_uint,
                b"void VP8LDoFillBitWindow(VP8LBitReader *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*br)
        .pos_
        .wrapping_add(::core::mem::size_of::<vp8l_val_t>() as size_t)
        < (*br).len_
    {
        (*br).val_ >>= VP8L_WBITS;
        (*br).bit_pos_ -= VP8L_WBITS;
        (*br).val_ |= (WebPMemToUint32((*br).buf_.offset((*br).pos_ as isize)) as vp8l_val_t)
            << VP8L_LBITS - VP8L_WBITS;
        (*br).pos_ = (*br).pos_.wrapping_add(VP8L_LOG8_WBITS as size_t);
        return;
    }
    ShiftBytes(br);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LReadBits(
    br: *mut VP8LBitReader,
    mut n_bits: ::core::ffi::c_int,
) -> uint32_t {
    '_c2rust_label: {
        if n_bits >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"n_bits >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/bit_reader_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                210 as ::core::ffi::c_uint,
                b"uint32_t VP8LReadBits(VP8LBitReader *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*br).eos_ == 0 && n_bits <= VP8L_MAX_NUM_BIT_READ {
        let val: uint32_t = VP8LPrefetchBits(br) as uint32_t & kBitMask[n_bits as usize];
        let new_bits: ::core::ffi::c_int = (*br).bit_pos_ + n_bits;
        (*br).bit_pos_ = new_bits;
        ShiftBytes(br);
        return val;
    } else {
        VP8LSetEndOfStream(br);
        return 0 as uint32_t;
    };
}
