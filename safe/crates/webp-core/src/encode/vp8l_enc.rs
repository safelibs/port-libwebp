#[repr(C)]
pub struct PixOrCopyBlock {
    _unused: [u8; 0],
}

extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn WebPPictureInitInternal(_: *mut WebPPicture, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn WebPPictureView(
        src: *const WebPPicture,
        left: ::core::ffi::c_int,
        top: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        dst: *mut WebPPicture,
    ) -> ::core::ffi::c_int;
    fn WebPPictureHasTransparency(picture: *const WebPPicture) -> ::core::ffi::c_int;
    fn VP8LHashChainInit(p: *mut VP8LHashChain, size: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn VP8LHashChainFill(
        p: *mut VP8LHashChain,
        quality: ::core::ffi::c_int,
        argb: *const uint32_t,
        xsize: ::core::ffi::c_int,
        ysize: ::core::ffi::c_int,
        low_effort: ::core::ffi::c_int,
        pic: *const WebPPicture,
        percent_range: ::core::ffi::c_int,
        percent: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8LHashChainClear(p: *mut VP8LHashChain);
    fn VP8LBackwardRefsInit(refs: *mut VP8LBackwardRefs, block_size: ::core::ffi::c_int);
    fn VP8LBackwardRefsClear(refs: *mut VP8LBackwardRefs);
    fn VP8LRefsCursorInit(refs: *const VP8LBackwardRefs) -> VP8LRefsCursor;
    fn VP8LRefsCursorNextBlock(c: *mut VP8LRefsCursor);
    fn VP8LGetBackwardReferences(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        argb: *const uint32_t,
        quality: ::core::ffi::c_int,
        low_effort: ::core::ffi::c_int,
        lz77_types_to_try: ::core::ffi::c_int,
        cache_bits_max: ::core::ffi::c_int,
        do_no_cache: ::core::ffi::c_int,
        hash_chain: *const VP8LHashChain,
        refs: *mut VP8LBackwardRefs,
        cache_bits_best: *mut ::core::ffi::c_int,
        pic: *const WebPPicture,
        percent_range: ::core::ffi::c_int,
        percent: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8LHistogramStoreRefs(refs: *const VP8LBackwardRefs, histo: *mut VP8LHistogram);
    fn VP8LFreeHistogram(histo: *mut VP8LHistogram);
    fn VP8LFreeHistogramSet(histo: *mut VP8LHistogramSet);
    fn VP8LAllocateHistogramSet(
        size: ::core::ffi::c_int,
        cache_bits: ::core::ffi::c_int,
    ) -> *mut VP8LHistogramSet;
    fn VP8LHistogramSetClear(set: *mut VP8LHistogramSet);
    fn VP8LAllocateHistogram(cache_bits: ::core::ffi::c_int) -> *mut VP8LHistogram;
    fn VP8LGetHistoImageSymbols(
        xsize: ::core::ffi::c_int,
        ysize: ::core::ffi::c_int,
        refs: *const VP8LBackwardRefs,
        quality: ::core::ffi::c_int,
        low_effort: ::core::ffi::c_int,
        histogram_bits: ::core::ffi::c_int,
        cache_bits: ::core::ffi::c_int,
        image_histo: *mut VP8LHistogramSet,
        tmp_histo: *mut VP8LHistogram,
        histogram_symbols: *mut uint16_t,
        pic: *const WebPPicture,
        percent_range: ::core::ffi::c_int,
        percent: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8LBitsEntropy(array: *const uint32_t, n: ::core::ffi::c_int) -> ::core::ffi::c_float;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
    fn WebPGetColorPalette(pic: *const WebPPicture, palette: *mut uint32_t) -> ::core::ffi::c_int;
    static mut VP8LSubtractGreenFromBlueAndRed: VP8LProcessEncBlueAndRedFunc;
    static mut VP8LBundleColorMap: VP8LBundleColorMapFunc;
    fn VP8LEncDspInit();
    static kLog2Table: [::core::ffi::c_float; 256];
    static mut VP8LFastLog2Slow: VP8LFastLog2SlowFunc;
    static kPrefixEncodeCode: [VP8LPrefixCode; 512];
    static kPrefixEncodeExtraBitsValue: [uint8_t; 512];
    fn VP8LBitWriterInit(bw: *mut VP8LBitWriter, expected_size: size_t) -> ::core::ffi::c_int;
    fn VP8LBitWriterClone(src: *const VP8LBitWriter, dst: *mut VP8LBitWriter)
        -> ::core::ffi::c_int;
    fn VP8LBitWriterFinish(bw: *mut VP8LBitWriter) -> *mut uint8_t;
    fn VP8LBitWriterWipeOut(bw: *mut VP8LBitWriter);
    fn VP8LBitWriterReset(bw_init: *const VP8LBitWriter, bw: *mut VP8LBitWriter);
    fn VP8LBitWriterSwap(src: *mut VP8LBitWriter, dst: *mut VP8LBitWriter);
    fn VP8LPutBitsFlushBits(bw: *mut VP8LBitWriter);
    fn VP8LPutBitsInternal(bw: *mut VP8LBitWriter, bits: uint32_t, n_bits: ::core::ffi::c_int);
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> ::core::ffi::c_int;
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: ::core::ffi::c_int,
        percent_store: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8ApplyNearLossless(
        picture: *const WebPPicture,
        quality: ::core::ffi::c_int,
        argb_dst: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn VP8LResidualImage(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        bits: ::core::ffi::c_int,
        low_effort: ::core::ffi::c_int,
        argb: *mut uint32_t,
        argb_scratch: *mut uint32_t,
        image: *mut uint32_t,
        near_lossless: ::core::ffi::c_int,
        exact: ::core::ffi::c_int,
        used_subtract_green: ::core::ffi::c_int,
        pic: *const WebPPicture,
        percent_range: ::core::ffi::c_int,
        percent: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8LColorSpaceTransform(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        bits: ::core::ffi::c_int,
        quality: ::core::ffi::c_int,
        argb: *mut uint32_t,
        image: *mut uint32_t,
        pic: *const WebPPicture,
        percent_range: ::core::ffi::c_int,
        percent: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8LCreateCompressedHuffmanTree(
        tree: *const HuffmanTreeCode,
        tokens: *mut HuffmanTreeToken,
        max_tokens: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VP8LCreateHuffmanTree(
        histogram: *mut uint32_t,
        tree_depth_limit: ::core::ffi::c_int,
        buf_rle: *mut uint8_t,
        huff_tree: *mut HuffmanTree,
        huff_code: *mut HuffmanTreeCode,
    );
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPConfig {
    pub lossless: ::core::ffi::c_int,
    pub quality: ::core::ffi::c_float,
    pub method: ::core::ffi::c_int,
    pub image_hint: WebPImageHint,
    pub target_size: ::core::ffi::c_int,
    pub target_PSNR: ::core::ffi::c_float,
    pub segments: ::core::ffi::c_int,
    pub sns_strength: ::core::ffi::c_int,
    pub filter_strength: ::core::ffi::c_int,
    pub filter_sharpness: ::core::ffi::c_int,
    pub filter_type: ::core::ffi::c_int,
    pub autofilter: ::core::ffi::c_int,
    pub alpha_compression: ::core::ffi::c_int,
    pub alpha_filtering: ::core::ffi::c_int,
    pub alpha_quality: ::core::ffi::c_int,
    pub pass: ::core::ffi::c_int,
    pub show_compressed: ::core::ffi::c_int,
    pub preprocessing: ::core::ffi::c_int,
    pub partitions: ::core::ffi::c_int,
    pub partition_limit: ::core::ffi::c_int,
    pub emulate_jpeg_size: ::core::ffi::c_int,
    pub thread_level: ::core::ffi::c_int,
    pub low_memory: ::core::ffi::c_int,
    pub near_lossless: ::core::ffi::c_int,
    pub exact: ::core::ffi::c_int,
    pub use_delta_palette: ::core::ffi::c_int,
    pub use_sharp_yuv: ::core::ffi::c_int,
    pub qmin: ::core::ffi::c_int,
    pub qmax: ::core::ffi::c_int,
}
pub type WebPImageHint = ::core::ffi::c_uint;
pub const WEBP_HINT_LAST: WebPImageHint = 4;
pub const WEBP_HINT_GRAPH: WebPImageHint = 3;
pub const WEBP_HINT_PHOTO: WebPImageHint = 2;
pub const WEBP_HINT_PICTURE: WebPImageHint = 1;
pub const WEBP_HINT_DEFAULT: WebPImageHint = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPPicture {
    pub use_argb: ::core::ffi::c_int,
    pub colorspace: WebPEncCSP,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub y: *mut uint8_t,
    pub u: *mut uint8_t,
    pub v: *mut uint8_t,
    pub y_stride: ::core::ffi::c_int,
    pub uv_stride: ::core::ffi::c_int,
    pub a: *mut uint8_t,
    pub a_stride: ::core::ffi::c_int,
    pub pad1: [uint32_t; 2],
    pub argb: *mut uint32_t,
    pub argb_stride: ::core::ffi::c_int,
    pub pad2: [uint32_t; 3],
    pub writer: WebPWriterFunction,
    pub custom_ptr: *mut ::core::ffi::c_void,
    pub extra_info_type: ::core::ffi::c_int,
    pub extra_info: *mut uint8_t,
    pub stats: *mut WebPAuxStats,
    pub error_code: WebPEncodingError,
    pub progress_hook: WebPProgressHook,
    pub user_data: *mut ::core::ffi::c_void,
    pub pad3: [uint32_t; 3],
    pub pad4: *mut uint8_t,
    pub pad5: *mut uint8_t,
    pub pad6: [uint32_t; 8],
    pub memory_: *mut ::core::ffi::c_void,
    pub memory_argb_: *mut ::core::ffi::c_void,
    pub pad7: [*mut ::core::ffi::c_void; 2],
}
pub type WebPProgressHook =
    Option<unsafe extern "C" fn(::core::ffi::c_int, *const WebPPicture) -> ::core::ffi::c_int>;
pub type WebPEncodingError = ::core::ffi::c_uint;
pub const VP8_ENC_ERROR_LAST: WebPEncodingError = 11;
pub const VP8_ENC_ERROR_USER_ABORT: WebPEncodingError = 10;
pub const VP8_ENC_ERROR_FILE_TOO_BIG: WebPEncodingError = 9;
pub const VP8_ENC_ERROR_BAD_WRITE: WebPEncodingError = 8;
pub const VP8_ENC_ERROR_PARTITION_OVERFLOW: WebPEncodingError = 7;
pub const VP8_ENC_ERROR_PARTITION0_OVERFLOW: WebPEncodingError = 6;
pub const VP8_ENC_ERROR_BAD_DIMENSION: WebPEncodingError = 5;
pub const VP8_ENC_ERROR_INVALID_CONFIGURATION: WebPEncodingError = 4;
pub const VP8_ENC_ERROR_NULL_PARAMETER: WebPEncodingError = 3;
pub const VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY: WebPEncodingError = 2;
pub const VP8_ENC_ERROR_OUT_OF_MEMORY: WebPEncodingError = 1;
pub const VP8_ENC_OK: WebPEncodingError = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPAuxStats {
    pub coded_size: ::core::ffi::c_int,
    pub PSNR: [::core::ffi::c_float; 5],
    pub block_count: [::core::ffi::c_int; 3],
    pub header_bytes: [::core::ffi::c_int; 2],
    pub residual_bytes: [[::core::ffi::c_int; 4]; 3],
    pub segment_size: [::core::ffi::c_int; 4],
    pub segment_quant: [::core::ffi::c_int; 4],
    pub segment_level: [::core::ffi::c_int; 4],
    pub alpha_data_size: ::core::ffi::c_int,
    pub layer_data_size: ::core::ffi::c_int,
    pub lossless_features: uint32_t,
    pub histogram_bits: ::core::ffi::c_int,
    pub transform_bits: ::core::ffi::c_int,
    pub cache_bits: ::core::ffi::c_int,
    pub palette_size: ::core::ffi::c_int,
    pub lossless_size: ::core::ffi::c_int,
    pub lossless_hdr_size: ::core::ffi::c_int,
    pub lossless_data_size: ::core::ffi::c_int,
    pub pad: [uint32_t; 2],
}
pub type WebPWriterFunction =
    Option<unsafe extern "C" fn(*const uint8_t, size_t, *const WebPPicture) -> ::core::ffi::c_int>;
pub type WebPEncCSP = ::core::ffi::c_uint;
pub const WEBP_CSP_ALPHA_BIT: WebPEncCSP = 4;
pub const WEBP_CSP_UV_MASK: WebPEncCSP = 3;
pub const WEBP_YUV420A: WebPEncCSP = 4;
pub const WEBP_YUV420: WebPEncCSP = 0;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const COLOR_INDEXING_TRANSFORM: C2RustUnnamed = 3;
pub const SUBTRACT_GREEN_TRANSFORM: C2RustUnnamed = 2;
pub const CROSS_COLOR_TRANSFORM: C2RustUnnamed = 1;
pub const PREDICTOR_TRANSFORM: C2RustUnnamed = 0;
pub type Mode = ::core::ffi::c_uint;
pub const kNone: Mode = 3;
pub const kCopy: Mode = 2;
pub const kCacheIdx: Mode = 1;
pub const kLiteral: Mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PixOrCopy {
    pub mode: uint8_t,
    pub len: uint16_t,
    pub argb_or_distance: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LHashChain {
    pub offset_length_: *mut uint32_t,
    pub size_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBackwardRefs {
    pub block_size_: ::core::ffi::c_int,
    pub error_: ::core::ffi::c_int,
    pub refs_: *mut PixOrCopyBlock,
    pub tail_: *mut *mut PixOrCopyBlock,
    pub free_blocks_: *mut PixOrCopyBlock,
    pub last_block_: *mut PixOrCopyBlock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LRefsCursor {
    pub cur_pos: *mut PixOrCopy,
    pub cur_block_: *mut PixOrCopyBlock,
    pub last_pos_: *const PixOrCopy,
}
pub type VP8LLZ77Type = ::core::ffi::c_uint;
pub const kLZ77Box: VP8LLZ77Type = 4;
pub const kLZ77RLE: VP8LLZ77Type = 2;
pub const kLZ77Standard: VP8LLZ77Type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LHistogram {
    pub literal_: *mut uint32_t,
    pub red_: [uint32_t; 256],
    pub blue_: [uint32_t; 256],
    pub alpha_: [uint32_t; 256],
    pub distance_: [uint32_t; 40],
    pub palette_code_bits_: ::core::ffi::c_int,
    pub trivial_symbol_: uint32_t,
    pub bit_cost_: ::core::ffi::c_float,
    pub literal_cost_: ::core::ffi::c_float,
    pub red_cost_: ::core::ffi::c_float,
    pub blue_cost_: ::core::ffi::c_float,
    pub is_used_: [uint8_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LHistogramSet {
    pub size: ::core::ffi::c_int,
    pub max_size: ::core::ffi::c_int,
    pub histograms: *mut *mut VP8LHistogram,
}
pub type VP8LProcessEncBlueAndRedFunc =
    Option<unsafe extern "C" fn(*mut uint32_t, ::core::ffi::c_int) -> ()>;
pub type VP8LBundleColorMapFunc = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut uint32_t,
    ) -> (),
>;
pub type VP8LFastLog2SlowFunc = Option<unsafe extern "C" fn(uint32_t) -> ::core::ffi::c_float>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LPrefixCode {
    pub code_: int8_t,
    pub extra_bits_: int8_t,
}
pub type vp8l_atype_t = uint64_t;
pub type vp8l_wtype_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBitWriter {
    pub bits_: vp8l_atype_t,
    pub used_: ::core::ffi::c_int,
    pub buf_: *mut uint8_t,
    pub cur_: *mut uint8_t,
    pub end_: *mut uint8_t,
    pub error_: ::core::ffi::c_int,
}
pub type WebPWorkerStatus = ::core::ffi::c_uint;
pub const WORK: WebPWorkerStatus = 2;
pub const OK: WebPWorkerStatus = 1;
pub const NOT_OK: WebPWorkerStatus = 0;
pub type WebPWorkerHook = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPWorker {
    pub impl_: *mut ::core::ffi::c_void,
    pub status_: WebPWorkerStatus,
    pub hook: WebPWorkerHook,
    pub data1: *mut ::core::ffi::c_void,
    pub data2: *mut ::core::ffi::c_void,
    pub had_error: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPWorkerInterface {
    pub Init: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Reset: Option<unsafe extern "C" fn(*mut WebPWorker) -> ::core::ffi::c_int>,
    pub Sync_0: Option<unsafe extern "C" fn(*mut WebPWorker) -> ::core::ffi::c_int>,
    pub Launch: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Execute: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub End: Option<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
}
pub type VP8LEncoderARGBContent = ::core::ffi::c_uint;
pub const kEncoderPalette: VP8LEncoderARGBContent = 3;
pub const kEncoderNearLossless: VP8LEncoderARGBContent = 2;
pub const kEncoderARGB: VP8LEncoderARGBContent = 1;
pub const kEncoderNone: VP8LEncoderARGBContent = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LEncoder {
    pub config_: *const WebPConfig,
    pub pic_: *const WebPPicture,
    pub argb_: *mut uint32_t,
    pub argb_content_: VP8LEncoderARGBContent,
    pub argb_scratch_: *mut uint32_t,
    pub transform_data_: *mut uint32_t,
    pub transform_mem_: *mut uint32_t,
    pub transform_mem_size_: size_t,
    pub current_width_: ::core::ffi::c_int,
    pub histo_bits_: ::core::ffi::c_int,
    pub transform_bits_: ::core::ffi::c_int,
    pub cache_bits_: ::core::ffi::c_int,
    pub use_cross_color_: ::core::ffi::c_int,
    pub use_subtract_green_: ::core::ffi::c_int,
    pub use_predict_: ::core::ffi::c_int,
    pub use_palette_: ::core::ffi::c_int,
    pub palette_size_: ::core::ffi::c_int,
    pub palette_: [uint32_t; 256],
    pub palette_sorted_: [uint32_t; 256],
    pub refs_: [VP8LBackwardRefs; 4],
    pub hash_chain_: VP8LHashChain,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StreamEncodeContext {
    pub config_: *const WebPConfig,
    pub picture_: *const WebPPicture,
    pub bw_: *mut VP8LBitWriter,
    pub enc_: *mut VP8LEncoder,
    pub use_cache_: ::core::ffi::c_int,
    pub crunch_configs_: [CrunchConfig; 8],
    pub num_crunch_configs_: ::core::ffi::c_int,
    pub red_and_blue_always_zero_: ::core::ffi::c_int,
    pub stats_: *mut WebPAuxStats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrunchConfig {
    pub entropy_idx_: ::core::ffi::c_int,
    pub palette_sorting_type_: PaletteSorting,
    pub sub_configs_: [CrunchSubConfig; 2],
    pub sub_configs_size_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrunchSubConfig {
    pub lz77_: ::core::ffi::c_int,
    pub do_no_cache_: ::core::ffi::c_int,
}
pub type PaletteSorting = ::core::ffi::c_uint;
pub const kUnusedPalette: PaletteSorting = 3;
pub const kModifiedZeng: PaletteSorting = 2;
pub const kMinimizeDelta: PaletteSorting = 1;
pub const kSortedDefault: PaletteSorting = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTreeCode {
    pub num_symbols: ::core::ffi::c_int,
    pub code_lengths: *mut uint8_t,
    pub codes: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_: uint32_t,
    pub value_: ::core::ffi::c_int,
    pub pool_index_left_: ::core::ffi::c_int,
    pub pool_index_right_: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTreeToken {
    pub code: uint8_t,
    pub extra_bits: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sum {
    pub index: uint8_t,
    pub sum: uint32_t,
}
pub const kPaletteAndSpatial: EntropyIx = 5;
pub const kSpatialSubGreen: EntropyIx = 3;
pub const kSpatial: EntropyIx = 1;
pub const kSubGreen: EntropyIx = 2;
pub const kPalette: EntropyIx = 4;
pub type EntropyIx = ::core::ffi::c_uint;
pub const kNumEntropyIx: EntropyIx = 6;
pub const kDirect: EntropyIx = 0;
pub const kHistoBlue: C2RustUnnamed_0 = 6;
pub const kHistoRed: C2RustUnnamed_0 = 4;
pub const kHistoBluePredSubGreen: C2RustUnnamed_0 = 11;
pub const kHistoRedPredSubGreen: C2RustUnnamed_0 = 9;
pub const kHistoBlueSubGreen: C2RustUnnamed_0 = 10;
pub const kHistoRedSubGreen: C2RustUnnamed_0 = 8;
pub const kHistoBluePred: C2RustUnnamed_0 = 7;
pub const kHistoRedPred: C2RustUnnamed_0 = 5;
pub const kHistoPalette: C2RustUnnamed_0 = 12;
pub const kHistoGreenPred: C2RustUnnamed_0 = 3;
pub const kHistoAlphaPred: C2RustUnnamed_0 = 1;
pub const kHistoGreen: C2RustUnnamed_0 = 2;
pub const kHistoAlpha: C2RustUnnamed_0 = 0;
pub const kHistoTotal: C2RustUnnamed_0 = 13;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const WEBP_ENCODER_ABI_VERSION: ::core::ffi::c_int = 0x20f as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn WebPPictureInit(mut picture: *mut WebPPicture) -> ::core::ffi::c_int {
    return WebPPictureInitInternal(picture, WEBP_ENCODER_ABI_VERSION);
}
pub const VP8L_SIGNATURE_SIZE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VP8L_MAGIC_BYTE: ::core::ffi::c_int = 0x2f as ::core::ffi::c_int;
pub const VP8L_IMAGE_SIZE_BITS: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const VP8L_VERSION_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const VP8L_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAX_PALETTE_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NUM_LITERAL_CODES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NUM_LENGTH_CODES: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const NUM_DISTANCE_CODES: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const CODE_LENGTH_CODES: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const MIN_HUFFMAN_BITS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAX_HUFFMAN_BITS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const TRANSFORM_PRESENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TAG_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNK_HEADER_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RIFF_HEADER_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const MAX_COLOR_CACHE_BITS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn PixOrCopyIsLiteral(p: *const PixOrCopy) -> ::core::ffi::c_int {
    return ((*p).mode as ::core::ffi::c_int == kLiteral as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyIsCacheIdx(p: *const PixOrCopy) -> ::core::ffi::c_int {
    return ((*p).mode as ::core::ffi::c_int == kCacheIdx as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyLiteral(
    p: *const PixOrCopy,
    mut component: ::core::ffi::c_int,
) -> uint32_t {
    '_c2rust_label: {
        if (*p).mode as ::core::ffi::c_int == kLiteral as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"p->mode == kLiteral\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                87 as ::core::ffi::c_uint,
                b"uint32_t PixOrCopyLiteral(const PixOrCopy *const, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*p).argb_or_distance >> component * 8 as ::core::ffi::c_int & 0xff as uint32_t;
}
#[inline]
unsafe extern "C" fn PixOrCopyLength(p: *const PixOrCopy) -> uint32_t {
    return (*p).len as uint32_t;
}
#[inline]
unsafe extern "C" fn PixOrCopyCacheIdx(p: *const PixOrCopy) -> uint32_t {
    '_c2rust_label: {
        if (*p).mode as ::core::ffi::c_int == kCacheIdx as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"p->mode == kCacheIdx\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                b"uint32_t PixOrCopyCacheIdx(const PixOrCopy *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*p).argb_or_distance < (1 as uint32_t) << 10 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"p->argb_or_distance < (1U << MAX_COLOR_CACHE_BITS)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                97 as ::core::ffi::c_uint,
                b"uint32_t PixOrCopyCacheIdx(const PixOrCopy *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*p).argb_or_distance;
}
#[inline]
unsafe extern "C" fn PixOrCopyDistance(p: *const PixOrCopy) -> uint32_t {
    '_c2rust_label: {
        if (*p).mode as ::core::ffi::c_int == kCopy as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"p->mode == kCopy\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_uint,
                b"uint32_t PixOrCopyDistance(const PixOrCopy *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (*p).argb_or_distance;
}
pub const MAX_REFS_BLOCK_PER_IMAGE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8LRefsCursorOk(c: *const VP8LRefsCursor) -> ::core::ffi::c_int {
    return ((*c).cur_pos != NULL as *mut PixOrCopy) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn VP8LRefsCursorNext(c: *mut VP8LRefsCursor) {
    '_c2rust_label: {
        if !c.is_null() {
        } else {
            __assert_fail(
                b"c != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                206 as ::core::ffi::c_uint,
                b"void VP8LRefsCursorNext(VP8LRefsCursor *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if VP8LRefsCursorOk(c) != 0 {
        } else {
            __assert_fail(
                b"VP8LRefsCursorOk(c)\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/enc/backward_references_enc.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_uint,
                b"void VP8LRefsCursorNext(VP8LRefsCursor *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*c).cur_pos = (*c).cur_pos.offset(1);
    if (*c).cur_pos == (*c).last_pos_ as *mut PixOrCopy {
        VP8LRefsCursorNextBlock(c);
    }
}
#[inline]
unsafe extern "C" fn VP8LHistogramNumCodes(
    mut palette_code_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return NUM_LITERAL_CODES
        + NUM_LENGTH_CODES
        + (if palette_code_bits > 0 as ::core::ffi::c_int {
            (1 as ::core::ffi::c_int) << palette_code_bits
        } else {
            0 as ::core::ffi::c_int
        });
}
pub const WEBP_ALIGN_CST: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
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
unsafe extern "C" fn PutLE16(data: *mut uint8_t, mut val: ::core::ffi::c_int) {
    '_c2rust_label: {
        if val < (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"val < (1 << 16)\0" as *const u8 as *const ::core::ffi::c_char,
                b"original/src/utils/utils.h\0" as *const u8 as *const ::core::ffi::c_char,
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
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> ::core::ffi::c_int {
    return 31 as ::core::ffi::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn VP8LSubSampleSize(
    mut size: uint32_t,
    mut sampling_bits: uint32_t,
) -> uint32_t {
    return size
        .wrapping_add(((1 as ::core::ffi::c_int) << sampling_bits) as uint32_t)
        .wrapping_sub(1 as uint32_t)
        >> sampling_bits;
}
pub const LOG_LOOKUP_IDX_MAX: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8LFastLog2(mut v: uint32_t) -> ::core::ffi::c_float {
    return if v < LOG_LOOKUP_IDX_MAX as uint32_t {
        kLog2Table[v as usize]
    } else {
        VP8LFastLog2Slow.expect("non-null function pointer")(v)
    };
}
#[inline]
unsafe extern "C" fn VP8LPrefixEncodeNoLUT(
    mut distance: ::core::ffi::c_int,
    code: *mut ::core::ffi::c_int,
    extra_bits: *mut ::core::ffi::c_int,
    extra_bits_value: *mut ::core::ffi::c_int,
) {
    distance -= 1;
    let highest_bit: ::core::ffi::c_int = BitsLog2Floor(distance as uint32_t) as ::core::ffi::c_int;
    let second_highest_bit: ::core::ffi::c_int =
        distance >> highest_bit - 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
    *extra_bits = highest_bit - 1 as ::core::ffi::c_int;
    *extra_bits_value =
        distance & ((1 as ::core::ffi::c_int) << *extra_bits) - 1 as ::core::ffi::c_int;
    *code = 2 as ::core::ffi::c_int * highest_bit + second_highest_bit;
}
pub const PREFIX_LOOKUP_IDX_MAX: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn VP8LPrefixEncode(
    mut distance: ::core::ffi::c_int,
    code: *mut ::core::ffi::c_int,
    extra_bits: *mut ::core::ffi::c_int,
    extra_bits_value: *mut ::core::ffi::c_int,
) {
    if distance < PREFIX_LOOKUP_IDX_MAX {
        let prefix_code: VP8LPrefixCode = kPrefixEncodeCode[distance as usize];
        *code = prefix_code.code_ as ::core::ffi::c_int;
        *extra_bits = prefix_code.extra_bits_ as ::core::ffi::c_int;
        *extra_bits_value = kPrefixEncodeExtraBitsValue[distance as usize] as ::core::ffi::c_int;
    } else {
        VP8LPrefixEncodeNoLUT(distance, code, extra_bits, extra_bits_value);
    };
}
#[inline]
unsafe extern "C" fn VP8LSubPixels(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let alpha_and_green: uint32_t = (0xff00ff as uint32_t)
        .wrapping_add(a & 0xff00ff00 as uint32_t)
        .wrapping_sub(b & 0xff00ff00 as uint32_t);
    let red_and_blue: uint32_t = (0xff00ff00 as uint32_t)
        .wrapping_add(a & 0xff00ff as uint32_t)
        .wrapping_sub(b & 0xff00ff as uint32_t);
    return alpha_and_green & 0xff00ff00 as uint32_t | red_and_blue & 0xff00ff as uint32_t;
}
#[inline]
unsafe extern "C" fn VP8LBitWriterNumBytes(bw: *const VP8LBitWriter) -> size_t {
    return ((*bw).cur_.offset_from((*bw).buf_) as ::core::ffi::c_long
        + ((*bw).used_ + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as size_t;
}
#[inline]
unsafe extern "C" fn VP8LPutBits(
    bw: *mut VP8LBitWriter,
    mut bits: uint32_t,
    mut n_bits: ::core::ffi::c_int,
) {
    if ::core::mem::size_of::<vp8l_wtype_t>() as usize == 4 as usize {
        if n_bits > 0 as ::core::ffi::c_int {
            if (*bw).used_ >= 32 as ::core::ffi::c_int {
                VP8LPutBitsFlushBits(bw);
            }
            (*bw).bits_ |= (bits as vp8l_atype_t) << (*bw).used_;
            (*bw).used_ += n_bits;
        }
    } else {
        VP8LPutBitsInternal(bw, bits, n_bits);
    };
}
pub const MAX_HUFF_IMAGE_SIZE: ::core::ffi::c_int = 2600 as ::core::ffi::c_int;
unsafe extern "C" fn PaletteCompareColorsForQsort(
    mut p1: *const ::core::ffi::c_void,
    mut p2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let a: uint32_t = WebPMemToUint32(p1 as *mut uint8_t) as uint32_t;
    let b: uint32_t = WebPMemToUint32(p2 as *mut uint8_t) as uint32_t;
    '_c2rust_label: {
        if a != b {
        } else {
            __assert_fail(
                b"a != b\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                38 as ::core::ffi::c_uint,
                b"int PaletteCompareColorsForQsort(const void *, const void *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return if a < b {
        -(1 as ::core::ffi::c_int)
    } else {
        1 as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn PaletteComponentDistance(mut v: uint32_t) -> uint32_t {
    return if v <= 128 as uint32_t {
        v
    } else {
        (256 as uint32_t).wrapping_sub(v)
    };
}
#[inline]
unsafe extern "C" fn PaletteColorDistance(mut col1: uint32_t, mut col2: uint32_t) -> uint32_t {
    let diff: uint32_t = VP8LSubPixels(col1, col2) as uint32_t;
    let kMoreWeightForRGBThanForAlpha: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    let mut score: uint32_t = 0;
    score = PaletteComponentDistance(diff >> 0 as ::core::ffi::c_int & 0xff as uint32_t);
    score = score.wrapping_add(PaletteComponentDistance(
        diff >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
    ));
    score = score.wrapping_add(PaletteComponentDistance(
        diff >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
    ));
    score = score.wrapping_mul(kMoreWeightForRGBThanForAlpha as uint32_t);
    score = score.wrapping_add(PaletteComponentDistance(
        diff >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
    ));
    return score;
}
#[inline]
unsafe extern "C" fn SwapColor(col1: *mut uint32_t, col2: *mut uint32_t) {
    let tmp: uint32_t = *col1;
    *col1 = *col2;
    *col2 = tmp;
}
#[inline]
unsafe extern "C" fn SearchColorNoIdx(
    mut sorted: *const uint32_t,
    mut color: uint32_t,
    mut num_colors: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut low: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut hi: ::core::ffi::c_int = num_colors;
    if *sorted.offset(low as isize) == color {
        return low;
    }
    loop {
        let mid: ::core::ffi::c_int = low + hi >> 1 as ::core::ffi::c_int;
        if *sorted.offset(mid as isize) == color {
            return mid;
        } else if *sorted.offset(mid as isize) < color {
            low = mid;
        } else {
            hi = mid;
        }
    }
}
unsafe extern "C" fn PaletteHasNonMonotonousDeltas(
    palette: *const uint32_t,
    mut num_colors: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut predict: uint32_t = 0 as uint32_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut sign_found: uint8_t = 0 as uint8_t;
    i = 0 as ::core::ffi::c_int;
    while i < num_colors {
        let diff: uint32_t = VP8LSubPixels(*palette.offset(i as isize), predict) as uint32_t;
        let rd: uint8_t = (diff >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let gd: uint8_t = (diff >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let bd: uint8_t = (diff >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        if rd as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            sign_found = (sign_found as ::core::ffi::c_int
                | if (rd as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }) as uint8_t;
        }
        if gd as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            sign_found = (sign_found as ::core::ffi::c_int
                | if (gd as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
                    8 as ::core::ffi::c_int
                } else {
                    16 as ::core::ffi::c_int
                }) as uint8_t;
        }
        if bd as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            sign_found = (sign_found as ::core::ffi::c_int
                | if (bd as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
                    64 as ::core::ffi::c_int
                } else {
                    128 as ::core::ffi::c_int
                }) as uint8_t;
        }
        predict = *palette.offset(i as isize);
        i += 1;
    }
    return (sign_found as ::core::ffi::c_int
        & (sign_found as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn PaletteSortMinimizeDeltas(
    palette_sorted: *const uint32_t,
    mut num_colors: ::core::ffi::c_int,
    palette: *mut uint32_t,
) {
    let mut predict: uint32_t = 0 as uint32_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    memcpy(
        palette as *mut ::core::ffi::c_void,
        palette_sorted as *const ::core::ffi::c_void,
        (num_colors as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
    if PaletteHasNonMonotonousDeltas(palette_sorted, num_colors) == 0 {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < num_colors {
        let mut best_ix: ::core::ffi::c_int = i;
        let mut best_score: uint32_t = !(0 as uint32_t);
        k = i;
        while k < num_colors {
            let cur_score: uint32_t =
                PaletteColorDistance(*palette.offset(k as isize), predict) as uint32_t;
            if best_score > cur_score {
                best_score = cur_score;
                best_ix = k;
            }
            k += 1;
        }
        SwapColor(
            palette.offset(best_ix as isize) as *mut uint32_t,
            palette.offset(i as isize) as *mut uint32_t,
        );
        predict = *palette.offset(i as isize);
        i += 1;
    }
}
unsafe extern "C" fn PrepareMapToPalette(
    mut palette: *const uint32_t,
    mut num_colors: uint32_t,
    mut sorted: *mut uint32_t,
    mut idx_map: *mut uint32_t,
) {
    let mut i: uint32_t = 0;
    memcpy(
        sorted as *mut ::core::ffi::c_void,
        palette as *const ::core::ffi::c_void,
        (num_colors as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    );
    qsort(
        sorted as *mut ::core::ffi::c_void,
        num_colors as size_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
        Some(
            PaletteCompareColorsForQsort
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    i = 0 as uint32_t;
    while i < num_colors {
        *idx_map.offset(SearchColorNoIdx(
            sorted as *const uint32_t,
            *palette.offset(i as isize),
            num_colors as ::core::ffi::c_int,
        ) as isize) = i;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn CoOccurrenceFindMax(
    cooccurrence: *const uint32_t,
    mut num_colors: uint32_t,
    c1: *mut uint8_t,
    c2: *mut uint8_t,
) {
    let mut best_sum: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut best_cooccurrence: uint32_t = 0;
    *c1 = 0 as uint8_t;
    i = 0 as uint32_t;
    while i < num_colors {
        let mut sum: uint32_t = 0 as uint32_t;
        j = 0 as uint32_t;
        while j < num_colors {
            sum = sum.wrapping_add(
                *cooccurrence.offset(i.wrapping_mul(num_colors).wrapping_add(j) as isize),
            );
            j = j.wrapping_add(1);
        }
        if sum > best_sum {
            best_sum = sum;
            *c1 = i as uint8_t;
        }
        i = i.wrapping_add(1);
    }
    *c2 = 0 as uint8_t;
    best_cooccurrence = 0 as ::core::ffi::c_uint as uint32_t;
    i = 0 as uint32_t;
    while i < num_colors {
        if *cooccurrence.offset((*c1 as uint32_t).wrapping_mul(num_colors).wrapping_add(i) as isize)
            > best_cooccurrence
        {
            best_cooccurrence = *cooccurrence
                .offset((*c1 as uint32_t).wrapping_mul(num_colors).wrapping_add(i) as isize);
            *c2 = i as uint8_t;
        }
        i = i.wrapping_add(1);
    }
    '_c2rust_label: {
        if *c1 as ::core::ffi::c_int != *c2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"*c1 != *c2\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                183 as ::core::ffi::c_uint,
                b"void CoOccurrenceFindMax(const uint32_t *const, uint32_t, uint8_t *const, uint8_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn CoOccurrenceBuild(
    pic: *const WebPPicture,
    palette: *const uint32_t,
    mut num_colors: uint32_t,
    mut cooccurrence: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut lines: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut line_top: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut line_current: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut line_tmp: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut src: *const uint32_t = (*pic).argb;
    let mut prev_pix: uint32_t = !*src.offset(0 as ::core::ffi::c_int as isize);
    let mut prev_idx: uint32_t = 0 as uint32_t;
    let mut idx_map: [uint32_t; 256] = [
        0 as ::core::ffi::c_int as uint32_t,
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
    let mut palette_sorted: [uint32_t; 256] = [0; 256];
    lines = WebPSafeMalloc(
        (2 as ::core::ffi::c_int * (*pic).width) as uint64_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
    ) as *mut uint32_t;
    if lines.is_null() {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    line_top = lines.offset(0 as ::core::ffi::c_int as isize) as *mut uint32_t;
    line_current = lines.offset((*pic).width as isize) as *mut uint32_t;
    PrepareMapToPalette(
        palette as *const uint32_t,
        num_colors,
        &raw mut palette_sorted as *mut uint32_t,
        &raw mut idx_map as *mut uint32_t,
    );
    y = 0 as ::core::ffi::c_int;
    while y < (*pic).height {
        x = 0 as ::core::ffi::c_int;
        while x < (*pic).width {
            let pix: uint32_t = *src.offset(x as isize);
            if pix != prev_pix {
                prev_idx = idx_map[SearchColorNoIdx(
                    &raw mut palette_sorted as *mut uint32_t as *const uint32_t,
                    pix,
                    num_colors as ::core::ffi::c_int,
                ) as usize];
                prev_pix = pix;
            }
            *line_current.offset(x as isize) = prev_idx;
            if x > 0 as ::core::ffi::c_int
                && prev_idx != *line_current.offset((x - 1 as ::core::ffi::c_int) as isize)
            {
                let left_idx: uint32_t =
                    *line_current.offset((x - 1 as ::core::ffi::c_int) as isize);
                let ref mut fresh3 = *cooccurrence
                    .offset(prev_idx.wrapping_mul(num_colors).wrapping_add(left_idx) as isize);
                *fresh3 = (*fresh3).wrapping_add(1);
                let ref mut fresh4 = *cooccurrence
                    .offset(left_idx.wrapping_mul(num_colors).wrapping_add(prev_idx) as isize);
                *fresh4 = (*fresh4).wrapping_add(1);
            }
            if y > 0 as ::core::ffi::c_int && prev_idx != *line_top.offset(x as isize) {
                let top_idx: uint32_t = *line_top.offset(x as isize);
                let ref mut fresh5 = *cooccurrence
                    .offset(prev_idx.wrapping_mul(num_colors).wrapping_add(top_idx) as isize);
                *fresh5 = (*fresh5).wrapping_add(1);
                let ref mut fresh6 = *cooccurrence
                    .offset(top_idx.wrapping_mul(num_colors).wrapping_add(prev_idx) as isize);
                *fresh6 = (*fresh6).wrapping_add(1);
            }
            x += 1;
        }
        line_tmp = line_top;
        line_top = line_current;
        line_current = line_tmp;
        src = src.offset((*pic).argb_stride as isize);
        y += 1;
    }
    WebPSafeFree(lines as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn PaletteSortModifiedZeng(
    pic: *const WebPPicture,
    palette_sorted: *const uint32_t,
    mut num_colors: uint32_t,
    palette: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut ind: uint32_t = 0;
    let mut remapping: [uint8_t; 256] = [0; 256];
    let mut cooccurrence: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut sums: [Sum; 256] = [Sum { index: 0, sum: 0 }; 256];
    let mut first: uint32_t = 0;
    let mut last: uint32_t = 0;
    let mut num_sums: uint32_t = 0;
    if num_colors <= 1 as uint32_t {
        return 1 as ::core::ffi::c_int;
    }
    cooccurrence = WebPSafeCalloc(
        num_colors.wrapping_mul(num_colors) as uint64_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
    ) as *mut uint32_t;
    if cooccurrence.is_null() {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    if CoOccurrenceBuild(pic, palette_sorted, num_colors, cooccurrence) == 0 {
        WebPSafeFree(cooccurrence as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    CoOccurrenceFindMax(
        cooccurrence,
        num_colors,
        (&raw mut remapping as *mut uint8_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
        (&raw mut remapping as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut uint8_t,
    );
    first = 0 as uint32_t;
    last = 1 as uint32_t;
    num_sums = num_colors.wrapping_sub(2 as uint32_t);
    if num_sums > 0 as uint32_t {
        let mut best_sum: *mut Sum =
            (&raw mut sums as *mut Sum).offset(0 as ::core::ffi::c_int as isize) as *mut Sum;
        (*best_sum).index = 0 as uint8_t;
        (*best_sum).sum = 0 as ::core::ffi::c_uint as uint32_t;
        i = 0 as uint32_t;
        j = 0 as uint32_t;
        while i < num_colors {
            if !(i == remapping[0 as ::core::ffi::c_int as usize] as uint32_t
                || i == remapping[1 as ::core::ffi::c_int as usize] as uint32_t)
            {
                sums[j as usize].index = i as uint8_t;
                sums[j as usize].sum = (*cooccurrence.offset(
                    i.wrapping_mul(num_colors)
                        .wrapping_add(remapping[0 as ::core::ffi::c_int as usize] as uint32_t)
                        as isize,
                ))
                .wrapping_add(
                    *cooccurrence.offset(
                        i.wrapping_mul(num_colors)
                            .wrapping_add(remapping[1 as ::core::ffi::c_int as usize] as uint32_t)
                            as isize,
                    ),
                );
                if sums[j as usize].sum > (*best_sum).sum {
                    best_sum = (&raw mut sums as *mut Sum).offset(j as isize) as *mut Sum;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        while num_sums > 0 as uint32_t {
            let best_index: uint8_t = (*best_sum).index;
            let mut delta: int32_t = 0 as int32_t;
            let n: int32_t = num_colors.wrapping_sub(num_sums) as int32_t;
            ind = first;
            j = 0 as uint32_t;
            while ind.wrapping_add(j).wrapping_rem(num_colors) != last.wrapping_add(1 as uint32_t) {
                let l_j: uint16_t =
                    remapping[ind.wrapping_add(j).wrapping_rem(num_colors) as usize] as uint16_t;
                delta = (delta as ::core::ffi::c_int
                    + ((n - 1 as int32_t - 2 as int32_t * j as int32_t)
                        * *cooccurrence.offset(
                            (best_index as uint32_t)
                                .wrapping_mul(num_colors)
                                .wrapping_add(l_j as uint32_t) as isize,
                        ) as int32_t) as ::core::ffi::c_int) as int32_t;
                j = j.wrapping_add(1);
            }
            if delta > 0 as int32_t {
                first = if first == 0 as uint32_t {
                    num_colors.wrapping_sub(1 as uint32_t)
                } else {
                    first.wrapping_sub(1 as uint32_t)
                };
                remapping[first as usize] = best_index;
            } else {
                last = last.wrapping_add(1);
                remapping[last as usize] = best_index;
            }
            *best_sum = sums[num_sums.wrapping_sub(1 as uint32_t) as usize];
            num_sums = num_sums.wrapping_sub(1);
            best_sum =
                (&raw mut sums as *mut Sum).offset(0 as ::core::ffi::c_int as isize) as *mut Sum;
            i = 0 as uint32_t;
            while i < num_sums {
                sums[i as usize].sum = sums[i as usize].sum.wrapping_add(
                    *cooccurrence.offset(
                        (best_index as uint32_t)
                            .wrapping_mul(num_colors)
                            .wrapping_add(sums[i as usize].index as uint32_t)
                            as isize,
                    ),
                );
                if sums[i as usize].sum > (*best_sum).sum {
                    best_sum = (&raw mut sums as *mut Sum).offset(i as isize) as *mut Sum;
                }
                i = i.wrapping_add(1);
            }
        }
    }
    '_c2rust_label: {
        if last.wrapping_add(1 as uint32_t).wrapping_rem(num_colors) == first {
        } else {
            __assert_fail(
                b"(last + 1) % num_colors == first\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                315 as ::core::ffi::c_uint,
                b"int PaletteSortModifiedZeng(const WebPPicture *const, const uint32_t *const, uint32_t, uint32_t *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    WebPSafeFree(cooccurrence as *mut ::core::ffi::c_void);
    i = 0 as uint32_t;
    while i < num_colors {
        *palette.offset(i as isize) = *palette_sorted
            .offset(remapping[first.wrapping_add(i).wrapping_rem(num_colors) as usize] as isize);
        i = i.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn AddSingleSubGreen(mut p: uint32_t, r: *mut uint32_t, b: *mut uint32_t) {
    let green: ::core::ffi::c_int = p as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
    let ref mut fresh14 = *r.offset(
        ((p as ::core::ffi::c_int >> 16 as ::core::ffi::c_int) - green & 0xff as ::core::ffi::c_int)
            as isize,
    );
    *fresh14 = (*fresh14).wrapping_add(1);
    let ref mut fresh15 = *b.offset(
        ((p as ::core::ffi::c_int >> 0 as ::core::ffi::c_int) - green & 0xff as ::core::ffi::c_int)
            as isize,
    );
    *fresh15 = (*fresh15).wrapping_add(1);
}
unsafe extern "C" fn AddSingle(
    mut p: uint32_t,
    a: *mut uint32_t,
    r: *mut uint32_t,
    g: *mut uint32_t,
    b: *mut uint32_t,
) {
    let ref mut fresh16 = *a.offset((p >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as isize);
    *fresh16 = (*fresh16).wrapping_add(1);
    let ref mut fresh17 = *r.offset((p >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as isize);
    *fresh17 = (*fresh17).wrapping_add(1);
    let ref mut fresh18 = *g.offset((p >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as isize);
    *fresh18 = (*fresh18).wrapping_add(1);
    let ref mut fresh19 = *b.offset((p >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as isize);
    *fresh19 = (*fresh19).wrapping_add(1);
}
#[inline]
unsafe extern "C" fn HashPix(mut pix: uint32_t) -> uint32_t {
    return ((((pix as uint64_t).wrapping_add((pix >> 19 as ::core::ffi::c_int) as uint64_t)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x39c5fba7 as ::core::ffi::c_ulonglong)
        & 0xffffffff as ::core::ffi::c_ulonglong)
        >> 24 as ::core::ffi::c_int) as uint32_t;
}
unsafe extern "C" fn AnalyzeEntropy(
    mut argb: *const uint32_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut argb_stride: ::core::ffi::c_int,
    mut use_palette: ::core::ffi::c_int,
    mut palette_size: ::core::ffi::c_int,
    mut transform_bits: ::core::ffi::c_int,
    min_entropy_ix: *mut EntropyIx,
    red_and_blue_always_zero: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut histo: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    if use_palette != 0 && palette_size <= 16 as ::core::ffi::c_int {
        *min_entropy_ix = kPalette;
        *red_and_blue_always_zero = 1 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    histo = WebPSafeCalloc(
        kHistoTotal as ::core::ffi::c_int as uint64_t,
        (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(256 as size_t),
    ) as *mut uint32_t;
    if !histo.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_int = 0;
        let mut y: ::core::ffi::c_int = 0;
        let mut prev_row: *const uint32_t = ::core::ptr::null::<uint32_t>();
        let mut curr_row: *const uint32_t = argb;
        let mut pix_prev: uint32_t = *argb.offset(0 as ::core::ffi::c_int as isize);
        y = 0 as ::core::ffi::c_int;
        while y < height {
            x = 0 as ::core::ffi::c_int;
            while x < width {
                let pix: uint32_t = *curr_row.offset(x as isize);
                let pix_diff: uint32_t = VP8LSubPixels(pix, pix_prev) as uint32_t;
                pix_prev = pix;
                if !(pix_diff == 0 as uint32_t
                    || !prev_row.is_null() && pix == *prev_row.offset(x as isize))
                {
                    AddSingle(
                        pix,
                        histo.offset(
                            (kHistoAlpha as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint32_t,
                        histo.offset(
                            (kHistoRed as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as isize,
                        ) as *mut uint32_t,
                        histo.offset(
                            (kHistoGreen as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint32_t,
                        histo.offset(
                            (kHistoBlue as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as isize,
                        ) as *mut uint32_t,
                    );
                    AddSingle(
                        pix_diff,
                        histo.offset(
                            (kHistoAlphaPred as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint32_t,
                        histo.offset(
                            (kHistoRedPred as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint32_t,
                        histo.offset(
                            (kHistoGreenPred as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint32_t,
                        histo.offset(
                            (kHistoBluePred as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint32_t,
                    );
                    AddSingleSubGreen(
                        pix,
                        histo.offset(
                            (kHistoRedSubGreen as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint32_t,
                        histo.offset(
                            (kHistoBlueSubGreen as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint32_t,
                    );
                    AddSingleSubGreen(
                        pix_diff,
                        histo.offset(
                            (kHistoRedPredSubGreen as ::core::ffi::c_int
                                * 256 as ::core::ffi::c_int) as isize,
                        ) as *mut uint32_t,
                        histo.offset(
                            (kHistoBluePredSubGreen as ::core::ffi::c_int
                                * 256 as ::core::ffi::c_int) as isize,
                        ) as *mut uint32_t,
                    );
                    let hash: uint32_t = HashPix(pix) as uint32_t;
                    let ref mut fresh7 = *histo.offset(
                        ((kHistoPalette as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                            as uint32_t)
                            .wrapping_add(hash) as isize,
                    );
                    *fresh7 = (*fresh7).wrapping_add(1);
                }
                x += 1;
            }
            prev_row = curr_row;
            curr_row = curr_row.offset(argb_stride as isize);
            y += 1;
        }
        let mut entropy_comp: [::core::ffi::c_float; 13] = [0.; 13];
        let mut entropy: [::core::ffi::c_float; 6] = [0.; 6];
        let mut k: ::core::ffi::c_int = 0;
        let mut last_mode_to_analyze: ::core::ffi::c_int = if use_palette != 0 {
            kPalette as ::core::ffi::c_int
        } else {
            kSpatialSubGreen as ::core::ffi::c_int
        };
        let mut j: ::core::ffi::c_int = 0;
        let ref mut fresh8 = *histo.offset(
            (kHistoRedPredSubGreen as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as isize,
        );
        *fresh8 = (*fresh8).wrapping_add(1);
        let ref mut fresh9 = *histo.offset(
            (kHistoBluePredSubGreen as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as isize,
        );
        *fresh9 = (*fresh9).wrapping_add(1);
        let ref mut fresh10 = *histo
            .offset((kHistoRedPred as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as isize);
        *fresh10 = (*fresh10).wrapping_add(1);
        let ref mut fresh11 = *histo
            .offset((kHistoGreenPred as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as isize);
        *fresh11 = (*fresh11).wrapping_add(1);
        let ref mut fresh12 = *histo
            .offset((kHistoBluePred as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as isize);
        *fresh12 = (*fresh12).wrapping_add(1);
        let ref mut fresh13 = *histo
            .offset((kHistoAlphaPred as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as isize);
        *fresh13 = (*fresh13).wrapping_add(1);
        j = 0 as ::core::ffi::c_int;
        while j < kHistoTotal as ::core::ffi::c_int {
            entropy_comp[j as usize] = VP8LBitsEntropy(
                histo.offset((j * 256 as ::core::ffi::c_int) as isize) as *mut uint32_t,
                256 as ::core::ffi::c_int,
            );
            j += 1;
        }
        entropy[kDirect as ::core::ffi::c_int as usize] = entropy_comp
            [kHistoAlpha as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoRed as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoGreen as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoBlue as ::core::ffi::c_int as usize];
        entropy[kSpatial as ::core::ffi::c_int as usize] = entropy_comp
            [kHistoAlphaPred as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoRedPred as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoGreenPred as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoBluePred as ::core::ffi::c_int as usize];
        entropy[kSubGreen as ::core::ffi::c_int as usize] = entropy_comp
            [kHistoAlpha as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoRedSubGreen as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoGreen as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoBlueSubGreen as ::core::ffi::c_int as usize];
        entropy[kSpatialSubGreen as ::core::ffi::c_int as usize] = entropy_comp
            [kHistoAlphaPred as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoRedPredSubGreen as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoGreenPred as ::core::ffi::c_int as usize]
            + entropy_comp[kHistoBluePredSubGreen as ::core::ffi::c_int as usize];
        entropy[kPalette as ::core::ffi::c_int as usize] =
            entropy_comp[kHistoPalette as ::core::ffi::c_int as usize];
        entropy[kSpatial as ::core::ffi::c_int as usize] +=
            VP8LSubSampleSize(width as uint32_t, transform_bits as uint32_t).wrapping_mul(
                VP8LSubSampleSize(height as uint32_t, transform_bits as uint32_t),
            ) as ::core::ffi::c_float
                * VP8LFastLog2(14 as uint32_t);
        entropy[kSpatialSubGreen as ::core::ffi::c_int as usize] +=
            VP8LSubSampleSize(width as uint32_t, transform_bits as uint32_t).wrapping_mul(
                VP8LSubSampleSize(height as uint32_t, transform_bits as uint32_t),
            ) as ::core::ffi::c_float
                * VP8LFastLog2(24 as uint32_t);
        entropy[kPalette as ::core::ffi::c_int as usize] +=
            (palette_size * 8 as ::core::ffi::c_int) as ::core::ffi::c_float;
        *min_entropy_ix = kDirect;
        k = kDirect as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        while k <= last_mode_to_analyze {
            if entropy[*min_entropy_ix as usize] > entropy[k as usize] {
                *min_entropy_ix = k as EntropyIx;
            }
            k += 1;
        }
        '_c2rust_label: {
            if *min_entropy_ix as ::core::ffi::c_int <= last_mode_to_analyze {
            } else {
                __assert_fail(
                    b"(int)*min_entropy_ix <= last_mode_to_analyze\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    500 as ::core::ffi::c_uint,
                    b"int AnalyzeEntropy(const uint32_t *, int, int, int, int, int, int, EntropyIx *const, int *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        *red_and_blue_always_zero = 1 as ::core::ffi::c_int;
        static mut kHistoPairs: [[uint8_t; 2]; 5] = [
            [
                kHistoRed as ::core::ffi::c_int as uint8_t,
                kHistoBlue as ::core::ffi::c_int as uint8_t,
            ],
            [
                kHistoRedPred as ::core::ffi::c_int as uint8_t,
                kHistoBluePred as ::core::ffi::c_int as uint8_t,
            ],
            [
                kHistoRedSubGreen as ::core::ffi::c_int as uint8_t,
                kHistoBlueSubGreen as ::core::ffi::c_int as uint8_t,
            ],
            [
                kHistoRedPredSubGreen as ::core::ffi::c_int as uint8_t,
                kHistoBluePredSubGreen as ::core::ffi::c_int as uint8_t,
            ],
            [
                kHistoRed as ::core::ffi::c_int as uint8_t,
                kHistoBlue as ::core::ffi::c_int as uint8_t,
            ],
        ];
        let red_histo: *const uint32_t = histo.offset(
            (256 as ::core::ffi::c_int
                * *(&raw const *(&raw const kHistoPairs as *const [uint8_t; 2])
                    .offset(*min_entropy_ix as isize) as *const uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int) as isize,
        ) as *mut uint32_t;
        let blue_histo: *const uint32_t = histo.offset(
            (256 as ::core::ffi::c_int
                * *(&raw const *(&raw const kHistoPairs as *const [uint8_t; 2])
                    .offset(*min_entropy_ix as isize) as *const uint8_t)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int) as isize,
        ) as *mut uint32_t;
        i = 1 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            if *red_histo.offset(i as isize) | *blue_histo.offset(i as isize) != 0 as uint32_t {
                *red_and_blue_always_zero = 0 as ::core::ffi::c_int;
                break;
            } else {
                i += 1;
            }
        }
        WebPSafeFree(histo as *mut ::core::ffi::c_void);
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn GetHistoBits(
    mut method: ::core::ffi::c_int,
    mut use_palette: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut histo_bits: ::core::ffi::c_int = (if use_palette != 0 {
        9 as ::core::ffi::c_int
    } else {
        7 as ::core::ffi::c_int
    }) - method;
    loop {
        let huff_image_size: ::core::ffi::c_int =
            VP8LSubSampleSize(width as uint32_t, histo_bits as uint32_t).wrapping_mul(
                VP8LSubSampleSize(height as uint32_t, histo_bits as uint32_t),
            ) as ::core::ffi::c_int;
        if huff_image_size <= MAX_HUFF_IMAGE_SIZE {
            break;
        }
        histo_bits += 1;
    }
    return if histo_bits < MIN_HUFFMAN_BITS {
        MIN_HUFFMAN_BITS
    } else if histo_bits > MAX_HUFFMAN_BITS {
        MAX_HUFFMAN_BITS
    } else {
        histo_bits
    };
}
unsafe extern "C" fn GetTransformBits(
    mut method: ::core::ffi::c_int,
    mut histo_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let max_transform_bits: ::core::ffi::c_int = if method < 4 as ::core::ffi::c_int {
        6 as ::core::ffi::c_int
    } else if method > 4 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        5 as ::core::ffi::c_int
    };
    let res: ::core::ffi::c_int = if histo_bits > max_transform_bits {
        max_transform_bits
    } else {
        histo_bits
    };
    '_c2rust_label: {
        if res <= 6 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"res <= MAX_TRANSFORM_BITS\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                549 as ::core::ffi::c_uint,
                b"int GetTransformBits(int, int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return res;
}
unsafe extern "C" fn EncoderAnalyze(
    enc: *mut VP8LEncoder,
    mut crunch_configs: *mut CrunchConfig,
    crunch_configs_size: *mut ::core::ffi::c_int,
    red_and_blue_always_zero: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pic: *const WebPPicture = (*enc).pic_;
    let width: ::core::ffi::c_int = (*pic).width;
    let height: ::core::ffi::c_int = (*pic).height;
    let config: *const WebPConfig = (*enc).config_;
    let method: ::core::ffi::c_int = (*config).method;
    let low_effort: ::core::ffi::c_int =
        ((*config).method == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut use_palette: ::core::ffi::c_int = 0;
    let mut n_lz77s: ::core::ffi::c_int = 0;
    let mut do_no_cache: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_c2rust_label: {
        if !pic.is_null() && !(*pic).argb.is_null() {
        } else {
            __assert_fail(
                b"pic != NULL && pic->argb != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                586 as ::core::ffi::c_uint,
                b"int EncoderAnalyze(VP8LEncoder *const, CrunchConfig *, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*enc).palette_size_ = WebPGetColorPalette(
        pic as *const WebPPicture,
        &raw mut (*enc).palette_sorted_ as *mut uint32_t,
    );
    use_palette = ((*enc).palette_size_ <= MAX_PALETTE_SIZE) as ::core::ffi::c_int;
    if use_palette == 0 {
        (*enc).palette_size_ = 0 as ::core::ffi::c_int;
    } else {
        qsort(
            &raw mut (*enc).palette_sorted_ as *mut uint32_t as *mut ::core::ffi::c_void,
            (*enc).palette_size_ as size_t,
            ::core::mem::size_of::<uint32_t>() as size_t,
            Some(
                PaletteCompareColorsForQsort
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    (*enc).histo_bits_ = GetHistoBits(method, use_palette, (*pic).width, (*pic).height);
    (*enc).transform_bits_ = GetTransformBits(method, (*enc).histo_bits_);
    if low_effort != 0 {
        (*crunch_configs.offset(0 as ::core::ffi::c_int as isize)).entropy_idx_ =
            if use_palette != 0 {
                kPalette as ::core::ffi::c_int
            } else {
                kSpatialSubGreen as ::core::ffi::c_int
            };
        (*crunch_configs.offset(0 as ::core::ffi::c_int as isize)).palette_sorting_type_ =
            (if use_palette != 0 {
                kSortedDefault as ::core::ffi::c_int
            } else {
                kUnusedPalette as ::core::ffi::c_int
            }) as PaletteSorting;
        n_lz77s = 1 as ::core::ffi::c_int;
        *crunch_configs_size = 1 as ::core::ffi::c_int;
    } else {
        let mut min_entropy_ix: EntropyIx = kDirect;
        n_lz77s = if (*enc).palette_size_ > 0 as ::core::ffi::c_int
            && (*enc).palette_size_ <= 16 as ::core::ffi::c_int
        {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        if AnalyzeEntropy(
            (*pic).argb,
            width,
            height,
            (*pic).argb_stride,
            use_palette,
            (*enc).palette_size_,
            (*enc).transform_bits_,
            &raw mut min_entropy_ix,
            red_and_blue_always_zero,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if method == 6 as ::core::ffi::c_int
            && (*config).quality == 100 as ::core::ffi::c_int as ::core::ffi::c_float
        {
            do_no_cache = 1 as ::core::ffi::c_int;
            *crunch_configs_size = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < kNumEntropyIx as ::core::ffi::c_int {
                if i != kPalette as ::core::ffi::c_int
                    && i != kPaletteAndSpatial as ::core::ffi::c_int
                    || use_palette != 0
                {
                    '_c2rust_label_0: {
                        if *crunch_configs_size
                            < kNumEntropyIx as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                        {
                        } else {
                            __assert_fail(
                                b"*crunch_configs_size < CRUNCH_CONFIGS_MAX\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                627 as ::core::ffi::c_uint,
                                b"int EncoderAnalyze(VP8LEncoder *const, CrunchConfig *, int *const, int *const)\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    (*crunch_configs.offset(*crunch_configs_size as isize)).entropy_idx_ = i;
                    if use_palette != 0
                        && (i == kPalette as ::core::ffi::c_int
                            || i == kPaletteAndSpatial as ::core::ffi::c_int)
                    {
                        (*crunch_configs.offset(*crunch_configs_size as isize))
                            .palette_sorting_type_ = kMinimizeDelta;
                        *crunch_configs_size += 1;
                        (*crunch_configs.offset(*crunch_configs_size as isize)).entropy_idx_ = i;
                        (*crunch_configs.offset(*crunch_configs_size as isize))
                            .palette_sorting_type_ = kModifiedZeng;
                    } else {
                        (*crunch_configs.offset(*crunch_configs_size as isize))
                            .palette_sorting_type_ = kUnusedPalette;
                    }
                    *crunch_configs_size += 1;
                }
                i += 1;
            }
        } else {
            *crunch_configs_size = 1 as ::core::ffi::c_int;
            (*crunch_configs.offset(0 as ::core::ffi::c_int as isize)).entropy_idx_ =
                min_entropy_ix as ::core::ffi::c_int;
            (*crunch_configs.offset(0 as ::core::ffi::c_int as isize)).palette_sorting_type_ =
                (if use_palette != 0 {
                    kMinimizeDelta as ::core::ffi::c_int
                } else {
                    kUnusedPalette as ::core::ffi::c_int
                }) as PaletteSorting;
            if (*config).quality >= 75 as ::core::ffi::c_int as ::core::ffi::c_float
                && method == 5 as ::core::ffi::c_int
            {
                do_no_cache = 1 as ::core::ffi::c_int;
                if min_entropy_ix as ::core::ffi::c_uint
                    == kPalette as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    *crunch_configs_size = 2 as ::core::ffi::c_int;
                    (*crunch_configs.offset(1 as ::core::ffi::c_int as isize)).entropy_idx_ =
                        kPaletteAndSpatial as ::core::ffi::c_int;
                    (*crunch_configs.offset(1 as ::core::ffi::c_int as isize))
                        .palette_sorting_type_ = kMinimizeDelta;
                }
            }
        }
    }
    '_c2rust_label_1: {
        if n_lz77s <= 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"n_lz77s <= CRUNCH_SUBCONFIGS_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                663 as ::core::ffi::c_uint,
                b"int EncoderAnalyze(VP8LEncoder *const, CrunchConfig *, int *const, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    i = 0 as ::core::ffi::c_int;
    while i < *crunch_configs_size {
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while j < n_lz77s {
            '_c2rust_label_2: {
                if j < 2 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"j < CRUNCH_SUBCONFIGS_MAX\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        667 as ::core::ffi::c_uint,
                        b"int EncoderAnalyze(VP8LEncoder *const, CrunchConfig *, int *const, int *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            (*crunch_configs.offset(i as isize)).sub_configs_[j as usize].lz77_ =
                if j == 0 as ::core::ffi::c_int {
                    kLZ77Standard as ::core::ffi::c_int | kLZ77RLE as ::core::ffi::c_int
                } else {
                    kLZ77Box as ::core::ffi::c_int
                };
            (*crunch_configs.offset(i as isize)).sub_configs_[j as usize].do_no_cache_ =
                do_no_cache;
            j += 1;
        }
        (*crunch_configs.offset(i as isize)).sub_configs_size_ = n_lz77s;
        i += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn EncoderInit(enc: *mut VP8LEncoder) -> ::core::ffi::c_int {
    let pic: *const WebPPicture = (*enc).pic_;
    let width: ::core::ffi::c_int = (*pic).width;
    let height: ::core::ffi::c_int = (*pic).height;
    let pix_cnt: ::core::ffi::c_int = width * height;
    let refs_block_size: ::core::ffi::c_int =
        (pix_cnt - 1 as ::core::ffi::c_int) / MAX_REFS_BLOCK_PER_IMAGE + 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    if VP8LHashChainInit(&raw mut (*enc).hash_chain_, pix_cnt) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        VP8LBackwardRefsInit(
            (&raw mut (*enc).refs_ as *mut VP8LBackwardRefs).offset(i as isize)
                as *mut VP8LBackwardRefs,
            refs_block_size,
        );
        i += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn GetHuffBitLengthsAndCodes(
    histogram_image: *const VP8LHistogramSet,
    huffman_codes: *mut HuffmanTreeCode,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut total_length_size: uint64_t = 0 as uint64_t;
    let mut mem_buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let histogram_image_size: ::core::ffi::c_int = (*histogram_image).size;
    let mut max_num_symbols: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buf_rle: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut huff_tree: *mut HuffmanTree = ::core::ptr::null_mut::<HuffmanTree>();
    i = 0 as ::core::ffi::c_int;
    while i < histogram_image_size {
        let histo: *const VP8LHistogram = *(*histogram_image).histograms.offset(i as isize);
        let codes: *mut HuffmanTreeCode =
            huffman_codes.offset((5 as ::core::ffi::c_int * i) as isize) as *mut HuffmanTreeCode;
        '_c2rust_label: {
            if !histo.is_null() {
            } else {
                __assert_fail(
                    b"histo != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    710 as ::core::ffi::c_uint,
                    b"int GetHuffBitLengthsAndCodes(const VP8LHistogramSet *const, HuffmanTreeCode *const)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        k = 0 as ::core::ffi::c_int;
        while k < 5 as ::core::ffi::c_int {
            let num_symbols: ::core::ffi::c_int = if k == 0 as ::core::ffi::c_int {
                VP8LHistogramNumCodes((*histo).palette_code_bits_) as ::core::ffi::c_int
            } else if k == 4 as ::core::ffi::c_int {
                NUM_DISTANCE_CODES
            } else {
                256 as ::core::ffi::c_int
            };
            (*codes.offset(k as isize)).num_symbols = num_symbols;
            total_length_size = total_length_size.wrapping_add(num_symbols as uint64_t);
            k += 1;
        }
        i += 1;
    }
    let mut codes_0: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    let mut lengths: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    mem_buf = WebPSafeCalloc(
        total_length_size,
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_add(::core::mem::size_of::<uint16_t>() as size_t),
    ) as *mut uint8_t;
    if !mem_buf.is_null() {
        codes_0 = mem_buf as *mut uint16_t;
        lengths = codes_0.offset(total_length_size as isize) as *mut uint16_t as *mut uint8_t;
        i = 0 as ::core::ffi::c_int;
        while i < 5 as ::core::ffi::c_int * histogram_image_size {
            let bit_length: ::core::ffi::c_int = (*huffman_codes.offset(i as isize)).num_symbols;
            let ref mut fresh1 = (*huffman_codes.offset(i as isize)).codes;
            *fresh1 = codes_0;
            let ref mut fresh2 = (*huffman_codes.offset(i as isize)).code_lengths;
            *fresh2 = lengths;
            codes_0 = codes_0.offset(bit_length as isize);
            lengths = lengths.offset(bit_length as isize);
            if max_num_symbols < bit_length {
                max_num_symbols = bit_length;
            }
            i += 1;
        }
        buf_rle = WebPSafeMalloc(1 as uint64_t, max_num_symbols as size_t) as *mut uint8_t;
        huff_tree = WebPSafeMalloc(
            (3 as ::core::ffi::c_ulonglong)
                .wrapping_mul(max_num_symbols as ::core::ffi::c_ulonglong) as uint64_t,
            ::core::mem::size_of::<HuffmanTree>() as size_t,
        ) as *mut HuffmanTree;
        if !(buf_rle.is_null() || huff_tree.is_null()) {
            i = 0 as ::core::ffi::c_int;
            while i < histogram_image_size {
                let codes_1: *mut HuffmanTreeCode = huffman_codes
                    .offset((5 as ::core::ffi::c_int * i) as isize)
                    as *mut HuffmanTreeCode;
                let histo_0: *mut VP8LHistogram = *(*histogram_image).histograms.offset(i as isize);
                VP8LCreateHuffmanTree(
                    (*histo_0).literal_,
                    15 as ::core::ffi::c_int,
                    buf_rle,
                    huff_tree,
                    codes_1.offset(0 as ::core::ffi::c_int as isize),
                );
                VP8LCreateHuffmanTree(
                    &raw mut (*histo_0).red_ as *mut uint32_t,
                    15 as ::core::ffi::c_int,
                    buf_rle,
                    huff_tree,
                    codes_1.offset(1 as ::core::ffi::c_int as isize),
                );
                VP8LCreateHuffmanTree(
                    &raw mut (*histo_0).blue_ as *mut uint32_t,
                    15 as ::core::ffi::c_int,
                    buf_rle,
                    huff_tree,
                    codes_1.offset(2 as ::core::ffi::c_int as isize),
                );
                VP8LCreateHuffmanTree(
                    &raw mut (*histo_0).alpha_ as *mut uint32_t,
                    15 as ::core::ffi::c_int,
                    buf_rle,
                    huff_tree,
                    codes_1.offset(3 as ::core::ffi::c_int as isize),
                );
                VP8LCreateHuffmanTree(
                    &raw mut (*histo_0).distance_ as *mut uint32_t,
                    15 as ::core::ffi::c_int,
                    buf_rle,
                    huff_tree,
                    codes_1.offset(4 as ::core::ffi::c_int as isize),
                );
                i += 1;
            }
            ok = 1 as ::core::ffi::c_int;
        }
    }
    WebPSafeFree(huff_tree as *mut ::core::ffi::c_void);
    WebPSafeFree(buf_rle as *mut ::core::ffi::c_void);
    if ok == 0 {
        WebPSafeFree(mem_buf as *mut ::core::ffi::c_void);
        memset(
            huffman_codes as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((5 as ::core::ffi::c_int * histogram_image_size) as size_t)
                .wrapping_mul(::core::mem::size_of::<HuffmanTreeCode>() as size_t),
        );
    }
    return ok;
}
unsafe extern "C" fn StoreHuffmanTreeOfHuffmanTreeToBitMask(
    bw: *mut VP8LBitWriter,
    mut code_length_bitdepth: *const uint8_t,
) {
    static mut kStorageOrder: [uint8_t; 19] = [
        17 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
    ];
    let mut i: ::core::ffi::c_int = 0;
    let mut codes_to_store: ::core::ffi::c_int = CODE_LENGTH_CODES;
    while codes_to_store > 4 as ::core::ffi::c_int {
        if *code_length_bitdepth
            .offset(kStorageOrder[(codes_to_store - 1 as ::core::ffi::c_int) as usize] as isize)
            as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            break;
        }
        codes_to_store -= 1;
    }
    VP8LPutBits(
        bw,
        (codes_to_store - 4 as ::core::ffi::c_int) as uint32_t,
        4 as ::core::ffi::c_int,
    );
    i = 0 as ::core::ffi::c_int;
    while i < codes_to_store {
        VP8LPutBits(
            bw,
            *code_length_bitdepth.offset(kStorageOrder[i as usize] as isize) as uint32_t,
            3 as ::core::ffi::c_int,
        );
        i += 1;
    }
}
unsafe extern "C" fn ClearHuffmanTreeIfOnlyOneSymbol(huffman_code: *mut HuffmanTreeCode) {
    let mut k: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    k = 0 as ::core::ffi::c_int;
    while k < (*huffman_code).num_symbols {
        if *(*huffman_code).code_lengths.offset(k as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            count += 1;
            if count > 1 as ::core::ffi::c_int {
                return;
            }
        }
        k += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < (*huffman_code).num_symbols {
        *(*huffman_code).code_lengths.offset(k as isize) = 0 as uint8_t;
        *(*huffman_code).codes.offset(k as isize) = 0 as uint16_t;
        k += 1;
    }
}
unsafe extern "C" fn StoreHuffmanTreeToBitMask(
    bw: *mut VP8LBitWriter,
    tokens: *const HuffmanTreeToken,
    num_tokens: ::core::ffi::c_int,
    huffman_code: *const HuffmanTreeCode,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < num_tokens {
        let ix: ::core::ffi::c_int = (*tokens.offset(i as isize)).code as ::core::ffi::c_int;
        let extra_bits: ::core::ffi::c_int =
            (*tokens.offset(i as isize)).extra_bits as ::core::ffi::c_int;
        VP8LPutBits(
            bw,
            *(*huffman_code).codes.offset(ix as isize) as uint32_t,
            *(*huffman_code).code_lengths.offset(ix as isize) as ::core::ffi::c_int,
        );
        match ix {
            16 => {
                VP8LPutBits(bw, extra_bits as uint32_t, 2 as ::core::ffi::c_int);
            }
            17 => {
                VP8LPutBits(bw, extra_bits as uint32_t, 3 as ::core::ffi::c_int);
            }
            18 => {
                VP8LPutBits(bw, extra_bits as uint32_t, 7 as ::core::ffi::c_int);
            }
            _ => {}
        }
        i += 1;
    }
}
unsafe extern "C" fn StoreFullHuffmanCode(
    bw: *mut VP8LBitWriter,
    huff_tree: *mut HuffmanTree,
    tokens: *mut HuffmanTreeToken,
    tree: *const HuffmanTreeCode,
) {
    let mut code_length_bitdepth: [uint8_t; 19] = [
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
    ];
    let mut code_length_bitdepth_symbols: [uint16_t; 19] = [
        0 as ::core::ffi::c_int as uint16_t,
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
    let max_tokens: ::core::ffi::c_int = (*tree).num_symbols;
    let mut num_tokens: ::core::ffi::c_int = 0;
    let mut huffman_code: HuffmanTreeCode = HuffmanTreeCode {
        num_symbols: 0,
        code_lengths: ::core::ptr::null_mut::<uint8_t>(),
        codes: ::core::ptr::null_mut::<uint16_t>(),
    };
    huffman_code.num_symbols = CODE_LENGTH_CODES;
    huffman_code.code_lengths = &raw mut code_length_bitdepth as *mut uint8_t;
    huffman_code.codes = &raw mut code_length_bitdepth_symbols as *mut uint16_t;
    VP8LPutBits(bw, 0 as uint32_t, 1 as ::core::ffi::c_int);
    num_tokens = VP8LCreateCompressedHuffmanTree(tree, tokens, max_tokens);
    let mut histogram: [uint32_t; 19] = [
        0 as ::core::ffi::c_int as uint32_t,
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
    let mut buf_rle: [uint8_t; 19] = [
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
    ];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < num_tokens {
        histogram[(*tokens.offset(i as isize)).code as usize] =
            histogram[(*tokens.offset(i as isize)).code as usize].wrapping_add(1);
        i += 1;
    }
    VP8LCreateHuffmanTree(
        &raw mut histogram as *mut uint32_t,
        7 as ::core::ffi::c_int,
        &raw mut buf_rle as *mut uint8_t,
        huff_tree,
        &raw mut huffman_code,
    );
    StoreHuffmanTreeOfHuffmanTreeToBitMask(bw, &raw mut code_length_bitdepth as *mut uint8_t);
    ClearHuffmanTreeIfOnlyOneSymbol(&raw mut huffman_code);
    let mut trailing_zero_bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut trimmed_length: ::core::ffi::c_int = num_tokens;
    let mut write_trimmed_length: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    let mut i_0: ::core::ffi::c_int = num_tokens;
    loop {
        let fresh0 = i_0;
        i_0 = i_0 - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        let ix: ::core::ffi::c_int = (*tokens.offset(i_0 as isize)).code as ::core::ffi::c_int;
        if !(ix == 0 as ::core::ffi::c_int
            || ix == 17 as ::core::ffi::c_int
            || ix == 18 as ::core::ffi::c_int)
        {
            break;
        }
        trimmed_length -= 1;
        trailing_zero_bits += code_length_bitdepth[ix as usize] as ::core::ffi::c_int;
        if ix == 17 as ::core::ffi::c_int {
            trailing_zero_bits += 3 as ::core::ffi::c_int;
        } else if ix == 18 as ::core::ffi::c_int {
            trailing_zero_bits += 7 as ::core::ffi::c_int;
        }
    }
    write_trimmed_length = (trimmed_length > 1 as ::core::ffi::c_int
        && trailing_zero_bits > 12 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    length = if write_trimmed_length != 0 {
        trimmed_length
    } else {
        num_tokens
    };
    VP8LPutBits(
        bw,
        write_trimmed_length as uint32_t,
        1 as ::core::ffi::c_int,
    );
    if write_trimmed_length != 0 {
        if trimmed_length == 2 as ::core::ffi::c_int {
            VP8LPutBits(
                bw,
                0 as uint32_t,
                3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
            );
        } else {
            let nbits: ::core::ffi::c_int =
                BitsLog2Floor((trimmed_length - 2 as ::core::ffi::c_int) as uint32_t)
                    as ::core::ffi::c_int;
            let nbitpairs: ::core::ffi::c_int =
                nbits / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            '_c2rust_label: {
                if trimmed_length > 2 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"trimmed_length > 2\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        887 as ::core::ffi::c_uint,
                        b"void StoreFullHuffmanCode(VP8LBitWriter *const, HuffmanTree *const, HuffmanTreeToken *const, const HuffmanTreeCode *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_0: {
                if (nbitpairs - 1 as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"nbitpairs - 1 < 8\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        888 as ::core::ffi::c_uint,
                        b"void StoreFullHuffmanCode(VP8LBitWriter *const, HuffmanTree *const, HuffmanTreeToken *const, const HuffmanTreeCode *const)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            VP8LPutBits(
                bw,
                (nbitpairs - 1 as ::core::ffi::c_int) as uint32_t,
                3 as ::core::ffi::c_int,
            );
            VP8LPutBits(
                bw,
                (trimmed_length - 2 as ::core::ffi::c_int) as uint32_t,
                nbitpairs * 2 as ::core::ffi::c_int,
            );
        }
    }
    StoreHuffmanTreeToBitMask(bw, tokens, length, &raw mut huffman_code);
}
unsafe extern "C" fn StoreHuffmanCode(
    bw: *mut VP8LBitWriter,
    huff_tree: *mut HuffmanTree,
    tokens: *mut HuffmanTreeToken,
    huffman_code: *const HuffmanTreeCode,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut symbols: [::core::ffi::c_int; 2] = [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int];
    let kMaxBits: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let kMaxSymbol: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << kMaxBits;
    i = 0 as ::core::ffi::c_int;
    while i < (*huffman_code).num_symbols && count < 3 as ::core::ffi::c_int {
        if *(*huffman_code).code_lengths.offset(i as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            if count < 2 as ::core::ffi::c_int {
                symbols[count as usize] = i;
            }
            count += 1;
        }
        i += 1;
    }
    if count == 0 as ::core::ffi::c_int {
        VP8LPutBits(bw, 0x1 as uint32_t, 4 as ::core::ffi::c_int);
    } else if count <= 2 as ::core::ffi::c_int
        && symbols[0 as ::core::ffi::c_int as usize] < kMaxSymbol
        && symbols[1 as ::core::ffi::c_int as usize] < kMaxSymbol
    {
        VP8LPutBits(bw, 1 as uint32_t, 1 as ::core::ffi::c_int);
        VP8LPutBits(
            bw,
            (count - 1 as ::core::ffi::c_int) as uint32_t,
            1 as ::core::ffi::c_int,
        );
        if symbols[0 as ::core::ffi::c_int as usize] <= 1 as ::core::ffi::c_int {
            VP8LPutBits(bw, 0 as uint32_t, 1 as ::core::ffi::c_int);
            VP8LPutBits(
                bw,
                symbols[0 as ::core::ffi::c_int as usize] as uint32_t,
                1 as ::core::ffi::c_int,
            );
        } else {
            VP8LPutBits(bw, 1 as uint32_t, 1 as ::core::ffi::c_int);
            VP8LPutBits(
                bw,
                symbols[0 as ::core::ffi::c_int as usize] as uint32_t,
                8 as ::core::ffi::c_int,
            );
        }
        if count == 2 as ::core::ffi::c_int {
            VP8LPutBits(
                bw,
                symbols[1 as ::core::ffi::c_int as usize] as uint32_t,
                8 as ::core::ffi::c_int,
            );
        }
    } else {
        StoreFullHuffmanCode(bw, huff_tree, tokens, huffman_code);
    };
}
#[inline]
unsafe extern "C" fn WriteHuffmanCode(
    bw: *mut VP8LBitWriter,
    code: *const HuffmanTreeCode,
    mut code_index: ::core::ffi::c_int,
) {
    let depth: ::core::ffi::c_int =
        *(*code).code_lengths.offset(code_index as isize) as ::core::ffi::c_int;
    let symbol: ::core::ffi::c_int =
        *(*code).codes.offset(code_index as isize) as ::core::ffi::c_int;
    VP8LPutBits(bw, symbol as uint32_t, depth);
}
#[inline]
unsafe extern "C" fn WriteHuffmanCodeWithExtraBits(
    bw: *mut VP8LBitWriter,
    code: *const HuffmanTreeCode,
    mut code_index: ::core::ffi::c_int,
    mut bits: ::core::ffi::c_int,
    mut n_bits: ::core::ffi::c_int,
) {
    let depth: ::core::ffi::c_int =
        *(*code).code_lengths.offset(code_index as isize) as ::core::ffi::c_int;
    let symbol: ::core::ffi::c_int =
        *(*code).codes.offset(code_index as isize) as ::core::ffi::c_int;
    VP8LPutBits(bw, (bits << depth | symbol) as uint32_t, depth + n_bits);
}
unsafe extern "C" fn StoreImageToBitMask(
    bw: *mut VP8LBitWriter,
    mut width: ::core::ffi::c_int,
    mut histo_bits: ::core::ffi::c_int,
    refs: *const VP8LBackwardRefs,
    mut histogram_symbols: *const uint16_t,
    huffman_codes: *const HuffmanTreeCode,
    pic: *const WebPPicture,
) -> ::core::ffi::c_int {
    let histo_xsize: ::core::ffi::c_int = (if histo_bits != 0 {
        VP8LSubSampleSize(width as uint32_t, histo_bits as uint32_t)
    } else {
        1 as uint32_t
    }) as ::core::ffi::c_int;
    let tile_mask: ::core::ffi::c_int = if histo_bits == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -((1 as ::core::ffi::c_int) << histo_bits)
    };
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tile_x: ::core::ffi::c_int = x & tile_mask;
    let mut tile_y: ::core::ffi::c_int = y & tile_mask;
    let mut histogram_ix: ::core::ffi::c_int =
        *histogram_symbols.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let mut codes: *const HuffmanTreeCode =
        huffman_codes.offset((5 as ::core::ffi::c_int * histogram_ix) as isize);
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    while VP8LRefsCursorOk(&raw mut c) != 0 {
        let v: *const PixOrCopy = c.cur_pos;
        if tile_x != x & tile_mask || tile_y != y & tile_mask {
            tile_x = x & tile_mask;
            tile_y = y & tile_mask;
            histogram_ix = *histogram_symbols
                .offset(((y >> histo_bits) * histo_xsize + (x >> histo_bits)) as isize)
                as ::core::ffi::c_int;
            codes = huffman_codes.offset((5 as ::core::ffi::c_int * histogram_ix) as isize);
        }
        if PixOrCopyIsLiteral(v) != 0 {
            static mut order: [uint8_t; 4] = [
                1 as ::core::ffi::c_int as uint8_t,
                2 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
            ];
            let mut k: ::core::ffi::c_int = 0;
            k = 0 as ::core::ffi::c_int;
            while k < 4 as ::core::ffi::c_int {
                let code: ::core::ffi::c_int =
                    PixOrCopyLiteral(v, order[k as usize] as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                WriteHuffmanCode(bw, codes.offset(k as isize), code);
                k += 1;
            }
        } else if PixOrCopyIsCacheIdx(v) != 0 {
            let code_0: ::core::ffi::c_int = PixOrCopyCacheIdx(v) as ::core::ffi::c_int;
            let literal_ix: ::core::ffi::c_int =
                256 as ::core::ffi::c_int + NUM_LENGTH_CODES + code_0;
            WriteHuffmanCode(bw, codes, literal_ix);
        } else {
            let mut bits: ::core::ffi::c_int = 0;
            let mut n_bits: ::core::ffi::c_int = 0;
            let mut code_1: ::core::ffi::c_int = 0;
            let distance: ::core::ffi::c_int = PixOrCopyDistance(v) as ::core::ffi::c_int;
            VP8LPrefixEncode(
                (*v).len as ::core::ffi::c_int,
                &raw mut code_1,
                &raw mut n_bits,
                &raw mut bits,
            );
            WriteHuffmanCodeWithExtraBits(
                bw,
                codes,
                256 as ::core::ffi::c_int + code_1,
                bits,
                n_bits,
            );
            VP8LPrefixEncode(distance, &raw mut code_1, &raw mut n_bits, &raw mut bits);
            WriteHuffmanCode(bw, codes.offset(4 as ::core::ffi::c_int as isize), code_1);
            VP8LPutBits(bw, bits as uint32_t, n_bits);
        }
        x = (x as uint32_t).wrapping_add(PixOrCopyLength(v)) as ::core::ffi::c_int
            as ::core::ffi::c_int;
        while x >= width {
            x -= width;
            y += 1;
        }
        VP8LRefsCursorNext(&raw mut c);
    }
    if (*bw).error_ != 0 {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn EncodeImageNoHuffman(
    bw: *mut VP8LBitWriter,
    argb: *const uint32_t,
    hash_chain: *mut VP8LHashChain,
    refs_array: *mut VP8LBackwardRefs,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_int,
    mut low_effort: ::core::ffi::c_int,
    pic: *const WebPPicture,
    mut percent_range: ::core::ffi::c_int,
    percent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut max_tokens: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut refs: *mut VP8LBackwardRefs = ::core::ptr::null_mut::<VP8LBackwardRefs>();
    let mut tokens: *mut HuffmanTreeToken = ::core::ptr::null_mut::<HuffmanTreeToken>();
    let mut huffman_codes: [HuffmanTreeCode; 5] = [
        HuffmanTreeCode {
            num_symbols: 0 as ::core::ffi::c_int,
            code_lengths: ::core::ptr::null_mut::<uint8_t>(),
            codes: ::core::ptr::null_mut::<uint16_t>(),
        },
        HuffmanTreeCode {
            num_symbols: 0,
            code_lengths: ::core::ptr::null_mut::<uint8_t>(),
            codes: ::core::ptr::null_mut::<uint16_t>(),
        },
        HuffmanTreeCode {
            num_symbols: 0,
            code_lengths: ::core::ptr::null_mut::<uint8_t>(),
            codes: ::core::ptr::null_mut::<uint16_t>(),
        },
        HuffmanTreeCode {
            num_symbols: 0,
            code_lengths: ::core::ptr::null_mut::<uint8_t>(),
            codes: ::core::ptr::null_mut::<uint16_t>(),
        },
        HuffmanTreeCode {
            num_symbols: 0,
            code_lengths: ::core::ptr::null_mut::<uint8_t>(),
            codes: ::core::ptr::null_mut::<uint16_t>(),
        },
    ];
    let histogram_symbols: [uint16_t; 1] = [0 as ::core::ffi::c_int as uint16_t];
    let mut cache_bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut histogram_image: *mut VP8LHistogramSet = ::core::ptr::null_mut::<VP8LHistogramSet>();
    let huff_tree: *mut HuffmanTree = WebPSafeMalloc(
        (3 as ::core::ffi::c_ulonglong).wrapping_mul(CODE_LENGTH_CODES as ::core::ffi::c_ulonglong)
            as uint64_t,
        ::core::mem::size_of::<HuffmanTree>() as size_t,
    ) as *mut HuffmanTree;
    if huff_tree.is_null() {
        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else if !(VP8LHashChainFill(
        hash_chain,
        quality,
        argb,
        width,
        height,
        low_effort,
        pic,
        percent_range / 2 as ::core::ffi::c_int,
        percent,
    ) == 0)
    {
        if !(VP8LGetBackwardReferences(
            width,
            height,
            argb,
            quality,
            0 as ::core::ffi::c_int,
            kLZ77Standard as ::core::ffi::c_int | kLZ77RLE as ::core::ffi::c_int,
            cache_bits,
            0 as ::core::ffi::c_int,
            hash_chain,
            refs_array,
            &raw mut cache_bits,
            pic,
            percent_range - percent_range / 2 as ::core::ffi::c_int,
            percent,
        ) == 0)
        {
            refs = refs_array.offset(0 as ::core::ffi::c_int as isize) as *mut VP8LBackwardRefs;
            histogram_image = VP8LAllocateHistogramSet(1 as ::core::ffi::c_int, cache_bits);
            if histogram_image.is_null() {
                WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
            } else {
                VP8LHistogramSetClear(histogram_image);
                VP8LHistogramStoreRefs(
                    refs,
                    *(*histogram_image)
                        .histograms
                        .offset(0 as ::core::ffi::c_int as isize),
                );
                '_c2rust_label: {
                    if (*histogram_image).size == 1 as ::core::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"histogram_image->size == 1\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            1067 as ::core::ffi::c_uint,
                            b"int EncodeImageNoHuffman(VP8LBitWriter *const, const uint32_t *const, VP8LHashChain *const, VP8LBackwardRefs *const, int, int, int, int, const WebPPicture *const, int, int *const)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                if GetHuffBitLengthsAndCodes(
                    histogram_image,
                    &raw mut huffman_codes as *mut HuffmanTreeCode,
                ) == 0
                {
                    WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                } else {
                    VP8LPutBits(bw, 0 as uint32_t, 1 as ::core::ffi::c_int);
                    i = 0 as ::core::ffi::c_int;
                    while i < 5 as ::core::ffi::c_int {
                        let codes: *mut HuffmanTreeCode =
                            (&raw mut huffman_codes as *mut HuffmanTreeCode).offset(i as isize)
                                as *mut HuffmanTreeCode;
                        if max_tokens < (*codes).num_symbols {
                            max_tokens = (*codes).num_symbols;
                        }
                        i += 1;
                    }
                    tokens = WebPSafeMalloc(
                        max_tokens as uint64_t,
                        ::core::mem::size_of::<HuffmanTreeToken>() as size_t,
                    ) as *mut HuffmanTreeToken;
                    if tokens.is_null() {
                        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        while i < 5 as ::core::ffi::c_int {
                            let codes_0: *mut HuffmanTreeCode =
                                (&raw mut huffman_codes as *mut HuffmanTreeCode).offset(i as isize)
                                    as *mut HuffmanTreeCode;
                            StoreHuffmanCode(bw, huff_tree, tokens, codes_0);
                            ClearHuffmanTreeIfOnlyOneSymbol(codes_0);
                            i += 1;
                        }
                        StoreImageToBitMask(
                            bw,
                            width,
                            0 as ::core::ffi::c_int,
                            refs,
                            &raw const histogram_symbols as *const uint16_t,
                            &raw mut huffman_codes as *mut HuffmanTreeCode,
                            pic,
                        ) == 0;
                    }
                }
            }
        }
    }
    WebPSafeFree(tokens as *mut ::core::ffi::c_void);
    WebPSafeFree(huff_tree as *mut ::core::ffi::c_void);
    VP8LFreeHistogramSet(histogram_image);
    WebPSafeFree(huffman_codes[0 as ::core::ffi::c_int as usize].codes as *mut ::core::ffi::c_void);
    return ((*pic).error_code as ::core::ffi::c_uint
        == VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn EncodeImageInternal(
    bw: *mut VP8LBitWriter,
    argb: *const uint32_t,
    hash_chain: *mut VP8LHashChain,
    mut refs_array: *mut VP8LBackwardRefs,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_int,
    mut low_effort: ::core::ffi::c_int,
    mut use_cache: ::core::ffi::c_int,
    config: *const CrunchConfig,
    mut cache_bits: *mut ::core::ffi::c_int,
    mut histogram_bits: ::core::ffi::c_int,
    mut init_byte_position: size_t,
    hdr_size: *mut ::core::ffi::c_int,
    data_size: *mut ::core::ffi::c_int,
    pic: *const WebPPicture,
    mut percent_range: ::core::ffi::c_int,
    percent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let histogram_image_xysize: uint32_t =
        (VP8LSubSampleSize(width as uint32_t, histogram_bits as uint32_t) as uint32_t)
            .wrapping_mul(
                VP8LSubSampleSize(height as uint32_t, histogram_bits as uint32_t) as uint32_t,
            );
    let mut remaining_percent: ::core::ffi::c_int = percent_range;
    let mut percent_start: ::core::ffi::c_int = *percent;
    let mut histogram_image: *mut VP8LHistogramSet = ::core::ptr::null_mut::<VP8LHistogramSet>();
    let mut tmp_histo: *mut VP8LHistogram = ::core::ptr::null_mut::<VP8LHistogram>();
    let mut histogram_image_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bit_array_size: size_t = 0 as size_t;
    let huff_tree: *mut HuffmanTree = WebPSafeMalloc(
        (3 as ::core::ffi::c_ulonglong).wrapping_mul(CODE_LENGTH_CODES as ::core::ffi::c_ulonglong)
            as uint64_t,
        ::core::mem::size_of::<HuffmanTree>() as size_t,
    ) as *mut HuffmanTree;
    let mut tokens: *mut HuffmanTreeToken = ::core::ptr::null_mut::<HuffmanTreeToken>();
    let mut huffman_codes: *mut HuffmanTreeCode = ::core::ptr::null_mut::<HuffmanTreeCode>();
    let histogram_symbols: *mut uint16_t = WebPSafeMalloc(
        histogram_image_xysize as uint64_t,
        ::core::mem::size_of::<uint16_t>() as size_t,
    ) as *mut uint16_t;
    let mut sub_configs_idx: ::core::ffi::c_int = 0;
    let mut cache_bits_init: ::core::ffi::c_int = 0;
    let mut write_histogram_image: ::core::ffi::c_int = 0;
    let mut bw_init: VP8LBitWriter = *bw;
    let mut bw_best: VP8LBitWriter = VP8LBitWriter {
        bits_: 0,
        used_: 0,
        buf_: ::core::ptr::null_mut::<uint8_t>(),
        cur_: ::core::ptr::null_mut::<uint8_t>(),
        end_: ::core::ptr::null_mut::<uint8_t>(),
        error_: 0,
    };
    let mut hdr_size_tmp: ::core::ffi::c_int = 0;
    let mut hash_chain_histogram: VP8LHashChain = VP8LHashChain {
        offset_length_: ::core::ptr::null_mut::<uint32_t>(),
        size_: 0,
    };
    let mut bw_size_best: size_t = !(0 as ::core::ffi::c_int as size_t);
    '_c2rust_label: {
        if histogram_bits >= 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"histogram_bits >= MIN_HUFFMAN_BITS\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1140 as ::core::ffi::c_uint,
                b"int EncodeImageInternal(VP8LBitWriter *const, const uint32_t *const, VP8LHashChain *const, VP8LBackwardRefs *, int, int, int, int, int, const CrunchConfig *const, int *, int, size_t, int *const, int *const, const WebPPicture *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if histogram_bits <= 9 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"histogram_bits <= MAX_HUFFMAN_BITS\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1141 as ::core::ffi::c_uint,
                b"int EncodeImageInternal(VP8LBitWriter *const, const uint32_t *const, VP8LHashChain *const, VP8LBackwardRefs *, int, int, int, int, int, const CrunchConfig *const, int *, int, size_t, int *const, int *const, const WebPPicture *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !hdr_size.is_null() {
        } else {
            __assert_fail(
                b"hdr_size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1142 as ::core::ffi::c_uint,
                b"int EncodeImageInternal(VP8LBitWriter *const, const uint32_t *const, VP8LHashChain *const, VP8LBackwardRefs *, int, int, int, int, int, const CrunchConfig *const, int *, int, size_t, int *const, int *const, const WebPPicture *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !data_size.is_null() {
        } else {
            __assert_fail(
                b"data_size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1143 as ::core::ffi::c_uint,
                b"int EncodeImageInternal(VP8LBitWriter *const, const uint32_t *const, VP8LHashChain *const, VP8LBackwardRefs *, int, int, int, int, int, const CrunchConfig *const, int *, int, size_t, int *const, int *const, const WebPPicture *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        &raw mut hash_chain_histogram as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VP8LHashChain>() as size_t,
    );
    if VP8LBitWriterInit(&raw mut bw_best, 0 as size_t) == 0 {
        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else if huff_tree.is_null()
        || histogram_symbols.is_null()
        || VP8LHashChainInit(
            &raw mut hash_chain_histogram,
            histogram_image_xysize as ::core::ffi::c_int,
        ) == 0
    {
        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else {
        percent_range = remaining_percent / 5 as ::core::ffi::c_int;
        if !(VP8LHashChainFill(
            hash_chain,
            quality,
            argb,
            width,
            height,
            low_effort,
            pic,
            percent_range,
            percent,
        ) == 0)
        {
            percent_start += percent_range;
            remaining_percent -= percent_range;
            if use_cache != 0 {
                cache_bits_init = if *cache_bits == 0 as ::core::ffi::c_int {
                    MAX_COLOR_CACHE_BITS
                } else {
                    *cache_bits
                };
            } else {
                cache_bits_init = 0 as ::core::ffi::c_int;
            }
            if ((*config).sub_configs_size_ > 1 as ::core::ffi::c_int
                || (*config).sub_configs_[0 as ::core::ffi::c_int as usize].do_no_cache_ != 0)
                && VP8LBitWriterClone(bw, &raw mut bw_best) == 0
            {
                WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
            } else {
                sub_configs_idx = 0 as ::core::ffi::c_int;
                's_117: loop {
                    if !(sub_configs_idx < (*config).sub_configs_size_) {
                        current_block = 8723848109087415604;
                        break;
                    }
                    let sub_config: *const CrunchSubConfig = (&raw const (*config).sub_configs_
                        as *const CrunchSubConfig)
                        .offset(sub_configs_idx as isize)
                        as *const CrunchSubConfig;
                    let mut cache_bits_best: ::core::ffi::c_int = 0;
                    let mut i_cache: ::core::ffi::c_int = 0;
                    let mut i_remaining_percent: ::core::ffi::c_int =
                        remaining_percent / (*config).sub_configs_size_;
                    let mut i_percent_range: ::core::ffi::c_int =
                        i_remaining_percent / 4 as ::core::ffi::c_int;
                    i_remaining_percent -= i_percent_range;
                    if VP8LGetBackwardReferences(
                        width,
                        height,
                        argb,
                        quality,
                        low_effort,
                        (*sub_config).lz77_,
                        cache_bits_init,
                        (*sub_config).do_no_cache_,
                        hash_chain,
                        refs_array.offset(0 as ::core::ffi::c_int as isize)
                            as *mut VP8LBackwardRefs,
                        &raw mut cache_bits_best,
                        pic,
                        i_percent_range,
                        percent,
                    ) == 0
                    {
                        current_block = 4962161540708780696;
                        break;
                    }
                    i_cache = 0 as ::core::ffi::c_int;
                    while i_cache
                        < (if (*sub_config).do_no_cache_ != 0 {
                            2 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        })
                    {
                        let cache_bits_tmp: ::core::ffi::c_int =
                            if i_cache == 0 as ::core::ffi::c_int {
                                cache_bits_best
                            } else {
                                0 as ::core::ffi::c_int
                            };
                        if i_cache == 1 as ::core::ffi::c_int
                            && cache_bits_best == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        VP8LBitWriterReset(&raw mut bw_init, bw);
                        histogram_image = VP8LAllocateHistogramSet(
                            histogram_image_xysize as ::core::ffi::c_int,
                            cache_bits_tmp,
                        );
                        tmp_histo = VP8LAllocateHistogram(cache_bits_tmp);
                        if histogram_image.is_null() || tmp_histo.is_null() {
                            WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                            current_block = 4962161540708780696;
                            break 's_117;
                        } else {
                            i_percent_range = i_remaining_percent / 3 as ::core::ffi::c_int;
                            i_remaining_percent -= i_percent_range;
                            if VP8LGetHistoImageSymbols(
                                width,
                                height,
                                refs_array.offset(i_cache as isize) as *mut VP8LBackwardRefs,
                                quality,
                                low_effort,
                                histogram_bits,
                                cache_bits_tmp,
                                histogram_image,
                                tmp_histo,
                                histogram_symbols,
                                pic,
                                i_percent_range,
                                percent,
                            ) == 0
                            {
                                current_block = 4962161540708780696;
                                break 's_117;
                            }
                            histogram_image_size = (*histogram_image).size;
                            bit_array_size =
                                (5 as ::core::ffi::c_int * histogram_image_size) as size_t;
                            huffman_codes = WebPSafeCalloc(
                                bit_array_size as uint64_t,
                                ::core::mem::size_of::<HuffmanTreeCode>() as size_t,
                            ) as *mut HuffmanTreeCode;
                            if huffman_codes.is_null()
                                || GetHuffBitLengthsAndCodes(histogram_image, huffman_codes) == 0
                            {
                                WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                                current_block = 4962161540708780696;
                                break 's_117;
                            } else {
                                VP8LFreeHistogramSet(histogram_image);
                                histogram_image = ::core::ptr::null_mut::<VP8LHistogramSet>();
                                VP8LFreeHistogram(tmp_histo);
                                tmp_histo = ::core::ptr::null_mut::<VP8LHistogram>();
                                if cache_bits_tmp > 0 as ::core::ffi::c_int {
                                    VP8LPutBits(bw, 1 as uint32_t, 1 as ::core::ffi::c_int);
                                    VP8LPutBits(
                                        bw,
                                        cache_bits_tmp as uint32_t,
                                        4 as ::core::ffi::c_int,
                                    );
                                } else {
                                    VP8LPutBits(bw, 0 as uint32_t, 1 as ::core::ffi::c_int);
                                }
                                write_histogram_image = (histogram_image_size
                                    > 1 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                                VP8LPutBits(
                                    bw,
                                    write_histogram_image as uint32_t,
                                    1 as ::core::ffi::c_int,
                                );
                                if write_histogram_image != 0 {
                                    let histogram_argb: *mut uint32_t = WebPSafeMalloc(
                                        histogram_image_xysize as uint64_t,
                                        ::core::mem::size_of::<uint32_t>() as size_t,
                                    )
                                        as *mut uint32_t;
                                    let mut max_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    let mut i: uint32_t = 0;
                                    if histogram_argb.is_null() {
                                        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                                        current_block = 4962161540708780696;
                                        break 's_117;
                                    } else {
                                        i = 0 as uint32_t;
                                        while i < histogram_image_xysize {
                                            let symbol_index: ::core::ffi::c_int =
                                                *histogram_symbols.offset(i as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xffff as ::core::ffi::c_int;
                                            *histogram_argb.offset(i as isize) = (symbol_index
                                                << 8 as ::core::ffi::c_int)
                                                as uint32_t;
                                            if symbol_index >= max_index {
                                                max_index = symbol_index + 1 as ::core::ffi::c_int;
                                            }
                                            i = i.wrapping_add(1);
                                        }
                                        histogram_image_size = max_index;
                                        VP8LPutBits(
                                            bw,
                                            (histogram_bits - 2 as ::core::ffi::c_int) as uint32_t,
                                            3 as ::core::ffi::c_int,
                                        );
                                        i_percent_range =
                                            i_remaining_percent / 2 as ::core::ffi::c_int;
                                        i_remaining_percent -= i_percent_range;
                                        if EncodeImageNoHuffman(
                                            bw,
                                            histogram_argb,
                                            &raw mut hash_chain_histogram,
                                            refs_array.offset(2 as ::core::ffi::c_int as isize)
                                                as *mut VP8LBackwardRefs,
                                            VP8LSubSampleSize(
                                                width as uint32_t,
                                                histogram_bits as uint32_t,
                                            )
                                                as ::core::ffi::c_int,
                                            VP8LSubSampleSize(
                                                height as uint32_t,
                                                histogram_bits as uint32_t,
                                            )
                                                as ::core::ffi::c_int,
                                            quality,
                                            low_effort,
                                            pic,
                                            i_percent_range,
                                            percent,
                                        ) == 0
                                        {
                                            WebPSafeFree(
                                                histogram_argb as *mut ::core::ffi::c_void,
                                            );
                                            current_block = 4962161540708780696;
                                            break 's_117;
                                        } else {
                                            WebPSafeFree(
                                                histogram_argb as *mut ::core::ffi::c_void,
                                            );
                                        }
                                    }
                                }
                                let mut i_0: ::core::ffi::c_int = 0;
                                let mut max_tokens: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                i_0 = 0 as ::core::ffi::c_int;
                                while i_0 < 5 as ::core::ffi::c_int * histogram_image_size {
                                    let codes: *mut HuffmanTreeCode =
                                        huffman_codes.offset(i_0 as isize) as *mut HuffmanTreeCode;
                                    if max_tokens < (*codes).num_symbols {
                                        max_tokens = (*codes).num_symbols;
                                    }
                                    i_0 += 1;
                                }
                                tokens = WebPSafeMalloc(
                                    max_tokens as uint64_t,
                                    ::core::mem::size_of::<HuffmanTreeToken>() as size_t,
                                ) as *mut HuffmanTreeToken;
                                if tokens.is_null() {
                                    WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                                    current_block = 4962161540708780696;
                                    break 's_117;
                                } else {
                                    i_0 = 0 as ::core::ffi::c_int;
                                    while i_0 < 5 as ::core::ffi::c_int * histogram_image_size {
                                        let codes_0: *mut HuffmanTreeCode = huffman_codes
                                            .offset(i_0 as isize)
                                            as *mut HuffmanTreeCode;
                                        StoreHuffmanCode(bw, huff_tree, tokens, codes_0);
                                        ClearHuffmanTreeIfOnlyOneSymbol(codes_0);
                                        i_0 += 1;
                                    }
                                    hdr_size_tmp = VP8LBitWriterNumBytes(bw)
                                        .wrapping_sub(init_byte_position)
                                        as ::core::ffi::c_int;
                                    if StoreImageToBitMask(
                                        bw,
                                        width,
                                        histogram_bits,
                                        refs_array.offset(i_cache as isize)
                                            as *mut VP8LBackwardRefs,
                                        histogram_symbols,
                                        huffman_codes,
                                        pic,
                                    ) == 0
                                    {
                                        current_block = 4962161540708780696;
                                        break 's_117;
                                    }
                                    if VP8LBitWriterNumBytes(bw) < bw_size_best {
                                        bw_size_best = VP8LBitWriterNumBytes(bw);
                                        *cache_bits = cache_bits_tmp;
                                        *hdr_size = hdr_size_tmp;
                                        *data_size = VP8LBitWriterNumBytes(bw)
                                            .wrapping_sub(init_byte_position)
                                            .wrapping_sub(*hdr_size as size_t)
                                            as ::core::ffi::c_int;
                                        VP8LBitWriterSwap(bw, &raw mut bw_best);
                                    }
                                    WebPSafeFree(tokens as *mut ::core::ffi::c_void);
                                    tokens = ::core::ptr::null_mut::<HuffmanTreeToken>();
                                    if !huffman_codes.is_null() {
                                        WebPSafeFree(
                                            (*huffman_codes).codes as *mut ::core::ffi::c_void,
                                        );
                                        WebPSafeFree(huffman_codes as *mut ::core::ffi::c_void);
                                        huffman_codes = ::core::ptr::null_mut::<HuffmanTreeCode>();
                                    }
                                    i_cache += 1;
                                }
                            }
                        }
                    }
                    sub_configs_idx += 1;
                }
                match current_block {
                    4962161540708780696 => {}
                    _ => {
                        VP8LBitWriterSwap(bw, &raw mut bw_best);
                        WebPReportProgress(pic, percent_start + remaining_percent, percent) == 0;
                    }
                }
            }
        }
    }
    WebPSafeFree(tokens as *mut ::core::ffi::c_void);
    WebPSafeFree(huff_tree as *mut ::core::ffi::c_void);
    VP8LFreeHistogramSet(histogram_image);
    VP8LFreeHistogram(tmp_histo);
    VP8LHashChainClear(&raw mut hash_chain_histogram);
    if !huffman_codes.is_null() {
        WebPSafeFree((*huffman_codes).codes as *mut ::core::ffi::c_void);
        WebPSafeFree(huffman_codes as *mut ::core::ffi::c_void);
    }
    WebPSafeFree(histogram_symbols as *mut ::core::ffi::c_void);
    VP8LBitWriterWipeOut(&raw mut bw_best);
    return ((*pic).error_code as ::core::ffi::c_uint
        == VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn ApplySubtractGreen(
    enc: *mut VP8LEncoder,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    bw: *mut VP8LBitWriter,
) {
    VP8LPutBits(bw, TRANSFORM_PRESENT as uint32_t, 1 as ::core::ffi::c_int);
    VP8LPutBits(
        bw,
        SUBTRACT_GREEN_TRANSFORM as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int,
    );
    VP8LSubtractGreenFromBlueAndRed.expect("non-null function pointer")(
        (*enc).argb_,
        width * height,
    );
}
unsafe extern "C" fn ApplyPredictFilter(
    enc: *const VP8LEncoder,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_int,
    mut low_effort: ::core::ffi::c_int,
    mut used_subtract_green: ::core::ffi::c_int,
    bw: *mut VP8LBitWriter,
    mut percent_range: ::core::ffi::c_int,
    percent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pred_bits: ::core::ffi::c_int = (*enc).transform_bits_;
    let transform_width: ::core::ffi::c_int =
        VP8LSubSampleSize(width as uint32_t, pred_bits as uint32_t) as ::core::ffi::c_int;
    let transform_height: ::core::ffi::c_int =
        VP8LSubSampleSize(height as uint32_t, pred_bits as uint32_t) as ::core::ffi::c_int;
    let near_lossless_strength: ::core::ffi::c_int = if (*enc).use_palette_ != 0 {
        100 as ::core::ffi::c_int
    } else {
        (*(*enc).config_).near_lossless
    };
    if VP8LResidualImage(
        width,
        height,
        pred_bits,
        low_effort,
        (*enc).argb_,
        (*enc).argb_scratch_,
        (*enc).transform_data_,
        near_lossless_strength,
        (*(*enc).config_).exact,
        used_subtract_green,
        (*enc).pic_,
        percent_range / 2 as ::core::ffi::c_int,
        percent,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    VP8LPutBits(bw, TRANSFORM_PRESENT as uint32_t, 1 as ::core::ffi::c_int);
    VP8LPutBits(
        bw,
        PREDICTOR_TRANSFORM as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int,
    );
    '_c2rust_label: {
        if pred_bits >= 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"pred_bits >= 2\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1382 as ::core::ffi::c_uint,
                b"int ApplyPredictFilter(const VP8LEncoder *const, int, int, int, int, int, VP8LBitWriter *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    VP8LPutBits(
        bw,
        (pred_bits - 2 as ::core::ffi::c_int) as uint32_t,
        3 as ::core::ffi::c_int,
    );
    return EncodeImageNoHuffman(
        bw,
        (*enc).transform_data_,
        &raw const (*enc).hash_chain_ as *mut VP8LHashChain,
        (&raw const (*enc).refs_ as *const VP8LBackwardRefs)
            .offset(0 as ::core::ffi::c_int as isize) as *const VP8LBackwardRefs
            as *mut VP8LBackwardRefs,
        transform_width,
        transform_height,
        quality,
        low_effort,
        (*enc).pic_,
        percent_range - percent_range / 2 as ::core::ffi::c_int,
        percent,
    );
}
unsafe extern "C" fn ApplyCrossColorFilter(
    enc: *const VP8LEncoder,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_int,
    mut low_effort: ::core::ffi::c_int,
    bw: *mut VP8LBitWriter,
    mut percent_range: ::core::ffi::c_int,
    percent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let ccolor_transform_bits: ::core::ffi::c_int = (*enc).transform_bits_;
    let transform_width: ::core::ffi::c_int =
        VP8LSubSampleSize(width as uint32_t, ccolor_transform_bits as uint32_t)
            as ::core::ffi::c_int;
    let transform_height: ::core::ffi::c_int =
        VP8LSubSampleSize(height as uint32_t, ccolor_transform_bits as uint32_t)
            as ::core::ffi::c_int;
    if VP8LColorSpaceTransform(
        width,
        height,
        ccolor_transform_bits,
        quality,
        (*enc).argb_,
        (*enc).transform_data_,
        (*enc).pic_,
        percent_range / 2 as ::core::ffi::c_int,
        percent,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    VP8LPutBits(bw, TRANSFORM_PRESENT as uint32_t, 1 as ::core::ffi::c_int);
    VP8LPutBits(
        bw,
        CROSS_COLOR_TRANSFORM as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int,
    );
    '_c2rust_label: {
        if ccolor_transform_bits >= 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"ccolor_transform_bits >= 2\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1406 as ::core::ffi::c_uint,
                b"int ApplyCrossColorFilter(const VP8LEncoder *const, int, int, int, int, VP8LBitWriter *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    VP8LPutBits(
        bw,
        (ccolor_transform_bits - 2 as ::core::ffi::c_int) as uint32_t,
        3 as ::core::ffi::c_int,
    );
    return EncodeImageNoHuffman(
        bw,
        (*enc).transform_data_,
        &raw const (*enc).hash_chain_ as *mut VP8LHashChain,
        (&raw const (*enc).refs_ as *const VP8LBackwardRefs)
            .offset(0 as ::core::ffi::c_int as isize) as *const VP8LBackwardRefs
            as *mut VP8LBackwardRefs,
        transform_width,
        transform_height,
        quality,
        low_effort,
        (*enc).pic_,
        percent_range - percent_range / 2 as ::core::ffi::c_int,
        percent,
    );
}
unsafe extern "C" fn WriteRiffHeader(
    pic: *const WebPPicture,
    mut riff_size: size_t,
    mut vp8l_size: size_t,
) -> ::core::ffi::c_int {
    let mut riff: [uint8_t; 21] = [
        'R' as i32 as uint8_t,
        'I' as i32 as uint8_t,
        'F' as i32 as uint8_t,
        'F' as i32 as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        'W' as i32 as uint8_t,
        'E' as i32 as uint8_t,
        'B' as i32 as uint8_t,
        'P' as i32 as uint8_t,
        'V' as i32 as uint8_t,
        'P' as i32 as uint8_t,
        '8' as i32 as uint8_t,
        'L' as i32 as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        VP8L_MAGIC_BYTE as uint8_t,
    ];
    PutLE32(
        (&raw mut riff as *mut uint8_t).offset(TAG_SIZE as isize),
        riff_size as uint32_t,
    );
    PutLE32(
        (&raw mut riff as *mut uint8_t)
            .offset(RIFF_HEADER_SIZE as isize)
            .offset(TAG_SIZE as isize),
        vp8l_size as uint32_t,
    );
    return (*pic).writer.expect("non-null function pointer")(
        &raw mut riff as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 21]>() as size_t,
        pic,
    );
}
unsafe extern "C" fn WriteImageSize(
    pic: *const WebPPicture,
    bw: *mut VP8LBitWriter,
) -> ::core::ffi::c_int {
    let width: ::core::ffi::c_int = (*pic).width - 1 as ::core::ffi::c_int;
    let height: ::core::ffi::c_int = (*pic).height - 1 as ::core::ffi::c_int;
    '_c2rust_label: {
        if width < 16383 as ::core::ffi::c_int && height < 16383 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"width < WEBP_MAX_DIMENSION && height < WEBP_MAX_DIMENSION\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1432 as ::core::ffi::c_uint,
                b"int WriteImageSize(const WebPPicture *const, VP8LBitWriter *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    VP8LPutBits(bw, width as uint32_t, VP8L_IMAGE_SIZE_BITS);
    VP8LPutBits(bw, height as uint32_t, VP8L_IMAGE_SIZE_BITS);
    return ((*bw).error_ == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn WriteRealAlphaAndVersion(
    bw: *mut VP8LBitWriter,
    mut has_alpha: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    VP8LPutBits(bw, has_alpha as uint32_t, 1 as ::core::ffi::c_int);
    VP8LPutBits(bw, VP8L_VERSION as uint32_t, VP8L_VERSION_BITS);
    return ((*bw).error_ == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn WriteImage(
    pic: *const WebPPicture,
    bw: *mut VP8LBitWriter,
    coded_size: *mut size_t,
) -> ::core::ffi::c_int {
    let webpll_data: *const uint8_t = VP8LBitWriterFinish(bw);
    let webpll_size: size_t = VP8LBitWriterNumBytes(bw) as size_t;
    let vp8l_size: size_t = (VP8L_SIGNATURE_SIZE as size_t).wrapping_add(webpll_size);
    let pad: size_t = vp8l_size & 1 as size_t;
    let riff_size: size_t = ((TAG_SIZE + CHUNK_HEADER_SIZE) as size_t)
        .wrapping_add(vp8l_size)
        .wrapping_add(pad);
    *coded_size = 0 as size_t;
    if (*bw).error_ != 0 {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    if WriteRiffHeader(pic, riff_size, vp8l_size) == 0
        || (*pic).writer.expect("non-null function pointer")(webpll_data, webpll_size, pic) == 0
    {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_BAD_WRITE);
    }
    if pad != 0 {
        let pad_byte: [uint8_t; 1] = [0 as ::core::ffi::c_int as uint8_t];
        if (*pic).writer.expect("non-null function pointer")(
            &raw const pad_byte as *const uint8_t,
            1 as size_t,
            pic,
        ) == 0
        {
            return WebPEncodingSetError(pic, VP8_ENC_ERROR_BAD_WRITE);
        }
    }
    *coded_size = (CHUNK_HEADER_SIZE as size_t).wrapping_add(riff_size);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn ClearTransformBuffer(enc: *mut VP8LEncoder) {
    WebPSafeFree((*enc).transform_mem_ as *mut ::core::ffi::c_void);
    (*enc).transform_mem_ = ::core::ptr::null_mut::<uint32_t>();
    (*enc).transform_mem_size_ = 0 as size_t;
}
unsafe extern "C" fn AllocateTransformBuffer(
    enc: *mut VP8LEncoder,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let image_size: uint64_t = (width as uint64_t).wrapping_mul(height as uint64_t);
    let argb_scratch_size: uint64_t = if (*enc).use_predict_ != 0 {
        (((width + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int) as uint64_t).wrapping_add(
            ((width * 2 as ::core::ffi::c_int) as uint64_t)
                .wrapping_add(::core::mem::size_of::<uint32_t>() as uint64_t)
                .wrapping_sub(1 as uint64_t)
                .wrapping_div(::core::mem::size_of::<uint32_t>() as uint64_t),
        )
    } else {
        0 as uint64_t
    };
    let transform_data_size: uint64_t = if (*enc).use_predict_ != 0 || (*enc).use_cross_color_ != 0
    {
        (VP8LSubSampleSize(width as uint32_t, (*enc).transform_bits_ as uint32_t) as uint64_t)
            .wrapping_mul(
                VP8LSubSampleSize(height as uint32_t, (*enc).transform_bits_ as uint32_t)
                    as uint64_t,
            )
    } else {
        0 as uint64_t
    };
    let max_alignment_in_words: uint64_t = (WEBP_ALIGN_CST as uint64_t)
        .wrapping_add(::core::mem::size_of::<uint32_t>() as uint64_t)
        .wrapping_sub(1 as uint64_t)
        .wrapping_div(::core::mem::size_of::<uint32_t>() as uint64_t);
    let mem_size: uint64_t = image_size
        .wrapping_add(max_alignment_in_words)
        .wrapping_add(argb_scratch_size)
        .wrapping_add(max_alignment_in_words)
        .wrapping_add(transform_data_size);
    let mut mem: *mut uint32_t = (*enc).transform_mem_;
    if mem.is_null() || mem_size > (*enc).transform_mem_size_ as uint64_t {
        ClearTransformBuffer(enc);
        mem =
            WebPSafeMalloc(mem_size, ::core::mem::size_of::<uint32_t>() as size_t) as *mut uint32_t;
        if mem.is_null() {
            return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        (*enc).transform_mem_ = mem;
        (*enc).transform_mem_size_ = mem_size as size_t;
        (*enc).argb_content_ = kEncoderNone;
    }
    (*enc).argb_ = mem;
    mem = ((mem.offset(image_size as isize) as uintptr_t).wrapping_add(WEBP_ALIGN_CST as uintptr_t)
        & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint32_t;
    (*enc).argb_scratch_ = mem;
    mem = ((mem.offset(argb_scratch_size as isize) as uintptr_t)
        .wrapping_add(WEBP_ALIGN_CST as uintptr_t)
        & !(WEBP_ALIGN_CST as uintptr_t)) as *mut uint32_t;
    (*enc).transform_data_ = mem;
    (*enc).current_width_ = width;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn MakeInputImageCopy(enc: *mut VP8LEncoder) -> ::core::ffi::c_int {
    let picture: *const WebPPicture = (*enc).pic_;
    let width: ::core::ffi::c_int = (*picture).width;
    let height: ::core::ffi::c_int = (*picture).height;
    if AllocateTransformBuffer(enc, width, height) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*enc).argb_content_ as ::core::ffi::c_uint
        == kEncoderARGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 1 as ::core::ffi::c_int;
    }
    let mut dst: *mut uint32_t = (*enc).argb_;
    let mut src: *const uint32_t = (*picture).argb;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (width as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        );
        dst = dst.offset(width as isize);
        src = src.offset((*picture).argb_stride as isize);
        y += 1;
    }
    (*enc).argb_content_ = kEncoderARGB;
    '_c2rust_label: {
        if (*enc).current_width_ == width {
        } else {
            __assert_fail(
                b"enc->current_width_ == width\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1546 as ::core::ffi::c_uint,
                b"int MakeInputImageCopy(VP8LEncoder *const)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return 1 as ::core::ffi::c_int;
}
pub const APPLY_PALETTE_GREEDY_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn SearchColorGreedy(
    mut palette: *const uint32_t,
    mut palette_size: ::core::ffi::c_int,
    mut color: uint32_t,
) -> uint32_t {
    '_c2rust_label: {
        if palette_size < 4 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"palette_size < APPLY_PALETTE_GREEDY_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1558 as ::core::ffi::c_uint,
                b"uint32_t SearchColorGreedy(const uint32_t *, int, uint32_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"3 == APPLY_PALETTE_GREEDY_MAX - 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1559 as ::core::ffi::c_uint,
                b"uint32_t SearchColorGreedy(const uint32_t *, int, uint32_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if color == *palette.offset(0 as ::core::ffi::c_int as isize) {
        return 0 as uint32_t;
    }
    if color == *palette.offset(1 as ::core::ffi::c_int as isize) {
        return 1 as uint32_t;
    }
    if color == *palette.offset(2 as ::core::ffi::c_int as isize) {
        return 2 as uint32_t;
    }
    return 3 as uint32_t;
}
#[inline]
unsafe extern "C" fn ApplyPaletteHash0(mut color: uint32_t) -> uint32_t {
    return color >> 8 as ::core::ffi::c_int & 0xff as uint32_t;
}
pub const PALETTE_INV_SIZE_BITS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn ApplyPaletteHash1(mut color: uint32_t) -> uint32_t {
    return ((color & 0xffffff as uint32_t) as ::core::ffi::c_ulonglong)
        .wrapping_mul(4222244071 as ::core::ffi::c_ulonglong) as uint32_t
        >> 32 as ::core::ffi::c_int - PALETTE_INV_SIZE_BITS;
}
#[inline]
unsafe extern "C" fn ApplyPaletteHash2(mut color: uint32_t) -> uint32_t {
    return ((color & 0xffffff as uint32_t) as ::core::ffi::c_ulonglong).wrapping_mul(
        ((1 as ::core::ffi::c_ulonglong) << 31 as ::core::ffi::c_int)
            .wrapping_sub(1 as ::core::ffi::c_ulonglong),
    ) as uint32_t
        >> 32 as ::core::ffi::c_int - PALETTE_INV_SIZE_BITS;
}
unsafe extern "C" fn ApplyPalette(
    mut src: *const uint32_t,
    mut src_stride: uint32_t,
    mut dst: *mut uint32_t,
    mut dst_stride: uint32_t,
    mut palette: *const uint32_t,
    mut palette_size: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut xbits: ::core::ffi::c_int,
    pic: *const WebPPicture,
) -> ::core::ffi::c_int {
    let tmp_row: *mut uint8_t = WebPSafeMalloc(
        width as uint64_t,
        ::core::mem::size_of::<uint8_t>() as size_t,
    ) as *mut uint8_t;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    if tmp_row.is_null() {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    if palette_size < APPLY_PALETTE_GREEDY_MAX {
        let mut prev_pix: uint32_t = *palette.offset(0 as ::core::ffi::c_int as isize);
        let mut prev_idx: uint32_t = 0 as uint32_t;
        y = 0 as ::core::ffi::c_int;
        while y < height {
            x = 0 as ::core::ffi::c_int;
            while x < width {
                let pix: uint32_t = *src.offset(x as isize);
                if pix != prev_pix {
                    prev_idx = SearchColorGreedy(palette as *const uint32_t, palette_size, pix);
                    prev_pix = pix;
                }
                *tmp_row.offset(x as isize) = prev_idx as uint8_t;
                x += 1;
            }
            VP8LBundleColorMap.expect("non-null function pointer")(tmp_row, width, xbits, dst);
            src = src.offset(src_stride as isize);
            dst = dst.offset(dst_stride as isize);
            y += 1;
        }
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut buffer: [uint16_t; 2048] = [0; 2048];
        let hash_functions: [Option<unsafe extern "C" fn(uint32_t) -> uint32_t>; 3] = [
            Some(ApplyPaletteHash0 as unsafe extern "C" fn(uint32_t) -> uint32_t),
            Some(ApplyPaletteHash1 as unsafe extern "C" fn(uint32_t) -> uint32_t),
            Some(ApplyPaletteHash2 as unsafe extern "C" fn(uint32_t) -> uint32_t),
        ];
        i = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            let mut use_LUT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            memset(
                &raw mut buffer as *mut uint16_t as *mut ::core::ffi::c_void,
                0xff as ::core::ffi::c_int,
                ::core::mem::size_of::<[uint16_t; 2048]>() as size_t,
            );
            j = 0 as ::core::ffi::c_int;
            while j < palette_size {
                let ind: uint32_t = hash_functions[i as usize].expect("non-null function pointer")(
                    *palette.offset(j as isize),
                ) as uint32_t;
                if buffer[ind as usize] as ::core::ffi::c_uint != 0xffff as ::core::ffi::c_uint {
                    use_LUT = 0 as ::core::ffi::c_int;
                    break;
                } else {
                    buffer[ind as usize] = j as uint16_t;
                    j += 1;
                }
            }
            if use_LUT != 0 {
                break;
            }
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            let mut prev_pix_0: uint32_t = *palette.offset(0 as ::core::ffi::c_int as isize);
            let mut prev_idx_0: uint32_t = 0 as uint32_t;
            y = 0 as ::core::ffi::c_int;
            while y < height {
                x = 0 as ::core::ffi::c_int;
                while x < width {
                    let pix_0: uint32_t = *src.offset(x as isize);
                    if pix_0 != prev_pix_0 {
                        prev_idx_0 = buffer[ApplyPaletteHash0(pix_0) as usize] as uint32_t;
                        prev_pix_0 = pix_0;
                    }
                    *tmp_row.offset(x as isize) = prev_idx_0 as uint8_t;
                    x += 1;
                }
                VP8LBundleColorMap.expect("non-null function pointer")(tmp_row, width, xbits, dst);
                src = src.offset(src_stride as isize);
                dst = dst.offset(dst_stride as isize);
                y += 1;
            }
        } else if i == 1 as ::core::ffi::c_int {
            let mut prev_pix_1: uint32_t = *palette.offset(0 as ::core::ffi::c_int as isize);
            let mut prev_idx_1: uint32_t = 0 as uint32_t;
            y = 0 as ::core::ffi::c_int;
            while y < height {
                x = 0 as ::core::ffi::c_int;
                while x < width {
                    let pix_1: uint32_t = *src.offset(x as isize);
                    if pix_1 != prev_pix_1 {
                        prev_idx_1 = buffer[ApplyPaletteHash1(pix_1) as usize] as uint32_t;
                        prev_pix_1 = pix_1;
                    }
                    *tmp_row.offset(x as isize) = prev_idx_1 as uint8_t;
                    x += 1;
                }
                VP8LBundleColorMap.expect("non-null function pointer")(tmp_row, width, xbits, dst);
                src = src.offset(src_stride as isize);
                dst = dst.offset(dst_stride as isize);
                y += 1;
            }
        } else if i == 2 as ::core::ffi::c_int {
            let mut prev_pix_2: uint32_t = *palette.offset(0 as ::core::ffi::c_int as isize);
            let mut prev_idx_2: uint32_t = 0 as uint32_t;
            y = 0 as ::core::ffi::c_int;
            while y < height {
                x = 0 as ::core::ffi::c_int;
                while x < width {
                    let pix_2: uint32_t = *src.offset(x as isize);
                    if pix_2 != prev_pix_2 {
                        prev_idx_2 = buffer[ApplyPaletteHash2(pix_2) as usize] as uint32_t;
                        prev_pix_2 = pix_2;
                    }
                    *tmp_row.offset(x as isize) = prev_idx_2 as uint8_t;
                    x += 1;
                }
                VP8LBundleColorMap.expect("non-null function pointer")(tmp_row, width, xbits, dst);
                src = src.offset(src_stride as isize);
                dst = dst.offset(dst_stride as isize);
                y += 1;
            }
        } else {
            let mut idx_map: [uint32_t; 256] = [0; 256];
            let mut palette_sorted: [uint32_t; 256] = [0; 256];
            PrepareMapToPalette(
                palette as *const uint32_t,
                palette_size as uint32_t,
                &raw mut palette_sorted as *mut uint32_t,
                &raw mut idx_map as *mut uint32_t,
            );
            let mut prev_pix_3: uint32_t = *palette.offset(0 as ::core::ffi::c_int as isize);
            let mut prev_idx_3: uint32_t = 0 as uint32_t;
            y = 0 as ::core::ffi::c_int;
            while y < height {
                x = 0 as ::core::ffi::c_int;
                while x < width {
                    let pix_3: uint32_t = *src.offset(x as isize);
                    if pix_3 != prev_pix_3 {
                        prev_idx_3 = idx_map[SearchColorNoIdx(
                            &raw mut palette_sorted as *mut uint32_t as *const uint32_t,
                            pix_3,
                            palette_size,
                        ) as usize];
                        prev_pix_3 = pix_3;
                    }
                    *tmp_row.offset(x as isize) = prev_idx_3 as uint8_t;
                    x += 1;
                }
                VP8LBundleColorMap.expect("non-null function pointer")(tmp_row, width, xbits, dst);
                src = src.offset(src_stride as isize);
                dst = dst.offset(dst_stride as isize);
                y += 1;
            }
        }
    }
    WebPSafeFree(tmp_row as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn MapImageFromPalette(
    enc: *mut VP8LEncoder,
    mut in_place: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pic: *const WebPPicture = (*enc).pic_;
    let width: ::core::ffi::c_int = (*pic).width;
    let height: ::core::ffi::c_int = (*pic).height;
    let palette: *const uint32_t = &raw mut (*enc).palette_ as *mut uint32_t;
    let mut src: *const uint32_t = if in_place != 0 {
        (*enc).argb_
    } else {
        (*pic).argb
    };
    let src_stride: ::core::ffi::c_int = if in_place != 0 {
        (*enc).current_width_
    } else {
        (*pic).argb_stride
    };
    let palette_size: ::core::ffi::c_int = (*enc).palette_size_;
    let mut xbits: ::core::ffi::c_int = 0;
    if palette_size <= 4 as ::core::ffi::c_int {
        xbits = if palette_size <= 2 as ::core::ffi::c_int {
            3 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        };
    } else {
        xbits = if palette_size <= 16 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if AllocateTransformBuffer(
        enc,
        VP8LSubSampleSize(width as uint32_t, xbits as uint32_t) as ::core::ffi::c_int,
        height,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if ApplyPalette(
        src,
        src_stride as uint32_t,
        (*enc).argb_,
        (*enc).current_width_ as uint32_t,
        palette,
        palette_size,
        width,
        height,
        xbits,
        pic,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    (*enc).argb_content_ = kEncoderPalette;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn EncodePalette(
    bw: *mut VP8LBitWriter,
    mut low_effort: ::core::ffi::c_int,
    enc: *mut VP8LEncoder,
    mut percent_range: ::core::ffi::c_int,
    percent: *mut ::core::ffi::c_int,
) -> WebPEncodingError {
    let mut i: ::core::ffi::c_int = 0;
    let mut tmp_palette: [uint32_t; 256] = [0; 256];
    let palette_size: ::core::ffi::c_int = (*enc).palette_size_;
    let palette: *const uint32_t = &raw mut (*enc).palette_ as *mut uint32_t;
    VP8LPutBits(bw, TRANSFORM_PRESENT as uint32_t, 1 as ::core::ffi::c_int);
    VP8LPutBits(
        bw,
        COLOR_INDEXING_TRANSFORM as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int,
    );
    '_c2rust_label: {
        if palette_size >= 1 as ::core::ffi::c_int && palette_size <= 256 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"palette_size >= 1 && palette_size <= MAX_PALETTE_SIZE\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1713 as ::core::ffi::c_uint,
                b"WebPEncodingError EncodePalette(VP8LBitWriter *const, int, VP8LEncoder *const, int, int *const)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    VP8LPutBits(
        bw,
        (palette_size - 1 as ::core::ffi::c_int) as uint32_t,
        8 as ::core::ffi::c_int,
    );
    i = palette_size - 1 as ::core::ffi::c_int;
    while i >= 1 as ::core::ffi::c_int {
        tmp_palette[i as usize] = VP8LSubPixels(
            *palette.offset(i as isize),
            *palette.offset((i - 1 as ::core::ffi::c_int) as isize),
        );
        i -= 1;
    }
    tmp_palette[0 as ::core::ffi::c_int as usize] =
        *palette.offset(0 as ::core::ffi::c_int as isize);
    return EncodeImageNoHuffman(
        bw,
        &raw mut tmp_palette as *mut uint32_t,
        &raw mut (*enc).hash_chain_,
        (&raw mut (*enc).refs_ as *mut VP8LBackwardRefs).offset(0 as ::core::ffi::c_int as isize)
            as *mut VP8LBackwardRefs,
        palette_size,
        1 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        low_effort,
        (*enc).pic_,
        percent_range,
        percent,
    ) as WebPEncodingError;
}
unsafe extern "C" fn VP8LEncoderNew(
    config: *const WebPConfig,
    picture: *const WebPPicture,
) -> *mut VP8LEncoder {
    let enc: *mut VP8LEncoder = WebPSafeCalloc(
        1 as uint64_t,
        ::core::mem::size_of::<VP8LEncoder>() as size_t,
    ) as *mut VP8LEncoder;
    if enc.is_null() {
        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
        return ::core::ptr::null_mut::<VP8LEncoder>();
    }
    (*enc).config_ = config;
    (*enc).pic_ = picture;
    (*enc).argb_content_ = kEncoderNone;
    VP8LEncDspInit();
    return enc;
}
unsafe extern "C" fn VP8LEncoderDelete(mut enc: *mut VP8LEncoder) {
    if !enc.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        VP8LHashChainClear(&raw mut (*enc).hash_chain_);
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            VP8LBackwardRefsClear(
                (&raw mut (*enc).refs_ as *mut VP8LBackwardRefs).offset(i as isize)
                    as *mut VP8LBackwardRefs,
            );
            i += 1;
        }
        ClearTransformBuffer(enc);
        WebPSafeFree(enc as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn EncodeStreamHook(
    mut input: *mut ::core::ffi::c_void,
    mut data2: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let params: *mut StreamEncodeContext = input as *mut StreamEncodeContext;
    let config: *const WebPConfig = (*params).config_;
    let picture: *const WebPPicture = (*params).picture_;
    let bw: *mut VP8LBitWriter = (*params).bw_;
    let enc: *mut VP8LEncoder = (*params).enc_;
    let use_cache: ::core::ffi::c_int = (*params).use_cache_;
    let crunch_configs: *const CrunchConfig =
        &raw mut (*params).crunch_configs_ as *mut CrunchConfig;
    let num_crunch_configs: ::core::ffi::c_int = (*params).num_crunch_configs_;
    let red_and_blue_always_zero: ::core::ffi::c_int = (*params).red_and_blue_always_zero_;
    let stats: *mut WebPAuxStats = (*params).stats_;
    let quality: ::core::ffi::c_int = (*config).quality as ::core::ffi::c_int;
    let low_effort: ::core::ffi::c_int =
        ((*config).method == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let width: ::core::ffi::c_int = (*picture).width;
    let height: ::core::ffi::c_int = (*picture).height;
    let byte_position: size_t = VP8LBitWriterNumBytes(bw) as size_t;
    let mut percent: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let mut use_near_lossless: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut hdr_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut data_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut use_delta_palette: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idx: ::core::ffi::c_int = 0;
    let mut best_size: size_t = !(0 as ::core::ffi::c_int as size_t);
    let mut bw_init: VP8LBitWriter = *bw;
    let mut bw_best: VP8LBitWriter = VP8LBitWriter {
        bits_: 0,
        used_: 0,
        buf_: ::core::ptr::null_mut::<uint8_t>(),
        cur_: ::core::ptr::null_mut::<uint8_t>(),
        end_: ::core::ptr::null_mut::<uint8_t>(),
        error_: 0,
    };
    if VP8LBitWriterInit(&raw mut bw_best, 0 as size_t) == 0
        || num_crunch_configs > 1 as ::core::ffi::c_int
            && VP8LBitWriterClone(bw, &raw mut bw_best) == 0
    {
        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else {
        idx = 0 as ::core::ffi::c_int;
        loop {
            if !(idx < num_crunch_configs) {
                current_block = 1352918242886884122;
                break;
            }
            let entropy_idx: ::core::ffi::c_int =
                (*crunch_configs.offset(idx as isize)).entropy_idx_;
            let mut remaining_percent: ::core::ffi::c_int =
                97 as ::core::ffi::c_int / num_crunch_configs;
            let mut percent_range: ::core::ffi::c_int = 0;
            (*enc).use_palette_ = (entropy_idx == kPalette as ::core::ffi::c_int
                || entropy_idx == kPaletteAndSpatial as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            (*enc).use_subtract_green_ = (entropy_idx == kSubGreen as ::core::ffi::c_int
                || entropy_idx == kSpatialSubGreen as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            (*enc).use_predict_ = (entropy_idx == kSpatial as ::core::ffi::c_int
                || entropy_idx == kSpatialSubGreen as ::core::ffi::c_int
                || entropy_idx == kPaletteAndSpatial as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if low_effort != 0 || (*enc).use_palette_ != 0 {
                (*enc).use_cross_color_ = 0 as ::core::ffi::c_int;
            } else {
                (*enc).use_cross_color_ = if red_and_blue_always_zero != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    (*enc).use_predict_
                };
            }
            (*enc).cache_bits_ = 0 as ::core::ffi::c_int;
            VP8LBackwardRefsClear(
                (&raw mut (*enc).refs_ as *mut VP8LBackwardRefs)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut VP8LBackwardRefs,
            );
            VP8LBackwardRefsClear(
                (&raw mut (*enc).refs_ as *mut VP8LBackwardRefs)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut VP8LBackwardRefs,
            );
            use_near_lossless = ((*config).near_lossless < 100 as ::core::ffi::c_int
                && (*enc).use_palette_ == 0
                && (*enc).use_predict_ == 0) as ::core::ffi::c_int;
            if use_near_lossless != 0 {
                if AllocateTransformBuffer(enc, width, height) == 0 {
                    current_block = 10761148025135623058;
                    break;
                }
                if (*enc).argb_content_ as ::core::ffi::c_uint
                    != kEncoderNearLossless as ::core::ffi::c_int as ::core::ffi::c_uint
                    && VP8ApplyNearLossless(picture, (*config).near_lossless, (*enc).argb_) == 0
                {
                    WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                    current_block = 10761148025135623058;
                    break;
                } else {
                    (*enc).argb_content_ = kEncoderNearLossless;
                }
            } else {
                (*enc).argb_content_ = kEncoderNone;
            }
            if (*enc).use_palette_ != 0 {
                if (*crunch_configs.offset(idx as isize)).palette_sorting_type_
                    as ::core::ffi::c_uint
                    == kSortedDefault as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    memcpy(
                        &raw mut (*enc).palette_ as *mut uint32_t as *mut ::core::ffi::c_void,
                        &raw mut (*enc).palette_sorted_ as *mut uint32_t
                            as *const ::core::ffi::c_void,
                        ((*enc).palette_size_ as size_t)
                            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                    );
                } else if (*crunch_configs.offset(idx as isize)).palette_sorting_type_
                    as ::core::ffi::c_uint
                    == kMinimizeDelta as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    PaletteSortMinimizeDeltas(
                        &raw mut (*enc).palette_sorted_ as *mut uint32_t,
                        (*enc).palette_size_,
                        &raw mut (*enc).palette_ as *mut uint32_t,
                    );
                } else {
                    '_c2rust_label: {
                        if (*crunch_configs.offset(idx as isize)).palette_sorting_type_
                            as ::core::ffi::c_uint
                            == kModifiedZeng as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                        } else {
                            __assert_fail(
                                b"crunch_configs[idx].palette_sorting_type_ == kModifiedZeng\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                1856 as ::core::ffi::c_uint,
                                b"int EncodeStreamHook(void *, void *)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    if PaletteSortModifiedZeng(
                        (*enc).pic_,
                        &raw mut (*enc).palette_sorted_ as *mut uint32_t,
                        (*enc).palette_size_ as uint32_t,
                        &raw mut (*enc).palette_ as *mut uint32_t,
                    ) == 0
                    {
                        current_block = 10761148025135623058;
                        break;
                    }
                }
                percent_range = remaining_percent / 4 as ::core::ffi::c_int;
                if EncodePalette(bw, low_effort, enc, percent_range, &raw mut percent) as u64 == 0 {
                    current_block = 10761148025135623058;
                    break;
                }
                remaining_percent -= percent_range;
                if MapImageFromPalette(enc, use_delta_palette) == 0 {
                    current_block = 10761148025135623058;
                    break;
                }
                if use_cache != 0
                    && (*enc).palette_size_ < (1 as ::core::ffi::c_int) << MAX_COLOR_CACHE_BITS
                {
                    (*enc).cache_bits_ =
                        BitsLog2Floor((*enc).palette_size_ as uint32_t) + 1 as ::core::ffi::c_int;
                }
            }
            if use_delta_palette == 0 {
                if (*enc).argb_content_ as ::core::ffi::c_uint
                    != kEncoderNearLossless as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*enc).argb_content_ as ::core::ffi::c_uint
                        != kEncoderPalette as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if MakeInputImageCopy(enc) == 0 {
                        current_block = 10761148025135623058;
                        break;
                    }
                }
                if (*enc).use_subtract_green_ != 0 {
                    ApplySubtractGreen(enc, (*enc).current_width_, height, bw);
                }
                if (*enc).use_predict_ != 0 {
                    percent_range = remaining_percent / 3 as ::core::ffi::c_int;
                    if ApplyPredictFilter(
                        enc,
                        (*enc).current_width_,
                        height,
                        quality,
                        low_effort,
                        (*enc).use_subtract_green_,
                        bw,
                        percent_range,
                        &raw mut percent,
                    ) == 0
                    {
                        current_block = 10761148025135623058;
                        break;
                    }
                    remaining_percent -= percent_range;
                }
                if (*enc).use_cross_color_ != 0 {
                    percent_range = remaining_percent / 2 as ::core::ffi::c_int;
                    if ApplyCrossColorFilter(
                        enc,
                        (*enc).current_width_,
                        height,
                        quality,
                        low_effort,
                        bw,
                        percent_range,
                        &raw mut percent,
                    ) == 0
                    {
                        current_block = 10761148025135623058;
                        break;
                    }
                    remaining_percent -= percent_range;
                }
            }
            VP8LPutBits(
                bw,
                (TRANSFORM_PRESENT == 0) as ::core::ffi::c_int as uint32_t,
                1 as ::core::ffi::c_int,
            );
            if EncodeImageInternal(
                bw,
                (*enc).argb_,
                &raw mut (*enc).hash_chain_,
                &raw mut (*enc).refs_ as *mut VP8LBackwardRefs,
                (*enc).current_width_,
                height,
                quality,
                low_effort,
                use_cache,
                crunch_configs.offset(idx as isize) as *const CrunchConfig,
                &raw mut (*enc).cache_bits_,
                (*enc).histo_bits_,
                byte_position,
                &raw mut hdr_size,
                &raw mut data_size,
                picture,
                remaining_percent,
                &raw mut percent,
            ) == 0
            {
                current_block = 10761148025135623058;
                break;
            }
            if VP8LBitWriterNumBytes(bw) < best_size {
                best_size = VP8LBitWriterNumBytes(bw);
                VP8LBitWriterSwap(bw, &raw mut bw_best);
                if !stats.is_null() {
                    (*stats).lossless_features = 0 as uint32_t;
                    if (*enc).use_predict_ != 0 {
                        (*stats).lossless_features |= 1 as uint32_t;
                    }
                    if (*enc).use_cross_color_ != 0 {
                        (*stats).lossless_features |= 2 as uint32_t;
                    }
                    if (*enc).use_subtract_green_ != 0 {
                        (*stats).lossless_features |= 4 as uint32_t;
                    }
                    if (*enc).use_palette_ != 0 {
                        (*stats).lossless_features |= 8 as uint32_t;
                    }
                    (*stats).histogram_bits = (*enc).histo_bits_;
                    (*stats).transform_bits = (*enc).transform_bits_;
                    (*stats).cache_bits = (*enc).cache_bits_;
                    (*stats).palette_size = (*enc).palette_size_;
                    (*stats).lossless_size =
                        best_size.wrapping_sub(byte_position) as ::core::ffi::c_int;
                    (*stats).lossless_hdr_size = hdr_size;
                    (*stats).lossless_data_size = data_size;
                }
            }
            if num_crunch_configs > 1 as ::core::ffi::c_int {
                VP8LBitWriterReset(&raw mut bw_init, bw);
            }
            idx += 1;
        }
        match current_block {
            10761148025135623058 => {}
            _ => {
                VP8LBitWriterSwap(&raw mut bw_best, bw);
            }
        }
    }
    VP8LBitWriterWipeOut(&raw mut bw_best);
    return ((*(*params).picture_).error_code as ::core::ffi::c_uint
        == VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LEncodeStream(
    config: *const WebPConfig,
    picture: *const WebPPicture,
    bw_main: *mut VP8LBitWriter,
    mut use_cache: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let enc_main: *mut VP8LEncoder = VP8LEncoderNew(config, picture) as *mut VP8LEncoder;
    let mut enc_side: *mut VP8LEncoder = ::core::ptr::null_mut::<VP8LEncoder>();
    let mut crunch_configs: [CrunchConfig; 8] = [CrunchConfig {
        entropy_idx_: 0,
        palette_sorting_type_: kSortedDefault,
        sub_configs_: [CrunchSubConfig {
            lz77_: 0,
            do_no_cache_: 0,
        }; 2],
        sub_configs_size_: 0,
    }; 8];
    let mut num_crunch_configs_main: ::core::ffi::c_int = 0;
    let mut num_crunch_configs_side: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idx: ::core::ffi::c_int = 0;
    let mut red_and_blue_always_zero: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut worker_main: WebPWorker = WebPWorker {
        impl_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        status_: NOT_OK,
        hook: None,
        data1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        had_error: 0,
    };
    let mut worker_side: WebPWorker = WebPWorker {
        impl_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        status_: NOT_OK,
        hook: None,
        data1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        had_error: 0,
    };
    let mut params_main: StreamEncodeContext = StreamEncodeContext {
        config_: ::core::ptr::null::<WebPConfig>(),
        picture_: ::core::ptr::null::<WebPPicture>(),
        bw_: ::core::ptr::null_mut::<VP8LBitWriter>(),
        enc_: ::core::ptr::null_mut::<VP8LEncoder>(),
        use_cache_: 0,
        crunch_configs_: [CrunchConfig {
            entropy_idx_: 0,
            palette_sorting_type_: kSortedDefault,
            sub_configs_: [CrunchSubConfig {
                lz77_: 0,
                do_no_cache_: 0,
            }; 2],
            sub_configs_size_: 0,
        }; 8],
        num_crunch_configs_: 0,
        red_and_blue_always_zero_: 0,
        stats_: ::core::ptr::null_mut::<WebPAuxStats>(),
    };
    let mut params_side: StreamEncodeContext = StreamEncodeContext {
        config_: ::core::ptr::null::<WebPConfig>(),
        picture_: ::core::ptr::null::<WebPPicture>(),
        bw_: ::core::ptr::null_mut::<VP8LBitWriter>(),
        enc_: ::core::ptr::null_mut::<VP8LEncoder>(),
        use_cache_: 0,
        crunch_configs_: [CrunchConfig {
            entropy_idx_: 0,
            palette_sorting_type_: kSortedDefault,
            sub_configs_: [CrunchSubConfig {
                lz77_: 0,
                do_no_cache_: 0,
            }; 2],
            sub_configs_size_: 0,
        }; 8],
        num_crunch_configs_: 0,
        red_and_blue_always_zero_: 0,
        stats_: ::core::ptr::null_mut::<WebPAuxStats>(),
    };
    let mut stats_side: WebPAuxStats = WebPAuxStats {
        coded_size: 0,
        PSNR: [0.; 5],
        block_count: [0; 3],
        header_bytes: [0; 2],
        residual_bytes: [[0; 4]; 3],
        segment_size: [0; 4],
        segment_quant: [0; 4],
        segment_level: [0; 4],
        alpha_data_size: 0,
        layer_data_size: 0,
        lossless_features: 0,
        histogram_bits: 0,
        transform_bits: 0,
        cache_bits: 0,
        palette_size: 0,
        lossless_size: 0,
        lossless_hdr_size: 0,
        lossless_data_size: 0,
        pad: [0; 2],
    };
    let mut bw_side: VP8LBitWriter = VP8LBitWriter {
        bits_: 0,
        used_: 0,
        buf_: ::core::ptr::null_mut::<uint8_t>(),
        cur_: ::core::ptr::null_mut::<uint8_t>(),
        end_: ::core::ptr::null_mut::<uint8_t>(),
        error_: 0,
    };
    let mut picture_side: WebPPicture = WebPPicture {
        use_argb: 0,
        colorspace: WEBP_YUV420,
        width: 0,
        height: 0,
        y: ::core::ptr::null_mut::<uint8_t>(),
        u: ::core::ptr::null_mut::<uint8_t>(),
        v: ::core::ptr::null_mut::<uint8_t>(),
        y_stride: 0,
        uv_stride: 0,
        a: ::core::ptr::null_mut::<uint8_t>(),
        a_stride: 0,
        pad1: [0; 2],
        argb: ::core::ptr::null_mut::<uint32_t>(),
        argb_stride: 0,
        pad2: [0; 3],
        writer: None,
        custom_ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra_info_type: 0,
        extra_info: ::core::ptr::null_mut::<uint8_t>(),
        stats: ::core::ptr::null_mut::<WebPAuxStats>(),
        error_code: VP8_ENC_OK,
        progress_hook: None,
        user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pad3: [0; 3],
        pad4: ::core::ptr::null_mut::<uint8_t>(),
        pad5: ::core::ptr::null_mut::<uint8_t>(),
        pad6: [0; 8],
        memory_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        memory_argb_: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pad7: [::core::ptr::null_mut::<::core::ffi::c_void>(); 2],
    };
    let worker_interface: *const WebPWorkerInterface =
        WebPGetWorkerInterface() as *const WebPWorkerInterface;
    let mut ok_main: ::core::ffi::c_int = 0;
    if enc_main.is_null() || VP8LBitWriterInit(&raw mut bw_side, 0 as size_t) == 0 {
        VP8LEncoderDelete(enc_main);
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    WebPPictureInit(&raw mut picture_side);
    if EncoderAnalyze(
        enc_main,
        &raw mut crunch_configs as *mut CrunchConfig,
        &raw mut num_crunch_configs_main,
        &raw mut red_and_blue_always_zero,
    ) == 0
        || EncoderInit(enc_main) == 0
    {
        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else {
        if (*config).thread_level > 0 as ::core::ffi::c_int {
            num_crunch_configs_side = num_crunch_configs_main / 2 as ::core::ffi::c_int;
            idx = 0 as ::core::ffi::c_int;
            while idx < num_crunch_configs_side {
                params_side.crunch_configs_[idx as usize] = crunch_configs
                    [(num_crunch_configs_main - num_crunch_configs_side + idx) as usize];
                idx += 1;
            }
            params_side.num_crunch_configs_ = num_crunch_configs_side;
        }
        num_crunch_configs_main -= num_crunch_configs_side;
        idx = 0 as ::core::ffi::c_int;
        while idx < num_crunch_configs_main {
            params_main.crunch_configs_[idx as usize] = crunch_configs[idx as usize];
            idx += 1;
        }
        params_main.num_crunch_configs_ = num_crunch_configs_main;
        let params_size: ::core::ffi::c_int = if num_crunch_configs_side > 0 as ::core::ffi::c_int {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        idx = 0 as ::core::ffi::c_int;
        loop {
            if !(idx < params_size) {
                current_block = 12497913735442871383;
                break;
            }
            let worker: *mut WebPWorker = if idx == 0 as ::core::ffi::c_int {
                &raw mut worker_main
            } else {
                &raw mut worker_side
            };
            let param: *mut StreamEncodeContext = if idx == 0 as ::core::ffi::c_int {
                &raw mut params_main
            } else {
                &raw mut params_side
            };
            (*param).config_ = config;
            (*param).use_cache_ = use_cache;
            (*param).red_and_blue_always_zero_ = red_and_blue_always_zero;
            if idx == 0 as ::core::ffi::c_int {
                (*param).picture_ = picture;
                (*param).stats_ = (*picture).stats;
                (*param).bw_ = bw_main;
                (*param).enc_ = enc_main;
            } else {
                if WebPPictureView(
                    picture,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    (*picture).width,
                    (*picture).height,
                    &raw mut picture_side,
                ) == 0
                {
                    '_c2rust_label: {
                        __assert_fail(
                            b"0\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            2024 as ::core::ffi::c_uint,
                            b"int VP8LEncodeStream(const WebPConfig *const, const WebPPicture *const, VP8LBitWriter *const, int)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    };
                }
                picture_side.progress_hook = None;
                (*param).picture_ = &raw mut picture_side;
                (*param).stats_ = if (*picture).stats.is_null() {
                    ::core::ptr::null_mut::<WebPAuxStats>()
                } else {
                    &raw mut stats_side
                };
                if VP8LBitWriterClone(bw_main, &raw mut bw_side) == 0 {
                    WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                    current_block = 3691112344844915034;
                    break;
                } else {
                    (*param).bw_ = &raw mut bw_side;
                    enc_side = VP8LEncoderNew(config, &raw mut picture_side);
                    if enc_side.is_null() || EncoderInit(enc_side) == 0 {
                        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                        current_block = 3691112344844915034;
                        break;
                    } else {
                        (*enc_side).histo_bits_ = (*enc_main).histo_bits_;
                        (*enc_side).transform_bits_ = (*enc_main).transform_bits_;
                        (*enc_side).palette_size_ = (*enc_main).palette_size_;
                        memcpy(
                            &raw mut (*enc_side).palette_ as *mut uint32_t
                                as *mut ::core::ffi::c_void,
                            &raw mut (*enc_main).palette_ as *mut uint32_t
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<[uint32_t; 256]>() as size_t,
                        );
                        memcpy(
                            &raw mut (*enc_side).palette_sorted_ as *mut uint32_t
                                as *mut ::core::ffi::c_void,
                            &raw mut (*enc_main).palette_sorted_ as *mut uint32_t
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<[uint32_t; 256]>() as size_t,
                        );
                        (*param).enc_ = enc_side;
                    }
                }
            }
            (*worker_interface).Init.expect("non-null function pointer")(worker);
            (*worker).data1 = param as *mut ::core::ffi::c_void;
            (*worker).data2 = NULL;
            (*worker).hook = Some(
                EncodeStreamHook
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ) as WebPWorkerHook;
            idx += 1;
        }
        match current_block {
            3691112344844915034 => {}
            _ => {
                if num_crunch_configs_side != 0 as ::core::ffi::c_int {
                    if (*worker_interface)
                        .Reset
                        .expect("non-null function pointer")(
                        &raw mut worker_side
                    ) == 0
                    {
                        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                        current_block = 3691112344844915034;
                    } else {
                        if !(*picture).stats.is_null() {
                            memcpy(
                                &raw mut stats_side as *mut ::core::ffi::c_void,
                                (*picture).stats as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<WebPAuxStats>() as size_t,
                            );
                        }
                        (*worker_interface)
                            .Launch
                            .expect("non-null function pointer")(
                            &raw mut worker_side
                        );
                        current_block = 13763002826403452995;
                    }
                } else {
                    current_block = 13763002826403452995;
                }
                match current_block {
                    3691112344844915034 => {}
                    _ => {
                        (*worker_interface)
                            .Execute
                            .expect("non-null function pointer")(
                            &raw mut worker_main
                        );
                        ok_main = (*worker_interface)
                            .Sync_0
                            .expect("non-null function pointer")(
                            &raw mut worker_main
                        );
                        (*worker_interface).End.expect("non-null function pointer")(
                            &raw mut worker_main,
                        );
                        if num_crunch_configs_side != 0 as ::core::ffi::c_int {
                            let ok_side: ::core::ffi::c_int = (*worker_interface)
                                .Sync_0
                                .expect("non-null function pointer")(
                                &raw mut worker_side
                            )
                                as ::core::ffi::c_int;
                            (*worker_interface).End.expect("non-null function pointer")(
                                &raw mut worker_side,
                            );
                            if ok_main == 0 || ok_side == 0 {
                                if (*picture).error_code as ::core::ffi::c_uint
                                    == VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    '_c2rust_label_0: {
                                        if picture_side.error_code as ::core::ffi::c_uint
                                            != VP8_ENC_OK as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                        {
                                        } else {
                                            __assert_fail(
                                                b"picture_side.error_code != VP8_ENC_OK\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                b"/home/yans/code/safelibs/ported/libwebp/original/src/enc/vp8l_enc.c\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                                2084 as ::core::ffi::c_uint,
                                                b"int VP8LEncodeStream(const WebPConfig *const, const WebPPicture *const, VP8LBitWriter *const, int)\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                            );
                                        }
                                    };
                                    WebPEncodingSetError(picture, picture_side.error_code);
                                }
                            } else if VP8LBitWriterNumBytes(&raw mut bw_side)
                                < VP8LBitWriterNumBytes(bw_main)
                            {
                                VP8LBitWriterSwap(bw_main, &raw mut bw_side);
                                if !(*picture).stats.is_null() {
                                    memcpy(
                                        (*picture).stats as *mut ::core::ffi::c_void,
                                        &raw mut stats_side as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<WebPAuxStats>() as size_t,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    VP8LBitWriterWipeOut(&raw mut bw_side);
    VP8LEncoderDelete(enc_main);
    VP8LEncoderDelete(enc_side);
    return ((*picture).error_code as ::core::ffi::c_uint
        == VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LEncodeImage(
    config: *const WebPConfig,
    picture: *const WebPPicture,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut width: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    let mut has_alpha: ::core::ffi::c_int = 0;
    let mut coded_size: size_t = 0;
    let mut percent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut initial_size: ::core::ffi::c_int = 0;
    let mut bw: VP8LBitWriter = VP8LBitWriter {
        bits_: 0,
        used_: 0,
        buf_: ::core::ptr::null_mut::<uint8_t>(),
        cur_: ::core::ptr::null_mut::<uint8_t>(),
        end_: ::core::ptr::null_mut::<uint8_t>(),
        error_: 0,
    };
    if picture.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if config.is_null() || (*picture).argb.is_null() {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_NULL_PARAMETER);
    }
    width = (*picture).width;
    height = (*picture).height;
    initial_size = if (*config).image_hint as ::core::ffi::c_uint
        == WEBP_HINT_GRAPH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        width * height
    } else {
        width * height * 2 as ::core::ffi::c_int
    };
    if VP8LBitWriterInit(&raw mut bw, initial_size as size_t) == 0 {
        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else {
        if WebPReportProgress(picture, 1 as ::core::ffi::c_int, &raw mut percent) == 0 {
            current_block = 15627310609501265878;
        } else {
            if !(*picture).stats.is_null() {
                let stats: *mut WebPAuxStats = (*picture).stats;
                memset(
                    stats as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<WebPAuxStats>() as size_t,
                );
                (*stats).PSNR[0 as ::core::ffi::c_int as usize] = 99.0f32;
                (*stats).PSNR[1 as ::core::ffi::c_int as usize] = 99.0f32;
                (*stats).PSNR[2 as ::core::ffi::c_int as usize] = 99.0f32;
                (*stats).PSNR[3 as ::core::ffi::c_int as usize] = 99.0f32;
                (*stats).PSNR[4 as ::core::ffi::c_int as usize] = 99.0f32;
            }
            if WriteImageSize(picture, &raw mut bw) == 0 {
                WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                current_block = 286668760143826228;
            } else {
                has_alpha = WebPPictureHasTransparency(picture);
                if WriteRealAlphaAndVersion(&raw mut bw, has_alpha) == 0 {
                    WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                    current_block = 286668760143826228;
                } else if WebPReportProgress(picture, 2 as ::core::ffi::c_int, &raw mut percent)
                    == 0
                {
                    current_block = 15627310609501265878;
                } else if VP8LEncodeStream(config, picture, &raw mut bw, 1 as ::core::ffi::c_int)
                    == 0
                {
                    current_block = 286668760143826228;
                } else if WebPReportProgress(picture, 99 as ::core::ffi::c_int, &raw mut percent)
                    == 0
                {
                    current_block = 15627310609501265878;
                } else if WriteImage(picture, &raw mut bw, &raw mut coded_size) == 0 {
                    current_block = 286668760143826228;
                } else if WebPReportProgress(picture, 100 as ::core::ffi::c_int, &raw mut percent)
                    == 0
                {
                    current_block = 15627310609501265878;
                } else {
                    if !(*picture).stats.is_null() {
                        (*(*picture).stats).coded_size += coded_size as ::core::ffi::c_int;
                        (*(*picture).stats).lossless_size = coded_size as ::core::ffi::c_int;
                    }
                    if !(*picture).extra_info.is_null() {
                        let mb_w: ::core::ffi::c_int =
                            width + 15 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int;
                        let mb_h: ::core::ffi::c_int =
                            height + 15 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int;
                        memset(
                            (*picture).extra_info as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ((mb_w * mb_h) as size_t)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                        );
                    }
                    current_block = 286668760143826228;
                }
            }
        }
        match current_block {
            286668760143826228 => {}
            _ => {
                WebPEncodingSetError(picture, VP8_ENC_ERROR_USER_ABORT);
            }
        }
    }
    if bw.error_ != 0 {
        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    VP8LBitWriterWipeOut(&raw mut bw);
    return ((*picture).error_code as ::core::ffi::c_uint
        == VP8_ENC_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
