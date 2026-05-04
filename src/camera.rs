use glam::Vec3;
use crate::math::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        // Задаем размеры виртуального экрана (viewport) перед камерой
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0; // Расстояние от камеры до экрана

        let origin = Vec3::ZERO; // Камера находится в центре координат (0, 0, 0)
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        
        // Находим левый нижний угол экрана, чтобы от него отсчитывать пиксели
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    // Получить луч для пикселя с координатами u, v (от 0.0 до 1.0)
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
