//! Mux and animation-encode module placeholders.
//!
//! This phase wires the safe workspace to the unchanged upstream mux
//! implementation through `libwebpmux` while reserving the `webp-core::mux`
//! module tree for the eventual in-Rust port.

pub mod anim_encode;
pub mod muxedit;
pub mod muxinternal;
pub mod muxread;
