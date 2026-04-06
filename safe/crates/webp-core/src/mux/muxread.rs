extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn VP8GetInfo(
        data: *const uint8_t,
        data_size: size_t,
        chunk_size: size_t,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8LGetInfo(
        data: *const uint8_t,
        data_size: size_t,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
        has_alpha: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WebPNewInternal(_: ::core::ffi::c_int) -> *mut WebPMux;
    fn WebPMuxDelete(mux: *mut WebPMux);
    static kChunks: [ChunkInfo; 11];
    fn ChunkInit(chunk: *mut WebPChunk);
    fn ChunkGetIdFromTag(tag: uint32_t) -> WebPChunkId;
    fn ChunkGetTagFromFourCC(fourcc: *const ::core::ffi::c_char) -> uint32_t;
    fn ChunkGetIndexFromFourCC(fourcc: *const ::core::ffi::c_char) -> CHUNK_INDEX;
    fn ChunkSearchList(
        first: *mut WebPChunk,
        nth: uint32_t,
        tag: uint32_t,
    ) -> *mut WebPChunk;
    fn ChunkAssignData(
        chunk: *mut WebPChunk,
        data: *const WebPData,
        copy_data: ::core::ffi::c_int,
        tag: uint32_t,
    ) -> WebPMuxError;
    fn ChunkSetHead(
        chunk: *mut WebPChunk,
        chunk_list: *mut *mut WebPChunk,
    ) -> WebPMuxError;
    fn ChunkAppend(
        chunk: *mut WebPChunk,
        chunk_list: *mut *mut *mut WebPChunk,
    ) -> WebPMuxError;
    fn ChunkRelease(chunk: *mut WebPChunk) -> *mut WebPChunk;
    fn ChunkDelete(chunk: *mut WebPChunk) -> *mut WebPChunk;
    fn ChunkListEmit(chunk_list: *const WebPChunk, dst: *mut uint8_t) -> *mut uint8_t;
    fn MuxImageInit(wpi: *mut WebPMuxImage);
    fn MuxImageDelete(wpi: *mut WebPMuxImage) -> *mut WebPMuxImage;
    fn MuxImageCount(
        wpi_list: *const WebPMuxImage,
        id: WebPChunkId,
    ) -> ::core::ffi::c_int;
    fn MuxImagePush(
        wpi: *const WebPMuxImage,
        wpi_list: *mut *mut WebPMuxImage,
    ) -> WebPMuxError;
    fn MuxImageGetNth(
        wpi_list: *mut *const WebPMuxImage,
        nth: uint32_t,
        wpi: *mut *mut WebPMuxImage,
    ) -> WebPMuxError;
    fn MuxEmitRiffHeader(data: *mut uint8_t, size: size_t) -> *mut uint8_t;
    fn MuxGetChunkListFromId(
        mux: *const WebPMux,
        id: WebPChunkId,
    ) -> *mut *mut WebPChunk;
    fn MuxValidate(mux: *const WebPMux) -> WebPMuxError;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
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
pub type WebPMuxAnimDispose = ::core::ffi::c_uint;
pub const WEBP_MUX_DISPOSE_BACKGROUND: WebPMuxAnimDispose = 1;
pub const WEBP_MUX_DISPOSE_NONE: WebPMuxAnimDispose = 0;
pub type WebPMuxAnimBlend = ::core::ffi::c_uint;
pub const WEBP_MUX_NO_BLEND: WebPMuxAnimBlend = 1;
pub const WEBP_MUX_BLEND: WebPMuxAnimBlend = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMuxFrameInfo {
    pub bitstream: WebPData,
    pub x_offset: ::core::ffi::c_int,
    pub y_offset: ::core::ffi::c_int,
    pub duration: ::core::ffi::c_int,
    pub id: WebPChunkId,
    pub dispose_method: WebPMuxAnimDispose,
    pub blend_method: WebPMuxAnimBlend,
    pub pad: [uint32_t; 1],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMuxAnimParams {
    pub bgcolor: uint32_t,
    pub loop_count: ::core::ffi::c_int,
}
pub type WebPMuxError = ::core::ffi::c_int;
pub const WEBP_MUX_NOT_ENOUGH_DATA: WebPMuxError = -4;
pub const WEBP_MUX_MEMORY_ERROR: WebPMuxError = -3;
pub const WEBP_MUX_BAD_DATA: WebPMuxError = -2;
pub const WEBP_MUX_INVALID_ARGUMENT: WebPMuxError = -1;
pub const WEBP_MUX_NOT_FOUND: WebPMuxError = 0;
pub const WEBP_MUX_OK: WebPMuxError = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChunkInfo {
    pub tag: uint32_t,
    pub id: WebPChunkId,
    pub size: uint32_t,
}
pub const IDX_VP8L: CHUNK_INDEX = 6;
pub const IDX_ANMF: CHUNK_INDEX = 3;
pub const IDX_VP8X: CHUNK_INDEX = 0;
pub const IDX_VP8: CHUNK_INDEX = 5;
pub type CHUNK_INDEX = ::core::ffi::c_uint;
pub const IDX_LAST_CHUNK: CHUNK_INDEX = 11;
pub const IDX_NIL: CHUNK_INDEX = 10;
pub const IDX_UNKNOWN: CHUNK_INDEX = 9;
pub const IDX_XMP: CHUNK_INDEX = 8;
pub const IDX_EXIF: CHUNK_INDEX = 7;
pub const IDX_ALPHA: CHUNK_INDEX = 4;
pub const IDX_ANIM: CHUNK_INDEX = 2;
pub const IDX_ICCP: CHUNK_INDEX = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const TAG_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNK_HEADER_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RIFF_HEADER_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const ANMF_CHUNK_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const VP8X_CHUNK_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MAX_IMAGE_AREA: ::core::ffi::c_ulonglong = (1 as ::core::ffi::c_ulonglong)
    << 32 as ::core::ffi::c_int;
pub const MAX_CHUNK_PAYLOAD: ::core::ffi::c_uint = (!(0 as ::core::ffi::c_uint))
    .wrapping_sub(CHUNK_HEADER_SIZE as ::core::ffi::c_uint)
    .wrapping_sub(1 as ::core::ffi::c_uint);
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
pub const WEBP_MUX_ABI_VERSION: ::core::ffi::c_int = 0x108 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPMuxNew() -> *mut WebPMux {
    return WebPNewInternal(WEBP_MUX_ABI_VERSION);
}
pub const NIL_TAG: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn SizeWithPadding(mut chunk_size: size_t) -> size_t {
    '_c2rust_label: {
        if chunk_size
            <= (!(0 as ::core::ffi::c_uint))
                .wrapping_sub(8 as ::core::ffi::c_uint)
                .wrapping_sub(1 as ::core::ffi::c_uint) as size_t
        {} else {
            __assert_fail(
                b"chunk_size <= MAX_CHUNK_PAYLOAD\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxi.h\0"
                    as *const u8 as *const ::core::ffi::c_char,
                150 as ::core::ffi::c_uint,
                b"size_t SizeWithPadding(size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (CHUNK_HEADER_SIZE as size_t)
        .wrapping_add(
            chunk_size.wrapping_add(1 as size_t) & !(1 as ::core::ffi::c_uint) as size_t,
        );
}
#[inline]
unsafe extern "C" fn ChunkDiskSize(mut chunk: *const WebPChunk) -> size_t {
    let data_size: size_t = (*chunk).data_.size;
    return SizeWithPadding(data_size);
}
#[inline]
unsafe extern "C" fn IsWPI(mut id: WebPChunkId) -> ::core::ffi::c_int {
    match id as ::core::ffi::c_uint {
        3 | 5 | 6 => return 1 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
#[inline]
unsafe extern "C" fn GetLE16(data: *const uint8_t) -> ::core::ffi::c_int {
    return (*data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 0 as ::core::ffi::c_int
        | (*data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn GetLE24(data: *const uint8_t) -> ::core::ffi::c_int {
    return GetLE16(data)
        | (*data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn GetLE32(data: *const uint8_t) -> uint32_t {
    return GetLE16(data) as uint32_t
        | (GetLE16(data.offset(2 as ::core::ffi::c_int as isize)) as uint32_t)
            << 16 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn PutLE16(data: *mut uint8_t, mut val: ::core::ffi::c_int) {
    '_c2rust_label: {
        if val < (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"val < (1 << 16)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/utils.h\0"
                    as *const u8 as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_uint,
                b"void PutLE16(uint8_t *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *data.offset(0 as ::core::ffi::c_int as isize) = (val >> 0 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int) as uint8_t;
    *data.offset(1 as ::core::ffi::c_int as isize) = (val >> 8 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn PutLE24(data: *mut uint8_t, mut val: ::core::ffi::c_int) {
    '_c2rust_label: {
        if val < (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"val < (1 << 24)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/utils.h\0"
                    as *const u8 as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_uint,
                b"void PutLE24(uint8_t *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    PutLE16(data, val & 0xffff as ::core::ffi::c_int);
    *data.offset(2 as ::core::ffi::c_int as isize) = (val >> 16 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn PutLE32(data: *mut uint8_t, mut val: uint32_t) {
    PutLE16(data, (val & 0xffff as uint32_t) as ::core::ffi::c_int);
    PutLE16(
        data.offset(2 as ::core::ffi::c_int as isize),
        (val >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn MuxGet(
    mux: *const WebPMux,
    mut idx: CHUNK_INDEX,
    mut nth: uint32_t,
    data: *mut WebPData,
) -> WebPMuxError {
    '_c2rust_label: {
        if !mux.is_null() {} else {
            __assert_fail(
                b"mux != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                37 as ::core::ffi::c_uint,
                b"WebPMuxError MuxGet(const WebPMux *const, CHUNK_INDEX, uint32_t, WebPData *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if IsWPI(kChunks[idx as usize].id) == 0 {} else {
            __assert_fail(
                b"!IsWPI(kChunks[idx].id)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                38 as ::core::ffi::c_uint,
                b"WebPMuxError MuxGet(const WebPMux *const, CHUNK_INDEX, uint32_t, WebPData *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    WebPDataInit(data);
    if idx as ::core::ffi::c_uint
        == IDX_VP8X as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let chunk: *const WebPChunk = ChunkSearchList(
            (*mux).vp8x_,
            nth,
            kChunks[IDX_VP8X as ::core::ffi::c_int as usize].tag,
        );
        if !chunk.is_null() {
            *data = (*chunk).data_;
            return WEBP_MUX_OK;
        } else {
            return WEBP_MUX_NOT_FOUND
        }
    }
    if idx as ::core::ffi::c_uint
        == IDX_ICCP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let chunk_0: *const WebPChunk = ChunkSearchList(
            (*mux).iccp_,
            nth,
            kChunks[IDX_ICCP as ::core::ffi::c_int as usize].tag,
        );
        if !chunk_0.is_null() {
            *data = (*chunk_0).data_;
            return WEBP_MUX_OK;
        } else {
            return WEBP_MUX_NOT_FOUND
        }
    }
    if idx as ::core::ffi::c_uint
        == IDX_ANIM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let chunk_1: *const WebPChunk = ChunkSearchList(
            (*mux).anim_,
            nth,
            kChunks[IDX_ANIM as ::core::ffi::c_int as usize].tag,
        );
        if !chunk_1.is_null() {
            *data = (*chunk_1).data_;
            return WEBP_MUX_OK;
        } else {
            return WEBP_MUX_NOT_FOUND
        }
    }
    if idx as ::core::ffi::c_uint
        == IDX_EXIF as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let chunk_2: *const WebPChunk = ChunkSearchList(
            (*mux).exif_,
            nth,
            kChunks[IDX_EXIF as ::core::ffi::c_int as usize].tag,
        );
        if !chunk_2.is_null() {
            *data = (*chunk_2).data_;
            return WEBP_MUX_OK;
        } else {
            return WEBP_MUX_NOT_FOUND
        }
    }
    if idx as ::core::ffi::c_uint == IDX_XMP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let chunk_3: *const WebPChunk = ChunkSearchList(
            (*mux).xmp_,
            nth,
            kChunks[IDX_XMP as ::core::ffi::c_int as usize].tag,
        );
        if !chunk_3.is_null() {
            *data = (*chunk_3).data_;
            return WEBP_MUX_OK;
        } else {
            return WEBP_MUX_NOT_FOUND
        }
    }
    '_c2rust_label_1: {
        if idx as ::core::ffi::c_uint
            != IDX_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
        {} else {
            __assert_fail(
                b"idx != IDX_UNKNOWN\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                46 as ::core::ffi::c_uint,
                b"WebPMuxError MuxGet(const WebPMux *const, CHUNK_INDEX, uint32_t, WebPData *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return WEBP_MUX_NOT_FOUND;
}
unsafe extern "C" fn ChunkVerifyAndAssign(
    mut chunk: *mut WebPChunk,
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut riff_size: size_t,
    mut copy_data: ::core::ffi::c_int,
) -> WebPMuxError {
    let mut chunk_size: uint32_t = 0;
    let mut chunk_data: WebPData = WebPData {
        bytes: ::core::ptr::null::<uint8_t>(),
        size: 0,
    };
    if data_size < CHUNK_HEADER_SIZE as size_t {
        return WEBP_MUX_NOT_ENOUGH_DATA;
    }
    chunk_size = GetLE32(data.offset(TAG_SIZE as isize));
    if chunk_size > MAX_CHUNK_PAYLOAD as uint32_t {
        return WEBP_MUX_BAD_DATA;
    }
    let chunk_disk_size: size_t = SizeWithPadding(chunk_size as size_t) as size_t;
    if chunk_disk_size > riff_size {
        return WEBP_MUX_BAD_DATA;
    }
    if chunk_disk_size > data_size {
        return WEBP_MUX_NOT_ENOUGH_DATA;
    }
    chunk_data.bytes = data.offset(CHUNK_HEADER_SIZE as isize);
    chunk_data.size = chunk_size as size_t;
    return ChunkAssignData(
        chunk,
        &raw mut chunk_data,
        copy_data,
        GetLE32(data.offset(0 as ::core::ffi::c_int as isize)),
    );
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageFinalize(wpi: *mut WebPMuxImage) -> ::core::ffi::c_int {
    let img: *const WebPChunk = (*wpi).img_;
    let image: *const WebPData = &raw const (*img).data_;
    let is_lossless: ::core::ffi::c_int = ((*img).tag_
        == kChunks[IDX_VP8L as ::core::ffi::c_int as usize].tag) as ::core::ffi::c_int;
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut vp8l_has_alpha: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let ok: ::core::ffi::c_int = if is_lossless != 0 {
        VP8LGetInfo(
            (*image).bytes,
            (*image).size,
            &raw mut w,
            &raw mut h,
            &raw mut vp8l_has_alpha,
        ) as ::core::ffi::c_int
    } else {
        VP8GetInfo((*image).bytes, (*image).size, (*image).size, &raw mut w, &raw mut h)
            as ::core::ffi::c_int
    };
    '_c2rust_label: {
        if !img.is_null() {} else {
            __assert_fail(
                b"img != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_uint,
                b"int MuxImageFinalize(WebPMuxImage *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if ok != 0 {
        if is_lossless != 0 && !(*wpi).alpha_.is_null() {
            ChunkDelete((*wpi).alpha_);
            (*wpi).alpha_ = ::core::ptr::null_mut::<WebPChunk>();
        }
        (*wpi).width_ = w;
        (*wpi).height_ = h;
        (*wpi).has_alpha_ = (vp8l_has_alpha != 0 || !(*wpi).alpha_.is_null())
            as ::core::ffi::c_int;
    }
    return ok;
}
unsafe extern "C" fn MuxImageParse(
    chunk: *const WebPChunk,
    mut copy_data: ::core::ffi::c_int,
    wpi: *mut WebPMuxImage,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut bytes: *const uint8_t = (*chunk).data_.bytes;
    let mut size: size_t = (*chunk).data_.size;
    let last: *const uint8_t = if bytes.is_null() {
        ::core::ptr::null::<uint8_t>()
    } else {
        bytes.offset(size as isize)
    };
    let mut subchunk: WebPChunk = WebPChunk {
        tag_: 0,
        owner_: 0,
        data_: WebPData {
            bytes: ::core::ptr::null::<uint8_t>(),
            size: 0,
        },
        next_: ::core::ptr::null_mut::<WebPChunk>(),
    };
    let mut subchunk_size: size_t = 0;
    let mut unknown_chunk_list: *mut *mut WebPChunk = &raw mut (*wpi).unknown_;
    ChunkInit(&raw mut subchunk);
    '_c2rust_label: {
        if (*chunk).tag_ == kChunks[IDX_ANMF as ::core::ffi::c_int as usize].tag
        {} else {
            __assert_fail(
                b"chunk->tag_ == kChunks[IDX_ANMF].tag\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_uint,
                b"int MuxImageParse(const WebPChunk *const, int, WebPMuxImage *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*wpi).is_partial_ == 0 {} else {
            __assert_fail(
                b"!wpi->is_partial_\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_uint,
                b"int MuxImageParse(const WebPChunk *const, int, WebPMuxImage *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    let hdr_size: size_t = ANMF_CHUNK_SIZE as size_t;
    let temp: WebPData = WebPData {
        bytes: bytes,
        size: hdr_size,
    };
    if !(size < hdr_size) {
        if !(ChunkAssignData(
            &raw mut subchunk,
            &raw const temp,
            copy_data,
            (*chunk).tag_,
        ) as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int)
        {
            if !(ChunkSetHead(&raw mut subchunk, &raw mut (*wpi).header_)
                as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int)
            {
                (*wpi).is_partial_ = 1 as ::core::ffi::c_int;
                subchunk_size = ChunkDiskSize(&raw mut subchunk)
                    .wrapping_sub(CHUNK_HEADER_SIZE as size_t);
                bytes = bytes.offset(subchunk_size as isize);
                size = size.wrapping_sub(subchunk_size);
                loop {
                    if !(bytes != last) {
                        current_block = 4488286894823169796;
                        break;
                    }
                    ChunkInit(&raw mut subchunk);
                    if ChunkVerifyAndAssign(
                        &raw mut subchunk,
                        bytes,
                        size,
                        size,
                        copy_data,
                    ) as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
                    {
                        current_block = 14941718520305430063;
                        break;
                    }
                    match ChunkGetIdFromTag(subchunk.tag_) as ::core::ffi::c_uint {
                        5 => {
                            if !(*wpi).alpha_.is_null() {
                                current_block = 14941718520305430063;
                                break;
                            }
                            if ChunkSetHead(&raw mut subchunk, &raw mut (*wpi).alpha_)
                                as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
                            {
                                current_block = 14941718520305430063;
                                break;
                            }
                            (*wpi).is_partial_ = 1 as ::core::ffi::c_int;
                        }
                        6 => {
                            if !(*wpi).img_.is_null() {
                                current_block = 14941718520305430063;
                                break;
                            }
                            if ChunkSetHead(&raw mut subchunk, &raw mut (*wpi).img_)
                                as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
                            {
                                current_block = 14941718520305430063;
                                break;
                            }
                            if MuxImageFinalize(wpi) == 0 {
                                current_block = 14941718520305430063;
                                break;
                            }
                            (*wpi).is_partial_ = 0 as ::core::ffi::c_int;
                        }
                        9 => {
                            if (*wpi).is_partial_ != 0 {
                                current_block = 14941718520305430063;
                                break;
                            }
                            if ChunkAppend(
                                &raw mut subchunk,
                                &raw mut unknown_chunk_list,
                            ) as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
                            {
                                current_block = 14941718520305430063;
                                break;
                            }
                        }
                        _ => {
                            current_block = 14941718520305430063;
                            break;
                        }
                    }
                    subchunk_size = ChunkDiskSize(&raw mut subchunk);
                    bytes = bytes.offset(subchunk_size as isize);
                    size = size.wrapping_sub(subchunk_size);
                }
                match current_block {
                    14941718520305430063 => {}
                    _ => {
                        if !((*wpi).is_partial_ != 0) {
                            return 1 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    ChunkRelease(&raw mut subchunk);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxCreateInternal(
    mut bitstream: *const WebPData,
    mut copy_data: ::core::ffi::c_int,
    mut version: ::core::ffi::c_int,
) -> *mut WebPMux {
    let mut current_block: u64;
    let mut riff_size: size_t = 0;
    let mut tag: uint32_t = 0;
    let mut end: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut mux: *mut WebPMux = ::core::ptr::null_mut::<WebPMux>();
    let mut wpi: *mut WebPMuxImage = ::core::ptr::null_mut::<WebPMuxImage>();
    let mut data: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut size: size_t = 0;
    let mut chunk: WebPChunk = WebPChunk {
        tag_: 0,
        owner_: 0,
        data_: WebPData {
            bytes: ::core::ptr::null::<uint8_t>(),
            size: 0,
        },
        next_: ::core::ptr::null_mut::<WebPChunk>(),
    };
    let mut chunk_list_ends: [*mut *mut WebPChunk; 11] = [
        ::core::ptr::null_mut::<*mut WebPChunk>(),
        ::core::ptr::null_mut::<*mut WebPChunk>(),
        ::core::ptr::null_mut::<*mut WebPChunk>(),
        ::core::ptr::null_mut::<*mut WebPChunk>(),
        ::core::ptr::null_mut::<*mut WebPChunk>(),
        ::core::ptr::null_mut::<*mut WebPChunk>(),
        ::core::ptr::null_mut::<*mut WebPChunk>(),
        ::core::ptr::null_mut::<*mut WebPChunk>(),
        ::core::ptr::null_mut::<*mut WebPChunk>(),
        ::core::ptr::null_mut::<*mut WebPChunk>(),
        ::core::ptr::null_mut::<*mut WebPChunk>(),
    ];
    ChunkInit(&raw mut chunk);
    if version >> 8 as ::core::ffi::c_int
        != 0x108 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<WebPMux>();
    }
    if bitstream.is_null() {
        return ::core::ptr::null_mut::<WebPMux>();
    }
    data = (*bitstream).bytes;
    size = (*bitstream).size;
    if data.is_null() {
        return ::core::ptr::null_mut::<WebPMux>();
    }
    if size < (RIFF_HEADER_SIZE + CHUNK_HEADER_SIZE) as size_t {
        return ::core::ptr::null_mut::<WebPMux>();
    }
    if GetLE32(data.offset(0 as ::core::ffi::c_int as isize))
        != ('R' as i32 | ('I' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('F' as i32 as uint32_t) << 24 as ::core::ffi::c_int
        || GetLE32(data.offset(CHUNK_HEADER_SIZE as isize))
            != ('W' as i32 | ('E' as i32) << 8 as ::core::ffi::c_int
                | ('B' as i32) << 16 as ::core::ffi::c_int) as uint32_t
                | ('P' as i32 as uint32_t) << 24 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<WebPMux>();
    }
    mux = WebPMuxNew();
    if mux.is_null() {
        return ::core::ptr::null_mut::<WebPMux>();
    }
    tag = GetLE32(data.offset(RIFF_HEADER_SIZE as isize));
    if !(tag != kChunks[IDX_VP8 as ::core::ffi::c_int as usize].tag
        && tag != kChunks[IDX_VP8L as ::core::ffi::c_int as usize].tag
        && tag != kChunks[IDX_VP8X as ::core::ffi::c_int as usize].tag)
    {
        riff_size = GetLE32(data.offset(TAG_SIZE as isize)) as size_t;
        if !(riff_size > MAX_CHUNK_PAYLOAD as size_t) {
            riff_size = SizeWithPadding(riff_size);
            if !(riff_size < CHUNK_HEADER_SIZE as size_t) {
                if !(riff_size > size) {
                    if size > riff_size.wrapping_add(CHUNK_HEADER_SIZE as size_t) {
                        size = riff_size.wrapping_add(CHUNK_HEADER_SIZE as size_t);
                    }
                    end = data.offset(size as isize);
                    data = data.offset(RIFF_HEADER_SIZE as isize);
                    size = size.wrapping_sub(RIFF_HEADER_SIZE as size_t);
                    wpi = WebPSafeMalloc(
                        1 as uint64_t,
                        ::core::mem::size_of::<WebPMuxImage>() as size_t,
                    ) as *mut WebPMuxImage;
                    if !wpi.is_null() {
                        MuxImageInit(wpi);
                        loop {
                            if !(data != end) {
                                current_block = 16415152177862271243;
                                break;
                            }
                            let mut data_size: size_t = 0;
                            let mut id: WebPChunkId = WEBP_CHUNK_VP8X;
                            if ChunkVerifyAndAssign(
                                &raw mut chunk,
                                data,
                                size,
                                riff_size,
                                copy_data,
                            ) as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
                            {
                                current_block = 10874050330108021723;
                                break;
                            }
                            data_size = ChunkDiskSize(&raw mut chunk);
                            id = ChunkGetIdFromTag(chunk.tag_);
                            match id as ::core::ffi::c_uint {
                                5 => {
                                    if !(*wpi).alpha_.is_null() {
                                        current_block = 10874050330108021723;
                                        break;
                                    }
                                    if ChunkSetHead(&raw mut chunk, &raw mut (*wpi).alpha_)
                                        as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
                                    {
                                        current_block = 10874050330108021723;
                                        break;
                                    }
                                    (*wpi).is_partial_ = 1 as ::core::ffi::c_int;
                                    current_block = 7018308795614528254;
                                }
                                6 => {
                                    if ChunkSetHead(&raw mut chunk, &raw mut (*wpi).img_)
                                        as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
                                    {
                                        current_block = 10874050330108021723;
                                        break;
                                    }
                                    if MuxImageFinalize(wpi) == 0 {
                                        current_block = 10874050330108021723;
                                        break;
                                    }
                                    (*wpi).is_partial_ = 0 as ::core::ffi::c_int;
                                    current_block = 8835992952772372450;
                                }
                                3 => {
                                    if (*wpi).is_partial_ != 0 {
                                        current_block = 10874050330108021723;
                                        break;
                                    }
                                    if MuxImageParse(&raw mut chunk, copy_data, wpi) == 0 {
                                        current_block = 10874050330108021723;
                                        break;
                                    }
                                    ChunkRelease(&raw mut chunk);
                                    current_block = 8835992952772372450;
                                }
                                _ => {
                                    if (*wpi).is_partial_ != 0 {
                                        current_block = 10874050330108021723;
                                        break;
                                    }
                                    if chunk_list_ends[id as usize].is_null() {
                                        chunk_list_ends[id as usize] = MuxGetChunkListFromId(
                                            mux,
                                            id,
                                        );
                                    }
                                    if ChunkAppend(
                                        &raw mut chunk,
                                        (&raw mut chunk_list_ends as *mut *mut *mut WebPChunk)
                                            .offset(id as isize) as *mut *mut *mut WebPChunk,
                                    ) as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
                                    {
                                        current_block = 10874050330108021723;
                                        break;
                                    }
                                    if id as ::core::ffi::c_uint
                                        == WEBP_CHUNK_VP8X as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        if data_size
                                            < (CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE) as size_t
                                        {
                                            current_block = 10874050330108021723;
                                            break;
                                        }
                                        (*mux).canvas_width_ = GetLE24(
                                            data.offset(12 as ::core::ffi::c_int as isize),
                                        ) + 1 as ::core::ffi::c_int;
                                        (*mux).canvas_height_ = GetLE24(
                                            data.offset(15 as ::core::ffi::c_int as isize),
                                        ) + 1 as ::core::ffi::c_int;
                                        current_block = 7018308795614528254;
                                    } else {
                                        current_block = 7018308795614528254;
                                    }
                                }
                            }
                            match current_block {
                                8835992952772372450 => {
                                    if MuxImagePush(wpi, &raw mut (*mux).images_)
                                        as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
                                    {
                                        current_block = 10874050330108021723;
                                        break;
                                    }
                                    MuxImageInit(wpi);
                                }
                                _ => {}
                            }
                            data = data.offset(data_size as isize);
                            size = size.wrapping_sub(data_size);
                            ChunkInit(&raw mut chunk);
                        }
                        match current_block {
                            10874050330108021723 => {}
                            _ => {
                                if !((*wpi).is_partial_ != 0) {
                                    if !(MuxValidate(mux) as ::core::ffi::c_int
                                        != WEBP_MUX_OK as ::core::ffi::c_int)
                                    {
                                        MuxImageDelete(wpi);
                                        return mux;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ChunkRelease(&raw mut chunk);
    MuxImageDelete(wpi);
    WebPMuxDelete(mux);
    return ::core::ptr::null_mut::<WebPMux>();
}
unsafe extern "C" fn ValidateForSingleImage(mux: *const WebPMux) -> WebPMuxError {
    let num_images: ::core::ffi::c_int = MuxImageCount((*mux).images_, WEBP_CHUNK_IMAGE)
        as ::core::ffi::c_int;
    let num_frames: ::core::ffi::c_int = MuxImageCount((*mux).images_, WEBP_CHUNK_ANMF)
        as ::core::ffi::c_int;
    if num_images == 0 as ::core::ffi::c_int {
        return WEBP_MUX_NOT_FOUND
    } else if num_images == 1 as ::core::ffi::c_int
        && num_frames == 0 as ::core::ffi::c_int
    {
        return WEBP_MUX_OK
    } else {
        return WEBP_MUX_INVALID_ARGUMENT
    };
}
unsafe extern "C" fn MuxGetCanvasInfo(
    mux: *const WebPMux,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut flags: *mut uint32_t,
) -> WebPMuxError {
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut f: uint32_t = 0 as uint32_t;
    let mut data: WebPData = WebPData {
        bytes: ::core::ptr::null::<uint8_t>(),
        size: 0,
    };
    '_c2rust_label: {
        if !mux.is_null() {} else {
            __assert_fail(
                b"mux != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                331 as ::core::ffi::c_uint,
                b"WebPMuxError MuxGetCanvasInfo(const WebPMux *const, int *, int *, uint32_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if MuxGet(mux, IDX_VP8X, 1 as uint32_t, &raw mut data) as ::core::ffi::c_int
        == WEBP_MUX_OK as ::core::ffi::c_int
    {
        if data.size < VP8X_CHUNK_SIZE as size_t {
            return WEBP_MUX_BAD_DATA;
        }
        f = GetLE32(data.bytes.offset(0 as ::core::ffi::c_int as isize));
        w = GetLE24(data.bytes.offset(4 as ::core::ffi::c_int as isize))
            + 1 as ::core::ffi::c_int;
        h = GetLE24(data.bytes.offset(7 as ::core::ffi::c_int as isize))
            + 1 as ::core::ffi::c_int;
    } else {
        let wpi: *const WebPMuxImage = (*mux).images_;
        w = (*mux).canvas_width_;
        h = (*mux).canvas_height_;
        if w == 0 as ::core::ffi::c_int && h == 0 as ::core::ffi::c_int
            && ValidateForSingleImage(mux) as ::core::ffi::c_int
                == WEBP_MUX_OK as ::core::ffi::c_int
        {
            '_c2rust_label_0: {
                if !wpi.is_null() {} else {
                    __assert_fail(
                        b"wpi != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        346 as ::core::ffi::c_uint,
                        b"WebPMuxError MuxGetCanvasInfo(const WebPMux *const, int *, int *, uint32_t *)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            w = (*wpi).width_;
            h = (*wpi).height_;
        }
        if !wpi.is_null() {
            if (*wpi).has_alpha_ != 0 {
                f |= ALPHA_FLAG as ::core::ffi::c_int as uint32_t;
            }
        }
    }
    if (w as uint64_t).wrapping_mul(h as uint64_t) as ::core::ffi::c_ulonglong
        >= MAX_IMAGE_AREA
    {
        return WEBP_MUX_BAD_DATA;
    }
    if !width.is_null() {
        *width = w;
    }
    if !height.is_null() {
        *height = h;
    }
    if !flags.is_null() {
        *flags = f;
    }
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxGetCanvasSize(
    mut mux: *const WebPMux,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> WebPMuxError {
    if mux.is_null() || width.is_null() || height.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return MuxGetCanvasInfo(mux, width, height, ::core::ptr::null_mut::<uint32_t>());
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxGetFeatures(
    mut mux: *const WebPMux,
    mut flags: *mut uint32_t,
) -> WebPMuxError {
    if mux.is_null() || flags.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return MuxGetCanvasInfo(
        mux,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        flags,
    );
}
unsafe extern "C" fn EmitVP8XChunk(
    dst: *mut uint8_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut flags: uint32_t,
) -> *mut uint8_t {
    let vp8x_size: size_t = (CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE) as size_t;
    '_c2rust_label: {
        if width >= 1 as ::core::ffi::c_int && height >= 1 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"width >= 1 && height >= 1\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                377 as ::core::ffi::c_uint,
                b"uint8_t *EmitVP8XChunk(uint8_t *const, int, int, uint32_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if width <= (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
            && height <= (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"width <= MAX_CANVAS_SIZE && height <= MAX_CANVAS_SIZE\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                378 as ::core::ffi::c_uint,
                b"uint8_t *EmitVP8XChunk(uint8_t *const, int, int, uint32_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if ((width as uint64_t).wrapping_mul(height as uint64_t)
            as ::core::ffi::c_ulonglong)
            < (1 as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"width * (uint64_t)height < MAX_IMAGE_AREA\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                379 as ::core::ffi::c_uint,
                b"uint8_t *EmitVP8XChunk(uint8_t *const, int, int, uint32_t)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    PutLE32(
        dst,
        ('V' as i32 | ('P' as i32) << 8 as ::core::ffi::c_int
            | ('8' as i32) << 16 as ::core::ffi::c_int) as uint32_t
            | ('X' as i32 as uint32_t) << 24 as ::core::ffi::c_int,
    );
    PutLE32(dst.offset(TAG_SIZE as isize), VP8X_CHUNK_SIZE as uint32_t);
    PutLE32(dst.offset(CHUNK_HEADER_SIZE as isize), flags);
    PutLE24(
        dst.offset(CHUNK_HEADER_SIZE as isize).offset(4 as ::core::ffi::c_int as isize),
        width - 1 as ::core::ffi::c_int,
    );
    PutLE24(
        dst.offset(CHUNK_HEADER_SIZE as isize).offset(7 as ::core::ffi::c_int as isize),
        height - 1 as ::core::ffi::c_int,
    );
    return dst.offset(vp8x_size as isize);
}
unsafe extern "C" fn SynthesizeBitstream(
    wpi: *const WebPMuxImage,
    bitstream: *mut WebPData,
) -> WebPMuxError {
    let mut dst: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let need_vp8x: ::core::ffi::c_int = ((*wpi).alpha_ != NULL as *mut WebPChunk)
        as ::core::ffi::c_int;
    let vp8x_size: size_t = (if need_vp8x != 0 {
        CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE
    } else {
        0 as ::core::ffi::c_int
    }) as size_t;
    let alpha_size: size_t = if need_vp8x != 0 {
        ChunkDiskSize((*wpi).alpha_) as size_t
    } else {
        0 as size_t
    };
    let size: size_t = (RIFF_HEADER_SIZE as size_t)
        .wrapping_add(vp8x_size)
        .wrapping_add(alpha_size)
        .wrapping_add(ChunkDiskSize((*wpi).img_) as size_t);
    let data: *mut uint8_t = WebPSafeMalloc(1 as uint64_t, size) as *mut uint8_t;
    if data.is_null() {
        return WEBP_MUX_MEMORY_ERROR;
    }
    '_c2rust_label: {
        if (*wpi).alpha_.is_null() || (*(*wpi).alpha_).next_.is_null() {} else {
            __assert_fail(
                b"wpi->alpha_ == NULL || wpi->alpha_->next_ == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                404 as ::core::ffi::c_uint,
                b"WebPMuxError SynthesizeBitstream(const WebPMuxImage *const, WebPData *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !(*wpi).img_.is_null() && (*(*wpi).img_).next_.is_null() {} else {
            __assert_fail(
                b"wpi->img_ != NULL && wpi->img_->next_ == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                405 as ::core::ffi::c_uint,
                b"WebPMuxError SynthesizeBitstream(const WebPMuxImage *const, WebPData *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    dst = MuxEmitRiffHeader(data, size);
    if need_vp8x != 0 {
        dst = EmitVP8XChunk(
            dst,
            (*wpi).width_,
            (*wpi).height_,
            ALPHA_FLAG as ::core::ffi::c_int as uint32_t,
        );
        dst = ChunkListEmit((*wpi).alpha_, dst);
    }
    dst = ChunkListEmit((*wpi).img_, dst);
    '_c2rust_label_1: {
        if dst == data.offset(size as isize) {} else {
            __assert_fail(
                b"dst == data + size\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                417 as ::core::ffi::c_uint,
                b"WebPMuxError SynthesizeBitstream(const WebPMuxImage *const, WebPData *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*bitstream).bytes = data;
    (*bitstream).size = size;
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxGetChunk(
    mut mux: *const WebPMux,
    mut fourcc: *const ::core::ffi::c_char,
    mut chunk_data: *mut WebPData,
) -> WebPMuxError {
    let mut idx: CHUNK_INDEX = IDX_VP8X;
    if mux.is_null() || fourcc.is_null() || chunk_data.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    idx = ChunkGetIndexFromFourCC(fourcc);
    if IsWPI(kChunks[idx as usize].id) != 0 {
        return WEBP_MUX_INVALID_ARGUMENT
    } else if idx as ::core::ffi::c_uint
        != IDX_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return MuxGet(mux, idx, 1 as uint32_t, chunk_data)
    } else {
        let chunk: *const WebPChunk = ChunkSearchList(
            (*mux).unknown_,
            1 as uint32_t,
            ChunkGetTagFromFourCC(fourcc),
        );
        if chunk.is_null() {
            return WEBP_MUX_NOT_FOUND;
        }
        *chunk_data = (*chunk).data_;
        return WEBP_MUX_OK;
    };
}
unsafe extern "C" fn MuxGetImageInternal(
    wpi: *const WebPMuxImage,
    info: *mut WebPMuxFrameInfo,
) -> WebPMuxError {
    (*info).x_offset = 0 as ::core::ffi::c_int;
    (*info).y_offset = 0 as ::core::ffi::c_int;
    (*info).duration = 1 as ::core::ffi::c_int;
    (*info).dispose_method = WEBP_MUX_DISPOSE_NONE;
    (*info).blend_method = WEBP_MUX_BLEND;
    (*info).id = ChunkGetIdFromTag((*(*wpi).img_).tag_);
    return SynthesizeBitstream(wpi, &raw mut (*info).bitstream);
}
unsafe extern "C" fn MuxGetFrameInternal(
    wpi: *const WebPMuxImage,
    frame: *mut WebPMuxFrameInfo,
) -> WebPMuxError {
    let is_frame: ::core::ffi::c_int = ((*(*wpi).header_).tag_
        == kChunks[IDX_ANMF as ::core::ffi::c_int as usize].tag) as ::core::ffi::c_int;
    let mut frame_data: *const WebPData = ::core::ptr::null::<WebPData>();
    if is_frame == 0 {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    '_c2rust_label: {
        if !(*wpi).header_.is_null() {} else {
            __assert_fail(
                b"wpi->header_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxread.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                463 as ::core::ffi::c_uint,
                b"WebPMuxError MuxGetFrameInternal(const WebPMuxImage *const, WebPMuxFrameInfo *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    frame_data = &raw mut (*(*wpi).header_).data_;
    if (*frame_data).size
        < kChunks[IDX_ANMF as ::core::ffi::c_int as usize].size as size_t
    {
        return WEBP_MUX_BAD_DATA;
    }
    (*frame).x_offset = 2 as ::core::ffi::c_int
        * GetLE24((*frame_data).bytes.offset(0 as ::core::ffi::c_int as isize));
    (*frame).y_offset = 2 as ::core::ffi::c_int
        * GetLE24((*frame_data).bytes.offset(3 as ::core::ffi::c_int as isize));
    let bits: uint8_t = *(*frame_data).bytes.offset(15 as ::core::ffi::c_int as isize);
    (*frame).duration = GetLE24(
        (*frame_data).bytes.offset(12 as ::core::ffi::c_int as isize),
    );
    (*frame).dispose_method = (if bits as ::core::ffi::c_int & 1 as ::core::ffi::c_int
        != 0
    {
        WEBP_MUX_DISPOSE_BACKGROUND as ::core::ffi::c_int
    } else {
        WEBP_MUX_DISPOSE_NONE as ::core::ffi::c_int
    }) as WebPMuxAnimDispose;
    (*frame).blend_method = (if bits as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0
    {
        WEBP_MUX_NO_BLEND as ::core::ffi::c_int
    } else {
        WEBP_MUX_BLEND as ::core::ffi::c_int
    }) as WebPMuxAnimBlend;
    (*frame).id = ChunkGetIdFromTag((*(*wpi).header_).tag_);
    return SynthesizeBitstream(wpi, &raw mut (*frame).bitstream);
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxGetFrame(
    mut mux: *const WebPMux,
    mut nth: uint32_t,
    mut frame: *mut WebPMuxFrameInfo,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    let mut wpi: *mut WebPMuxImage = ::core::ptr::null_mut::<WebPMuxImage>();
    if mux.is_null() || frame.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxImageGetNth(
        &raw const (*mux).images_ as *mut *const WebPMuxImage,
        nth,
        &raw mut wpi,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    if (*wpi).header_.is_null() {
        return MuxGetImageInternal(wpi, frame)
    } else {
        return MuxGetFrameInternal(wpi, frame)
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxGetAnimationParams(
    mut mux: *const WebPMux,
    mut params: *mut WebPMuxAnimParams,
) -> WebPMuxError {
    let mut anim: WebPData = WebPData {
        bytes: ::core::ptr::null::<uint8_t>(),
        size: 0,
    };
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null() || params.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxGet(mux, IDX_ANIM, 1 as uint32_t, &raw mut anim);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    if anim.size < kChunks[WEBP_CHUNK_ANIM as ::core::ffi::c_int as usize].size as size_t
    {
        return WEBP_MUX_BAD_DATA;
    }
    (*params).bgcolor = GetLE32(anim.bytes);
    (*params).loop_count = GetLE16(anim.bytes.offset(4 as ::core::ffi::c_int as isize));
    return WEBP_MUX_OK;
}
unsafe extern "C" fn ChunkGetIndexFromId(mut id: WebPChunkId) -> CHUNK_INDEX {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while kChunks[i as usize].id as ::core::ffi::c_uint
        != WEBP_CHUNK_NIL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if id as ::core::ffi::c_uint == kChunks[i as usize].id as ::core::ffi::c_uint {
            return i as CHUNK_INDEX;
        }
        i += 1;
    }
    return IDX_NIL;
}
unsafe extern "C" fn CountChunks(
    chunk_list: *const WebPChunk,
    mut tag: uint32_t,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *const WebPChunk = ::core::ptr::null::<WebPChunk>();
    current = chunk_list;
    while !current.is_null() {
        if tag == NIL_TAG as uint32_t || (*current).tag_ == tag {
            count += 1;
        }
        current = (*current).next_;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxNumChunks(
    mut mux: *const WebPMux,
    mut id: WebPChunkId,
    mut num_elements: *mut ::core::ffi::c_int,
) -> WebPMuxError {
    if mux.is_null() || num_elements.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if IsWPI(id) != 0 {
        *num_elements = MuxImageCount((*mux).images_, id);
    } else {
        let mut chunk_list: *const *mut WebPChunk = MuxGetChunkListFromId(mux, id);
        let idx: CHUNK_INDEX = ChunkGetIndexFromId(id) as CHUNK_INDEX;
        *num_elements = CountChunks(*chunk_list, kChunks[idx as usize].tag);
    }
    return WEBP_MUX_OK;
}
