mod math;
mod camera;
mod geometry;
mod material;
mod scene;
mod obj;

use glam::Vec3;
use minifb::{Key, Window, WindowOptions};
use math::Ray;
use camera::Camera;
use scene::Scene;
use rand::Rng;
use std::fs::File;
use std::io::Write;
use rayon::prelude::*;
use obj::build_scene;


const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const SAMPLES: u32 = 10;




fn ray_color(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth >= 50 {
        return Vec3::ZERO;
    }

    if let Some(hit) = scene.intersect(ray) {
        if hit.material.emission.length_squared() > 0.0 {
            return hit.material.emission;
        }

        let mut final_color = Vec3::ZERO;

        // compute probabilty of every triangle light
        // proba = intensity * area * cos theta / r^2
        let mut total_weight = 0.0;
        let mut weights = Vec::with_capacity(scene.lights_ids.len());

        for &light_id in &scene.lights_ids{
            let light = &scene.triangles[light_id];

            let light_center = (light.v0 + light.v1 + light.v2) / 3.0;
            let dir_to_light = light_center - hit.point;
            let dist_sq = dir_to_light.length_squared().max(0.01);
            let dir_to_light_norm = dir_to_light.normalize();

            let e1 = light.v1 - light.v0;
            let e2 = light.v2 - light.v0;
            let light_normal = e1.cross(e2).normalize();

            let cos_theta_y = light_normal.dot(-dir_to_light_norm).max(0.0);
            let base_power = light.material.emission.length() * light.area();
            let weight = (base_power * cos_theta_y) / dist_sq;
            
            weights.push(weight);
            total_weight += weight;
        }

        let mut rng = rand::rng(); 
        let random_val: f32 = rng.random_range(0.0..total_weight);
        
        let mut current_sum = 0.0;
        let mut selected_idx = scene.lights_ids.len() - 1; 

        for (i, &weight) in weights.iter().enumerate() {
            current_sum += weight;
            if random_val < current_sum {
                selected_idx = i;
                break;
            }
        }

        let light_tri = &scene.triangles[scene.lights_ids[selected_idx]];
        let light_prob = weights[selected_idx] / total_weight;


        let light_point = light_tri.sample_point();
        let dir_to_light = light_point - hit.point;
        let distance_to_light = dir_to_light.length();
        let dir_to_light_norm = dir_to_light / distance_to_light;

        let shadow_ray = Ray::new(hit.point + hit.normal * 0.001, dir_to_light_norm);

        if let Some(shadow_hit) = scene.intersect(&shadow_ray) {
            if shadow_hit.t >= distance_to_light - 0.01 {
                let cos_theta = hit.normal.dot(dir_to_light_norm).max(0.0);
                
                let e1 = light_tri.v1 - light_tri.v0;
                let e2 = light_tri.v2 - light_tri.v0;
                let light_normal = e1.cross(e2).normalize(); 
                
                let cos_theta_prime = light_normal.dot(-dir_to_light_norm).max(0.0);

                if cos_theta_prime > 0.0 {
                    let geometry_term = (cos_theta * cos_theta_prime) / (distance_to_light * distance_to_light);
                    let brdf = hit.material.color / std::f32::consts::PI;

                    final_color += (brdf * light_tri.material.emission * geometry_term * light_tri.area()) / light_prob;
                }
            }
        }

        let mut rng = rand::rng();

        let max_color = hit.material.color.x.max(hit.material.color.y).max(hit.material.color.z);
        let survival_prob = max_color.clamp(0.1, 0.95);

        let is_survived = if depth > 2 {
            rng.random_range(0.0..1.0) <= survival_prob
        } else {
            true
        };

        if !is_survived {
            return final_color;
        }

        let rr_factor = if depth > 2 { 1.0 / survival_prob } else { 1.0 };


        let prob: f32 = rng.random_range(0.0..1.0);

        if prob < hit.material.kd {
            let (nt, nb, n) = math::create_coordinate_system(hit.normal);
            let local_dir = math::random_cosine_direction();
            let world_dir = nt * local_dir.x + nb * local_dir.y + n * local_dir.z;
            
            let bounce_ray = Ray::new(hit.point + hit.normal * 0.001, world_dir.normalize());
            let indirect_color = ray_color(&bounce_ray, scene, depth + 1);
            
            final_color += hit.material.color * indirect_color * rr_factor;

        } else if prob < hit.material.kd + hit.material.ks {
            let dot = ray.direction.dot(hit.normal);
            let reflect_dir = ray.direction - hit.normal * (2.0 * dot);
            
            let bounce_ray = Ray::new(hit.point + hit.normal * 0.001, reflect_dir.normalize());
            let indirect_color = ray_color(&bounce_ray, scene, depth + 1);
            
            final_color += hit.material.color * indirect_color * rr_factor;
        }

        return final_color;
    }

    Vec3::ZERO
}

fn to_u32_color(color: Vec3) -> u32 {
    let r = (color.x.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (color.y.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (color.z.clamp(0.0, 1.0) * 255.0) as u32;
    (r << 16) | (g << 8) | b
}

fn save_image_ppm(buffer: &[u32], width: usize, height: usize, filename: &str) {
    let mut file = File::create(filename).expect("can't create file");
    write!(file, "P3\n{} {}\n255\n", width, height).unwrap();
    
    for &pixel in buffer {
        let r = (pixel >> 16) & 255;
        let g = (pixel >> 8) & 255;
        let b = pixel & 255;
        writeln!(file, "{} {} {}", r, g, b).unwrap();
    }
    println!("saved in {}", filename);
}

fn main() {
    let mut window = Window::new(
        "Path Tracer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).expect("can't create new window");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera::new(aspect_ratio, Vec3::new(0.0, 0.0, 0.0));
    let scene = build_scene();

    println!("start, samples per pixel: {}", SAMPLES);

    let inv_gamma = 1.0 / 2.2f32;

    buffer.par_chunks_mut(WIDTH)
        .enumerate()
        .for_each(|(y, buffer_row)| {
            let mut rng = rand::rng(); 

            for x in 0..WIDTH {
                let mut accum_color = Vec3::ZERO;

                for _ in 0..SAMPLES{
                    let random_u: f32 = rng.random_range(0.0..1.0);
                    let random_v: f32 = rng.random_range(0.0..1.0);
                    let u = (x as f32 + random_u) / (WIDTH - 1) as f32;
                    let v = ((HEIGHT - 1 - y) as f32 + random_v) / (HEIGHT - 1) as f32;

                    let ray = camera.get_ray(u, v);
                    
                    if !scene.lights_ids.is_empty() {
                        accum_color += ray_color(&ray, &scene, 0);
                    }
                }
                
                let mut final_color = accum_color / (SAMPLES as f32);

                // gamma and clipping 
                final_color.x = final_color.x.min(1.0).powf(inv_gamma);
                final_color.y = final_color.y.min(1.0).powf(inv_gamma);
                final_color.z = final_color.z.min(1.0).powf(inv_gamma);

                buffer_row[x] = to_u32_color(final_color);
            }
        });


    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
