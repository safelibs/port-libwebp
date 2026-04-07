use crate::decode::webp_dec::{
    WebPBitstreamFeatures, WebPGetFeaturesInternal, VP8_STATUS_NOT_ENOUGH_DATA, VP8_STATUS_OK,
};
use crate::rust_alloc::{boxed::Box, vec::Vec};
use core::{cmp::min, ffi::c_void, ptr, slice};
use webp_abi::{
    WebPChunkIterator, WebPData, WebPDemuxState, WebPDemuxer, WebPFormatFeature, WebPIterator,
    WebPMuxAnimBlend, WebPMuxAnimDispose, ALL_VALID_FLAGS, ALPHA_FLAG, ANIMATION_FLAG, EXIF_FLAG,
    ICCP_FLAG, WEBP_ABI_IS_INCOMPATIBLE, WEBP_DECODER_ABI_VERSION, WEBP_DEMUX_ABI_VERSION,
    WEBP_MUX_BLEND, WEBP_MUX_DISPOSE_BACKGROUND, WEBP_MUX_DISPOSE_NONE, WEBP_MUX_NO_BLEND,
    XMP_FLAG,
};

const DMUX_MAJ_VERSION: i32 = 1;
const DMUX_MIN_VERSION: i32 = 3;
const DMUX_REV_VERSION: i32 = 2;

const TAG_SIZE: usize = 4;
const CHUNK_HEADER_SIZE: usize = 8;
const RIFF_HEADER_SIZE: usize = 12;
const ANMF_CHUNK_SIZE: usize = 16;
const ANIM_CHUNK_SIZE: usize = 6;
const VP8X_CHUNK_SIZE: usize = 10;
const MAX_IMAGE_AREA: u64 = 1_u64 << 32;
const MAX_CHUNK_PAYLOAD: u32 = u32::MAX - CHUNK_HEADER_SIZE as u32 - 1;

#[derive(Copy, Clone, Eq, PartialEq)]
enum ParseStatus {
    Ok,
    NeedMoreData,
    Error,
}

#[derive(Copy, Clone)]
struct MemBuffer {
    start: usize,
    end: usize,
    riff_end: usize,
    buf_size: usize,
    buf: *const u8,
}

#[derive(Copy, Clone, Default)]
struct ChunkData {
    offset: usize,
    size: usize,
}

#[derive(Clone, Default)]
struct Frame {
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    has_alpha: i32,
    duration: i32,
    dispose_method: WebPMuxAnimDispose,
    blend_method: WebPMuxAnimBlend,
    frame_num: i32,
    complete: i32,
    img_components: [ChunkData; 2],
}

#[derive(Clone, Default)]
struct Chunk {
    data: ChunkData,
}

struct Demuxer {
    mem: MemBuffer,
    state: WebPDemuxState,
    is_ext_format: bool,
    feature_flags: u32,
    canvas_width: i32,
    canvas_height: i32,
    loop_count: i32,
    bgcolor: u32,
    num_frames: i32,
    frames: Vec<Frame>,
    chunks: Vec<Chunk>,
}

struct ChunkParser {
    id: [u8; TAG_SIZE],
    parse: fn(&mut Demuxer) -> ParseStatus,
    valid: fn(&Demuxer) -> bool,
}

const MASTER_CHUNKS: [ChunkParser; 3] = [
    ChunkParser {
        id: *b"VP8 ",
        parse: parse_single_image,
        valid: is_valid_simple_format,
    },
    ChunkParser {
        id: *b"VP8L",
        parse: parse_single_image,
        valid: is_valid_simple_format,
    },
    ChunkParser {
        id: *b"VP8X",
        parse: parse_vp8x,
        valid: is_valid_extended_format,
    },
];

#[inline]
fn mk_fourcc(a: u8, b: u8, c: u8, d: u8) -> u32 {
    u32::from(a) | (u32::from(b) << 8) | (u32::from(c) << 16) | (u32::from(d) << 24)
}

#[inline]
unsafe fn mem_bytes(mem: &MemBuffer) -> &[u8] {
    // SAFETY: `mem.buf` is borrowed from the caller-owned bitstream for `buf_size` bytes.
    unsafe { slice::from_raw_parts(mem.buf, mem.buf_size) }
}

#[inline]
fn mem_data_size(mem: &MemBuffer) -> usize {
    mem.end.saturating_sub(mem.start)
}

#[inline]
fn size_is_invalid(mem: &MemBuffer, size: usize) -> bool {
    size > mem.riff_end.saturating_sub(mem.start)
}

#[inline]
fn skip(mem: &mut MemBuffer, size: usize) {
    mem.start = mem.start.saturating_add(size);
}

#[inline]
fn rewind(mem: &mut MemBuffer, size: usize) {
    mem.start = mem.start.saturating_sub(size);
}

#[inline]
unsafe fn get_buffer(mem: &MemBuffer) -> *const u8 {
    // SAFETY: callers guard `mem.start <= mem.buf_size`.
    unsafe { mem.buf.add(mem.start) }
}

#[inline]
unsafe fn read_byte(mem: &mut MemBuffer) -> u8 {
    // SAFETY: callers ensure at least one readable byte remains.
    let byte = unsafe { *get_buffer(mem) };
    skip(mem, 1);
    byte
}

