#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

macro_rules! c_enum {
    (
        $(#[$meta:meta])*
        pub struct $name:ident($repr:ty) {
            $(
                $(#[$const_meta:meta])*
                const $const_name:ident = $value:expr;
            )+
        }
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
        pub struct $name(pub $repr);

        impl $name {
            $(
                $(#[$const_meta])*
                pub const $const_name: Self = Self($value);
            )+
        }

        $(
            $(#[$const_meta])*
            pub const $const_name: $name = $name($value);
        )+
    };
}

macro_rules! opaque_type {
    (pub struct $name:ident;) => {
        #[repr(C)]
        #[derive(Debug)]
        pub struct $name {
            _private: [u8; 0],
        }
    };
}

pub mod decode;
pub mod demux;
pub mod encode;
pub mod mux;
pub mod sharpyuv;
pub mod types;

pub use decode::*;
pub use demux::*;
pub use encode::*;
pub use mux::*;
pub use sharpyuv::*;
pub use types::*;
