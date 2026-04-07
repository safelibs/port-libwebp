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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn WebPFree(ptr: *mut ::core::ffi::c_void);
    fn VP8LCheckSignature(data: *const uint8_t, size: size_t) -> ::core::ffi::c_int;
    fn WebPMuxCreateInternal(
        _: *const WebPData,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut WebPMux;
    fn WebPMuxNumChunks(
        mux: *const WebPMux,
        id: WebPChunkId,
        num_elements: *mut ::core::ffi::c_int,
    ) -> WebPMuxError;
    static kChunks: [ChunkInfo; 11];
    fn ChunkInit(chunk: *mut WebPChunk);
    fn ChunkGetIndexFromTag(tag: uint32_t) -> CHUNK_INDEX;
    fn ChunkGetIdFromTag(tag: uint32_t) -> WebPChunkId;
    fn ChunkGetTagFromFourCC(fourcc: *const ::core::ffi::c_char) -> uint32_t;
    fn ChunkAssignData(
        chunk: *mut WebPChunk,
        data: *const WebPData,
        copy_data: ::core::ffi::c_int,
        tag: uint32_t,
    ) -> WebPMuxError;
    fn ChunkSetHead(chunk: *mut WebPChunk, chunk_list: *mut *mut WebPChunk) -> WebPMuxError;
    fn ChunkRelease(chunk: *mut WebPChunk) -> *mut WebPChunk;
    fn ChunkDelete(chunk: *mut WebPChunk) -> *mut WebPChunk;
    fn ChunkListDelete(chunk_list: *mut *mut WebPChunk);
    fn ChunkListDiskSize(chunk_list: *const WebPChunk) -> size_t;
    fn ChunkListEmit(chunk_list: *const WebPChunk, dst: *mut uint8_t) -> *mut uint8_t;
    fn MuxImageInit(wpi: *mut WebPMuxImage);
    fn MuxImageRelease(wpi: *mut WebPMuxImage) -> *mut WebPMuxImage;
    fn MuxImageDelete(wpi: *mut WebPMuxImage) -> *mut WebPMuxImage;
    fn MuxImageCount(wpi_list: *const WebPMuxImage, id: WebPChunkId) -> ::core::ffi::c_int;
    fn MuxImageFinalize(wpi: *mut WebPMuxImage) -> ::core::ffi::c_int;
    fn MuxImagePush(wpi: *const WebPMuxImage, wpi_list: *mut *mut WebPMuxImage) -> WebPMuxError;
    fn MuxImageDeleteNth(wpi_list: *mut *mut WebPMuxImage, nth: uint32_t) -> WebPMuxError;
    fn MuxImageGetNth(
        wpi_list: *mut *const WebPMuxImage,
        nth: uint32_t,
        wpi: *mut *mut WebPMuxImage,
    ) -> WebPMuxError;
    fn MuxImageDiskSize(wpi: *const WebPMuxImage) -> size_t;
    fn MuxImageEmit(wpi: *const WebPMuxImage, dst: *mut uint8_t) -> *mut uint8_t;
    fn MuxHasAlpha(images: *const WebPMuxImage) -> ::core::ffi::c_int;
    fn MuxEmitRiffHeader(data: *mut uint8_t, size: size_t) -> *mut uint8_t;
    fn MuxGetChunkListFromId(mux: *const WebPMux, id: WebPChunkId) -> *mut *mut WebPChunk;
    fn MuxValidate(mux: *const WebPMux) -> WebPMuxError;
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
pub const IDX_UNKNOWN: CHUNK_INDEX = 9;
pub type CHUNK_INDEX = ::core::ffi::c_uint;
pub const IDX_LAST_CHUNK: CHUNK_INDEX = 11;
pub const IDX_NIL: CHUNK_INDEX = 10;
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
pub const CHUNK_HEADER_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RIFF_HEADER_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const ANMF_CHUNK_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ANIM_CHUNK_SIZE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const VP8X_CHUNK_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MAX_CANVAS_SIZE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int;
pub const MAX_IMAGE_AREA: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int;
pub const MAX_LOOP_COUNT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int;
pub const MAX_DURATION: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int;
pub const MAX_POSITION_OFFSET: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn WebPDataClear(mut webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        WebPFree((*webp_data).bytes as *mut ::core::ffi::c_void);
        WebPDataInit(webp_data);
    }
}
pub const WEBP_MUX_ABI_VERSION: ::core::ffi::c_int = 0x108 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPMuxCreate(
    mut bitstream: *const WebPData,
    mut copy_data: ::core::ffi::c_int,
) -> *mut WebPMux {
    return WebPMuxCreateInternal(bitstream, copy_data, WEBP_MUX_ABI_VERSION);
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
unsafe extern "C" fn PutLE24(data: *mut uint8_t, mut val: ::core::ffi::c_int) {
    '_c2rust_label: {
        if val < (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"val < (1 << 24)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/utils.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_uint,
                b"void PutLE24(uint8_t *const, int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    PutLE16(data, val & 0xffff as ::core::ffi::c_int);
    *data.offset(2 as ::core::ffi::c_int as isize) =
        (val >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn PutLE32(data: *mut uint8_t, mut val: uint32_t) {
    PutLE16(data, (val & 0xffff as uint32_t) as ::core::ffi::c_int);
    PutLE16(
        data.offset(2 as ::core::ffi::c_int as isize),
        (val >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn MuxInit(mux: *mut WebPMux) {
    '_c2rust_label: {
        if !mux.is_null() {
        } else {
            __assert_fail(
                b"mux != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                23 as ::core::ffi::c_uint,
                b"void MuxInit(WebPMux *const)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        mux as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<WebPMux>() as size_t,
    );
    (*mux).canvas_width_ = 0 as ::core::ffi::c_int;
    (*mux).canvas_height_ = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPNewInternal(mut version: ::core::ffi::c_int) -> *mut WebPMux {
    if version >> 8 as ::core::ffi::c_int != 0x108 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<WebPMux>();
    } else {
        let mux: *mut WebPMux =
            WebPSafeMalloc(1 as uint64_t, ::core::mem::size_of::<WebPMux>() as size_t)
                as *mut WebPMux;
        if !mux.is_null() {
            MuxInit(mux);
        }
        return mux;
    };
}
unsafe extern "C" fn DeleteAllImages(wpi_list: *mut *mut WebPMuxImage) {
    while !(*wpi_list).is_null() {
        *wpi_list = MuxImageDelete(*wpi_list);
    }
}
unsafe extern "C" fn MuxRelease(mux: *mut WebPMux) {
    '_c2rust_label: {
        if !mux.is_null() {
        } else {
            __assert_fail(
                b"mux != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                47 as ::core::ffi::c_uint,
                b"void MuxRelease(WebPMux *const)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    DeleteAllImages(&raw mut (*mux).images_);
    ChunkListDelete(&raw mut (*mux).vp8x_);
    ChunkListDelete(&raw mut (*mux).iccp_);
    ChunkListDelete(&raw mut (*mux).anim_);
    ChunkListDelete(&raw mut (*mux).exif_);
    ChunkListDelete(&raw mut (*mux).xmp_);
    ChunkListDelete(&raw mut (*mux).unknown_);
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxDelete(mut mux: *mut WebPMux) {
    if !mux.is_null() {
        MuxRelease(mux);
        WebPSafeFree(mux as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn MuxSet(
    mux: *mut WebPMux,
    mut tag: uint32_t,
    data: *const WebPData,
    mut copy_data: ::core::ffi::c_int,
) -> WebPMuxError {
    let mut chunk: WebPChunk = WebPChunk {
        tag_: 0,
        owner_: 0,
        data_: WebPData {
            bytes: ::core::ptr::null::<uint8_t>(),
            size: 0,
        },
        next_: ::core::ptr::null_mut::<WebPChunk>(),
    };
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    let idx: CHUNK_INDEX = ChunkGetIndexFromTag(tag) as CHUNK_INDEX;
    '_c2rust_label: {
        if !mux.is_null() {
        } else {
            __assert_fail(
                b"mux != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                83 as ::core::ffi::c_uint,
                b"WebPMuxError MuxSet(WebPMux *const, uint32_t, const WebPData *const, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if IsWPI(kChunks[idx as usize].id) == 0 {
        } else {
            __assert_fail(
                b"!IsWPI(kChunks[idx].id)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_uint,
                b"WebPMuxError MuxSet(WebPMux *const, uint32_t, const WebPData *const, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    ChunkInit(&raw mut chunk);
    if idx as ::core::ffi::c_uint == IDX_VP8X as ::core::ffi::c_int as ::core::ffi::c_uint {
        err = ChunkAssignData(&raw mut chunk, data, copy_data, tag);
        if err as ::core::ffi::c_int == WEBP_MUX_OK as ::core::ffi::c_int {
            err = ChunkSetHead(&raw mut chunk, &raw mut (*mux).vp8x_);
            if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
                ChunkRelease(&raw mut chunk);
            }
        }
        return err;
    }
    if idx as ::core::ffi::c_uint == IDX_ICCP as ::core::ffi::c_int as ::core::ffi::c_uint {
        err = ChunkAssignData(&raw mut chunk, data, copy_data, tag);
        if err as ::core::ffi::c_int == WEBP_MUX_OK as ::core::ffi::c_int {
            err = ChunkSetHead(&raw mut chunk, &raw mut (*mux).iccp_);
            if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
                ChunkRelease(&raw mut chunk);
            }
        }
        return err;
    }
    if idx as ::core::ffi::c_uint == IDX_ANIM as ::core::ffi::c_int as ::core::ffi::c_uint {
        err = ChunkAssignData(&raw mut chunk, data, copy_data, tag);
        if err as ::core::ffi::c_int == WEBP_MUX_OK as ::core::ffi::c_int {
            err = ChunkSetHead(&raw mut chunk, &raw mut (*mux).anim_);
            if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
                ChunkRelease(&raw mut chunk);
            }
        }
        return err;
    }
    if idx as ::core::ffi::c_uint == IDX_EXIF as ::core::ffi::c_int as ::core::ffi::c_uint {
        err = ChunkAssignData(&raw mut chunk, data, copy_data, tag);
        if err as ::core::ffi::c_int == WEBP_MUX_OK as ::core::ffi::c_int {
            err = ChunkSetHead(&raw mut chunk, &raw mut (*mux).exif_);
            if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
                ChunkRelease(&raw mut chunk);
            }
        }
        return err;
    }
    if idx as ::core::ffi::c_uint == IDX_XMP as ::core::ffi::c_int as ::core::ffi::c_uint {
        err = ChunkAssignData(&raw mut chunk, data, copy_data, tag);
        if err as ::core::ffi::c_int == WEBP_MUX_OK as ::core::ffi::c_int {
            err = ChunkSetHead(&raw mut chunk, &raw mut (*mux).xmp_);
            if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
                ChunkRelease(&raw mut chunk);
            }
        }
        return err;
    }
    if idx as ::core::ffi::c_uint == IDX_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint {
        err = ChunkAssignData(&raw mut chunk, data, copy_data, tag);
        if err as ::core::ffi::c_int == WEBP_MUX_OK as ::core::ffi::c_int {
            err = ChunkSetHead(&raw mut chunk, &raw mut (*mux).unknown_);
            if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
                ChunkRelease(&raw mut chunk);
            }
        }
        return err;
    }
    return err;
}
unsafe extern "C" fn CreateFrameData(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    info: *const WebPMuxFrameInfo,
    frame: *mut WebPData,
) -> WebPMuxError {
    let mut frame_bytes: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let frame_size: size_t = kChunks[IDX_ANMF as ::core::ffi::c_int as usize].size as size_t;
    '_c2rust_label: {
        if width > 0 as ::core::ffi::c_int
            && height > 0 as ::core::ffi::c_int
            && (*info).duration >= 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"width > 0 && height > 0 && info->duration >= 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_uint,
                b"WebPMuxError CreateFrameData(int, int, const WebPMuxFrameInfo *const, WebPData *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*info).dispose_method as ::core::ffi::c_uint
            == (*info).dispose_method as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"info->dispose_method == (info->dispose_method & 1)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_uint,
                b"WebPMuxError CreateFrameData(int, int, const WebPMuxFrameInfo *const, WebPData *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    frame_bytes = WebPSafeMalloc(1 as uint64_t, frame_size) as *mut uint8_t;
    if frame_bytes.is_null() {
        return WEBP_MUX_MEMORY_ERROR;
    }
    PutLE24(
        frame_bytes.offset(0 as ::core::ffi::c_int as isize),
        (*info).x_offset / 2 as ::core::ffi::c_int,
    );
    PutLE24(
        frame_bytes.offset(3 as ::core::ffi::c_int as isize),
        (*info).y_offset / 2 as ::core::ffi::c_int,
    );
    PutLE24(
        frame_bytes.offset(6 as ::core::ffi::c_int as isize),
        width - 1 as ::core::ffi::c_int,
    );
    PutLE24(
        frame_bytes.offset(9 as ::core::ffi::c_int as isize),
        height - 1 as ::core::ffi::c_int,
    );
    PutLE24(
        frame_bytes.offset(12 as ::core::ffi::c_int as isize),
        (*info).duration,
    );
    *frame_bytes.offset(15 as ::core::ffi::c_int as isize) =
        ((if (*info).blend_method as ::core::ffi::c_uint
            == WEBP_MUX_NO_BLEND as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) | (if (*info).dispose_method as ::core::ffi::c_uint
            == WEBP_MUX_DISPOSE_BACKGROUND as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })) as uint8_t;
    (*frame).bytes = frame_bytes;
    (*frame).size = frame_size;
    return WEBP_MUX_OK;
}
unsafe extern "C" fn GetImageData(
    bitstream: *const WebPData,
    image: *mut WebPData,
    alpha: *mut WebPData,
    is_lossless: *mut ::core::ffi::c_int,
) -> WebPMuxError {
    WebPDataInit(alpha);
    if (*bitstream).size < TAG_SIZE as size_t
        || memcmp(
            (*bitstream).bytes as *const ::core::ffi::c_void,
            b"RIFF\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            TAG_SIZE as size_t,
        ) != 0
    {
        *image = *bitstream;
    } else {
        let mut wpi: *const WebPMuxImage = ::core::ptr::null::<WebPMuxImage>();
        let mux: *mut WebPMux = WebPMuxCreate(bitstream, 0 as ::core::ffi::c_int) as *mut WebPMux;
        if mux.is_null() {
            return WEBP_MUX_BAD_DATA;
        }
        wpi = (*mux).images_;
        '_c2rust_label: {
            if !wpi.is_null() && !(*wpi).img_.is_null() {
            } else {
                __assert_fail(
                    b"wpi != NULL && wpi->img_ != NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    143 as ::core::ffi::c_uint,
                    b"WebPMuxError GetImageData(const WebPData *const, WebPData *const, WebPData *const, int *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        *image = (*(*wpi).img_).data_;
        if !(*wpi).alpha_.is_null() {
            *alpha = (*(*wpi).alpha_).data_;
        }
        WebPMuxDelete(mux);
    }
    *is_lossless = VP8LCheckSignature((*image).bytes, (*image).size);
    return WEBP_MUX_OK;
}
unsafe extern "C" fn DeleteChunks(
    mut chunk_list: *mut *mut WebPChunk,
    mut tag: uint32_t,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    '_c2rust_label: {
        if !chunk_list.is_null() {
        } else {
            __assert_fail(
                b"chunk_list\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                156 as ::core::ffi::c_uint,
                b"WebPMuxError DeleteChunks(WebPChunk **, uint32_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    while !(*chunk_list).is_null() {
        let chunk: *mut WebPChunk = *chunk_list;
        if (*chunk).tag_ == tag {
            *chunk_list = ChunkDelete(chunk);
            err = WEBP_MUX_OK;
        } else {
            chunk_list = &raw mut (*chunk).next_;
        }
    }
    return err;
}
unsafe extern "C" fn MuxDeleteAllNamedData(mux: *mut WebPMux, mut tag: uint32_t) -> WebPMuxError {
    let id: WebPChunkId = ChunkGetIdFromTag(tag) as WebPChunkId;
    '_c2rust_label: {
        if !mux.is_null() {
        } else {
            __assert_fail(
                b"mux != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_uint,
                b"WebPMuxError MuxDeleteAllNamedData(WebPMux *const, uint32_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if IsWPI(id) != 0 {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return DeleteChunks(MuxGetChunkListFromId(mux, id), tag);
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxSetChunk(
    mut mux: *mut WebPMux,
    mut fourcc: *const ::core::ffi::c_char,
    mut chunk_data: *const WebPData,
    mut copy_data: ::core::ffi::c_int,
) -> WebPMuxError {
    let mut tag: uint32_t = 0;
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null()
        || fourcc.is_null()
        || chunk_data.is_null()
        || (*chunk_data).bytes.is_null()
        || (*chunk_data).size > MAX_CHUNK_PAYLOAD as size_t
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    tag = ChunkGetTagFromFourCC(fourcc);
    err = MuxDeleteAllNamedData(mux, tag);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
        && err as ::core::ffi::c_int != WEBP_MUX_NOT_FOUND as ::core::ffi::c_int
    {
        return err;
    }
    return MuxSet(mux, tag, chunk_data, copy_data);
}
unsafe extern "C" fn AddDataToChunkList(
    data: *const WebPData,
    mut copy_data: ::core::ffi::c_int,
    mut tag: uint32_t,
    mut chunk_list: *mut *mut WebPChunk,
) -> WebPMuxError {
    let mut chunk: WebPChunk = WebPChunk {
        tag_: 0,
        owner_: 0,
        data_: WebPData {
            bytes: ::core::ptr::null::<uint8_t>(),
            size: 0,
        },
        next_: ::core::ptr::null_mut::<WebPChunk>(),
    };
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    ChunkInit(&raw mut chunk);
    err = ChunkAssignData(&raw mut chunk, data, copy_data, tag);
    if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
        err = ChunkSetHead(&raw mut chunk, chunk_list);
        if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
            return WEBP_MUX_OK;
        }
    }
    ChunkRelease(&raw mut chunk);
    return err;
}
unsafe extern "C" fn SetAlphaAndImageChunks(
    bitstream: *const WebPData,
    mut copy_data: ::core::ffi::c_int,
    wpi: *mut WebPMuxImage,
) -> WebPMuxError {
    let mut is_lossless: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut image: WebPData = WebPData {
        bytes: ::core::ptr::null::<uint8_t>(),
        size: 0,
    };
    let mut alpha: WebPData = WebPData {
        bytes: ::core::ptr::null::<uint8_t>(),
        size: 0,
    };
    let mut err: WebPMuxError = GetImageData(
        bitstream,
        &raw mut image,
        &raw mut alpha,
        &raw mut is_lossless,
    );
    let image_tag: ::core::ffi::c_int = (if is_lossless != 0 {
        kChunks[IDX_VP8L as ::core::ffi::c_int as usize].tag
    } else {
        kChunks[IDX_VP8 as ::core::ffi::c_int as usize].tag
    }) as ::core::ffi::c_int;
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    if !alpha.bytes.is_null() {
        err = AddDataToChunkList(
            &raw mut alpha,
            copy_data,
            kChunks[IDX_ALPHA as ::core::ffi::c_int as usize].tag,
            &raw mut (*wpi).alpha_,
        );
        if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
            return err;
        }
    }
    err = AddDataToChunkList(
        &raw mut image,
        copy_data,
        image_tag as uint32_t,
        &raw mut (*wpi).img_,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    return (if MuxImageFinalize(wpi) != 0 {
        WEBP_MUX_OK as ::core::ffi::c_int
    } else {
        WEBP_MUX_INVALID_ARGUMENT as ::core::ffi::c_int
    }) as WebPMuxError;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxSetImage(
    mut mux: *mut WebPMux,
    mut bitstream: *const WebPData,
    mut copy_data: ::core::ffi::c_int,
) -> WebPMuxError {
    let mut wpi: WebPMuxImage = WebPMuxImage {
        header_: ::core::ptr::null_mut::<WebPChunk>(),
        alpha_: ::core::ptr::null_mut::<WebPChunk>(),
        img_: ::core::ptr::null_mut::<WebPChunk>(),
        unknown_: ::core::ptr::null_mut::<WebPChunk>(),
        width_: 0,
        height_: 0,
        has_alpha_: 0,
        is_partial_: 0,
        next_: ::core::ptr::null_mut::<WebPMuxImage>(),
    };
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null()
        || bitstream.is_null()
        || (*bitstream).bytes.is_null()
        || (*bitstream).size > MAX_CHUNK_PAYLOAD as size_t
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if !(*mux).images_.is_null() {
        DeleteAllImages(&raw mut (*mux).images_);
    }
    MuxImageInit(&raw mut wpi);
    err = SetAlphaAndImageChunks(bitstream, copy_data, &raw mut wpi);
    if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
        err = MuxImagePush(&raw mut wpi, &raw mut (*mux).images_);
        if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
            return WEBP_MUX_OK;
        }
    }
    MuxImageRelease(&raw mut wpi);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxPushFrame(
    mut mux: *mut WebPMux,
    mut info: *const WebPMuxFrameInfo,
    mut copy_data: ::core::ffi::c_int,
) -> WebPMuxError {
    let mut wpi: WebPMuxImage = WebPMuxImage {
        header_: ::core::ptr::null_mut::<WebPChunk>(),
        alpha_: ::core::ptr::null_mut::<WebPChunk>(),
        img_: ::core::ptr::null_mut::<WebPChunk>(),
        unknown_: ::core::ptr::null_mut::<WebPChunk>(),
        width_: 0,
        height_: 0,
        has_alpha_: 0,
        is_partial_: 0,
        next_: ::core::ptr::null_mut::<WebPMuxImage>(),
    };
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null() || info.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if (*info).id as ::core::ffi::c_uint
        != WEBP_CHUNK_ANMF as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if (*info).bitstream.bytes.is_null() || (*info).bitstream.size > MAX_CHUNK_PAYLOAD as size_t {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if !(*mux).images_.is_null() {
        let image: *const WebPMuxImage = (*mux).images_;
        let image_id: uint32_t = if !(*image).header_.is_null() {
            ChunkGetIdFromTag((*(*image).header_).tag_) as uint32_t
        } else {
            WEBP_CHUNK_IMAGE as ::core::ffi::c_int as uint32_t
        };
        if image_id != (*info).id as uint32_t {
            return WEBP_MUX_INVALID_ARGUMENT;
        }
    }
    MuxImageInit(&raw mut wpi);
    err = SetAlphaAndImageChunks(&raw const (*info).bitstream, copy_data, &raw mut wpi);
    if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
        '_c2rust_label: {
            if !wpi.img_.is_null() {
            } else {
                __assert_fail(
                    b"wpi.img_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    291 as ::core::ffi::c_uint,
                    b"WebPMuxError WebPMuxPushFrame(WebPMux *, const WebPMuxFrameInfo *, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        let mut frame: WebPData = WebPData {
            bytes: ::core::ptr::null::<uint8_t>(),
            size: 0,
        };
        let tag: uint32_t = kChunks[IDX_ANMF as ::core::ffi::c_int as usize].tag;
        let mut tmp: WebPMuxFrameInfo = *info;
        tmp.x_offset &= !(1 as ::core::ffi::c_int);
        tmp.y_offset &= !(1 as ::core::ffi::c_int);
        if tmp.x_offset < 0 as ::core::ffi::c_int
            || tmp.x_offset >= MAX_POSITION_OFFSET
            || tmp.y_offset < 0 as ::core::ffi::c_int
            || tmp.y_offset >= MAX_POSITION_OFFSET
            || (tmp.duration < 0 as ::core::ffi::c_int || tmp.duration >= MAX_DURATION)
            || tmp.dispose_method as ::core::ffi::c_uint
                != tmp.dispose_method as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint
        {
            err = WEBP_MUX_INVALID_ARGUMENT;
        } else {
            err = CreateFrameData(wpi.width_, wpi.height_, &raw mut tmp, &raw mut frame);
            if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
                err = AddDataToChunkList(
                    &raw mut frame,
                    1 as ::core::ffi::c_int,
                    tag,
                    &raw mut wpi.header_,
                );
                WebPDataClear(&raw mut frame);
                if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
                    err = MuxImagePush(&raw mut wpi, &raw mut (*mux).images_);
                    if !(err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int) {
                        return WEBP_MUX_OK;
                    }
                }
            }
        }
    }
    MuxImageRelease(&raw mut wpi);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxSetAnimationParams(
    mut mux: *mut WebPMux,
    mut params: *const WebPMuxAnimParams,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    let mut data: [uint8_t; 6] = [0; 6];
    let anim: WebPData = WebPData {
        bytes: &raw mut data as *mut uint8_t,
        size: ANIM_CHUNK_SIZE as size_t,
    };
    if mux.is_null() || params.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if (*params).loop_count < 0 as ::core::ffi::c_int || (*params).loop_count >= MAX_LOOP_COUNT {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxDeleteAllNamedData(mux, kChunks[IDX_ANIM as ::core::ffi::c_int as usize].tag);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
        && err as ::core::ffi::c_int != WEBP_MUX_NOT_FOUND as ::core::ffi::c_int
    {
        return err;
    }
    PutLE32(&raw mut data as *mut uint8_t, (*params).bgcolor);
    PutLE16(
        (&raw mut data as *mut uint8_t).offset(4 as ::core::ffi::c_int as isize),
        (*params).loop_count,
    );
    return MuxSet(
        mux,
        kChunks[IDX_ANIM as ::core::ffi::c_int as usize].tag,
        &raw const anim,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxSetCanvasSize(
    mut mux: *mut WebPMux,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if width < 0 as ::core::ffi::c_int
        || height < 0 as ::core::ffi::c_int
        || width > MAX_CANVAS_SIZE
        || height > MAX_CANVAS_SIZE
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if (width as uint64_t).wrapping_mul(height as uint64_t) as ::core::ffi::c_ulonglong
        >= MAX_IMAGE_AREA
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if width * height == 0 as ::core::ffi::c_int && width | height != 0 as ::core::ffi::c_int {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxDeleteAllNamedData(mux, kChunks[IDX_VP8X as ::core::ffi::c_int as usize].tag);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
        && err as ::core::ffi::c_int != WEBP_MUX_NOT_FOUND as ::core::ffi::c_int
    {
        return err;
    }
    (*mux).canvas_width_ = width;
    (*mux).canvas_height_ = height;
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxDeleteChunk(
    mut mux: *mut WebPMux,
    mut fourcc: *const ::core::ffi::c_char,
) -> WebPMuxError {
    if mux.is_null() || fourcc.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return MuxDeleteAllNamedData(mux, ChunkGetTagFromFourCC(fourcc));
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxDeleteFrame(
    mut mux: *mut WebPMux,
    mut nth: uint32_t,
) -> WebPMuxError {
    if mux.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return MuxImageDeleteNth(&raw mut (*mux).images_, nth);
}
unsafe extern "C" fn GetFrameInfo(
    frame_chunk: *const WebPChunk,
    x_offset: *mut ::core::ffi::c_int,
    y_offset: *mut ::core::ffi::c_int,
    duration: *mut ::core::ffi::c_int,
) -> WebPMuxError {
    let data: *const WebPData = &raw const (*frame_chunk).data_;
    let expected_data_size: size_t = ANMF_CHUNK_SIZE as size_t;
    '_c2rust_label: {
        if (*frame_chunk).tag_ == kChunks[IDX_ANMF as ::core::ffi::c_int as usize].tag {
        } else {
            __assert_fail(
                b"frame_chunk->tag_ == kChunks[IDX_ANMF].tag\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                394 as ::core::ffi::c_uint,
                b"WebPMuxError GetFrameInfo(const WebPChunk *const, int *const, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !frame_chunk.is_null() {
        } else {
            __assert_fail(
                b"frame_chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                395 as ::core::ffi::c_uint,
                b"WebPMuxError GetFrameInfo(const WebPChunk *const, int *const, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).size != expected_data_size {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    *x_offset =
        2 as ::core::ffi::c_int * GetLE24((*data).bytes.offset(0 as ::core::ffi::c_int as isize));
    *y_offset =
        2 as ::core::ffi::c_int * GetLE24((*data).bytes.offset(3 as ::core::ffi::c_int as isize));
    *duration = GetLE24((*data).bytes.offset(12 as ::core::ffi::c_int as isize));
    return WEBP_MUX_OK;
}
unsafe extern "C" fn GetImageInfo(
    wpi: *const WebPMuxImage,
    x_offset: *mut ::core::ffi::c_int,
    y_offset: *mut ::core::ffi::c_int,
    duration: *mut ::core::ffi::c_int,
    width: *mut ::core::ffi::c_int,
    height: *mut ::core::ffi::c_int,
) -> WebPMuxError {
    let frame_chunk: *const WebPChunk = (*wpi).header_;
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    '_c2rust_label: {
        if !wpi.is_null() {
        } else {
            __assert_fail(
                b"wpi != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                410 as ::core::ffi::c_uint,
                b"WebPMuxError GetImageInfo(const WebPMuxImage *const, int *const, int *const, int *const, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !frame_chunk.is_null() {
        } else {
            __assert_fail(
                b"frame_chunk != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                411 as ::core::ffi::c_uint,
                b"WebPMuxError GetImageInfo(const WebPMuxImage *const, int *const, int *const, int *const, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    err = GetFrameInfo(frame_chunk, x_offset, y_offset, duration);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    if !width.is_null() {
        *width = (*wpi).width_;
    }
    if !height.is_null() {
        *height = (*wpi).height_;
    }
    return WEBP_MUX_OK;
}
unsafe extern "C" fn GetAdjustedCanvasSize(
    mux: *const WebPMux,
    width: *mut ::core::ffi::c_int,
    height: *mut ::core::ffi::c_int,
) -> WebPMuxError {
    let mut wpi: *mut WebPMuxImage = ::core::ptr::null_mut::<WebPMuxImage>();
    '_c2rust_label: {
        if !mux.is_null() {
        } else {
            __assert_fail(
                b"mux != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                427 as ::core::ffi::c_uint,
                b"WebPMuxError GetAdjustedCanvasSize(const WebPMux *const, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !width.is_null() && !height.is_null() {
        } else {
            __assert_fail(
                b"width != NULL && height != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                428 as ::core::ffi::c_uint,
                b"WebPMuxError GetAdjustedCanvasSize(const WebPMux *const, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    wpi = (*mux).images_;
    '_c2rust_label_1: {
        if !wpi.is_null() {
        } else {
            __assert_fail(
                b"wpi != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                431 as ::core::ffi::c_uint,
                b"WebPMuxError GetAdjustedCanvasSize(const WebPMux *const, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !(*wpi).img_.is_null() {
        } else {
            __assert_fail(
                b"wpi->img_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                432 as ::core::ffi::c_uint,
                b"WebPMuxError GetAdjustedCanvasSize(const WebPMux *const, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if !(*wpi).next_.is_null() {
        let mut max_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut max_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        '_c2rust_label_3: {
            if !(*wpi).header_.is_null() {
            } else {
                __assert_fail(
                    b"wpi->header_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    437 as ::core::ffi::c_uint,
                    b"WebPMuxError GetAdjustedCanvasSize(const WebPMux *const, int *const, int *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        while !wpi.is_null() {
            let mut x_offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut y_offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut duration: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut w: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut h: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let err: WebPMuxError = GetImageInfo(
                wpi,
                &raw mut x_offset,
                &raw mut y_offset,
                &raw mut duration,
                &raw mut w,
                &raw mut h,
            ) as WebPMuxError;
            let max_x_pos: ::core::ffi::c_int = x_offset + w;
            let max_y_pos: ::core::ffi::c_int = y_offset + h;
            if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
                return err;
            }
            '_c2rust_label_4: {
                if x_offset < (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"x_offset < MAX_POSITION_OFFSET\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        446 as ::core::ffi::c_uint,
                        b"WebPMuxError GetAdjustedCanvasSize(const WebPMux *const, int *const, int *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_5: {
                if y_offset < (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"y_offset < MAX_POSITION_OFFSET\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        447 as ::core::ffi::c_uint,
                        b"WebPMuxError GetAdjustedCanvasSize(const WebPMux *const, int *const, int *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            if max_x_pos > max_x {
                max_x = max_x_pos;
            }
            if max_y_pos > max_y {
                max_y = max_y_pos;
            }
            wpi = (*wpi).next_;
        }
        *width = max_x;
        *height = max_y;
    } else {
        *width = (*wpi).width_;
        *height = (*wpi).height_;
    }
    return WEBP_MUX_OK;
}
unsafe extern "C" fn CreateVP8XChunk(mux: *mut WebPMux) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_OK;
    let mut flags: uint32_t = 0 as uint32_t;
    let mut width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut data: [uint8_t; 10] = [0; 10];
    let vp8x: WebPData = WebPData {
        bytes: &raw mut data as *mut uint8_t,
        size: VP8X_CHUNK_SIZE as size_t,
    };
    let mut images: *const WebPMuxImage = ::core::ptr::null::<WebPMuxImage>();
    '_c2rust_label: {
        if !mux.is_null() {
        } else {
            __assert_fail(
                b"mux != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                476 as ::core::ffi::c_uint,
                b"WebPMuxError CreateVP8XChunk(WebPMux *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    images = (*mux).images_;
    if images.is_null() || (*images).img_.is_null() || (*(*images).img_).data_.bytes.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxDeleteAllNamedData(mux, kChunks[IDX_VP8X as ::core::ffi::c_int as usize].tag);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int
        && err as ::core::ffi::c_int != WEBP_MUX_NOT_FOUND as ::core::ffi::c_int
    {
        return err;
    }
    if !(*mux).iccp_.is_null() && !(*(*mux).iccp_).data_.bytes.is_null() {
        flags |= ICCP_FLAG as ::core::ffi::c_int as uint32_t;
    }
    if !(*mux).exif_.is_null() && !(*(*mux).exif_).data_.bytes.is_null() {
        flags |= EXIF_FLAG as ::core::ffi::c_int as uint32_t;
    }
    if !(*mux).xmp_.is_null() && !(*(*mux).xmp_).data_.bytes.is_null() {
        flags |= XMP_FLAG as ::core::ffi::c_int as uint32_t;
    }
    if !(*images).header_.is_null() {
        if (*(*images).header_).tag_ == kChunks[IDX_ANMF as ::core::ffi::c_int as usize].tag {
            flags |= ANIMATION_FLAG as ::core::ffi::c_int as uint32_t;
        }
    }
    if MuxImageCount(images, WEBP_CHUNK_ALPHA) > 0 as ::core::ffi::c_int {
        flags |= ALPHA_FLAG as ::core::ffi::c_int as uint32_t;
    }
    err = GetAdjustedCanvasSize(mux, &raw mut width, &raw mut height);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    if width <= 0 as ::core::ffi::c_int || height <= 0 as ::core::ffi::c_int {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if width > MAX_CANVAS_SIZE || height > MAX_CANVAS_SIZE {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if (*mux).canvas_width_ != 0 as ::core::ffi::c_int
        || (*mux).canvas_height_ != 0 as ::core::ffi::c_int
    {
        if width > (*mux).canvas_width_ || height > (*mux).canvas_height_ {
            return WEBP_MUX_INVALID_ARGUMENT;
        }
        width = (*mux).canvas_width_;
        height = (*mux).canvas_height_;
    }
    if flags == 0 as uint32_t && (*mux).unknown_.is_null() {
        return WEBP_MUX_OK;
    }
    if MuxHasAlpha(images) != 0 {
        flags |= ALPHA_FLAG as ::core::ffi::c_int as uint32_t;
    }
    PutLE32(
        (&raw mut data as *mut uint8_t).offset(0 as ::core::ffi::c_int as isize),
        flags,
    );
    PutLE24(
        (&raw mut data as *mut uint8_t).offset(4 as ::core::ffi::c_int as isize),
        width - 1 as ::core::ffi::c_int,
    );
    PutLE24(
        (&raw mut data as *mut uint8_t).offset(7 as ::core::ffi::c_int as isize),
        height - 1 as ::core::ffi::c_int,
    );
    return MuxSet(
        mux,
        kChunks[IDX_VP8X as ::core::ffi::c_int as usize].tag,
        &raw const vp8x,
        1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn MuxCleanup(mux: *mut WebPMux) -> WebPMuxError {
    let mut num_frames: ::core::ffi::c_int = 0;
    let mut num_anim_chunks: ::core::ffi::c_int = 0;
    let mut err: WebPMuxError = WebPMuxNumChunks(
        mux,
        kChunks[IDX_ANMF as ::core::ffi::c_int as usize].id,
        &raw mut num_frames,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    if num_frames == 1 as ::core::ffi::c_int {
        let mut frame: *mut WebPMuxImage = ::core::ptr::null_mut::<WebPMuxImage>();
        err = MuxImageGetNth(
            &raw mut (*mux).images_ as *mut *const WebPMuxImage,
            1 as uint32_t,
            &raw mut frame,
        );
        if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
            return err;
        }
        '_c2rust_label: {
            if !frame.is_null() {
            } else {
                __assert_fail(
                    b"frame != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    560 as ::core::ffi::c_uint,
                    b"WebPMuxError MuxCleanup(WebPMux *const)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if !(*frame).header_.is_null()
            && ((*mux).canvas_width_ == 0 as ::core::ffi::c_int
                && (*mux).canvas_height_ == 0 as ::core::ffi::c_int
                || (*frame).width_ == (*mux).canvas_width_
                    && (*frame).height_ == (*mux).canvas_height_)
        {
            '_c2rust_label_0: {
                if (*(*frame).header_).tag_ == kChunks[IDX_ANMF as ::core::ffi::c_int as usize].tag
                {
                } else {
                    __assert_fail(
                        b"frame->header_->tag_ == kChunks[IDX_ANMF].tag\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        565 as ::core::ffi::c_uint,
                        b"WebPMuxError MuxCleanup(WebPMux *const)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            ChunkDelete((*frame).header_);
            (*frame).header_ = ::core::ptr::null_mut::<WebPChunk>();
            num_frames = 0 as ::core::ffi::c_int;
        }
    }
    err = WebPMuxNumChunks(
        mux,
        kChunks[IDX_ANIM as ::core::ffi::c_int as usize].id,
        &raw mut num_anim_chunks,
    );
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    if num_anim_chunks >= 1 as ::core::ffi::c_int && num_frames == 0 as ::core::ffi::c_int {
        err = MuxDeleteAllNamedData(mux, kChunks[IDX_ANIM as ::core::ffi::c_int as usize].tag);
        if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
            return err;
        }
    }
    return WEBP_MUX_OK;
}
unsafe extern "C" fn ImageListDiskSize(mut wpi_list: *const WebPMuxImage) -> size_t {
    let mut size: size_t = 0 as size_t;
    while !wpi_list.is_null() {
        size = size.wrapping_add(MuxImageDiskSize(wpi_list));
        wpi_list = (*wpi_list).next_;
    }
    return size;
}
unsafe extern "C" fn ImageListEmit(
    mut wpi_list: *const WebPMuxImage,
    mut dst: *mut uint8_t,
) -> *mut uint8_t {
    while !wpi_list.is_null() {
        dst = MuxImageEmit(wpi_list, dst);
        wpi_list = (*wpi_list).next_;
    }
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxAssemble(
    mut mux: *mut WebPMux,
    mut assembled_data: *mut WebPData,
) -> WebPMuxError {
    let mut size: size_t = 0 as size_t;
    let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dst: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if assembled_data.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    memset(
        assembled_data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<WebPData>() as size_t,
    );
    if mux.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxCleanup(mux);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    err = CreateVP8XChunk(mux);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        return err;
    }
    size = ChunkListDiskSize((*mux).vp8x_)
        .wrapping_add(ChunkListDiskSize((*mux).iccp_))
        .wrapping_add(ChunkListDiskSize((*mux).anim_))
        .wrapping_add(ImageListDiskSize((*mux).images_))
        .wrapping_add(ChunkListDiskSize((*mux).exif_))
        .wrapping_add(ChunkListDiskSize((*mux).xmp_))
        .wrapping_add(ChunkListDiskSize((*mux).unknown_))
        .wrapping_add(RIFF_HEADER_SIZE as size_t);
    data = WebPSafeMalloc(1 as uint64_t, size) as *mut uint8_t;
    if data.is_null() {
        return WEBP_MUX_MEMORY_ERROR;
    }
    dst = MuxEmitRiffHeader(data, size);
    dst = ChunkListEmit((*mux).vp8x_, dst);
    dst = ChunkListEmit((*mux).iccp_, dst);
    dst = ChunkListEmit((*mux).anim_, dst);
    dst = ImageListEmit((*mux).images_, dst);
    dst = ChunkListEmit((*mux).exif_, dst);
    dst = ChunkListEmit((*mux).xmp_, dst);
    dst = ChunkListEmit((*mux).unknown_, dst);
    '_c2rust_label: {
        if dst == data.offset(size as isize) {
        } else {
            __assert_fail(
                b"dst == data + size\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/mux/muxedit.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                640 as ::core::ffi::c_uint,
                b"WebPMuxError WebPMuxAssemble(WebPMux *, WebPData *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    err = MuxValidate(mux);
    if err as ::core::ffi::c_int != WEBP_MUX_OK as ::core::ffi::c_int {
        WebPSafeFree(data as *mut ::core::ffi::c_void);
        data = ::core::ptr::null_mut::<uint8_t>();
        size = 0 as size_t;
    }
    (*assembled_data).bytes = data;
    (*assembled_data).size = size;
    return err;
}
