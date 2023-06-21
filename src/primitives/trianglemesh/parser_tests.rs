use super::*;

use glam::{Vec2, Vec3};

#[test]
fn parse_single_vertex_position_test() {
    let position = "v 0.0 0.0 0.0";

    let (_, (p, n, m)) = parser::parse_vertex_data(position).unwrap();
    assert_eq!(p.len(), 1);
    assert_eq!(n.len(), 0);
    assert_eq!(m.len(), 0);

    let expected_pos = vec![Vec3::new(0.0, 0.0, 0.0)];
    assert_eq!(p, expected_pos);
}

#[test]
fn parse_single_vertex_normal_test() {
    let normal = "vn 1.0 0.0 0.0";

    let (_, (p, n, m)) = parser::parse_vertex_data(normal).unwrap();
    assert_eq!(p.len(), 0);
    assert_eq!(n.len(), 1);
    assert_eq!(m.len(), 0);

    let expected_normal = vec![Vec3::new(1.0, 0.0, 0.0)];
    assert_eq!(n, expected_normal);
}

#[test]
fn parse_single_vertex_texture_test() {
    let texture = "vt 0.0 0.0";

    let (_, (p, n, m)) = parser::parse_vertex_data(texture).unwrap();
    assert_eq!(p.len(), 0);
    assert_eq!(n.len(), 0);
    assert_eq!(m.len(), 1);

    let expected_texture = vec![Vec2::new(0.0, 0.0)];
    assert_eq!(m, expected_texture);
}

#[test]
fn parse_multiple_vertex_data_test() {
    let vertex_data = "v 1.000000 1.000000 -1.000000
v 1.000000 -1.000000 -1.000000
vn -0.0000 1.0000 -0.0000
vn -0.0000 -0.0000 1.0000
vt 0.625000 0.500000
vt 0.375000 0.500000";

    let (_, (p, n, m)) = parser::parse_vertex_data(vertex_data).unwrap();
    assert_eq!(p.len(), 2);
    assert_eq!(n.len(), 2);
    assert_eq!(m.len(), 2);

    let expected_pos = vec![
        Vec3::new(1.000000, 1.000000, -1.000000),
        Vec3::new(1.000000, -1.000000, -1.000000),
    ];
    assert_eq!(p, expected_pos);

    let expected_normal = vec![
        Vec3::new(-0.0000, 1.0000, -0.0000),
        Vec3::new(-0.0000, -0.0000, 1.0000),
    ];

    assert_eq!(n, expected_normal);

    let expected_texture = vec![Vec2::new(0.625000, 0.500000), Vec2::new(0.375000, 0.500000)];

    assert_eq!(m, expected_texture);
}