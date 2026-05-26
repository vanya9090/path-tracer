mod math;
mod camera;
mod geometry;
mod material;
mod scene;
mod obj;
mod postprocessing;

use glam::Vec3;
use math::Ray;
use camera::Camera;
use scene::Scene;
use rand::Rng;
use std::{f32::INFINITY, fs::File};
use std::io::Write;
use rayon::prelude::*;
use obj::build_scene;

use geometry::{Triangle, HitRecord};
use postprocessing::{PostProcessingPipeline,
    Clipping,
    GammaCorrection,
    FrameData};


const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const SAMPLES: u32 = 128;
const EPS: f32 = 0.001;
const MAX_DEPTH: u32 = 50;
const FILENAME: &str = "image.ppm";


fn select_light(scene: &Scene) -> (f32, &Triangle){
    let mut rng = rand::rng(); 
    let random_val: f32 = rng.random_range(0.0..scene.total_light_weight);
    
    let selected_idx = scene.lights_cdf.partition_point(|&cdf| cdf < random_val);
    let selected_idx = selected_idx.min(scene.lights_ids.len() - 1);
    
    let actual_triangle_id = scene.lights_ids[selected_idx];
    let light_tri = &scene.triangles[actual_triangle_id];
    
    let weight = if selected_idx == 0 {
        scene.lights_cdf[0]
    } else {
        scene.lights_cdf[selected_idx] - scene.lights_cdf[selected_idx - 1]
    };
    let light_prob = weight / scene.total_light_weight;

    (light_prob, light_tri)
}


fn comute_direct_lighting(hit: &HitRecord, scene: &Scene) -> Vec3 {
    let (light_prob, light_tri) = select_light(scene);

    let light_point = light_tri.sample_point();
    let dir_to_light = light_point - hit.point;
    let distance_to_light = dir_to_light.length();
    let dir_to_light_norm = dir_to_light / distance_to_light;

    let shadow_ray = Ray::new(hit.point + hit.normal * EPS, dir_to_light_norm);

    if let Some(shadow_hit) = scene.intersect(&shadow_ray) {
        if shadow_hit.t >= distance_to_light - EPS {
            let cos_theta = hit.normal.dot(dir_to_light_norm).max(0.0);
            
            let e1 = light_tri.v1 - light_tri.v0;
            let e2 = light_tri.v2 - light_tri.v0;
            let light_normal = e1.cross(e2).normalize(); 
            
            let cos_theta_prime = light_normal.dot(-dir_to_light_norm).max(0.0);

            if cos_theta_prime > 0.0 {
                let geometry_term = (cos_theta * cos_theta_prime) / (distance_to_light * distance_to_light);
                let brdf = hit.material.color / std::f32::consts::PI;

                return (brdf * light_tri.material.emission * geometry_term * light_tri.area()) / light_prob;
            }
        }
    }
    Vec3::ZERO
}


fn russian_roulette(color: Vec3, depth: u32) -> Option<f32> {
    if depth <= 2 {
        return Some(1.0);
    }
    
    let mut rng = rand::rng();
    let max_color = color.x.max(color.y).max(color.z);
    let survival_prob = max_color.clamp(0.1, 0.95);
    
    if rng.random_range(0.0..1.0) <= survival_prob {
        Some(1.0 / survival_prob)
    } else {
        None
    }
}


fn sample_next_ray(ray: &Ray, hit: &HitRecord) -> Option<Ray> {
    let mut rng = rand::rng();
    let prob: f32 = rng.random_range(0.0..1.0);

    let origin = hit.point + hit.normal * EPS;

    if prob < hit.material.kd {
        let (nt, nb, n) = math::create_coordinate_system(hit.normal);
        let local_dir = math::random_cosine_direction();
        let world_dir = nt * local_dir.x + nb * local_dir.y + n * local_dir.z;
        
        Some(Ray::new(origin, world_dir.normalize()))

    } else if prob < hit.material.kd + hit.material.ks {
        let dot = ray.direction.dot(hit.normal);
        let reflect_dir = ray.direction - hit.normal * (2.0 * dot);
        
        Some(Ray::new(origin, reflect_dir.normalize()))
    } else {
        None
    }
}


fn ray_color(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth >= MAX_DEPTH {
        return Vec3::ZERO;
    }

    if let Some(hit) = scene.intersect(ray) {
        if hit.material.emission.length_squared() > 0.0 {
            return hit.material.emission;
        }

        let mut final_color = comute_direct_lighting(&hit, &scene);

        let rr_factor = match russian_roulette(hit.material.color, depth) {
            Some(factor) => factor,
            None => return final_color,
        };

        if let Some(bounce_ray) = sample_next_ray(ray, &hit) {
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
    let mut color_buffer: Vec<Vec3> = vec![Vec3::ZERO; WIDTH * HEIGHT];
    let mut depth_buffer: Vec<f32> = vec![INFINITY; WIDTH * HEIGHT];

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera::new(aspect_ratio, Vec3::new(0.0, 0.0, 0.0));
    let scene = build_scene();
    let pipeline = PostProcessingPipeline::new()
        .add_step(Clipping)
        .add_step(GammaCorrection {gamma: 2.2});

    println!("start, samples per pixel: {}", SAMPLES);

    color_buffer.par_chunks_mut(WIDTH)
        .zip(depth_buffer.par_chunks_mut(WIDTH))
        .enumerate()
        .for_each(|(y, (color_row, depth_row))| {
            let mut rng = rand::rng(); 

            for x in 0..WIDTH {
                let mut accum_color = Vec3::ZERO;

                for s in 0..SAMPLES{
                    let random_u: f32 = rng.random_range(0.0..1.0);
                    let random_v: f32 = rng.random_range(0.0..1.0);
                    let u = (x as f32 + random_u) / (WIDTH - 1) as f32;
                    let v = ((HEIGHT - 1 - y) as f32 + random_v) / (HEIGHT - 1) as f32;

                    let ray = camera.get_ray(u, v);
                    
                    if !scene.lights_ids.is_empty() {
                        accum_color += ray_color(&ray, &scene, 0);
                    }

                    if s == 0 {
                        if let Some(hit) = scene.intersect(&ray) {
                            depth_row[x] = hit.t;
                        }
                    }
                }
                
                color_row[x] = accum_color / (SAMPLES as f32);
            }
        });

    let mut frame = FrameData {width: WIDTH,
        height: HEIGHT,
        colors: color_buffer,
        depths: depth_buffer};

    pipeline.execute(&mut frame);

    let mut display_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    display_buffer.par_iter_mut()
        .zip(frame.colors.par_iter())
        .for_each(|(pixel, color)| {
            *pixel = to_u32_color(*color);
        });


    save_image_ppm(&display_buffer, WIDTH, HEIGHT, FILENAME)
}
