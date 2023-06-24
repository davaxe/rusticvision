pub mod parser;

use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub ambient_color: Vec3,
    pub diffuse_color: Vec3,
    pub specular_color: Vec3,
    pub specular_highlight: f32,
    pub emissive_color: Vec3,
    pub transparency: f32,
    pub index_of_refraction: f32,
}

impl Material {
    pub fn new(
        ambient_color: Vec3,
        diffuse_color: Vec3,
        specular_color: Vec3,
        specular_highlight: f32,
        emissive_color: Vec3,
        transparency: f32,
        index_of_refraction: f32,
    ) -> Self {
        Self {
            ambient_color,
            diffuse_color,
            specular_color,
            specular_highlight,
            emissive_color,
            transparency,
            index_of_refraction,
        }
    }

    

}

impl Default for Material {
    fn default() -> Self {
        Self::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            1.0,
        )
    }
}

#[cfg(test)]
mod parser_tests;
