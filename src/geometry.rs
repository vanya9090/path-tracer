use glam::Vec3;
use crate::math::Ray;
use crate::material::Material;
use rand::Rng;

pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub material: Material,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Self {
        Self { v0, v1, v2, material}
    }

    // Moller-Trumbore intersection algorithm
    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let epsilon = 1e-6;

        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;

        let h = ray.direction.cross(e2);
        let a = e1.dot(h);

        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(e1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * e2.dot(q);

        if t > epsilon {
            let point = ray.at(t);
            let mut normal = e1.cross(e2).normalize();
            if ray.direction.dot(normal) > 0.0 {
                normal = -normal;
            }

            return Some(HitRecord { t, point, normal, material: self.material });
        }

        None
    }

    pub fn area(&self) -> f32 {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        e1.cross(e2).length() * 0.5
    }

    // uniform sampling on triangle
    pub fn sample_point(&self) -> Vec3 {
        let mut rng = rand::rng();
        
        let mut r1: f32 = rng.random_range(0.0..1.0);
        let mut r2: f32 = rng.random_range(0.0..1.0);

        if r1 + r2 > 1.0 {
            r1 = 1.0 - r1;
            r2 = 1.0 - r2;
        }

        self.v0 + (self.v1 - self.v0) * r1 + (self.v2 - self.v0) * r2
    }
}
