extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn WebPMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPFree(ptr: *mut ::core::ffi::c_void);
    fn WebPMuxGetFeatures(mux: *const WebPMux, flags: *mut uint32_t) -> WebPMuxError;
    fn WebPMuxNumChunks(
        mux: *const WebPMux,
        id: WebPChunkId,
        num_elements: *mut ::core::ffi::c_int,
    ) -> WebPMuxError;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPData {
    pub bytes: *const uint8_t,
    pub size: size_t,
}
pub type WebPFeatureFlags = ::core::ffi::c_uint;
pub const ALL_VALID_FLAGS: WebPFeatureFlags = 62;
pub const ICCP_FLAG: WebPFeatureFlags = 32;
pub const ALPHA_FLAG: WebPFeatureFlags = 16;
pub const EXIF_FLAG: WebPFeatureFlags = 8;
pub const XMP_FLAG: WebPFeatureFlags = 4;
pub const ANIMATION_FLAG: WebPFeatureFlags = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMux {
    pub images_: *mut WebPMuxImage,
    pub iccp_: *mut WebPChunk,
    pub exif_: *mut WebPChunk,
    pub xmp_: *mut WebPChunk,
    pub anim_: *mut WebPChunk,
    pub vp8x_: *mut WebPChunk,
    pub unknown_: *mut WebPChunk,
    pub canvas_width_: ::core::ffi::c_int,
    pub canvas_height_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPChunk {
    pub tag_: uint32_t,
    pub owner_: ::core::ffi::c_int,
    pub data_: WebPData,
    pub next_: *mut WebPChunk,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMuxImage {
    pub header_: *mut WebPChunk,
    pub alpha_: *mut WebPChunk,
    pub img_: *mut WebPChunk,
    pub unknown_: *mut WebPChunk,
    pub width_: ::core::ffi::c_int,
    pub height_: ::core::ffi::c_int,
    pub has_alpha_: ::core::ffi::c_int,
    pub is_partial_: ::core::ffi::c_int,
    pub next_: *mut WebPMuxImage,
}
pub type WebPChunkId = ::core::ffi::c_uint;
pub const WEBP_CHUNK_NIL: WebPChunkId = 10;
pub const WEBP_CHUNK_UNKNOWN: WebPChunkId = 9;
pub const WEBP_CHUNK_XMP: WebPChunkId = 8;
pub const WEBP_CHUNK_EXIF: WebPChunkId = 7;
pub const WEBP_CHUNK_IMAGE: WebPChunkId = 6;
pub const WEBP_CHUNK_ALPHA: WebPChunkId = 5;
pub const WEBP_CHUNK_DEPRECATED: WebPChunkId = 4;
pub const WEBP_CHUNK_ANMF: WebPChunkId = 3;
pub const WEBP_CHUNK_ANIM: WebPChunkId = 2;
pub const WEBP_CHUNK_ICCP: WebPChunkId = 1;
pub const WEBP_CHUNK_VP8X: WebPChunkId = 0;
pub type WebPMuxError = ::core::ffi::c_int;
pub const WEBP_MUX_NOT_ENOUGH_DATA: WebPMuxError = -4;
pub const WEBP_MUX_MEMORY_ERROR: WebPMuxError = -3;
pub const WEBP_MUX_BAD_DATA: WebPMuxError = -2;
pub const WEBP_MUX_INVALID_ARGUMENT: WebPMuxError = -1;
pub const WEBP_MUX_NOT_FOUND: WebPMuxError = 0;
pub const WEBP_MUX_OK: WebPMuxError = 1;
pub type CHUNK_INDEX = ::core::ffi::c_uint;
pub const IDX_LAST_CHUNK: CHUNK_INDEX = 11;
pub const IDX_NIL: CHUNK_INDEX = 10;
pub const IDX_UNKNOWN: CHUNK_INDEX = 9;
pub const IDX_XMP: CHUNK_INDEX = 8;
pub const IDX_EXIF: CHUNK_INDEX = 7;
pub const IDX_VP8L: CHUNK_INDEX = 6;
pub const IDX_VP8: CHUNK_INDEX = 5;
pub const IDX_ALPHA: CHUNK_INDEX = 4;
pub const IDX_ANMF: CHUNK_INDEX = 3;
pub const IDX_ANIM: CHUNK_INDEX = 2;
pub const IDX_ICCP: CHUNK_INDEX = 1;
pub const IDX_VP8X: CHUNK_INDEX = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChunkInfo {
    pub tag: uint32_t,
    pub id: WebPChunkId,
    pub size: uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const TAG_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNK_SIZE_BYTES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNK_HEADER_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RIFF_HEADER_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const ANMF_CHUNK_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ANIM_CHUNK_SIZE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const VP8X_CHUNK_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPDataInit(mut webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        memset(
            webp_data as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<WebPData>() as size_t,
        );
    }
}
#[inline]
unsafe extern "C" fn WebPDataClear(mut webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        WebPFree((*webp_data).bytes as *mut ::core::ffi::c_void);
        WebPDataInit(webp_data);
    }
}
#[inline]
unsafe extern "C" fn WebPDataCopy(
    mut src: *const WebPData,
    mut dst: *mut WebPData,
) -> ::core::ffi::c_int {
    if src.is_null() || dst.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    WebPDataInit(dst);
    if !(*src).bytes.is_null() && (*src).size != 0 as size_t {
        (*dst).bytes = WebPMalloc((*src).size) as *mut uint8_t;
        if (*dst).bytes.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        memcpy(
            (*dst).bytes as *mut ::core::ffi::c_void,
            (*src).bytes as *const ::core::ffi::c_void,
            (*src).size,
        );
        (*dst).size = (*src).size;
    }
    return 1 as ::core::ffi::c_int;
}
pub const MUX_MAJ_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MUX_MIN_VERSION: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MUX_REV_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NIL_TAG: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn SizeWithPadding(mut chunk_size: size_t) -> size_t {
    '_c2rust_label: {
        if chunk_size
            <= (!(0 as ::core::ffi::c_uint))
                .wrapping_sub(8 as ::core::ffi::c_uint)
                .wrapping_sub(1 as ::core::ffi::c_uint) as size_t
        {
        } else {
            __assert_fail(
                b"chunk_size <= MAX_CHUNK_PAYLOAD\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxi.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                150 as ::core::ffi::c_uint,
                b"size_t SizeWithPadding(size_t)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return (CHUNK_HEADER_SIZE as size_t).wrapping_add(
        chunk_size.wrapping_add(1 as size_t) & !(1 as ::core::ffi::c_uint) as size_t,
    );
}
#[inline]
unsafe extern "C" fn ChunkDiskSize(mut chunk: *const WebPChunk) -> size_t {
    let data_size: size_t = (*chunk).data_.size;
    return SizeWithPadding(data_size);
}
#[inline]
unsafe extern "C" fn PutLE16(data: *mut uint8_t, mut val: ::core::ffi::c_int) {
    '_c2rust_label: {
        if val < (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"val < (1 << 16)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/utils.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_uint,
                b"void PutLE16(uint8_t *const, int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    *data.offset(0 as ::core::ffi::c_int as isize) =
        (val >> 0 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
    *data.offset(1 as ::core::ffi::c_int as isize) =
        (val >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn PutLE32(data: *mut uint8_t, mut val: uint32_t) {
    PutLE16(data, (val & 0xffff as uint32_t) as ::core::ffi::c_int);
    PutLE16(
        data.offset(2 as ::core::ffi::c_int as isize),
        (val >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
}
pub const UNDEFINED_CHUNK_SIZE: uint32_t = -(1 as ::core::ffi::c_int) as uint32_t;
#[no_mangle]
pub static mut kChunks: [ChunkInfo; 11] = [
    ChunkInfo {
        tag: ('V' as i32
            | ('P' as i32) << 8 as ::core::ffi::c_int
            | ('8' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('X' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
        id: WEBP_CHUNK_VP8X,
        size: VP8X_CHUNK_SIZE as uint32_t,
    },
    ChunkInfo {
        tag: ('I' as i32
            | ('C' as i32) << 8 as ::core::ffi::c_int
            | ('C' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('P' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
        id: WEBP_CHUNK_ICCP,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: ('A' as i32
            | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('I' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('M' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
        id: WEBP_CHUNK_ANIM,
        size: ANIM_CHUNK_SIZE as uint32_t,
    },
    ChunkInfo {
        tag: ('A' as i32
            | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('M' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('F' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
        id: WEBP_CHUNK_ANMF,
        size: ANMF_CHUNK_SIZE as uint32_t,
    },
    ChunkInfo {
        tag: ('A' as i32
            | ('L' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('H' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
        id: WEBP_CHUNK_ALPHA,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: ('V' as i32
            | ('P' as i32) << 8 as ::core::ffi::c_int
            | ('8' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | (' ' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
        id: WEBP_CHUNK_IMAGE,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: ('V' as i32
            | ('P' as i32) << 8 as ::core::ffi::c_int
            | ('8' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('L' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
        id: WEBP_CHUNK_IMAGE,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: ('E' as i32
            | ('X' as i32) << 8 as ::core::ffi::c_int
            | ('I' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('F' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
        id: WEBP_CHUNK_EXIF,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: ('X' as i32
            | ('M' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | (' ' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
        id: WEBP_CHUNK_XMP,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: NIL_TAG as uint32_t,
        id: WEBP_CHUNK_UNKNOWN,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: NIL_TAG as uint32_t,
        id: WEBP_CHUNK_NIL,
        size: UNDEFINED_CHUNK_SIZE,
    },
];
#[no_mangle]
pub unsafe extern "C" fn WebPGetMuxVersion() -> ::core::ffi::c_int {
    return MUX_MAJ_VERSION << 16 as ::core::ffi::c_int
        | MUX_MIN_VERSION << 8 as ::core::ffi::c_int
        | MUX_REV_VERSION;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkInit(chunk: *mut WebPChunk) {
    '_c2rust_label: {
        if !chunk.is_null() {
        } else {
            __assert_fail(
                b"chunk\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                46 as ::core::ffi::c_uint,
                b"void ChunkInit(WebPChunk *const)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        chunk as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<WebPChunk>() as size_t,
    );
    (*chunk).tag_ = NIL_TAG as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkRelease(chunk: *mut WebPChunk) -> *mut WebPChunk {
    let mut next: *mut WebPChunk = ::core::ptr::null_mut::<WebPChunk>();
    if chunk.is_null() {
        return ::core::ptr::null_mut::<WebPChunk>();
    }
    if (*chunk).owner_ != 0 {
        WebPDataClear(&raw mut (*chunk).data_);
    }
    next = (*chunk).next_;
    ChunkInit(chunk);
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkGetIndexFromTag(mut tag: uint32_t) -> CHUNK_INDEX {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while kChunks[i as usize].tag != NIL_TAG as uint32_t {
        if tag == kChunks[i as usize].tag {
            return i as CHUNK_INDEX;
        }
        i += 1;
    }
    return IDX_UNKNOWN;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkGetIdFromTag(mut tag: uint32_t) -> WebPChunkId {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while kChunks[i as usize].tag != NIL_TAG as uint32_t {
        if tag == kChunks[i as usize].tag {
            return kChunks[i as usize].id;
        }
        i += 1;
    }
    return WEBP_CHUNK_UNKNOWN;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkGetTagFromFourCC(mut fourcc: *const ::core::ffi::c_char) -> uint32_t {
    return (*fourcc.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        | (*fourcc.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | (*fourcc.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as uint32_t
        | (*fourcc.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
            << 24 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkGetIndexFromFourCC(
    mut fourcc: *const ::core::ffi::c_char,
) -> CHUNK_INDEX {
    let tag: uint32_t = ChunkGetTagFromFourCC(fourcc) as uint32_t;
    return ChunkGetIndexFromTag(tag);
}
unsafe extern "C" fn ChunkSearchNextInList(
    mut chunk: *mut WebPChunk,
    mut tag: uint32_t,
) -> *mut WebPChunk {
    while !chunk.is_null() && (*chunk).tag_ != tag {
        chunk = (*chunk).next_;
    }
    return chunk;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkSearchList(
    mut first: *mut WebPChunk,
    mut nth: uint32_t,
    mut tag: uint32_t,
) -> *mut WebPChunk {
    let mut iter: uint32_t = nth;
    first = ChunkSearchNextInList(first, tag);
    if first.is_null() {
        return ::core::ptr::null_mut::<WebPChunk>();
    }
    loop {
        iter = iter.wrapping_sub(1);
        if !(iter != 0 as uint32_t) {
            break;
        }
        let mut next_chunk: *mut WebPChunk = ChunkSearchNextInList((*first).next_, tag);
        if next_chunk.is_null() {
            break;
        }
        first = next_chunk;
    }
    return if nth > 0 as uint32_t && iter > 0 as uint32_t {
        ::core::ptr::null_mut::<WebPChunk>()
    } else {
        first
    };
}
#[no_mangle]
pub unsafe extern "C" fn ChunkAssignData(
    mut chunk: *mut WebPChunk,
    data: *const WebPData,
    mut copy_data: ::core::ffi::c_int,
    mut tag: uint32_t,
) -> WebPMuxError {
    if tag == kChunks[IDX_VP8X as ::core::ffi::c_int as usize].tag
        || tag == kChunks[IDX_ANIM as ::core::ffi::c_int as usize].tag
    {
        copy_data = 1 as ::core::ffi::c_int;
    }
    ChunkRelease(chunk);
    if !data.is_null() {
        if copy_data != 0 {
            if WebPDataCopy(data, &raw mut (*chunk).data_) == 0 {
                return WEBP_MUX_MEMORY_ERROR;
            }
            (*chunk).owner_ = 1 as ::core::ffi::c_int;
        } else {
            (*chunk).data_ = *data;
        }
    }
    (*chunk).tag_ = tag;
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkSetHead(
    chunk: *mut WebPChunk,
    chunk_list: *mut *mut WebPChunk,
) -> WebPMuxError {
    let mut new_chunk: *mut WebPChunk = ::core::ptr::null_mut::<WebPChunk>();
    '_c2rust_label: {
        if !chunk_list.is_null() {
        } else {
            __assert_fail(
                b"chunk_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                142 as ::core::ffi::c_uint,
                b"WebPMuxError ChunkSetHead(WebPChunk *const, WebPChunk **const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if !(*chunk_list).is_null() {
        return WEBP_MUX_NOT_FOUND;
    }
    new_chunk = WebPSafeMalloc(1 as uint64_t, ::core::mem::size_of::<WebPChunk>() as size_t)
        as *mut WebPChunk;
    if new_chunk.is_null() {
        return WEBP_MUX_MEMORY_ERROR;
    }
    *new_chunk = *chunk;
    (*chunk).owner_ = 0 as ::core::ffi::c_int;
    (*new_chunk).next_ = ::core::ptr::null_mut::<WebPChunk>();
    *chunk_list = new_chunk;
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkAppend(
    chunk: *mut WebPChunk,
    chunk_list: *mut *mut *mut WebPChunk,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    '_c2rust_label: {
        if !chunk_list.is_null() && !(*chunk_list).is_null() {
        } else {
            __assert_fail(
                b"chunk_list != NULL && *chunk_list != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                159 as ::core::ffi::c_uint,
                b"WebPMuxError ChunkAppend(WebPChunk *const, WebPChunk ***const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (**chunk_list).is_null() {
        err = ChunkSetHead(chunk, *chunk_list);
    } else {
        let mut last_chunk: *mut WebPChunk = **chunk_list;
        while !(*last_chunk).next_.is_null() {
            last_chunk = (*last_chunk).next_;
        }
        err = ChunkSetHead(chunk, &raw mut (*last_chunk).next_);
        if err as ::core::ffi::c_int == WEBP_MUX_OK as ::core::ffi::c_int {
            *chunk_list = &raw mut (*last_chunk).next_;
        }
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkDelete(chunk: *mut WebPChunk) -> *mut WebPChunk {
    let next: *mut WebPChunk = ChunkRelease(chunk) as *mut WebPChunk;
    WebPSafeFree(chunk as *mut ::core::ffi::c_void);
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkListDelete(chunk_list: *mut *mut WebPChunk) {
    while !(*chunk_list).is_null() {
        *chunk_list = ChunkDelete(*chunk_list);
    }
}
unsafe extern "C" fn ChunkEmit(chunk: *const WebPChunk, mut dst: *mut uint8_t) -> *mut uint8_t {
    let chunk_size: size_t = (*chunk).data_.size;
    '_c2rust_label: {
        if !chunk.is_null() {
        } else {
            __assert_fail(
                b"chunk\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                192 as ::core::ffi::c_uint,
                b"uint8_t *ChunkEmit(const WebPChunk *const, uint8_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*chunk).tag_ != 0 as uint32_t {
        } else {
            __assert_fail(
                b"chunk->tag_ != NIL_TAG\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_uint,
                b"uint8_t *ChunkEmit(const WebPChunk *const, uint8_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    PutLE32(dst.offset(0 as ::core::ffi::c_int as isize), (*chunk).tag_);
    PutLE32(dst.offset(TAG_SIZE as isize), chunk_size as uint32_t);
    '_c2rust_label_1: {
        if chunk_size == chunk_size as uint32_t as size_t {
        } else {
            __assert_fail(
                b"chunk_size == (uint32_t)chunk_size\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                196 as ::core::ffi::c_uint,
                b"uint8_t *ChunkEmit(const WebPChunk *const, uint8_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    memcpy(
        dst.offset(CHUNK_HEADER_SIZE as isize) as *mut ::core::ffi::c_void,
        (*chunk).data_.bytes as *const ::core::ffi::c_void,
        chunk_size,
    );
    if chunk_size & 1 as size_t != 0 {
        *dst.offset((CHUNK_HEADER_SIZE as size_t).wrapping_add(chunk_size) as isize) = 0 as uint8_t;
    }
    return dst.offset(ChunkDiskSize(chunk) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ChunkListEmit(
    mut chunk_list: *const WebPChunk,
    mut dst: *mut uint8_t,
) -> *mut uint8_t {
    while !chunk_list.is_null() {
        dst = ChunkEmit(chunk_list, dst);
        chunk_list = (*chunk_list).next_;
    }
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkListDiskSize(mut chunk_list: *const WebPChunk) -> size_t {
    let mut size: size_t = 0 as size_t;
    while !chunk_list.is_null() {
        size = size.wrapping_add(ChunkDiskSize(chunk_list));
        chunk_list = (*chunk_list).next_;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageInit(wpi: *mut WebPMuxImage) {
    '_c2rust_label: {
        if !wpi.is_null() {
        } else {
            __assert_fail(
                b"wpi\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_uint,
                b"void MuxImageInit(WebPMuxImage *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        wpi as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<WebPMuxImage>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageRelease(wpi: *mut WebPMuxImage) -> *mut WebPMuxImage {
    let mut next: *mut WebPMuxImage = ::core::ptr::null_mut::<WebPMuxImage>();
    if wpi.is_null() {
        return ::core::ptr::null_mut::<WebPMuxImage>();
    }
    ChunkListDelete(&raw mut (*wpi).header_);
    ChunkListDelete(&raw mut (*wpi).alpha_);
    ChunkListDelete(&raw mut (*wpi).img_);
    ChunkListDelete(&raw mut (*wpi).unknown_);
    next = (*wpi).next_;
    MuxImageInit(wpi);
    return next;
}
unsafe extern "C" fn GetChunkListFromId(
    wpi: *const WebPMuxImage,
    mut id: WebPChunkId,
) -> *mut *mut WebPChunk {
    '_c2rust_label: {
        if !wpi.is_null() {
        } else {
            __assert_fail(
                b"wpi != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                249 as ::core::ffi::c_uint,
                b"WebPChunk **GetChunkListFromId(const WebPMuxImage *const, WebPChunkId)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    match id as ::core::ffi::c_uint {
        3 => return &raw const (*wpi).header_ as *mut *mut WebPChunk,
        5 => return &raw const (*wpi).alpha_ as *mut *mut WebPChunk,
        6 => return &raw const (*wpi).img_ as *mut *mut WebPChunk,
        _ => return ::core::ptr::null_mut::<*mut WebPChunk>(),
    };
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageCount(
    mut wpi_list: *const WebPMuxImage,
    mut id: WebPChunkId,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *const WebPMuxImage = ::core::ptr::null::<WebPMuxImage>();
    current = wpi_list;
    while !current.is_null() {
        if id as ::core::ffi::c_uint == WEBP_CHUNK_NIL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            count += 1;
        } else {
            let wpi_chunk: *const WebPChunk = *GetChunkListFromId(current, id);
            if !wpi_chunk.is_null() {
                let wpi_chunk_id: WebPChunkId = ChunkGetIdFromTag((*wpi_chunk).tag_) as WebPChunkId;
                if wpi_chunk_id as ::core::ffi::c_uint == id as ::core::ffi::c_uint {
                    count += 1;
                }
            }
        }
        current = (*current).next_;
    }
    return count;
}
unsafe extern "C" fn SearchImageToGetOrDelete(
    mut wpi_list: *mut *mut WebPMuxImage,
    mut nth: uint32_t,
    location: *mut *mut *mut WebPMuxImage,
) -> ::core::ffi::c_int {
    let mut count: uint32_t = 0 as uint32_t;
    '_c2rust_label: {
        if !wpi_list.is_null() {
        } else {
            __assert_fail(
                b"wpi_list\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                281 as ::core::ffi::c_uint,
                b"int SearchImageToGetOrDelete(WebPMuxImage **, uint32_t, WebPMuxImage ***const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    *location = wpi_list;
    if nth == 0 as uint32_t {
        nth = MuxImageCount(*wpi_list, WEBP_CHUNK_NIL) as uint32_t;
        if nth == 0 as uint32_t {
            return 0 as ::core::ffi::c_int;
        }
    }
    while !(*wpi_list).is_null() {
        let cur_wpi: *mut WebPMuxImage = *wpi_list;
        count = count.wrapping_add(1);
        if count == nth {
            return 1 as ::core::ffi::c_int;
        }
        wpi_list = &raw mut (*cur_wpi).next_;
        *location = wpi_list;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImagePush(
    mut wpi: *const WebPMuxImage,
    mut wpi_list: *mut *mut WebPMuxImage,
) -> WebPMuxError {
    let mut new_wpi: *mut WebPMuxImage = ::core::ptr::null_mut::<WebPMuxImage>();
    while !(*wpi_list).is_null() {
        let cur_wpi: *mut WebPMuxImage = *wpi_list;
        if (*cur_wpi).next_.is_null() {
            break;
        }
        wpi_list = &raw mut (*cur_wpi).next_;
    }
    new_wpi = WebPSafeMalloc(
        1 as uint64_t,
        ::core::mem::size_of::<WebPMuxImage>() as size_t,
    ) as *mut WebPMuxImage;
    if new_wpi.is_null() {
        return WEBP_MUX_MEMORY_ERROR;
    }
    *new_wpi = *wpi;
    (*new_wpi).next_ = ::core::ptr::null_mut::<WebPMuxImage>();
    if !(*wpi_list).is_null() {
        (**wpi_list).next_ = new_wpi;
    } else {
        *wpi_list = new_wpi;
    }
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageDelete(wpi: *mut WebPMuxImage) -> *mut WebPMuxImage {
    let next: *mut WebPMuxImage = MuxImageRelease(wpi) as *mut WebPMuxImage;
    WebPSafeFree(wpi as *mut ::core::ffi::c_void);
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageDeleteNth(
    mut wpi_list: *mut *mut WebPMuxImage,
    mut nth: uint32_t,
) -> WebPMuxError {
    '_c2rust_label: {
        if !wpi_list.is_null() {
        } else {
            __assert_fail(
                b"wpi_list\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                335 as ::core::ffi::c_uint,
                b"WebPMuxError MuxImageDeleteNth(WebPMuxImage **, uint32_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if SearchImageToGetOrDelete(wpi_list, nth, &raw mut wpi_list) == 0 {
        return WEBP_MUX_NOT_FOUND;
    }
    *wpi_list = MuxImageDelete(*wpi_list);
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageGetNth(
    mut wpi_list: *mut *const WebPMuxImage,
    mut nth: uint32_t,
    mut wpi: *mut *mut WebPMuxImage,
) -> WebPMuxError {
    '_c2rust_label: {
        if !wpi_list.is_null() {
        } else {
            __assert_fail(
                b"wpi_list\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                348 as ::core::ffi::c_uint,
                b"WebPMuxError MuxImageGetNth(const WebPMuxImage **, uint32_t, WebPMuxImage **)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !wpi.is_null() {
        } else {
            __assert_fail(
                b"wpi\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                349 as ::core::ffi::c_uint,
                b"WebPMuxError MuxImageGetNth(const WebPMuxImage **, uint32_t, WebPMuxImage **)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if SearchImageToGetOrDelete(
        wpi_list as *mut *mut WebPMuxImage,
        nth,
        &raw mut wpi_list as *mut *mut *mut WebPMuxImage,
    ) == 0
    {
        return WEBP_MUX_NOT_FOUND;
    }
    *wpi = *wpi_list as *mut WebPMuxImage;
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageDiskSize(wpi: *const WebPMuxImage) -> size_t {
    let mut size: size_t = 0 as size_t;
    if !(*wpi).header_.is_null() {
        size = size.wrapping_add(ChunkDiskSize((*wpi).header_));
    }
    if !(*wpi).alpha_.is_null() {
        size = size.wrapping_add(ChunkDiskSize((*wpi).alpha_));
    }
    if !(*wpi).img_.is_null() {
        size = size.wrapping_add(ChunkDiskSize((*wpi).img_));
    }
    if !(*wpi).unknown_.is_null() {
        size = size.wrapping_add(ChunkListDiskSize((*wpi).unknown_));
    }
    return size;
}
unsafe extern "C" fn ChunkEmitSpecial(
    header: *const WebPChunk,
    mut total_size: size_t,
    mut dst: *mut uint8_t,
) -> *mut uint8_t {
    let header_size: size_t = (*header).data_.size;
    let offset_to_next: size_t = total_size.wrapping_sub(CHUNK_HEADER_SIZE as size_t);
    '_c2rust_label: {
        if (*header).tag_ == kChunks[IDX_ANMF as ::core::ffi::c_int as usize].tag {
        } else {
            __assert_fail(
                b"header->tag_ == kChunks[IDX_ANMF].tag\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                376 as ::core::ffi::c_uint,
                b"uint8_t *ChunkEmitSpecial(const WebPChunk *const, size_t, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    PutLE32(dst.offset(0 as ::core::ffi::c_int as isize), (*header).tag_);
    PutLE32(dst.offset(TAG_SIZE as isize), offset_to_next as uint32_t);
    '_c2rust_label_0: {
        if header_size == header_size as uint32_t as size_t {
        } else {
            __assert_fail(
                b"header_size == (uint32_t)header_size\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                379 as ::core::ffi::c_uint,
                b"uint8_t *ChunkEmitSpecial(const WebPChunk *const, size_t, uint8_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    memcpy(
        dst.offset(CHUNK_HEADER_SIZE as isize) as *mut ::core::ffi::c_void,
        (*header).data_.bytes as *const ::core::ffi::c_void,
        header_size,
    );
    if header_size & 1 as size_t != 0 {
        *dst.offset((CHUNK_HEADER_SIZE as size_t).wrapping_add(header_size) as isize) =
            0 as uint8_t;
    }
    return dst.offset(ChunkDiskSize(header) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageEmit(
    wpi: *const WebPMuxImage,
    mut dst: *mut uint8_t,
) -> *mut uint8_t {
    '_c2rust_label: {
        if !wpi.is_null() {
        } else {
            __assert_fail(
                b"wpi\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                392 as ::core::ffi::c_uint,
                b"uint8_t *MuxImageEmit(const WebPMuxImage *const, uint8_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if !(*wpi).header_.is_null() {
        dst = ChunkEmitSpecial((*wpi).header_, MuxImageDiskSize(wpi), dst);
    }
    if !(*wpi).alpha_.is_null() {
        dst = ChunkEmit((*wpi).alpha_, dst);
    }
    if !(*wpi).img_.is_null() {
        dst = ChunkEmit((*wpi).img_, dst);
    }
    if !(*wpi).unknown_.is_null() {
        dst = ChunkListEmit((*wpi).unknown_, dst);
    }
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn MuxHasAlpha(mut images: *const WebPMuxImage) -> ::core::ffi::c_int {
    while !images.is_null() {
        if (*images).has_alpha_ != 0 {
            return 1 as ::core::ffi::c_int;
        }
        images = (*images).next_;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MuxEmitRiffHeader(data: *mut uint8_t, mut size: size_t) -> *mut uint8_t {
    PutLE32(
        data.offset(0 as ::core::ffi::c_int as isize),
        ('R' as i32
            | ('I' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('F' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
    );
    PutLE32(
        data.offset(TAG_SIZE as isize),
        (size as uint32_t).wrapping_sub(CHUNK_HEADER_SIZE as uint32_t),
    );
    '_c2rust_label: {
        if size == size as uint32_t as size_t {
        } else {
            __assert_fail(
                b"size == (uint32_t)size\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                416 as ::core::ffi::c_uint,
                b"uint8_t *MuxEmitRiffHeader(uint8_t *const, size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    PutLE32(
        data.offset(TAG_SIZE as isize)
            .offset(CHUNK_SIZE_BYTES as isize),
        ('W' as i32
            | ('E' as i32) << 8 as ::core::ffi::c_int
            | ('B' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('P' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
    );
    return data.offset(RIFF_HEADER_SIZE as isize);
}
#[no_mangle]
pub unsafe extern "C" fn MuxGetChunkListFromId(
    mut mux: *const WebPMux,
    mut id: WebPChunkId,
) -> *mut *mut WebPChunk {
    '_c2rust_label: {
        if !mux.is_null() {
        } else {
            __assert_fail(
                b"mux != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxinternal.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                422 as ::core::ffi::c_uint,
                b"WebPChunk **MuxGetChunkListFromId(const WebPMux *, WebPChunkId)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    match id as ::core::ffi::c_uint {
        0 => return &raw const (*mux).vp8x_ as *mut *mut WebPChunk,
        1 => return &raw const (*mux).iccp_ as *mut *mut WebPChunk,
        2 => return &raw const (*mux).anim_ as *mut *mut WebPChunk,
        7 => return &raw const (*mux).exif_ as *mut *mut WebPChunk,
        8 => return &raw const (*mux).xmp_ as *mut *mut WebPChunk,
        _ => return &raw const (*mux).unknown_ as *mut *mut WebPChunk,
    };
}
unsafe extern "C" fn IsNotCompatible(
    mut feature: ::core::ffi::c_int,
    mut num_items: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return ((feature != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        != (num_items > 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
pub const NO_FLAG: WebPFeatureFlags = 0 as WebPFeatureFlags;
unsafe extern "C" fn ValidateChunk(
    mux: *const WebPMux,
    mut idx: CHUNK_INDEX,
    mut feature: WebPFeatureFlags,
    mut vp8x_flags: uint32_t,
    mut max: ::core::ffi::c_int,
    mut num: *mut ::core::ffi::c_int,
) -> WebPMuxError {
    let err: WebPMuxError = WebPMuxNumChunks(mux, kChunks[idx as usize].id, num) as WebPMuxError;
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    if max > -(1 as ::core::ffi::c_int) && *num > max {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if feature as ::core::ffi::c_uint != NO_FLAG as ::core::ffi::c_uint
        && IsNotCompatible(
            (vp8x_flags & feature as uint32_t) as ::core::ffi::c_int,
            *num,
        ) != 0
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn MuxValidate(mux: *const WebPMux) -> WebPMuxError {
    let mut num_iccp: ::core::ffi::c_int = 0;
    let mut num_exif: ::core::ffi::c_int = 0;
    let mut num_xmp: ::core::ffi::c_int = 0;
    let mut num_anim: ::core::ffi::c_int = 0;
    let mut num_frames: ::core::ffi::c_int = 0;
    let mut num_vp8x: ::core::ffi::c_int = 0;
    let mut num_images: ::core::ffi::c_int = 0;
    let mut num_alpha: ::core::ffi::c_int = 0;
    let mut flags: uint32_t = 0;
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if (*mux).images_.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = WebPMuxGetFeatures(mux, &raw mut flags);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_ICCP,
        ICCP_FLAG,
        flags,
        1 as ::core::ffi::c_int,
        &raw mut num_iccp,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_EXIF,
        EXIF_FLAG,
        flags,
        1 as ::core::ffi::c_int,
        &raw mut num_exif,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_XMP,
        XMP_FLAG,
        flags,
        1 as ::core::ffi::c_int,
        &raw mut num_xmp,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_ANIM,
        NO_FLAG,
        flags,
        1 as ::core::ffi::c_int,
        &raw mut num_anim,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_ANMF,
        NO_FLAG,
        flags,
        -(1 as ::core::ffi::c_int),
        &raw mut num_frames,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    let has_animation: ::core::ffi::c_int =
        (flags & ANIMATION_FLAG as ::core::ffi::c_int as uint32_t != 0) as ::core::ffi::c_int;
    if has_animation != 0
        && (num_anim == 0 as ::core::ffi::c_int || num_frames == 0 as ::core::ffi::c_int)
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if has_animation == 0
        && (num_anim == 1 as ::core::ffi::c_int || num_frames > 0 as ::core::ffi::c_int)
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if has_animation == 0 {
        let mut images: *const WebPMuxImage = (*mux).images_;
        if images.is_null() || !(*images).next_.is_null() {
            return WEBP_MUX_INVALID_ARGUMENT;
        }
        if (*mux).canvas_width_ > 0 as ::core::ffi::c_int {
            if (*images).width_ != (*mux).canvas_width_
                || (*images).height_ != (*mux).canvas_height_
            {
                return WEBP_MUX_INVALID_ARGUMENT;
            }
        }
    }
    err = ValidateChunk(
        mux,
        IDX_VP8X,
        NO_FLAG,
        flags,
        1 as ::core::ffi::c_int,
        &raw mut num_vp8x,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_VP8,
        NO_FLAG,
        flags,
        -(1 as ::core::ffi::c_int),
        &raw mut num_images,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    if num_vp8x == 0 as ::core::ffi::c_int && num_images != 1 as ::core::ffi::c_int {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if MuxHasAlpha((*mux).images_) != 0 {
        if num_vp8x > 0 as ::core::ffi::c_int {
            if flags & ALPHA_FLAG as ::core::ffi::c_int as uint32_t == 0 {
                return WEBP_MUX_INVALID_ARGUMENT;
            }
        } else {
            err = WebPMuxNumChunks(mux, WEBP_CHUNK_ALPHA, &raw mut num_alpha);
            if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
                return err;
            }
            if num_alpha > 0 as ::core::ffi::c_int {
                return WEBP_MUX_INVALID_ARGUMENT;
            }
        }
    }
    return WEBP_MUX_OK;
}
