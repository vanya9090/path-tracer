use glam::Vec3;
use crate::math::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    aspect_ratio: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, origin: Vec3) -> Self {
        let mut cam = Self {
            origin,
            lower_left_corner: Vec3::ZERO,
            horizontal: Vec3::ZERO,
            vertical: Vec3::ZERO,
            aspect_ratio,
        };
        cam.update_geometry();
        cam
    }

    pub fn update_geometry(&mut self) {
        let viewport_height = 2.0;
        let viewport_width = self.aspect_ratio * viewport_height;
        let focal_length = 1.0;

        self.horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        self.vertical = Vec3::new(0.0, viewport_height, 0.0);
        
        self.lower_left_corner = self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
