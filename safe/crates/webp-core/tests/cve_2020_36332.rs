use webp_core::decode::vp8l_dec::compact_huffman_group_indices;

#[test]
fn cve_2020_36332() {
    let mut groups = [0xffff_u32; 4];
    let (mapping, unique_groups) =
        compact_huffman_group_indices(&mut groups).expect("sparse group ids should compact");

    assert_eq!(unique_groups, 1);
    assert_eq!(groups, [0, 0, 0, 0]);
    assert_eq!(mapping.len(), 0x10000);
    assert_eq!(mapping[0xffff], 0);
    assert_eq!(mapping[0], -1);
}
