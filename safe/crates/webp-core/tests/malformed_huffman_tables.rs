use webp_core::decode::vp8l_dec::checked_huffman_table_allocation_size;
use webp_core::utils::huffman::{
    HuffmanTables, VP8LHuffmanTablesAllocate, VP8LHuffmanTablesDeallocate,
};

#[test]
fn malformed_huffman_tables() {
    let mut tables = unsafe { core::mem::zeroed::<HuffmanTables>() };

    assert_eq!(checked_huffman_table_allocation_size(i32::MAX, 2), None);
    assert_eq!(checked_huffman_table_allocation_size(0, 1), None);

    assert_eq!(unsafe { VP8LHuffmanTablesAllocate(0, &mut tables) }, 0);
    assert!(tables.root.start.is_null());
    assert!(tables.root.curr_table.is_null());
    assert_eq!(tables.root.size, 0);

    let allocation_size =
        checked_huffman_table_allocation_size(1, 5004).expect("single-group table should fit");
    assert_eq!(
        unsafe { VP8LHuffmanTablesAllocate(allocation_size, &mut tables) },
        1
    );
    assert!(!tables.root.start.is_null());
    unsafe { VP8LHuffmanTablesDeallocate(&mut tables) };
}
