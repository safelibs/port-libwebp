extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static VP8EntropyCost: [uint16_t; 256];
    static VP8EncBands: [uint8_t; 17];
    fn VP8PutBit(
        bw: *mut VP8BitWriter,
        bit: ::core::ffi::c_int,
        prob: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    static VP8Cat3: [uint8_t; 0];
    static VP8Cat4: [uint8_t; 0];
    static VP8Cat5: [uint8_t; 0];
    static VP8Cat6: [uint8_t; 0];
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NUM_PROBAS: C2RustUnnamed = 11;
pub const NUM_CTX: C2RustUnnamed = 3;
pub const NUM_BANDS: C2RustUnnamed = 8;
pub const NUM_TYPES: C2RustUnnamed = 4;
pub const MAX_NUM_PARTITIONS: C2RustUnnamed = 8;
pub const NUM_MODE_LF_DELTAS: C2RustUnnamed = 4;
pub const NUM_REF_LF_DELTAS: C2RustUnnamed = 4;
pub const NUM_MB_SEGMENTS: C2RustUnnamed = 4;
pub const MB_FEATURE_TREE_PROBS: C2RustUnnamed = 3;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Residual {
    pub first: ::core::ffi::c_int,
    pub last: ::core::ffi::c_int,
    pub coeffs: *const int16_t,
    pub coeff_type: ::core::ffi::c_int,
    pub prob: *mut ProbaArray,
    pub stats: *mut StatsArray,
    pub costs: CostArrayPtr,
}
pub type CostArrayPtr = *mut [*const uint16_t; 3];
pub type StatsArray = [[proba_t; 11]; 3];
pub type proba_t = uint32_t;
pub type ProbaArray = [[uint8_t; 11]; 3];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8BitWriter {
    pub range_: int32_t,
    pub value_: int32_t,
    pub run_: ::core::ffi::c_int,
    pub nb_bits_: ::core::ffi::c_int,
    pub buf_: *mut uint8_t,
    pub pos_: size_t,
    pub max_pos_: size_t,
    pub error_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8TBuffer {
    pub pages_: *mut VP8Tokens,
    pub last_page_: *mut *mut VP8Tokens,
    pub tokens_: *mut uint16_t,
    pub left_: ::core::ffi::c_int,
    pub page_size_: ::core::ffi::c_int,
    pub error_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Tokens {
    pub next_: *mut VP8Tokens,
}
pub type token_t = uint16_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn VP8RecordStats(
    mut bit: ::core::ffi::c_int,
    stats: *mut proba_t,
) -> ::core::ffi::c_int {
    let mut p: proba_t = *stats;
    if p >= 0xfffe0000 as proba_t {
        p = p.wrapping_add(1 as proba_t) >> 1 as ::core::ffi::c_int & 0x7fff7fff as proba_t;
    }
    p = (p as ::core::ffi::c_uint)
        .wrapping_add((0x10000 as ::core::ffi::c_uint).wrapping_add(bit as ::core::ffi::c_uint))
        as proba_t as proba_t;
    *stats = p;
    return bit;
}
#[inline]
unsafe extern "C" fn VP8BitCost(
    mut bit: ::core::ffi::c_int,
    mut proba: uint8_t,
) -> ::core::ffi::c_int {
    return if bit == 0 {
        VP8EntropyCost[proba as usize] as ::core::ffi::c_int
    } else {
        VP8EntropyCost[(255 as ::core::ffi::c_int - proba as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
    };
}
pub const MIN_PAGE_SIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const FIXED_PROBA_BIT: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 14 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn VP8TBufferInit(b: *mut VP8TBuffer, mut page_size: ::core::ffi::c_int) {
    (*b).tokens_ = ::core::ptr::null_mut::<uint16_t>();
    (*b).pages_ = ::core::ptr::null_mut::<VP8Tokens>();
    (*b).last_page_ = &raw mut (*b).pages_;
    (*b).left_ = 0 as ::core::ffi::c_int;
    (*b).page_size_ = if page_size < MIN_PAGE_SIZE {
        MIN_PAGE_SIZE
    } else {
        page_size
    };
    (*b).error_ = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8TBufferClear(b: *mut VP8TBuffer) {
    if !b.is_null() {
        let mut p: *mut VP8Tokens = (*b).pages_;
        while !p.is_null() {
            let next: *mut VP8Tokens = (*p).next_;
            WebPSafeFree(p as *mut ::core::ffi::c_void);
            p = next;
        }
        VP8TBufferInit(b, (*b).page_size_);
    }
}
unsafe extern "C" fn TBufferNewPage(b: *mut VP8TBuffer) -> ::core::ffi::c_int {
    let mut page: *mut VP8Tokens = ::core::ptr::null_mut::<VP8Tokens>();
    if (*b).error_ == 0 {
        let size: size_t = (::core::mem::size_of::<VP8Tokens>() as size_t).wrapping_add(
            ((*b).page_size_ as size_t).wrapping_mul(::core::mem::size_of::<token_t>() as size_t),
        );
        page = WebPSafeMalloc(1 as uint64_t, size) as *mut VP8Tokens;
    }
    if page.is_null() {
        (*b).error_ = 1 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    (*page).next_ = ::core::ptr::null_mut::<VP8Tokens>();
    *(*b).last_page_ = page;
    (*b).last_page_ = &raw mut (*page).next_;
    (*b).left_ = (*b).page_size_;
    (*b).tokens_ = page.offset(1 as ::core::ffi::c_int as isize) as *mut VP8Tokens as *const token_t
        as *mut token_t as *mut uint16_t;
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn AddToken(
    b: *mut VP8TBuffer,
    mut bit: uint32_t,
    mut proba_idx: uint32_t,
    stats: *mut proba_t,
) -> uint32_t {
    '_c2rust_label: {
        if proba_idx < (1 as uint32_t) << 14 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"proba_idx < FIXED_PROBA_BIT\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/token_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                93 as ::core::ffi::c_uint,
                b"uint32_t AddToken(VP8TBuffer *const, uint32_t, uint32_t, proba_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if bit <= 1 as uint32_t {
        } else {
            __assert_fail(
                b"bit <= 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/token_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_uint,
                b"uint32_t AddToken(VP8TBuffer *const, uint32_t, uint32_t, proba_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if (*b).left_ > 0 as ::core::ffi::c_int || TBufferNewPage(b) != 0 {
        (*b).left_ -= 1;
        let slot: ::core::ffi::c_int = (*b).left_;
        *(*b).tokens_.offset(slot as isize) =
            (bit << 15 as ::core::ffi::c_int | proba_idx) as uint16_t;
    }
    VP8RecordStats(bit as ::core::ffi::c_int, stats);
    return bit;
}
#[inline]
unsafe extern "C" fn AddConstantToken(b: *mut VP8TBuffer, mut bit: uint32_t, mut proba: uint32_t) {
    '_c2rust_label: {
        if proba < 256 as uint32_t {
        } else {
            __assert_fail(
                b"proba < 256\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/token_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_uint,
                b"void AddConstantToken(VP8TBuffer *const, uint32_t, uint32_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if bit <= 1 as uint32_t {
        } else {
            __assert_fail(
                b"bit <= 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/token_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                106 as ::core::ffi::c_uint,
                b"void AddConstantToken(VP8TBuffer *const, uint32_t, uint32_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*b).left_ > 0 as ::core::ffi::c_int || TBufferNewPage(b) != 0 {
        (*b).left_ -= 1;
        let slot: ::core::ffi::c_int = (*b).left_;
        *(*b).tokens_.offset(slot as isize) =
            (bit << 15 as ::core::ffi::c_int | FIXED_PROBA_BIT as uint32_t | proba) as uint16_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8RecordCoeffTokens(
    mut ctx: ::core::ffi::c_int,
    res: *const VP8Residual,
    tokens: *mut VP8TBuffer,
) -> ::core::ffi::c_int {
    let coeffs: *const int16_t = (*res).coeffs;
    let coeff_type: ::core::ffi::c_int = (*res).coeff_type;
    let last: ::core::ffi::c_int = (*res).last;
    let mut n: ::core::ffi::c_int = (*res).first;
    let mut base_id: uint32_t = (NUM_PROBAS as ::core::ffi::c_int
        * (ctx
            + NUM_CTX as ::core::ffi::c_int * (n + NUM_BANDS as ::core::ffi::c_int * coeff_type)))
        as uint32_t;
    let mut s: *mut proba_t = &raw mut *(&raw mut *(*res).stats.offset(n as isize)
        as *mut [proba_t; 11])
        .offset(ctx as isize) as *mut proba_t;
    if AddToken(
        tokens,
        (last >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint32_t,
        base_id.wrapping_add(0 as uint32_t),
        s.offset(0 as ::core::ffi::c_int as isize),
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    while n < 16 as ::core::ffi::c_int {
        let fresh1 = n;
        n = n + 1;
        let c: ::core::ffi::c_int = *coeffs.offset(fresh1 as isize) as ::core::ffi::c_int;
        let sign: ::core::ffi::c_int = (c < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let v: uint32_t = (if sign != 0 { -c } else { c }) as uint32_t;
        if AddToken(
            tokens,
            (v != 0 as uint32_t) as ::core::ffi::c_int as uint32_t,
            base_id.wrapping_add(1 as uint32_t),
            s.offset(1 as ::core::ffi::c_int as isize),
        ) == 0
        {
            base_id = (NUM_PROBAS as ::core::ffi::c_int
                * (0 as ::core::ffi::c_int
                    + NUM_CTX as ::core::ffi::c_int
                        * (VP8EncBands[n as usize] as ::core::ffi::c_int
                            + NUM_BANDS as ::core::ffi::c_int * coeff_type)))
                as uint32_t;
            s = &raw mut *(&raw mut *(*res)
                .stats
                .offset(*(&raw const VP8EncBands as *const uint8_t).offset(n as isize) as isize)
                as *mut [proba_t; 11])
                .offset(0 as ::core::ffi::c_int as isize) as *mut proba_t;
        } else {
            if AddToken(
                tokens,
                (v > 1 as uint32_t) as ::core::ffi::c_int as uint32_t,
                base_id.wrapping_add(2 as uint32_t),
                s.offset(2 as ::core::ffi::c_int as isize),
            ) == 0
            {
                base_id = (NUM_PROBAS as ::core::ffi::c_int
                    * (1 as ::core::ffi::c_int
                        + NUM_CTX as ::core::ffi::c_int
                            * (VP8EncBands[n as usize] as ::core::ffi::c_int
                                + NUM_BANDS as ::core::ffi::c_int * coeff_type)))
                    as uint32_t;
                s = &raw mut *(&raw mut *(*res)
                    .stats
                    .offset(*(&raw const VP8EncBands as *const uint8_t).offset(n as isize) as isize)
                    as *mut [proba_t; 11])
                    .offset(1 as ::core::ffi::c_int as isize) as *mut proba_t;
            } else {
                if AddToken(
                    tokens,
                    (v > 4 as uint32_t) as ::core::ffi::c_int as uint32_t,
                    base_id.wrapping_add(3 as uint32_t),
                    s.offset(3 as ::core::ffi::c_int as isize),
                ) == 0
                {
                    if AddToken(
                        tokens,
                        (v != 2 as uint32_t) as ::core::ffi::c_int as uint32_t,
                        base_id.wrapping_add(4 as uint32_t),
                        s.offset(4 as ::core::ffi::c_int as isize),
                    ) != 0
                    {
                        AddToken(
                            tokens,
                            (v == 4 as uint32_t) as ::core::ffi::c_int as uint32_t,
                            base_id.wrapping_add(5 as uint32_t),
                            s.offset(5 as ::core::ffi::c_int as isize),
                        );
                    }
                } else if AddToken(
                    tokens,
                    (v > 10 as uint32_t) as ::core::ffi::c_int as uint32_t,
                    base_id.wrapping_add(6 as uint32_t),
                    s.offset(6 as ::core::ffi::c_int as isize),
                ) == 0
                {
                    if AddToken(
                        tokens,
                        (v > 6 as uint32_t) as ::core::ffi::c_int as uint32_t,
                        base_id.wrapping_add(7 as uint32_t),
                        s.offset(7 as ::core::ffi::c_int as isize),
                    ) == 0
                    {
                        AddConstantToken(
                            tokens,
                            (v == 6 as uint32_t) as ::core::ffi::c_int as uint32_t,
                            159 as uint32_t,
                        );
                    } else {
                        AddConstantToken(
                            tokens,
                            (v >= 9 as uint32_t) as ::core::ffi::c_int as uint32_t,
                            165 as uint32_t,
                        );
                        AddConstantToken(
                            tokens,
                            (v & 1 as uint32_t == 0) as ::core::ffi::c_int as uint32_t,
                            145 as uint32_t,
                        );
                    }
                } else {
                    let mut mask: ::core::ffi::c_int = 0;
                    let mut tab: *const uint8_t = ::core::ptr::null::<uint8_t>();
                    let mut residue: uint32_t = v.wrapping_sub(3 as uint32_t);
                    if residue < ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint32_t
                    {
                        AddToken(
                            tokens,
                            0 as uint32_t,
                            base_id.wrapping_add(8 as uint32_t),
                            s.offset(8 as ::core::ffi::c_int as isize),
                        );
                        AddToken(
                            tokens,
                            0 as uint32_t,
                            base_id.wrapping_add(9 as uint32_t),
                            s.offset(9 as ::core::ffi::c_int as isize),
                        );
                        residue = residue.wrapping_sub(
                            ((8 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint32_t,
                        );
                        mask = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
                        tab = &raw const VP8Cat3 as *const uint8_t;
                    } else if residue
                        < ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint32_t
                    {
                        AddToken(
                            tokens,
                            0 as uint32_t,
                            base_id.wrapping_add(8 as uint32_t),
                            s.offset(8 as ::core::ffi::c_int as isize),
                        );
                        AddToken(
                            tokens,
                            1 as uint32_t,
                            base_id.wrapping_add(9 as uint32_t),
                            s.offset(9 as ::core::ffi::c_int as isize),
                        );
                        residue = residue.wrapping_sub(
                            ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint32_t,
                        );
                        mask = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
                        tab = &raw const VP8Cat4 as *const uint8_t;
                    } else if residue
                        < ((8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t
                    {
                        AddToken(
                            tokens,
                            1 as uint32_t,
                            base_id.wrapping_add(8 as uint32_t),
                            s.offset(8 as ::core::ffi::c_int as isize),
                        );
                        AddToken(
                            tokens,
                            0 as uint32_t,
                            base_id.wrapping_add(10 as uint32_t),
                            s.offset(9 as ::core::ffi::c_int as isize),
                        );
                        residue = residue.wrapping_sub(
                            ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint32_t,
                        );
                        mask = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
                        tab = &raw const VP8Cat5 as *const uint8_t;
                    } else {
                        AddToken(
                            tokens,
                            1 as uint32_t,
                            base_id.wrapping_add(8 as uint32_t),
                            s.offset(8 as ::core::ffi::c_int as isize),
                        );
                        AddToken(
                            tokens,
                            1 as uint32_t,
                            base_id.wrapping_add(10 as uint32_t),
                            s.offset(9 as ::core::ffi::c_int as isize),
                        );
                        residue = residue.wrapping_sub(
                            ((8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t,
                        );
                        mask = (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
                        tab = &raw const VP8Cat6 as *const uint8_t;
                    }
                    while mask != 0 {
                        let fresh2 = tab;
                        tab = tab.offset(1);
                        AddConstantToken(
                            tokens,
                            (residue & mask as uint32_t != 0) as ::core::ffi::c_int as uint32_t,
                            *fresh2 as uint32_t,
                        );
                        mask >>= 1 as ::core::ffi::c_int;
                    }
                }
                base_id = (NUM_PROBAS as ::core::ffi::c_int
                    * (2 as ::core::ffi::c_int
                        + NUM_CTX as ::core::ffi::c_int
                            * (VP8EncBands[n as usize] as ::core::ffi::c_int
                                + NUM_BANDS as ::core::ffi::c_int * coeff_type)))
                    as uint32_t;
                s = &raw mut *(&raw mut *(*res)
                    .stats
                    .offset(*(&raw const VP8EncBands as *const uint8_t).offset(n as isize) as isize)
                    as *mut [proba_t; 11])
                    .offset(2 as ::core::ffi::c_int as isize) as *mut proba_t;
            }
            AddConstantToken(tokens, sign as uint32_t, 128 as uint32_t);
            if n == 16 as ::core::ffi::c_int
                || AddToken(
                    tokens,
                    (n <= last) as ::core::ffi::c_int as uint32_t,
                    base_id.wrapping_add(0 as uint32_t),
                    s.offset(0 as ::core::ffi::c_int as isize),
                ) == 0
            {
                return 1 as ::core::ffi::c_int;
            }
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EmitTokens(
    b: *mut VP8TBuffer,
    bw: *mut VP8BitWriter,
    probas: *const uint8_t,
    mut final_pass: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *const VP8Tokens = (*b).pages_;
    '_c2rust_label: {
        if (*b).error_ == 0 {
        } else {
            __assert_fail(
                b"!b->error_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/token_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                203 as ::core::ffi::c_uint,
                b"int VP8EmitTokens(VP8TBuffer *const, VP8BitWriter *const, const uint8_t *const, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    while !p.is_null() {
        let next: *const VP8Tokens = (*p).next_;
        let N: ::core::ffi::c_int = if next.is_null() {
            (*b).left_
        } else {
            0 as ::core::ffi::c_int
        };
        let mut n: ::core::ffi::c_int = (*b).page_size_;
        let tokens: *const token_t =
            p.offset(1 as ::core::ffi::c_int as isize) as *const VP8Tokens as *const token_t;
        loop {
            let fresh0 = n;
            n = n - 1;
            if !(fresh0 > N) {
                break;
            }
            let token: token_t = *tokens.offset(n as isize);
            let bit: ::core::ffi::c_int =
                token as ::core::ffi::c_int >> 15 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
            if token as ::core::ffi::c_uint & FIXED_PROBA_BIT != 0 {
                VP8PutBit(
                    bw,
                    bit,
                    (token as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                        as ::core::ffi::c_int,
                );
            } else {
                VP8PutBit(
                    bw,
                    bit,
                    *probas.offset(
                        (token as ::core::ffi::c_uint & 0x3fff as ::core::ffi::c_uint) as isize,
                    ) as ::core::ffi::c_int,
                );
            }
        }
        if final_pass != 0 {
            WebPSafeFree(p as *mut ::core::ffi::c_void);
        }
        p = next;
    }
    if final_pass != 0 {
        (*b).pages_ = ::core::ptr::null_mut::<VP8Tokens>();
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EstimateTokenSize(
    b: *mut VP8TBuffer,
    probas: *const uint8_t,
) -> size_t {
    let mut size: size_t = 0 as size_t;
    let mut p: *const VP8Tokens = (*b).pages_;
    '_c2rust_label: {
        if (*b).error_ == 0 {
        } else {
            __assert_fail(
                b"!b->error_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/token_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                229 as ::core::ffi::c_uint,
                b"size_t VP8EstimateTokenSize(VP8TBuffer *const, const uint8_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    while !p.is_null() {
        let next: *const VP8Tokens = (*p).next_;
        let N: ::core::ffi::c_int = if next.is_null() {
            (*b).left_
        } else {
            0 as ::core::ffi::c_int
        };
        let mut n: ::core::ffi::c_int = (*b).page_size_;
        let tokens: *const token_t =
            p.offset(1 as ::core::ffi::c_int as isize) as *const VP8Tokens as *const token_t;
        loop {
            let fresh3 = n;
            n = n - 1;
            if !(fresh3 > N) {
                break;
            }
            let token: token_t = *tokens.offset(n as isize);
            let bit: ::core::ffi::c_int =
                token as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int;
            if token as ::core::ffi::c_uint & FIXED_PROBA_BIT != 0 {
                size = size.wrapping_add(VP8BitCost(
                    bit,
                    (token as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint) as uint8_t,
                ) as size_t);
            } else {
                size = size.wrapping_add(VP8BitCost(
                    bit,
                    *probas.offset(
                        (token as ::core::ffi::c_uint & 0x3fff as ::core::ffi::c_uint) as isize,
                    ),
                ) as size_t);
            }
        }
        p = next;
    }
    return size;
}
