use crate::geometry::{Triangle, HitRecord};
use crate::math::Ray;

pub struct Scene {
    pub triangles: Vec<Triangle>,
}

impl Scene {
    pub fn new() -> Self {
        Self { triangles: Vec::new() }
    }

    pub fn add(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = f32::MAX;

        for triangle in &self.triangles {
            if let Some(hit) = triangle.intersect(ray) {
                if hit.t < closest_t {
                    closest_t = hit.t;
                    closest_hit = Some(hit);
                }
            }
        }

        closest_hit
    }
}