#[inline]
unsafe fn read_le16s(mem: &mut MemBuffer) -> i32 {
    let bytes = unsafe { mem_bytes(mem) };
    let value = u16::from_le_bytes([bytes[mem.start], bytes[mem.start + 1]]);
    skip(mem, 2);
    i32::from(value)
}

#[inline]
unsafe fn read_le24s(mem: &mut MemBuffer) -> i32 {
    let bytes = unsafe { mem_bytes(mem) };
    let value = u32::from(bytes[mem.start])
        | (u32::from(bytes[mem.start + 1]) << 8)
        | (u32::from(bytes[mem.start + 2]) << 16);
    skip(mem, 3);
    value as i32
}

#[inline]
unsafe fn read_le32(mem: &mut MemBuffer) -> u32 {
    let bytes = unsafe { mem_bytes(mem) };
    let value = u32::from_le_bytes([
        bytes[mem.start],
        bytes[mem.start + 1],
        bytes[mem.start + 2],
        bytes[mem.start + 3],
    ]);
    skip(mem, 4);
    value
}

#[inline]
fn init_mem_buffer(data: *const u8, size: usize) -> MemBuffer {
    MemBuffer {
        start: 0,
        end: size,
        riff_end: 0,
        buf_size: size,
        buf: data,
    }
}

#[inline]
unsafe fn call_get_features(
    data: *const u8,
    size: usize,
    features: &mut WebPBitstreamFeatures,
) -> u32 {
    // SAFETY: the decoder feature probe accepts the same bitstream lifetime as the caller.
    unsafe { WebPGetFeaturesInternal(data, size, features, WEBP_DECODER_ABI_VERSION) }
}

fn add_chunk(dmux: &mut Demuxer, chunk: Chunk) {
    dmux.chunks.push(chunk);
}

fn add_frame(dmux: &mut Demuxer, frame: Frame) -> bool {
    if dmux.frames.last().is_some_and(|last| last.complete == 0) {
        return false;
    }
    dmux.frames.push(frame);
    true
}

fn set_frame_info(
    start_offset: usize,
    size: usize,
    frame_num: i32,
    complete: bool,
    features: &WebPBitstreamFeatures,
    frame: &mut Frame,
) {
    frame.img_components[0].offset = start_offset;
    frame.img_components[0].size = size;
    frame.width = features.width;
    frame.height = features.height;
    frame.has_alpha |= features.has_alpha;
    frame.frame_num = frame_num;
    frame.complete = i32::from(complete);
}

fn store_frame(
    frame_num: i32,
    min_size: usize,
    mem: &mut MemBuffer,
    frame: &mut Frame,
) -> ParseStatus {
    let mut alpha_chunks = 0;
    let mut image_chunks = 0;
    let mut done = mem_data_size(mem) < CHUNK_HEADER_SIZE || mem_data_size(mem) < min_size;
    let mut status = ParseStatus::Ok;

    if done {
        return ParseStatus::NeedMoreData;
    }

    while !done && status == ParseStatus::Ok {
        let chunk_start_offset = mem.start;
        // SAFETY: loop guards ensure a full chunk header is available.
        let fourcc = unsafe { read_le32(mem) };
        // SAFETY: loop guards ensure a full chunk header is available.
        let payload_size = unsafe { read_le32(mem) };
        let payload_size_padded = payload_size as usize + (payload_size as usize & 1);
        let payload_available = min(payload_size_padded, mem_data_size(mem));
        let chunk_size = CHUNK_HEADER_SIZE + payload_available;

        if payload_size > MAX_CHUNK_PAYLOAD {
            return ParseStatus::Error;
        }
        if size_is_invalid(mem, payload_size_padded) {
            return ParseStatus::Error;
        }
        if payload_size_padded > mem_data_size(mem) {
            status = ParseStatus::NeedMoreData;
        }

        match fourcc {
            value if value == mk_fourcc(b'A', b'L', b'P', b'H') => {
                if alpha_chunks == 0 {
                    alpha_chunks += 1;
                    frame.img_components[1].offset = chunk_start_offset;
                    frame.img_components[1].size = chunk_size;
                    frame.has_alpha = 1;
                    frame.frame_num = frame_num;
                    skip(mem, payload_available);
                } else {
                    rewind(mem, CHUNK_HEADER_SIZE);
                    done = true;
                }
            }
            value if value == mk_fourcc(b'V', b'P', b'8', b'L') => {
                if alpha_chunks > 0 {
                    return ParseStatus::Error;
                }
                if image_chunks == 0 {
                    let mut features = WebPBitstreamFeatures {
                        width: 0,
                        height: 0,
                        has_alpha: 0,
                        has_animation: 0,
                        format: 0,
                        pad: [0; 5],
                    };
                    let vp8_status = unsafe {
                        call_get_features(
                            unsafe { mem.buf.add(chunk_start_offset) },
                            chunk_size,
                            &mut features,
                        )
                    };
                    if status == ParseStatus::NeedMoreData
                        && vp8_status == VP8_STATUS_NOT_ENOUGH_DATA
                    {
                        return ParseStatus::NeedMoreData;
                    }
                    if vp8_status != VP8_STATUS_OK {
                        return ParseStatus::Error;
                    }
                    image_chunks += 1;
                    set_frame_info(
                        chunk_start_offset,
                        chunk_size,
                        frame_num,
                        status == ParseStatus::Ok,
                        &features,
                        frame,
                    );
                    skip(mem, payload_available);
                } else {
                    rewind(mem, CHUNK_HEADER_SIZE);
                    done = true;
                }
            }
            value if value == mk_fourcc(b'V', b'P', b'8', b' ') => {
                if image_chunks == 0 {
                    let mut features = WebPBitstreamFeatures {
                        width: 0,
                        height: 0,
                        has_alpha: 0,
                        has_animation: 0,
                        format: 0,
                        pad: [0; 5],
                    };
                    let vp8_status = unsafe {
                        call_get_features(
                            unsafe { mem.buf.add(chunk_start_offset) },
                            chunk_size,
                            &mut features,
                        )
                    };
                    if status == ParseStatus::NeedMoreData
                        && vp8_status == VP8_STATUS_NOT_ENOUGH_DATA
                    {
                        return ParseStatus::NeedMoreData;
                    }
                    if vp8_status != VP8_STATUS_OK {
                        return ParseStatus::Error;
                    }
                    image_chunks += 1;
                    set_frame_info(
                        chunk_start_offset,
                        chunk_size,
                        frame_num,
                        status == ParseStatus::Ok,
                        &features,
                        frame,
                    );
                    skip(mem, payload_available);
                } else {
                    rewind(mem, CHUNK_HEADER_SIZE);
                    done = true;
                }
            }
            _ => {
                rewind(mem, CHUNK_HEADER_SIZE);
                done = true;
            }
        }

        if mem.start == mem.riff_end {
            done = true;
        } else if mem_data_size(mem) < CHUNK_HEADER_SIZE {
            status = ParseStatus::NeedMoreData;
        }
    }

    status
}

