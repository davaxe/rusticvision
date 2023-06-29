use glam::Vec3A;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub ambient_color: Vec3A,
    pub diffuse_color: Vec3A,
    pub specular_color: Vec3A,
    pub specular_highlight: f32,
    pub emissive_color: Vec3A,
    pub transparency: f32,
    pub index_of_refraction: f32,
}

impl Material {
    pub fn new(
        ambient_color: Vec3A,
        diffuse_color: Vec3A,
        specular_color: Vec3A,
        specular_highlight: f32,
        emissive_color: Vec3A,
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
            Vec3A::new(0.0, 0.0, 0.0),
            Vec3A::new(0.0, 0.0, 0.0),
            Vec3A::new(0.0, 0.0, 0.0),
            0.0,
            Vec3A::new(0.0, 0.0, 0.0),
            0.0,
            1.0,
        )
    }
}