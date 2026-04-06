extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn WebPSafeFree(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: uint8_t,
    pub value: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode32 {
    pub bits: ::core::ffi::c_int,
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTablesSegment {
    pub start: *mut HuffmanCode,
    pub curr_table: *mut HuffmanCode,
    pub next: *mut HuffmanTablesSegment,
    pub size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTables {
    pub root: HuffmanTablesSegment,
    pub curr_segment: *mut HuffmanTablesSegment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HTreeGroup {
    pub htrees: [*mut HuffmanCode; 5],
    pub is_trivial_literal: ::core::ffi::c_int,
    pub literal_arb: uint32_t,
    pub is_trivial_code: ::core::ffi::c_int,
    pub use_packed_table: ::core::ffi::c_int,
    pub packed_table: [HuffmanCode32; 64],
}
pub const MAX_ALLOWED_CODE_LENGTH: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn VP8LHtreeGroupsNew(
    mut num_htree_groups: ::core::ffi::c_int,
) -> *mut HTreeGroup {
    let htree_groups: *mut HTreeGroup = WebPSafeMalloc(
        num_htree_groups as uint64_t,
        ::core::mem::size_of::<HTreeGroup>() as size_t,
    ) as *mut HTreeGroup;
    if htree_groups.is_null() {
        return ::core::ptr::null_mut::<HTreeGroup>();
    }
    '_c2rust_label: {
        if num_htree_groups <= 0x10000 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"num_htree_groups <= MAX_HTREE_GROUPS\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/huffman_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                31 as ::core::ffi::c_uint,
                b"HTreeGroup *VP8LHtreeGroupsNew(int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return htree_groups;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHtreeGroupsFree(htree_groups: *mut HTreeGroup) {
    if !htree_groups.is_null() {
        WebPSafeFree(htree_groups as *mut ::core::ffi::c_void);
    }
}
#[inline]
unsafe extern "C" fn GetNextKey(mut key: uint32_t, mut len: ::core::ffi::c_int) -> uint32_t {
    let mut step: uint32_t =
        ((1 as ::core::ffi::c_int) << len - 1 as ::core::ffi::c_int) as uint32_t;
    while key & step != 0 {
        step >>= 1 as ::core::ffi::c_int;
    }
    return if step != 0 {
        (key & step.wrapping_sub(1 as uint32_t)).wrapping_add(step)
    } else {
        key
    };
}
#[inline]
unsafe extern "C" fn ReplicateValue(
    mut table: *mut HuffmanCode,
    mut step: ::core::ffi::c_int,
    mut end: ::core::ffi::c_int,
    mut code: HuffmanCode,
) {
    '_c2rust_label: {
        if end % step == 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"end % step == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/huffman_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                56 as ::core::ffi::c_uint,
                b"void ReplicateValue(HuffmanCode *, int, int, HuffmanCode)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    loop {
        end -= step;
        *table.offset(end as isize) = code;
        if !(end > 0 as ::core::ffi::c_int) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn NextTableBitSize(
    count: *const ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
    mut root_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut left: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << len - root_bits;
    while len < MAX_ALLOWED_CODE_LENGTH {
        left -= *count.offset(len as isize);
        if left <= 0 as ::core::ffi::c_int {
            break;
        }
        len += 1;
        left <<= 1 as ::core::ffi::c_int;
    }
    return len - root_bits;
}
unsafe extern "C" fn BuildHuffmanTable(
    root_table: *mut HuffmanCode,
    mut root_bits: ::core::ffi::c_int,
    mut code_lengths: *const ::core::ffi::c_int,
    mut code_lengths_size: ::core::ffi::c_int,
    mut sorted: *mut uint16_t,
) -> ::core::ffi::c_int {
    let mut table: *mut HuffmanCode = root_table;
    let mut total_size: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << root_bits;
    let mut len: ::core::ffi::c_int = 0;
    let mut symbol: ::core::ffi::c_int = 0;
    let mut count: [::core::ffi::c_int; 16] = [0 as ::core::ffi::c_int; 16];
    let mut offset: [::core::ffi::c_int; 16] = [0; 16];
    '_c2rust_label: {
        if code_lengths_size != 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"code_lengths_size != 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/huffman_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_uint,
                b"int BuildHuffmanTable(HuffmanCode *const, int, const int *, int, uint16_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !code_lengths.is_null() {
        } else {
            __assert_fail(
                b"code_lengths != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/huffman_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                93 as ::core::ffi::c_uint,
                b"int BuildHuffmanTable(HuffmanCode *const, int, const int *, int, uint16_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !root_table.is_null() && !sorted.is_null() || root_table.is_null() && sorted.is_null() {
        } else {
            __assert_fail(
                b"(root_table != NULL && sorted != NULL) || (root_table == NULL && sorted == NULL)\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/huffman_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                95 as ::core::ffi::c_uint,
                b"int BuildHuffmanTable(HuffmanCode *const, int, const int *, int, uint16_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if root_bits > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"root_bits > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/huffman_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                b"int BuildHuffmanTable(HuffmanCode *const, int, const int *, int, uint16_t *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    symbol = 0 as ::core::ffi::c_int;
    while symbol < code_lengths_size {
        if *code_lengths.offset(symbol as isize) > MAX_ALLOWED_CODE_LENGTH {
            return 0 as ::core::ffi::c_int;
        }
        count[*code_lengths.offset(symbol as isize) as usize] += 1;
        symbol += 1;
    }
    if count[0 as ::core::ffi::c_int as usize] == code_lengths_size {
        return 0 as ::core::ffi::c_int;
    }
    offset[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    len = 1 as ::core::ffi::c_int;
    while len < MAX_ALLOWED_CODE_LENGTH {
        if count[len as usize] > (1 as ::core::ffi::c_int) << len {
            return 0 as ::core::ffi::c_int;
        }
        offset[(len + 1 as ::core::ffi::c_int) as usize] =
            offset[len as usize] + count[len as usize];
        len += 1;
    }
    symbol = 0 as ::core::ffi::c_int;
    while symbol < code_lengths_size {
        let symbol_code_length: ::core::ffi::c_int = *code_lengths.offset(symbol as isize);
        if *code_lengths.offset(symbol as isize) > 0 as ::core::ffi::c_int {
            if !sorted.is_null() {
                let fresh0 = offset[symbol_code_length as usize];
                offset[symbol_code_length as usize] = offset[symbol_code_length as usize] + 1;
                *sorted.offset(fresh0 as isize) = symbol as uint16_t;
            } else {
                offset[symbol_code_length as usize] += 1;
            }
        }
        symbol += 1;
    }
    if offset[MAX_ALLOWED_CODE_LENGTH as usize] == 1 as ::core::ffi::c_int {
        if !sorted.is_null() {
            let mut code: HuffmanCode = HuffmanCode { bits: 0, value: 0 };
            code.bits = 0 as uint8_t;
            code.value = *sorted.offset(0 as ::core::ffi::c_int as isize);
            ReplicateValue(table, 1 as ::core::ffi::c_int, total_size, code);
        }
        return total_size;
    }
    let mut step: ::core::ffi::c_int = 0;
    let mut low: uint32_t = 0xffffffff as uint32_t;
    let mut mask: uint32_t = (total_size - 1 as ::core::ffi::c_int) as uint32_t;
    let mut key: uint32_t = 0 as uint32_t;
    let mut num_nodes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut num_open: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut table_bits: ::core::ffi::c_int = root_bits;
    let mut table_size: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << table_bits;
    symbol = 0 as ::core::ffi::c_int;
    len = 1 as ::core::ffi::c_int;
    step = 2 as ::core::ffi::c_int;
    while len <= root_bits {
        num_open <<= 1 as ::core::ffi::c_int;
        num_nodes += num_open;
        num_open -= count[len as usize];
        if num_open < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if !root_table.is_null() {
            while count[len as usize] > 0 as ::core::ffi::c_int {
                let mut code_0: HuffmanCode = HuffmanCode { bits: 0, value: 0 };
                code_0.bits = len as uint8_t;
                let fresh1 = symbol;
                symbol = symbol + 1;
                code_0.value = *sorted.offset(fresh1 as isize);
                ReplicateValue(
                    table.offset(key as isize) as *mut HuffmanCode,
                    step,
                    table_size,
                    code_0,
                );
                key = GetNextKey(key, len);
                count[len as usize] -= 1;
            }
        }
        len += 1;
        step <<= 1 as ::core::ffi::c_int;
    }
    len = root_bits + 1 as ::core::ffi::c_int;
    step = 2 as ::core::ffi::c_int;
    while len <= MAX_ALLOWED_CODE_LENGTH {
        num_open <<= 1 as ::core::ffi::c_int;
        num_nodes += num_open;
        num_open -= count[len as usize];
        if num_open < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        while count[len as usize] > 0 as ::core::ffi::c_int {
            let mut code_1: HuffmanCode = HuffmanCode { bits: 0, value: 0 };
            if key & mask != low {
                if !root_table.is_null() {
                    table = table.offset(table_size as isize);
                }
                table_bits =
                    NextTableBitSize(&raw mut count as *mut ::core::ffi::c_int, len, root_bits);
                table_size = (1 as ::core::ffi::c_int) << table_bits;
                total_size += table_size;
                low = key & mask;
                if !root_table.is_null() {
                    (*root_table.offset(low as isize)).bits = (table_bits + root_bits) as uint8_t;
                    (*root_table.offset(low as isize)).value =
                        (table.offset_from(root_table) as ::core::ffi::c_long
                            - low as ::core::ffi::c_long) as uint16_t;
                }
            }
            if !root_table.is_null() {
                code_1.bits = (len - root_bits) as uint8_t;
                let fresh2 = symbol;
                symbol = symbol + 1;
                code_1.value = *sorted.offset(fresh2 as isize);
                ReplicateValue(
                    table.offset((key >> root_bits) as isize) as *mut HuffmanCode,
                    step,
                    table_size,
                    code_1,
                );
            }
            key = GetNextKey(key, len);
            count[len as usize] -= 1;
        }
        len += 1;
        step <<= 1 as ::core::ffi::c_int;
    }
    if num_nodes
        != 2 as ::core::ffi::c_int * offset[MAX_ALLOWED_CODE_LENGTH as usize]
            - 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    return total_size;
}
pub const SORTED_SIZE_CUTOFF: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn VP8LBuildHuffmanTable(
    root_table: *mut HuffmanTables,
    mut root_bits: ::core::ffi::c_int,
    mut code_lengths: *const ::core::ffi::c_int,
    mut code_lengths_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let total_size: ::core::ffi::c_int = BuildHuffmanTable(
        ::core::ptr::null_mut::<HuffmanCode>(),
        root_bits,
        code_lengths,
        code_lengths_size,
        ::core::ptr::null_mut::<uint16_t>(),
    ) as ::core::ffi::c_int;
    '_c2rust_label: {
        if code_lengths_size
            <= ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int)
                + 256 as ::core::ffi::c_int
                + 24 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"code_lengths_size <= MAX_CODE_LENGTHS_SIZE\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/code/safelibs/ported/libwebp/original/src/utils/huffman_utils.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                221 as ::core::ffi::c_uint,
                b"int VP8LBuildHuffmanTable(HuffmanTables *const, int, const int *, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if total_size == 0 as ::core::ffi::c_int || root_table.is_null() {
        return total_size;
    }
    if (*(*root_table).curr_segment)
        .curr_table
        .offset(total_size as isize)
        >= (*(*root_table).curr_segment)
            .start
            .offset((*(*root_table).curr_segment).size as isize)
    {
        let segment_size: ::core::ffi::c_int = (*(*root_table).curr_segment).size;
        let mut next: *mut HuffmanTablesSegment = WebPSafeMalloc(
            1 as uint64_t,
            ::core::mem::size_of::<HuffmanTablesSegment>() as size_t,
        ) as *mut HuffmanTablesSegment;
        if next.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*next).size = if total_size > segment_size {
            total_size
        } else {
            segment_size
        };
        (*next).start = WebPSafeMalloc(
            (*next).size as uint64_t,
            ::core::mem::size_of::<HuffmanCode>() as size_t,
        ) as *mut HuffmanCode;
        if (*next).start.is_null() {
            WebPSafeFree(next as *mut ::core::ffi::c_void);
            return 0 as ::core::ffi::c_int;
        }
        (*next).curr_table = (*next).start;
        (*next).next = ::core::ptr::null_mut::<HuffmanTablesSegment>();
        (*(*root_table).curr_segment).next = next;
        (*root_table).curr_segment = next as *mut HuffmanTablesSegment;
    }
    if code_lengths_size <= SORTED_SIZE_CUTOFF {
        let mut sorted: [uint16_t; 512] = [0; 512];
        BuildHuffmanTable(
            (*(*root_table).curr_segment).curr_table,
            root_bits,
            code_lengths,
            code_lengths_size,
            &raw mut sorted as *mut uint16_t,
        );
    } else {
        let sorted_0: *mut uint16_t = WebPSafeMalloc(
            code_lengths_size as uint64_t,
            ::core::mem::size_of::<uint16_t>() as size_t,
        ) as *mut uint16_t;
        if sorted_0.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        BuildHuffmanTable(
            (*(*root_table).curr_segment).curr_table,
            root_bits,
            code_lengths,
            code_lengths_size,
            sorted_0 as *mut uint16_t,
        );
        WebPSafeFree(sorted_0 as *mut ::core::ffi::c_void);
    }
    return total_size;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHuffmanTablesAllocate(
    mut size: ::core::ffi::c_int,
    mut huffman_tables: *mut HuffmanTables,
) -> ::core::ffi::c_int {
    if huffman_tables.is_null() || size <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let root: *mut HuffmanTablesSegment = &raw mut (*huffman_tables).root;
    (*huffman_tables).curr_segment = root;
    (*root).next = ::core::ptr::null_mut::<HuffmanTablesSegment>();
    (*root).start = ::core::ptr::null_mut::<HuffmanCode>();
    (*root).curr_table = ::core::ptr::null_mut::<HuffmanCode>();
    (*root).size = 0 as ::core::ffi::c_int;
    (*root).start = WebPSafeMalloc(
        size as uint64_t,
        ::core::mem::size_of::<HuffmanCode>() as size_t,
    ) as *mut HuffmanCode;
    if (*root).start.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*root).curr_table = (*root).start;
    (*root).size = size;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHuffmanTablesDeallocate(huffman_tables: *mut HuffmanTables) {
    let mut current: *mut HuffmanTablesSegment = ::core::ptr::null_mut::<HuffmanTablesSegment>();
    let mut next: *mut HuffmanTablesSegment = ::core::ptr::null_mut::<HuffmanTablesSegment>();
    if huffman_tables.is_null() {
        return;
    }
    current = &raw mut (*huffman_tables).root;
    next = (*current).next as *mut HuffmanTablesSegment;
    WebPSafeFree((*current).start as *mut ::core::ffi::c_void);
    (*current).start = ::core::ptr::null_mut::<HuffmanCode>();
    (*current).next = ::core::ptr::null_mut::<HuffmanTablesSegment>();
    current = next;
    while !current.is_null() {
        next = (*current).next as *mut HuffmanTablesSegment;
        WebPSafeFree((*current).start as *mut ::core::ffi::c_void);
        WebPSafeFree(current as *mut ::core::ffi::c_void);
        current = next;
    }
}