fn new_frame(mem: &MemBuffer, min_size: usize, actual_size: usize) -> Result<Frame, ParseStatus> {
    if size_is_invalid(mem, min_size) {
        return Err(ParseStatus::Error);
    }
    if actual_size < min_size {
        return Err(ParseStatus::Error);
    }
    if mem_data_size(mem) < min_size {
        return Err(ParseStatus::NeedMoreData);
    }
    Ok(Frame::default())
}

fn parse_animation_frame(dmux: &mut Demuxer, frame_chunk_size: usize) -> ParseStatus {
    let is_animation = (dmux.feature_flags & ANIMATION_FLAG.0 as u32) != 0;
    let anmf_payload_size = frame_chunk_size.saturating_sub(ANMF_CHUNK_SIZE);
    let mem = &mut dmux.mem;
    let mut frame = match new_frame(mem, ANMF_CHUNK_SIZE, frame_chunk_size) {
        Ok(frame) => frame,
        Err(status) => return status,
    };

    // SAFETY: `new_frame` validated that the fixed ANMF payload is available.
    frame.x_offset = 2 * unsafe { read_le24s(mem) };
    frame.y_offset = 2 * unsafe { read_le24s(mem) };
    frame.width = 1 + unsafe { read_le24s(mem) };
    frame.height = 1 + unsafe { read_le24s(mem) };
    frame.duration = unsafe { read_le24s(mem) };
    let bits = unsafe { read_byte(mem) };
    frame.dispose_method = if (bits & 1) != 0 {
        WEBP_MUX_DISPOSE_BACKGROUND
    } else {
        WEBP_MUX_DISPOSE_NONE
    };
    frame.blend_method = if (bits & 2) != 0 {
        WEBP_MUX_NO_BLEND
    } else {
        WEBP_MUX_BLEND
    };
    if (frame.width as u64) * (frame.height as u64) >= MAX_IMAGE_AREA {
        return ParseStatus::Error;
    }

    let start_offset = mem.start;
    let status = store_frame(dmux.num_frames + 1, anmf_payload_size, mem, &mut frame);
    if status != ParseStatus::Error && mem.start.saturating_sub(start_offset) > anmf_payload_size {
        return ParseStatus::Error;
    }
    if status != ParseStatus::Error && is_animation && frame.frame_num > 0 {
        if add_frame(dmux, frame) {
            dmux.num_frames += 1;
        } else {
            return ParseStatus::Error;
        }
    }
    status
}

fn store_chunk(dmux: &mut Demuxer, start_offset: usize, size: usize) {
    add_chunk(
        dmux,
        Chunk {
            data: ChunkData {
                offset: start_offset,
                size,
            },
        },
    );
}

