use super::{parser, Material};

use glam::Vec3;

#[test]
fn single_materials_test() {
    let input = "# Blender 3.5.1 MTL File: 'None'
# www.blender.org

newmtl Material
Ns 250.000000
Ka 1.000000 1.000000 1.000000
Kd 0.800000 0.800000 0.800000
Ks 0.500000 0.500000 0.500000
Ke 0.000000 0.000000 0.000000
Ni 1.450000
d 1.000000
illum 2";

    let expected_materials = vec![
        Material {
            ambient_color: Vec3::new(1.0, 1.0, 1.0),
            diffuse_color: Vec3::new(0.8, 0.8, 0.8),
            specular_color: Vec3::new(0.5, 0.5, 0.5),
            specular_highlight: 250.0,
            emissive_color: Vec3::new(0.0, 0.0, 0.0),
            transparency: 1.0,
            index_of_refraction: 1.45,
        }
    ];

    let (_, (mats, _)) = parser::materials(input).unwrap();
    assert_eq!(expected_materials, mats);
}

#[test]
fn multiple_materials_test() {
    let input = "newmtl Material
Ns 250.000000
Ka 1.000000 1.000000 1.000000
Kd 0.800000 0.800000 0.800000
Ks 0.500000 0.500000 0.500000
Ke 0.000000 0.000000 0.000000
Ni 1.450000
d 1.000000
illum 2
newmtl Red
Ns 250.000000
Ka 1.000000 0.000000 0.000000
Kd 1.000000 0.000000 0.000000
Ks 0.500000 0.500000 0.500000
Ke 0.000000 0.000000 0.000000
Ni 1.450000
d 1.000000
illum 2";
    let expected_materials = vec![
        Material {
            ambient_color: Vec3::new(1.0, 1.0, 1.0),
            diffuse_color: Vec3::new(0.8, 0.8, 0.8),
            specular_color: Vec3::new(0.5, 0.5, 0.5),
            specular_highlight: 250.0,
            emissive_color: Vec3::new(0.0, 0.0, 0.0),
            transparency: 1.0,
            index_of_refraction: 1.45,
        },
        Material {
            ambient_color: Vec3::new(1.0, 0.0, 0.0),
            diffuse_color: Vec3::new(1.0, 0.0, 0.0),
            specular_color: Vec3::new(0.5, 0.5, 0.5),
            specular_highlight: 250.0,
            emissive_color: Vec3::new(0.0, 0.0, 0.0),
            transparency: 1.0,
            index_of_refraction: 1.45,
        }
    ];
    let (_, (mats,_)) = parser::materials(input).unwrap();
    assert_eq!(expected_materials, mats);
}
