use core::{ffi::c_void, mem};

use webp_core::encode::picture_enc::{
    WebPMemoryWrite, WebPMemoryWriter, WebPPicture, WebPPictureAlloc, WebPPictureInitInternal,
    VP8_ENC_ERROR_OUT_OF_MEMORY, WEBP_ENCODER_ABI_VERSION, WEBP_YUV420A,
};

#[test]
fn cve_2016_9085() {
    let mut writer = WebPMemoryWriter {
        mem: core::ptr::null_mut(),
        size: usize::MAX - 4,
        max_size: usize::MAX - 4,
        pad: [0; 1],
    };
    let mut picture = unsafe { mem::zeroed::<WebPPicture>() };

    assert_eq!(
        unsafe { WebPPictureInitInternal(&mut picture, WEBP_ENCODER_ABI_VERSION) },
        1
    );
    picture.custom_ptr = (&mut writer as *mut WebPMemoryWriter).cast::<c_void>();

    let chunk = [0u8; 8];
    assert_eq!(
        unsafe { WebPMemoryWrite(chunk.as_ptr(), chunk.len(), &picture) },
        0
    );
    assert!(writer.mem.is_null());
    assert_eq!(writer.size, usize::MAX - 4);
    assert_eq!(writer.max_size, usize::MAX - 4);
}

#[test]
fn oversized_picture_alloc() {
    let mut picture = unsafe { mem::zeroed::<WebPPicture>() };

    assert_eq!(
        unsafe { WebPPictureInitInternal(&mut picture, WEBP_ENCODER_ABI_VERSION) },
        1
    );
    picture.use_argb = 1;
    picture.width = i32::MAX;
    picture.height = i32::MAX;

    assert_eq!(unsafe { WebPPictureAlloc(&mut picture) }, 0);
    assert_eq!(picture.error_code, VP8_ENC_ERROR_OUT_OF_MEMORY);
    assert!(picture.memory_argb_.is_null());
    assert!(picture.argb.is_null());
}

#[test]
fn oversized_picture_alloc_yuva() {
    let mut picture = unsafe { mem::zeroed::<WebPPicture>() };

    assert_eq!(
        unsafe { WebPPictureInitInternal(&mut picture, WEBP_ENCODER_ABI_VERSION) },
        1
    );
    picture.use_argb = 0;
    picture.colorspace = WEBP_YUV420A;
    picture.width = i32::MAX;
    picture.height = i32::MAX;

    assert_eq!(unsafe { WebPPictureAlloc(&mut picture) }, 0);
    assert_eq!(picture.error_code, VP8_ENC_ERROR_OUT_OF_MEMORY);
    assert!(picture.memory_.is_null());
    assert!(picture.y.is_null());
    assert!(picture.u.is_null());
    assert!(picture.v.is_null());
    assert!(picture.a.is_null());
}