fn read_header(mem: &mut MemBuffer) -> ParseStatus {
    let min_size = RIFF_HEADER_SIZE + CHUNK_HEADER_SIZE;
    if mem_data_size(mem) < min_size {
        return ParseStatus::NeedMoreData;
    }

    let bytes = unsafe { mem_bytes(mem) };
    if &bytes[mem.start..mem.start + TAG_SIZE] != b"RIFF"
        || &bytes[mem.start + CHUNK_HEADER_SIZE..mem.start + CHUNK_HEADER_SIZE + TAG_SIZE]
            != b"WEBP"
    {
        return ParseStatus::Error;
    }

    let riff_size = u32::from_le_bytes([
        bytes[mem.start + TAG_SIZE],
        bytes[mem.start + TAG_SIZE + 1],
        bytes[mem.start + TAG_SIZE + 2],
        bytes[mem.start + TAG_SIZE + 3],
    ]);
    if riff_size < CHUNK_HEADER_SIZE as u32 || riff_size > MAX_CHUNK_PAYLOAD {
        return ParseStatus::Error;
    }

    mem.riff_end = riff_size as usize + CHUNK_HEADER_SIZE;
    if mem.buf_size > mem.riff_end {
        mem.buf_size = mem.riff_end;
        mem.end = mem.riff_end;
    }
    skip(mem, RIFF_HEADER_SIZE);
    ParseStatus::Ok
}

fn parse_single_image(dmux: &mut Demuxer) -> ParseStatus {
    if !dmux.frames.is_empty() {
        return ParseStatus::Error;
    }
    if size_is_invalid(&dmux.mem, CHUNK_HEADER_SIZE) {
        return ParseStatus::Error;
    }
    if mem_data_size(&dmux.mem) < CHUNK_HEADER_SIZE {
        return ParseStatus::NeedMoreData;
    }

    let mut frame = Frame::default();
    let status = store_frame(1, 0, &mut dmux.mem, &mut frame);
    if status != ParseStatus::Error {
        let has_alpha = (dmux.feature_flags & ALPHA_FLAG.0 as u32) != 0;
        if !has_alpha && frame.img_components[1].size > 0 {
            frame.img_components[1] = ChunkData::default();
            frame.has_alpha = 0;
        }
        if !dmux.is_ext_format && frame.width > 0 && frame.height > 0 {
            dmux.state = WebPDemuxState::WEBP_DEMUX_PARSED_HEADER;
            dmux.canvas_width = frame.width;
            dmux.canvas_height = frame.height;
            if frame.has_alpha != 0 {
                dmux.feature_flags |= ALPHA_FLAG.0 as u32;
            }
        }
        if !add_frame(dmux, frame) {
            return ParseStatus::Error;
        }
        dmux.num_frames = 1;
    }
    status
}

fn parse_vp8x_chunks(dmux: &mut Demuxer) -> ParseStatus {
    let is_animation = (dmux.feature_flags & ANIMATION_FLAG.0 as u32) != 0;
    let mut anim_chunks = 0;
    let mut status = ParseStatus::Ok;

    while status == ParseStatus::Ok {
        let chunk_start_offset = dmux.mem.start;
        // SAFETY: caller ensures at least a chunk header is present before entering.
        let fourcc = unsafe { read_le32(&mut dmux.mem) };
        let chunk_size = unsafe { read_le32(&mut dmux.mem) };
        let chunk_size_padded = chunk_size as usize + (chunk_size as usize & 1);

        if chunk_size > MAX_CHUNK_PAYLOAD {
            return ParseStatus::Error;
        }
        if size_is_invalid(&dmux.mem, chunk_size_padded) {
            return ParseStatus::Error;
        }

        match fourcc {
            value if value == mk_fourcc(b'V', b'P', b'8', b'X') => return ParseStatus::Error,
            value
                if value == mk_fourcc(b'A', b'L', b'P', b'H')
                    || value == mk_fourcc(b'V', b'P', b'8', b' ')
                    || value == mk_fourcc(b'V', b'P', b'8', b'L') =>
            {
                if anim_chunks > 0 || is_animation {
                    return ParseStatus::Error;
                }
                rewind(&mut dmux.mem, CHUNK_HEADER_SIZE);
                status = parse_single_image(dmux);
            }
            value if value == mk_fourcc(b'A', b'N', b'I', b'M') => {
                if chunk_size_padded < ANIM_CHUNK_SIZE {
                    return ParseStatus::Error;
                }
                if mem_data_size(&dmux.mem) < chunk_size_padded {
                    status = ParseStatus::NeedMoreData;
                } else if anim_chunks == 0 {
                    anim_chunks += 1;
                    dmux.bgcolor = unsafe { read_le32(&mut dmux.mem) };
                    dmux.loop_count = unsafe { read_le16s(&mut dmux.mem) };
                    skip(&mut dmux.mem, chunk_size_padded - ANIM_CHUNK_SIZE);
                } else {
                    skip(&mut dmux.mem, chunk_size_padded);
                }
            }
            value if value == mk_fourcc(b'A', b'N', b'M', b'F') => {
                if anim_chunks == 0 {
                    return ParseStatus::Error;
                }
                status = parse_animation_frame(dmux, chunk_size_padded);
            }
            value if value == mk_fourcc(b'I', b'C', b'C', b'P') => {
                let should_store = (dmux.feature_flags & ICCP_FLAG.0 as u32) != 0;
                if chunk_size_padded <= mem_data_size(&dmux.mem) {
                    if should_store {
                        store_chunk(
                            dmux,
                            chunk_start_offset,
                            CHUNK_HEADER_SIZE + chunk_size as usize,
                        );
                    }
                    skip(&mut dmux.mem, chunk_size_padded);
                } else {
                    status = ParseStatus::NeedMoreData;
                }
            }
            value if value == mk_fourcc(b'E', b'X', b'I', b'F') => {
                let should_store = (dmux.feature_flags & EXIF_FLAG.0 as u32) != 0;
                if chunk_size_padded <= mem_data_size(&dmux.mem) {
                    if should_store {
                        store_chunk(
                            dmux,
                            chunk_start_offset,
                            CHUNK_HEADER_SIZE + chunk_size as usize,
                        );
                    }
                    skip(&mut dmux.mem, chunk_size_padded);
                } else {
                    status = ParseStatus::NeedMoreData;
                }
            }
            value if value == mk_fourcc(b'X', b'M', b'P', b' ') => {
                let should_store = (dmux.feature_flags & XMP_FLAG.0 as u32) != 0;
                if chunk_size_padded <= mem_data_size(&dmux.mem) {
                    if should_store {
                        store_chunk(
                            dmux,
                            chunk_start_offset,
                            CHUNK_HEADER_SIZE + chunk_size as usize,
                        );
                    }
                    skip(&mut dmux.mem, chunk_size_padded);
                } else {
                    status = ParseStatus::NeedMoreData;
                }
            }
            _ => {
                if chunk_size_padded <= mem_data_size(&dmux.mem) {
                    store_chunk(
                        dmux,
                        chunk_start_offset,
                        CHUNK_HEADER_SIZE + chunk_size as usize,
                    );
                    skip(&mut dmux.mem, chunk_size_padded);
                } else {
                    status = ParseStatus::NeedMoreData;
                }
            }
        }

        if dmux.mem.start == dmux.mem.riff_end {
            break;
        }
        if mem_data_size(&dmux.mem) < CHUNK_HEADER_SIZE {
            status = ParseStatus::NeedMoreData;
        }
    }

    status
}

