#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]

extern crate alloc as rust_alloc;

pub mod alloc;
pub mod checked;
mod compat;
pub mod cpu;
pub mod decode;
pub mod demux;
pub mod dsp;
pub mod encode;
pub mod sharpyuv;
pub mod threading;
pub mod utils;

pub use alloc::{webp_free, webp_malloc, webp_safe_calloc, webp_safe_free, webp_safe_malloc};
pub use cpu::{
    default_cpu_info, kAVX, kAVX2, kMIPS32, kMIPSdspR2, kMSA, kNEON, kSSE2, kSSE3, kSSE4_1,
    kSlowSSSE3, CPUFeature, VP8CPUInfo,
};
pub use threading::{
    webp_get_worker_interface, webp_set_worker_interface, WebPWorker, WebPWorkerHook,
    WebPWorkerInterface, WebPWorkerStatus, NOT_OK, OK, WORK,
};
