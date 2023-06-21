use std::collections::HashMap;

use super::{parser::parse_triangle_indices, TriangleIndex};

#[test]
fn single_triangle_face_test() {
    let input = "usemtl A\nf 1/1/1 2/1/0 3/1/1";
    let map = vec![("A".to_string(), 0)]
        .into_iter()
        .collect::<HashMap<String, usize>>();
    let expected_index = vec![TriangleIndex::new((0, 1, 2), 0, 0)];

    let (_, index) = parse_triangle_indices(input, &map).expect("Test panic!");
    assert_eq!(index, expected_index);
}

#[test]
fn multiple_triangle_face_test() {
    let input = "usemtl A\nf 1/1/1 2/1/0 3/1/1\nf 1/1/1 2/1/1 3/1/1";
    let map = vec![("A".to_string(), 0)]
        .into_iter()
        .collect::<HashMap<String, usize>>();
    let expected_indices = vec![
        TriangleIndex::new((0, 1, 2), 0, 0),
        TriangleIndex::new((0, 1, 2), 0, 0),
    ];

    let (_, indices) = parse_triangle_indices(input, &map).expect("Test panic!");
    assert_eq!(indices, expected_indices);
}

#[test]
fn multiple_material_face_test() {
    let input = "usemtl A\nf 1/1/1 2/1/1 3/1/1\nusemtl B\nf 1/1/1 2/1/1 3/1/1";
    let map = vec![("A".to_string(), 0), ("B".to_string(), 1)]
        .into_iter()
        .collect::<HashMap<String, usize>>();

    let expected_indices = vec![
        TriangleIndex::new((0, 1, 2), 0, 0),
        TriangleIndex::new((0, 1, 2), 0, 1),
    ];

    let (_, indices) = parse_triangle_indices(input, &map).expect("Test panic!");
    assert_eq!(indices, expected_indices);
}