fn parse_vp8x(dmux: &mut Demuxer) -> ParseStatus {
    let mem = &mut dmux.mem;
    if mem_data_size(mem) < CHUNK_HEADER_SIZE {
        return ParseStatus::NeedMoreData;
    }

    dmux.is_ext_format = true;
    skip(mem, TAG_SIZE);
    let mut vp8x_size = unsafe { read_le32(mem) };
    if vp8x_size > MAX_CHUNK_PAYLOAD || vp8x_size < VP8X_CHUNK_SIZE as u32 {
        return ParseStatus::Error;
    }
    vp8x_size += vp8x_size & 1;
    let vp8x_size = vp8x_size as usize;
    if size_is_invalid(mem, vp8x_size) {
        return ParseStatus::Error;
    }
    if mem_data_size(mem) < vp8x_size {
        return ParseStatus::NeedMoreData;
    }

    dmux.feature_flags = unsafe { read_byte(mem) } as u32;
    skip(mem, 3);
    dmux.canvas_width = 1 + unsafe { read_le24s(mem) };
    dmux.canvas_height = 1 + unsafe { read_le24s(mem) };
    if (dmux.canvas_width as u64) * (dmux.canvas_height as u64) >= MAX_IMAGE_AREA {
        return ParseStatus::Error;
    }
    skip(mem, vp8x_size - VP8X_CHUNK_SIZE);
    dmux.state = WebPDemuxState::WEBP_DEMUX_PARSED_HEADER;

    if size_is_invalid(mem, CHUNK_HEADER_SIZE) {
        return ParseStatus::Error;
    }
    if mem_data_size(mem) < CHUNK_HEADER_SIZE {
        return ParseStatus::NeedMoreData;
    }
    parse_vp8x_chunks(dmux)
}

fn is_valid_simple_format(dmux: &Demuxer) -> bool {
    if dmux.state == WebPDemuxState::WEBP_DEMUX_PARSING_HEADER {
        return true;
    }
    if dmux.canvas_width <= 0 || dmux.canvas_height <= 0 {
        return false;
    }
    let Some(frame) = dmux.frames.first() else {
        return dmux.state != WebPDemuxState::WEBP_DEMUX_DONE;
    };
    if frame.width <= 0 || frame.height <= 0 {
        return false;
    }
    true
}

fn check_frame_bounds(frame: &Frame, exact: bool, canvas_width: i32, canvas_height: i32) -> bool {
    if exact {
        if frame.x_offset != 0 || frame.y_offset != 0 {
            return false;
        }
        if frame.width != canvas_width || frame.height != canvas_height {
            return false;
        }
    } else {
        if frame.x_offset < 0 || frame.y_offset < 0 {
            return false;
        }
        if i64::from(frame.width) + i64::from(frame.x_offset) > i64::from(canvas_width) {
            return false;
        }
        if i64::from(frame.height) + i64::from(frame.y_offset) > i64::from(canvas_height) {
            return false;
        }
    }
    true
}

