pub const WEBP_MAX_ALLOCABLE_MEMORY: u64 = if usize::BITS > 34 {
    1_u64 << 34
} else {
    (1_u64 << 31) - (1_u64 << 16)
};

#[inline]
pub const fn check_size_overflow(size: u64) -> bool {
    size == size as usize as u64
}

#[inline]
pub fn checked_allocation_size(nmemb: u64, size: usize) -> Option<usize> {
    if nmemb == 0 {
        return Some(0);
    }
    if (size as u64) > WEBP_MAX_ALLOCABLE_MEMORY / nmemb {
        return None;
    }
    let total = nmemb.checked_mul(size as u64)?;
    check_size_overflow(total).then_some(total as usize)
}

#[inline]
pub fn checked_mul_usize(a: usize, b: usize) -> Option<usize> {
    a.checked_mul(b)
}

#[inline]
pub fn checked_add_usize(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)
}
