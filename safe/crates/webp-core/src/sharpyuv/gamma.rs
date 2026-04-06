use core::hint::spin_loop;
use core::sync::atomic::{AtomicU8, Ordering};

const GAMMA_TO_LINEAR_TAB_BITS: i32 = 10;
const GAMMA_TO_LINEAR_TAB_SIZE: usize = 1 << GAMMA_TO_LINEAR_TAB_BITS;
const LINEAR_TO_GAMMA_TAB_BITS: i32 = 9;
const LINEAR_TO_GAMMA_TAB_SIZE: usize = 1 << LINEAR_TO_GAMMA_TAB_BITS;
const GAMMA_TO_LINEAR_BITS: i32 = 16;
const K_GAMMA_F: f64 = 1.0 / 0.45;

unsafe extern "C" {
    fn pow(value: f64, power: f64) -> f64;
}

static TABLE_STATE: AtomicU8 = AtomicU8::new(0);
static mut GAMMA_TO_LINEAR_TABLE: [u32; GAMMA_TO_LINEAR_TAB_SIZE + 2] =
    [0; GAMMA_TO_LINEAR_TAB_SIZE + 2];
static mut LINEAR_TO_GAMMA_TABLE: [u32; LINEAR_TO_GAMMA_TAB_SIZE + 2] =
    [0; LINEAR_TO_GAMMA_TAB_SIZE + 2];

#[inline]
fn shift(value: i32, amount: i32) -> i32 {
    if amount >= 0 {
        value << amount
    } else {
        value >> -amount
    }
}

fn initialize_tables() {
    let mut gamma_to_linear = [0_u32; GAMMA_TO_LINEAR_TAB_SIZE + 2];
    let mut linear_to_gamma = [0_u32; LINEAR_TO_GAMMA_TAB_SIZE + 2];
    let a = 0.099_296_826_809_44_f64;
    let thresh = 0.018_053_968_510_807_f64;
    let final_scale = (1_i32 << GAMMA_TO_LINEAR_BITS) as f64;

    let norm = 1.0 / GAMMA_TO_LINEAR_TAB_SIZE as f64;
    let a_rec = 1.0 / (1.0 + a);
    for (index, slot) in gamma_to_linear
        .iter_mut()
        .take(GAMMA_TO_LINEAR_TAB_SIZE + 1)
        .enumerate()
    {
        let g = norm * index as f64;
        let value = if g <= thresh * 4.5 {
            g / 4.5
        } else {
            // SAFETY: libm's `pow` is pure for finite inputs and matches the C implementation.
            unsafe { pow(a_rec * (g + a), K_GAMMA_F) }
        };
        *slot = (value * final_scale + 0.5) as u32;
    }
    gamma_to_linear[GAMMA_TO_LINEAR_TAB_SIZE + 1] = gamma_to_linear[GAMMA_TO_LINEAR_TAB_SIZE];

    let scale = 1.0 / LINEAR_TO_GAMMA_TAB_SIZE as f64;
    for (index, slot) in linear_to_gamma
        .iter_mut()
        .take(LINEAR_TO_GAMMA_TAB_SIZE + 1)
        .enumerate()
    {
        let g = scale * index as f64;
        let value = if g <= thresh {
            4.5 * g
        } else {
            // SAFETY: libm's `pow` is pure for finite inputs and matches the C implementation.
            unsafe { (1.0 + a) * pow(g, 1.0 / K_GAMMA_F) - a }
        };
        *slot = (final_scale * value + 0.5) as u32;
    }
    linear_to_gamma[LINEAR_TO_GAMMA_TAB_SIZE + 1] = linear_to_gamma[LINEAR_TO_GAMMA_TAB_SIZE];

    // SAFETY: initialization is serialized through `TABLE_STATE`, so these unique writes cannot race.
    unsafe {
        GAMMA_TO_LINEAR_TABLE = gamma_to_linear;
        LINEAR_TO_GAMMA_TABLE = linear_to_gamma;
    }
}

pub fn init_gamma_tables() {
    loop {
        match TABLE_STATE.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => {
                initialize_tables();
                TABLE_STATE.store(2, Ordering::Release);
                return;
            }
            Err(1) => spin_loop(),
            Err(2) => return,
            Err(_) => spin_loop(),
        }
    }
}

#[inline]
fn fixed_point_interpolation(
    value: i32,
    table: &[u32],
    tab_pos_shift_right: i32,
    tab_value_shift: i32,
) -> u32 {
    let tab_pos = shift(value, -tab_pos_shift_right) as usize;
    let fractional = value - ((tab_pos as i32) << tab_pos_shift_right);
    let v0 = shift(table[tab_pos] as i32, tab_value_shift) as u32;
    let v1 = shift(table[tab_pos + 1] as i32, tab_value_shift) as u32;
    let interpolated = (v1 - v0) * fractional as u32;
    let half = if tab_pos_shift_right > 0 {
        1_u32 << (tab_pos_shift_right - 1)
    } else {
        0
    };
    v0 + ((interpolated + half) >> tab_pos_shift_right)
}

pub fn gamma_to_linear(value: u16, bit_depth: i32) -> u32 {
    init_gamma_tables();
    let shift_amount = GAMMA_TO_LINEAR_TAB_BITS - bit_depth;
    // SAFETY: the tables are fully initialized before use and then treated as immutable.
    let table = unsafe {
        core::slice::from_raw_parts(
            core::ptr::addr_of!(GAMMA_TO_LINEAR_TABLE).cast::<u32>(),
            GAMMA_TO_LINEAR_TAB_SIZE + 2,
        )
    };
    if shift_amount > 0 {
        table[(value as usize) << shift_amount]
    } else {
        fixed_point_interpolation(value as i32, table, -shift_amount, 0)
    }
}

pub fn linear_to_gamma(value: u32, bit_depth: i32) -> u16 {
    init_gamma_tables();
    // SAFETY: the tables are fully initialized before use and then treated as immutable.
    let table = unsafe {
        core::slice::from_raw_parts(
            core::ptr::addr_of!(LINEAR_TO_GAMMA_TABLE).cast::<u32>(),
            LINEAR_TO_GAMMA_TAB_SIZE + 2,
        )
    };
    fixed_point_interpolation(
        value as i32,
        table,
        GAMMA_TO_LINEAR_BITS - LINEAR_TO_GAMMA_TAB_BITS,
        bit_depth - GAMMA_TO_LINEAR_BITS,
    ) as u16
}
