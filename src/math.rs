use glam::Vec3;
use rand::Rng;
use std::f32::consts::PI;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    // P(t) = A + t * b
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

// Создание локальной системы координат вокруг нормали
pub fn create_coordinate_system(n: Vec3) -> (Vec3, Vec3, Vec3) {
    let nt = if n.x.abs() > n.y.abs() {
        Vec3::new(n.z, 0.0, -n.x) / (n.x * n.x + n.z * n.z).sqrt()
    } else {
        Vec3::new(0.0, -n.z, n.y) / (n.y * n.y + n.z * n.z).sqrt()
    };
    let nb = n.cross(nt);
    (nt, nb, n)
}

// Генерация случайного направления с косинусным распределением
pub fn random_cosine_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let r1: f32 = rng.gen_range(0.0..1.0);
    let r2: f32 = rng.gen_range(0.0..1.0);

    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z) // z - это направление "вверх" в локальных координатах
}
