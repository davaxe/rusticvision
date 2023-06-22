use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};

use crate::primitives::Ray;

pub struct Camera {
    projection: Mat4,
    inverse_projection: Mat4,
    view: Mat4,
    inverse_view: Mat4,
    position: Vec3,
    direction: Vec3,
    z_near: f32,
    z_far: f32,
    vertical_fov: f32,
    aspect_ratio: f32,
    width: u32,
    height: u32,
}

impl Camera {
    fn new(
        position: Vec3,
        direction: Vec3,
        z_near: f32,
        z_far: f32,
        vertical_fov: f32,
        width: u32,
        height: u32,
    ) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let projection = Mat4::perspective_rh(vertical_fov.to_radians(), aspect_ratio, z_near, z_far);
        let inverse_projection = projection.inverse();
        let view = Mat4::look_at_rh(position, position + direction, Vec3::Y);
        let inverse_view = view.inverse();
        Self {
            projection,
            inverse_projection,
            view,
            inverse_view,
            position,
            direction,
            z_near,
            z_far,
            vertical_fov,
            aspect_ratio,
            width,
            height,
        }
    }

    pub fn get_camera_rays(&self) -> Vec<Ray> {
        let mut rays = Vec::with_capacity((self.width * self.height) as usize);
        for x in 0..=self.width {
            for y in 0..=self.height {
                rays.push(self.get_ray(x, y));
            }
        }
        rays
    }

    #[inline]
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        let (x, y) = (x as f32, y as f32);
        let (width, height) = (self.width as f32, self.height as f32);
        let coord = Vec2::new(x / width, y / height);
        let coord = coord * 2.0 - Vec2::ONE;
        let target = self.inverse_projection * Vec4::new(coord.x, -coord.y, 1.0, 1.0);
        let target = (target.xyz() / target.w).normalize();
        let ray_dir = (self.inverse_view * Vec4::new(target.x, target.y, target.z, 0.0)).xyz();
        Ray::new(self.position, ray_dir)
    }
}

impl Default for Camera {
    fn default() -> Self {
        CameraBuilder::new().build()
    }
}

pub struct CameraBuilder {
    position: Option<Vec3>,
    direction: Option<Vec3>,
    z_near: Option<f32>,
    z_far: Option<f32>,
    vertical_fov: Option<f32>,
    width: Option<u32>,
    height: Option<u32>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            position: None,
            direction: None,
            z_near: None,
            z_far: None,
            vertical_fov: None,
            width: None,
            height: None,
        }
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = Some(position);
        self
    }

    pub fn with_direction(mut self, direction: Vec3) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn with_z_near(mut self, z_near: f32) -> Self {
        self.z_near = Some(z_near);
        self
    }

    pub fn with_z_far(mut self, z_far: f32) -> Self {
        self.z_far = Some(z_far);
        self
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_vertical_fov(mut self, vertical_fov: f32) -> Self {
        self.vertical_fov = Some(vertical_fov);
        self
    }

    pub fn build(self) -> Camera {
        let position = self.position.unwrap_or(Vec3::ZERO);
        let direction = self.direction.unwrap_or(Vec3::Z);
        let z_near = self.z_near.unwrap_or(0.1);
        let z_far = self.z_far.unwrap_or(100.0);
        let vertical_fov = self.vertical_fov.unwrap_or(60.0);
        let width = self.width.unwrap_or(800);
        let height = self.height.unwrap_or(600);
        Camera::new(
            position,
            direction,
            z_near,
            z_far,
            vertical_fov,
            width,
            height,
        )
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}