fn is_valid_extended_format(dmux: &Demuxer) -> bool {
    let is_animation = (dmux.feature_flags & ANIMATION_FLAG.0 as u32) != 0;
    if dmux.state == WebPDemuxState::WEBP_DEMUX_PARSING_HEADER {
        return true;
    }
    if dmux.canvas_width <= 0 || dmux.canvas_height <= 0 {
        return false;
    }
    if dmux.loop_count < 0 {
        return false;
    }
    if dmux.state == WebPDemuxState::WEBP_DEMUX_DONE && dmux.frames.is_empty() {
        return false;
    }
    if dmux.feature_flags & !(ALL_VALID_FLAGS.0 as u32) != 0 {
        return false;
    }

    for (index, frame) in dmux.frames.iter().enumerate() {
        let image = &frame.img_components[0];
        let alpha = &frame.img_components[1];

        if !is_animation && frame.frame_num > 1 {
            return false;
        }

        if frame.complete != 0 {
            if alpha.size == 0 && image.size == 0 {
                return false;
            }
            if alpha.size > 0 && alpha.offset > image.offset {
                return false;
            }
            if frame.width <= 0 || frame.height <= 0 {
                return false;
            }
        } else {
            if dmux.state == WebPDemuxState::WEBP_DEMUX_DONE {
                return false;
            }
            if alpha.size > 0 && image.size > 0 && alpha.offset > image.offset {
                return false;
            }
            if index + 1 != dmux.frames.len() {
                return false;
            }
        }

        if frame.width > 0
            && frame.height > 0
            && !check_frame_bounds(frame, !is_animation, dmux.canvas_width, dmux.canvas_height)
        {
            return false;
        }
    }
    true
}

fn init_demux(mem: &MemBuffer) -> Demuxer {
    Demuxer {
        mem: *mem,
        state: WebPDemuxState::WEBP_DEMUX_PARSING_HEADER,
        is_ext_format: false,
        feature_flags: 0,
        canvas_width: -1,
        canvas_height: -1,
        loop_count: 1,
        bgcolor: 0xffff_ffff,
        num_frames: 0,
        frames: Vec::new(),
        chunks: Vec::new(),
    }
}

fn create_raw_image_demuxer(mem: &MemBuffer) -> Result<Box<Demuxer>, ParseStatus> {
    let mut features = WebPBitstreamFeatures {
        width: 0,
        height: 0,
        has_alpha: 0,
        has_animation: 0,
        format: 0,
        pad: [0; 5],
    };
    let status = unsafe { call_get_features(mem.buf, mem.buf_size, &mut features) };
    if status != VP8_STATUS_OK {
        return Err(if status == VP8_STATUS_NOT_ENOUGH_DATA {
            ParseStatus::NeedMoreData
        } else {
            ParseStatus::Error
        });
    }

    let mut dmux = Box::new(init_demux(mem));
    let mut frame = Frame::default();
    set_frame_info(0, mem.buf_size, 1, true, &features, &mut frame);
    let canvas_width = frame.width;
    let canvas_height = frame.height;
    let has_alpha = frame.has_alpha;
    if !add_frame(&mut dmux, frame) {
        return Err(ParseStatus::Error);
    }
    dmux.state = WebPDemuxState::WEBP_DEMUX_DONE;
    dmux.canvas_width = canvas_width;
    dmux.canvas_height = canvas_height;
    if has_alpha != 0 {
        dmux.feature_flags |= ALPHA_FLAG.0 as u32;
    }
    dmux.num_frames = 1;
    if !is_valid_simple_format(&dmux) {
        return Err(ParseStatus::Error);
    }
    Ok(dmux)
}

#[inline]
unsafe fn demuxer_ref<'a>(dmux: *const WebPDemuxer) -> Option<&'a Demuxer> {
    (!dmux.is_null()).then(|| unsafe { &*dmux.cast::<Demuxer>() })
}

#[inline]
unsafe fn demuxer_mut<'a>(dmux: *mut WebPDemuxer) -> Option<&'a mut Demuxer> {
    (!dmux.is_null()).then(|| unsafe { &mut *dmux.cast::<Demuxer>() })
}

pub unsafe fn WebPGetDemuxVersion() -> i32 {
    (DMUX_MAJ_VERSION << 16) | (DMUX_MIN_VERSION << 8) | DMUX_REV_VERSION
}

