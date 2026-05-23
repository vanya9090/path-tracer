use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Vec3,    // Цвет для диффузных поверхностей
    pub emission: Vec3, // Свечение (для источников света)
    pub kd: f32,        // Вероятность диффузного отражения (0.0 - 1.0)
    pub ks: f32,        // Вероятность зеркального отражения (0.0 - 1.0)
}

impl Material {
    pub fn new(color: Vec3, emission: Vec3, kd: f32, ks: f32) -> Self {
        Self { color, emission, kd, ks }
    }

    pub fn diffuse(color: Vec3) -> Self {
        Self::new(color, Vec3::ZERO, 1.0, 0.0)
    }

    pub fn specular() -> Self {
        Self::new(Vec3::new(1.0, 1.0, 1.0), Vec3::ZERO, 0.0, 1.0)
    }

    pub fn light(emission: Vec3) -> Self {
        Self::new(Vec3::ZERO, emission, 0.0, 0.0)
    }
}
