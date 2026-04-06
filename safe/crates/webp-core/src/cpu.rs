use core::ffi::c_int;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct CPUFeature(pub c_int);

pub const kSSE2: CPUFeature = CPUFeature(0);
pub const kSSE3: CPUFeature = CPUFeature(1);
pub const kSlowSSSE3: CPUFeature = CPUFeature(2);
pub const kSSE4_1: CPUFeature = CPUFeature(3);
pub const kAVX: CPUFeature = CPUFeature(4);
pub const kAVX2: CPUFeature = CPUFeature(5);
pub const kNEON: CPUFeature = CPUFeature(6);
pub const kMIPS32: CPUFeature = CPUFeature(7);
pub const kMIPSdspR2: CPUFeature = CPUFeature(8);
pub const kMSA: CPUFeature = CPUFeature(9);

pub type VP8CPUInfo = Option<unsafe extern "C" fn(feature: CPUFeature) -> c_int>;

pub unsafe extern "C" fn default_cpu_info(feature: CPUFeature) -> c_int {
    match feature.0 {
        0 => cfg!(target_arch = "x86_64") as c_int,
        1 => cfg!(target_feature = "sse3") as c_int,
        2 => 0,
        3 => cfg!(target_feature = "sse4.1") as c_int,
        4 => cfg!(target_feature = "avx") as c_int,
        5 => cfg!(target_feature = "avx2") as c_int,
        6 => {
            (cfg!(target_feature = "neon")
                || cfg!(target_arch = "aarch64")
                || cfg!(target_arch = "arm")) as c_int
        }
        7 => cfg!(target_arch = "mips") as c_int,
        8 => cfg!(target_arch = "mips") as c_int,
        9 => cfg!(target_arch = "mips64") as c_int,
        _ => 0,
    }
}