pub unsafe fn WebPDemuxInternal(
    data: *const WebPData,
    allow_partial: i32,
    state: *mut WebPDemuxState,
    version: i32,
) -> *mut WebPDemuxer {
    if !state.is_null() {
        // SAFETY: caller provided a writable state pointer.
        unsafe { *state = WebPDemuxState::WEBP_DEMUX_PARSE_ERROR };
    }
    if WEBP_ABI_IS_INCOMPATIBLE(version, WEBP_DEMUX_ABI_VERSION) {
        return ptr::null_mut();
    }
    let Some(data) = (!data.is_null()).then(|| unsafe { &*data }) else {
        return ptr::null_mut();
    };
    if data.bytes.is_null() || data.size == 0 {
        return ptr::null_mut();
    }

    let mut mem = init_mem_buffer(data.bytes, data.size);
    let mut status = read_header(&mut mem);
    if status != ParseStatus::Ok {
        if status == ParseStatus::Error {
            match create_raw_image_demuxer(&mem) {
                Ok(dmux) => {
                    if !state.is_null() {
                        unsafe { *state = WebPDemuxState::WEBP_DEMUX_DONE };
                    }
                    return Box::into_raw(dmux).cast();
                }
                Err(raw_status) => status = raw_status,
            }
        }
        if !state.is_null() {
            unsafe {
                *state = if status == ParseStatus::NeedMoreData {
                    WebPDemuxState::WEBP_DEMUX_PARSING_HEADER
                } else {
                    WebPDemuxState::WEBP_DEMUX_PARSE_ERROR
                };
            }
        }
        return ptr::null_mut();
    }

    let partial = mem.buf_size < mem.riff_end;
    if allow_partial == 0 && partial {
        return ptr::null_mut();
    }

    let mut dmux = Box::new(init_demux(&mem));
    status = ParseStatus::Error;
    for parser in MASTER_CHUNKS {
        let matches = unsafe { mem_bytes(&dmux.mem) }
            .get(dmux.mem.start..dmux.mem.start + TAG_SIZE)
            .is_some_and(|tag| tag == parser.id);
        if !matches {
            continue;
        }
        status = (parser.parse)(&mut dmux);
        if status == ParseStatus::Ok {
            dmux.state = WebPDemuxState::WEBP_DEMUX_DONE;
        }
        if status == ParseStatus::NeedMoreData && !partial {
            status = ParseStatus::Error;
        }
        if status != ParseStatus::Error && !(parser.valid)(&dmux) {
            status = ParseStatus::Error;
        }
        if status == ParseStatus::Error {
            dmux.state = WebPDemuxState::WEBP_DEMUX_PARSE_ERROR;
        }
        break;
    }
    if !state.is_null() {
        unsafe { *state = dmux.state };
    }
    if status == ParseStatus::Error {
        return ptr::null_mut();
    }
    Box::into_raw(dmux).cast()
}

pub unsafe fn WebPDemuxDelete(dmux: *mut WebPDemuxer) {
    if dmux.is_null() {
        return;
    }
    // SAFETY: `dmux` was allocated with `Box::into_raw` in this module.
    unsafe { drop(Box::from_raw(dmux.cast::<Demuxer>())) };
}

pub unsafe fn WebPDemuxGetI(dmux: *const WebPDemuxer, feature: WebPFormatFeature) -> u32 {
    let Some(dmux) = (unsafe { demuxer_ref(dmux) }) else {
        return 0;
    };
    match feature {
        WebPFormatFeature::WEBP_FF_FORMAT_FLAGS => dmux.feature_flags,
        WebPFormatFeature::WEBP_FF_CANVAS_WIDTH => dmux.canvas_width as u32,
        WebPFormatFeature::WEBP_FF_CANVAS_HEIGHT => dmux.canvas_height as u32,
        WebPFormatFeature::WEBP_FF_LOOP_COUNT => dmux.loop_count as u32,
        WebPFormatFeature::WEBP_FF_BACKGROUND_COLOR => dmux.bgcolor,
        WebPFormatFeature::WEBP_FF_FRAME_COUNT => dmux.num_frames as u32,
        _ => 0,
    }
}

fn get_frame(dmux: &Demuxer, frame_num: i32) -> Option<&Frame> {
    dmux.frames
        .iter()
        .find(|frame| frame.frame_num == frame_num)
}

unsafe fn get_frame_payload(mem: &MemBuffer, frame: &Frame) -> Option<(*const u8, usize)> {
    let image = &frame.img_components[0];
    let alpha = &frame.img_components[1];
    if image.size == 0 {
        return None;
    }
    let mut start_offset = image.offset;
    let mut payload_size = image.size;
    if alpha.size > 0 {
        let inter_size = image
            .offset
            .saturating_sub(alpha.offset.saturating_add(alpha.size));
        start_offset = alpha.offset;
        payload_size = payload_size
            .saturating_add(alpha.size)
            .saturating_add(inter_size);
    }
    Some((unsafe { mem.buf.add(start_offset) }, payload_size))
}

fn synthesize_frame(dmux: &Demuxer, frame: &Frame, iter: &mut WebPIterator) -> bool {
    let Some((payload, payload_size)) = (unsafe { get_frame_payload(&dmux.mem, frame) }) else {
        return false;
    };
    iter.frame_num = frame.frame_num;
    iter.num_frames = dmux.num_frames;
    iter.x_offset = frame.x_offset;
    iter.y_offset = frame.y_offset;
    iter.width = frame.width;
    iter.height = frame.height;
    iter.has_alpha = frame.has_alpha;
    iter.duration = frame.duration;
    iter.dispose_method = frame.dispose_method;
    iter.blend_method = frame.blend_method;
    iter.complete = frame.complete;
    iter.fragment = WebPData {
        bytes: payload,
        size: payload_size,
    };
    true
}

fn set_frame(frame_num: i32, iter: &mut WebPIterator) -> bool {
    let Some(dmux) = (unsafe { demuxer_ref(iter.private_.cast::<WebPDemuxer>()) }) else {
        return false;
    };
    if frame_num < 0 || frame_num > dmux.num_frames {
        return false;
    }
    let frame_num = if frame_num == 0 {
        dmux.num_frames
    } else {
        frame_num
    };
    let Some(frame) = get_frame(dmux, frame_num) else {
        return false;
    };
    synthesize_frame(dmux, frame, iter)
}

