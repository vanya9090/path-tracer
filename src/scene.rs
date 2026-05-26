use crate::geometry::{Triangle, HitRecord};
use crate::math::Ray;

pub struct Scene {
    pub triangles: Vec<Triangle>,
    pub lights_ids: Vec<usize>,
    pub lights_cdf: Vec<f32>,
    pub total_light_weight: f32,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            lights_ids: Vec::new(), 
            lights_cdf: Vec::new(),
            total_light_weight: 0.0,
        }
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
    
    pub fn update_lights(&mut self) {
        self.lights_ids.clear();
        self.lights_cdf.clear();
        self.total_light_weight = 0.0;

        for (i, t) in self.triangles.iter().enumerate() {
            let emission = t.material.emission.length();
            if emission > 0.0 {
                self.lights_ids.push(i);
                
                let weight = t.area() * emission; 
                self.total_light_weight += weight;
                self.lights_cdf.push(self.total_light_weight);
            }
        }
    }
}