pub unsafe fn WebPDemuxGetFrame(
    dmux: *const WebPDemuxer,
    frame_number: i32,
    iter: *mut WebPIterator,
) -> i32 {
    if iter.is_null() {
        return 0;
    }
    // SAFETY: `iter` is a valid caller-provided output pointer.
    unsafe {
        ptr::write_bytes(iter, 0, 1);
        (*iter).private_ = dmux.cast_mut().cast::<c_void>();
        i32::from(set_frame(frame_number, &mut *iter))
    }
}

pub unsafe fn WebPDemuxNextFrame(iter: *mut WebPIterator) -> i32 {
    if iter.is_null() {
        return 0;
    }
    // SAFETY: `iter` remains owned by the caller.
    unsafe { i32::from(set_frame((*iter).frame_num + 1, &mut *iter)) }
}

pub unsafe fn WebPDemuxPrevFrame(iter: *mut WebPIterator) -> i32 {
    if iter.is_null() {
        return 0;
    }
    // SAFETY: `iter` remains owned by the caller.
    unsafe {
        if (*iter).frame_num <= 1 {
            0
        } else {
            i32::from(set_frame((*iter).frame_num - 1, &mut *iter))
        }
    }
}

pub unsafe fn WebPDemuxReleaseIterator(_iter: *mut WebPIterator) {}

fn chunk_matches(mem: &MemBuffer, chunk: &Chunk, fourcc: &[u8; TAG_SIZE]) -> bool {
    let bytes = unsafe { mem_bytes(mem) };
    bytes
        .get(chunk.data.offset..chunk.data.offset + TAG_SIZE)
        .is_some_and(|header| header == fourcc)
}

fn chunk_count(dmux: &Demuxer, fourcc: &[u8; TAG_SIZE]) -> i32 {
    dmux.chunks
        .iter()
        .filter(|chunk| chunk_matches(&dmux.mem, chunk, fourcc))
        .count() as i32
}

fn get_chunk<'a>(dmux: &'a Demuxer, fourcc: &[u8; TAG_SIZE], chunk_num: i32) -> Option<&'a Chunk> {
    let mut count = 0;
    for chunk in &dmux.chunks {
        if chunk_matches(&dmux.mem, chunk, fourcc) {
            count += 1;
            if count == chunk_num {
                return Some(chunk);
            }
        }
    }
    None
}

fn set_chunk(fourcc: *const u8, chunk_num: i32, iter: &mut WebPChunkIterator) -> bool {
    let Some(dmux) = (unsafe { demuxer_ref(iter.private_.cast::<WebPDemuxer>()) }) else {
        return false;
    };
    if fourcc.is_null() || chunk_num < 0 {
        return false;
    }
    let mut tag = [0u8; TAG_SIZE];
    // SAFETY: `fourcc` points at four bytes owned by the demux bitstream.
    unsafe { ptr::copy_nonoverlapping(fourcc, tag.as_mut_ptr(), TAG_SIZE) };
    let count = chunk_count(dmux, &tag);
    if count == 0 {
        return false;
    }
    let chunk_num = if chunk_num == 0 { count } else { chunk_num };
    let Some(chunk) = get_chunk(dmux, &tag, chunk_num) else {
        return false;
    };
    iter.chunk = WebPData {
        bytes: unsafe { dmux.mem.buf.add(chunk.data.offset + CHUNK_HEADER_SIZE) },
        size: chunk.data.size.saturating_sub(CHUNK_HEADER_SIZE),
    };
    iter.num_chunks = count;
    iter.chunk_num = chunk_num;
    true
}

pub unsafe fn WebPDemuxGetChunk(
    dmux: *const WebPDemuxer,
    fourcc: *const i8,
    chunk_num: i32,
    iter: *mut WebPChunkIterator,
) -> i32 {
    if iter.is_null() {
        return 0;
    }
    // SAFETY: `iter` is a valid caller-provided output pointer.
    unsafe {
        ptr::write_bytes(iter, 0, 1);
        (*iter).private_ = dmux.cast_mut().cast::<c_void>();
        i32::from(set_chunk(fourcc.cast::<u8>(), chunk_num, &mut *iter))
    }
}

pub unsafe fn WebPDemuxNextChunk(iter: *mut WebPChunkIterator) -> i32 {
    if iter.is_null() {
        return 0;
    }
    // SAFETY: iterator payload pointers always refer to chunk data preceded by a chunk header.
    unsafe {
        if (*iter).chunk.bytes.is_null() {
            0
        } else {
            i32::from(set_chunk(
                (*iter).chunk.bytes.sub(CHUNK_HEADER_SIZE),
                (*iter).chunk_num + 1,
                &mut *iter,
            ))
        }
    }
}

pub unsafe fn WebPDemuxPrevChunk(iter: *mut WebPChunkIterator) -> i32 {
    if iter.is_null() {
        return 0;
    }
    // SAFETY: iterator payload pointers always refer to chunk data preceded by a chunk header.
    unsafe {
        if (*iter).chunk_num <= 1 || (*iter).chunk.bytes.is_null() {
            0
        } else {
            i32::from(set_chunk(
                (*iter).chunk.bytes.sub(CHUNK_HEADER_SIZE),
                (*iter).chunk_num - 1,
                &mut *iter,
            ))
        }
    }
}

pub unsafe fn WebPDemuxReleaseChunkIterator(_iter: *mut WebPChunkIterator) {}
