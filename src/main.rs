mod math;
mod camera;
mod geometry;
mod material;
mod scene;

use glam::Vec3;
use minifb::{Key, Window, WindowOptions};
use math::Ray;
use camera::Camera;
use geometry::Triangle;
use material::Material;
use scene::Scene;
use rand::Rng;
use std::fs::File;
use std::io::Write;
use rayon::prelude::*;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

// Вспомогательная функция для добавления прямоугольника (из 2 треугольников)
fn add_quad(scene: &mut Scene, v0: Vec3, v1: Vec3, v2: Vec3, v3: Vec3, mat: Material) {
    scene.add(Triangle::new(v0, v1, v2, mat));
    scene.add(Triangle::new(v0, v2, v3, mat));
}

fn add_pyramid(scene: &mut Scene, center: Vec3, size: f32, height: f32, mat: Material) {
    let half = size / 2.0;
    let base_y = center.y;
    
    let v0 = Vec3::new(center.x - half, base_y, center.z - half);
    let v1 = Vec3::new(center.x + half, base_y, center.z - half);
    let v2 = Vec3::new(center.x + half, base_y, center.z + half);
    let v3 = Vec3::new(center.x - half, base_y, center.z + half);
    let apex = Vec3::new(center.x, base_y + height, center.z);

    scene.add(Triangle::new(v0, v2, v1, mat));
    scene.add(Triangle::new(v0, v3, v2, mat));

    scene.add(Triangle::new(v0, v1, apex, mat));
    scene.add(Triangle::new(v1, v2, apex, mat));
    scene.add(Triangle::new(v2, v3, apex, mat));
    scene.add(Triangle::new(v3, v0, apex, mat));
}

fn add_sphere(scene: &mut Scene, center: Vec3, radius: f32, mat: Material) {
    let lat_steps = 8;
    let lon_steps = 8;

    for i in 0..lat_steps {
        let lat0 = std::f32::consts::PI * (-0.5 + (i as f32) / lat_steps as f32);
        let y0 = radius * lat0.sin();
        let r0 = radius * lat0.cos();

        let lat1 = std::f32::consts::PI * (-0.5 + ((i + 1) as f32) / lat_steps as f32);
        let y1 = radius * lat1.sin();
        let r1 = radius * lat1.cos();

        for j in 0..lon_steps {
            let lon0 = 2.0 * std::f32::consts::PI * (j as f32) / lon_steps as f32;
            let lon1 = 2.0 * std::f32::consts::PI * ((j + 1) as f32) / lon_steps as f32;

            let v00 = center + Vec3::new(r0 * lon0.cos(), y0, r0 * lon0.sin());
            let v10 = center + Vec3::new(r0 * lon1.cos(), y0, r0 * lon1.sin());
            let v01 = center + Vec3::new(r1 * lon0.cos(), y1, r1 * lon0.sin());
            let v11 = center + Vec3::new(r1 * lon1.cos(), y1, r1 * lon1.sin());

            scene.add(Triangle::new(v00, v10, v01, mat));
            scene.add(Triangle::new(v10, v11, v01, mat));
        }
    }
}

fn add_box(scene: &mut Scene, min: Vec3, max: Vec3, mat: Material) {
    add_quad(scene, Vec3::new(min.x, min.y, max.z), Vec3::new(min.x, max.y, max.z), Vec3::new(max.x, max.y, max.z), Vec3::new(max.x, min.y, max.z), mat);
    add_quad(scene, Vec3::new(max.x, min.y, min.z), Vec3::new(max.x, max.y, min.z), Vec3::new(min.x, max.y, min.z), Vec3::new(min.x, min.y, min.z), mat);
    add_quad(scene, Vec3::new(min.x, min.y, min.z), Vec3::new(min.x, max.y, min.z), Vec3::new(min.x, max.y, max.z), Vec3::new(min.x, min.y, max.z), mat);
    add_quad(scene, Vec3::new(max.x, min.y, max.z), Vec3::new(max.x, max.y, max.z), Vec3::new(max.x, max.y, min.z), Vec3::new(max.x, min.y, min.z), mat);
    add_quad(scene, Vec3::new(min.x, max.y, max.z), Vec3::new(min.x, max.y, min.z), Vec3::new(max.x, max.y, min.z), Vec3::new(max.x, max.y, max.z), mat);
}

// Построение Корнеллской коробки
fn build_scene() -> Scene {
    let mut scene = Scene::new();

    // Базовые материалы комнаты
    let red = Material::diffuse(Vec3::new(0.8, 0.1, 0.1));
    let green = Material::diffuse(Vec3::new(0.1, 0.8, 0.1));
    let white = Material::diffuse(Vec3::new(0.8, 0.8, 0.8));
    
    // Материалы объектов
    let mirror = Material::specular();
    let yellow = Material::diffuse(Vec3::new(0.8, 0.7, 0.1));
    let magenta = Material::diffuse(Vec3::new(0.7, 0.1, 0.7));

    // =====================================
    // СТЕНЫ КОМНАТЫ
    // =====================================
    add_quad(&mut scene, Vec3::new(-1.0, -1.0, 1.0), Vec3::new(-1.0, 1.0, 1.0), Vec3::new(-1.0, 1.0, -3.0), Vec3::new(-1.0, -1.0, -3.0), red); // Левая
    add_quad(&mut scene, Vec3::new(1.0, -1.0, -3.0), Vec3::new(1.0, 1.0, -3.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, -1.0, 1.0), green); // Правая
    add_quad(&mut scene, Vec3::new(-1.0, -1.0, 1.0), Vec3::new(-1.0, -1.0, -3.0), Vec3::new(1.0, -1.0, -3.0), Vec3::new(1.0, -1.0, 1.0), white); // Пол
    add_quad(&mut scene, Vec3::new(-1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, -3.0), Vec3::new(-1.0, 1.0, -3.0), white); // Потолок
    add_quad(&mut scene, Vec3::new(-1.0, -1.0, -3.0), Vec3::new(-1.0, 1.0, -3.0), Vec3::new(1.0, 1.0, -3.0), Vec3::new(1.0, -1.0, -3.0), white); // Задняя
    add_quad(&mut scene, Vec3::new(1.0, -1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(-1.0, 1.0, 1.0), Vec3::new(-1.0, -1.0, 1.0), white); // Передняя

    // =====================================
    // ОБЪЕКТЫ
    // =====================================
    // 1. Высокий зеркальный блок справа (отражает всю комнату)
    add_box(&mut scene, Vec3::new(0.2, -1.0, -2.5), Vec3::new(0.7, 0.2, -1.9), mirror);

    // 2. Желтая пирамида слева на заднем плане
    add_pyramid(&mut scene, Vec3::new(-0.5, -1.0, -2.2), 0.7, 0.9, yellow);

    // 3. Пурпурная полигональная сфера на переднем плане
    // add_sphere(&mut scene, Vec3::new(-0.3, -0.65, -1.2), 0.35, magenta);

    // =====================================
    // ИСТОЧНИКИ СВЕТА
    // =====================================
    // 1. Основная яркая лампа на потолке
    let light_main = Material::light(Vec3::new(2.0, 2.0, 2.0));
    add_quad(&mut scene, 
        Vec3::new(-0.3, 0.99, -1.7), Vec3::new(0.3, 0.99, -1.7), 
        Vec3::new(0.3, 0.99, -2.3), Vec3::new(-0.3, 0.99, -2.3), light_main);

    // 2. Лампа на правой зеленой стене (теплый свет)
    let light_wall = Material::light(Vec3::new(1.0, 1.0, 1.0));
    add_quad(&mut scene, 
        Vec3::new(0.99, -0.2, -1.5), Vec3::new(0.99, 0.2, -1.5), 
        Vec3::new(0.99, 0.2, -1.9), Vec3::new(0.99, -0.2, -1.9), light_wall);

    // 3. ПАРЯЩАЯ голубая сфера-лампочка (холодный неоновый свет)
    // Будет висеть в воздухе между камерой и остальными объектами
    let light_floating = Material::light(Vec3::new(0.5, 3.0, 15.0));
    add_sphere(&mut scene, Vec3::new(0.3, -0.1, -1.0), 0.15, light_floating);

    scene
}

// Оставляем параметр depth
fn ray_color(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth >= 50 {
        return Vec3::ZERO;
    }

    if let Some(hit) = scene.intersect(ray) {
        if hit.material.emission.length_squared() > 0.0 {
            return hit.material.emission;
        }

        let mut final_color = Vec3::ZERO;

        // ==========================================
        // 1. ПРЯМОЕ ОСВЕЩЕНИЕ (Direct Illumination)
        // ==========================================
        let lights: Vec<&Triangle> = scene.triangles.iter()
            .filter(|t| t.material.emission.length_squared() > 0.0)
            .collect();

        if !lights.is_empty() {
            // ШАГ 1: Вычисляем "мощность" каждого треугольника-источника.
            // Мощность = яркость (длина вектора emission) * площадь треугольника
            let mut total_power = 0.0;
            let mut powers = Vec::with_capacity(lights.len());

            for light in &lights {
                let power = light.material.emission.length() * light.area();
                powers.push(power);
                total_power += power;
            }

            // ШАГ 2: Выборка по значимости (Importance Sampling)
            // Бросаем случайную величину от 0 до суммарной мощности
            let mut rng = rand::rng(); // или SmallRng::from_entropy(), если используете его
            let random_val: f32 = rng.random_range(0.0..total_power);
            
            let mut current_sum = 0.0;
            let mut selected_idx = 0;

            // Находим, в какой "отрезок мощности" попало наше случайное число
            for (i, &power) in powers.iter().enumerate() {
                current_sum += power;
                if random_val <= current_sum {
                    selected_idx = i;
                    break;
                }
            }

            let light_tri = lights[selected_idx];
            
            // ВАЖНО: Вероятность того, что мы выбрали именно этот треугольник
            let light_prob = powers[selected_idx] / total_power;

            // ШАГ 3: Расчет освещения от выбранного источника
            let light_point = light_tri.sample_point();
            let dir_to_light = light_point - hit.point;
            let distance_to_light = dir_to_light.length();
            let dir_to_light_norm = dir_to_light / distance_to_light;

            let shadow_ray = Ray::new(hit.point + hit.normal * 0.001, dir_to_light_norm);

            if let Some(shadow_hit) = scene.intersect(&shadow_ray) {
                if shadow_hit.t >= distance_to_light - 0.01 {
                    let cos_theta = hit.normal.dot(dir_to_light_norm).max(0.0);
                    
                    // Улучшение: Теперь нормаль лампы считается честно из ее вершин,
                    // а не захардкожена. Это позволит ставить лампы на стены!
                    let e1 = light_tri.v1 - light_tri.v0;
                    let e2 = light_tri.v2 - light_tri.v0;
                    let light_normal = e1.cross(e2).normalize(); 
                    
                    // Проверяем, светит ли лампа в нашу сторону (лицевой ли стороной)
                    let cos_theta_prime = light_normal.dot(-dir_to_light_norm).max(0.0);

                    if cos_theta_prime > 0.0 {
                        let geometry_term = (cos_theta * cos_theta_prime) / (distance_to_light * distance_to_light);
                        let brdf = hit.material.color / std::f32::consts::PI;

                        // ФОРМУЛА МОНТЕ-КАРЛО: 
                        // Вклад = (BRDF * Emission * Геометрия * Площадь_лампы) / Вероятность_выбора_лампы
                        final_color += (brdf * light_tri.material.emission * geometry_term * light_tri.area()) / light_prob;
                    }
                }
            }
        }

        // ==========================================
        // 2. РУССКАЯ РУЛЕТКА (Russian Roulette)
        // ==========================================
        let mut rng = rand::rng();

        // Максимальная компонента цвета - это наша вероятность выжить (от 0.0 до 1.0)
        let max_color = hit.material.color.x.max(hit.material.color.y).max(hit.material.color.z);
        // Зажимаем вероятность, чтобы луч всегда имел хотя бы 10% шанс выжить (избегаем деления на 0)
        let survival_prob = max_color.clamp(0.1, 0.95);

        // Чтобы картинка была качественнее, не убиваем лучи на первых 2-х отскоках
        let is_survived = if depth > 2 {
            rng.random_range(0.0..1.0) <= survival_prob
        } else {
            true
        };

        if !is_survived {
            // Луч поглощен! Возвращаем только то, что собрали от прямой лампы
            return final_color;
        }

        // ЗАМЕНА 2: Фактор компенсации. Выжившие лучи должны стать ярче.
        let rr_factor = if depth > 2 { 1.0 / survival_prob } else { 1.0 };

        // ==========================================
        // 3. ГЛОБАЛЬНОЕ ОСВЕЩЕНИЕ (Indirect Illumination)
        // ==========================================
        let prob: f32 = rng.gen_range(0.0..1.0);

        if prob < hit.material.kd {
            let (nt, nb, n) = math::create_coordinate_system(hit.normal);
            let local_dir = math::random_cosine_direction();
            let world_dir = nt * local_dir.x + nb * local_dir.y + n * local_dir.z;
            
            let bounce_ray = Ray::new(hit.point + hit.normal * 0.001, world_dir.normalize());
            let indirect_color = ray_color(&bounce_ray, scene, depth + 1);
            
            // ЗАМЕНА 3: Умножаем на rr_factor
            final_color += hit.material.color * indirect_color * rr_factor;

        } else if prob < hit.material.kd + hit.material.ks {
            let dot = ray.direction.dot(hit.normal);
            let reflect_dir = ray.direction - hit.normal * (2.0 * dot);
            
            let bounce_ray = Ray::new(hit.point + hit.normal * 0.001, reflect_dir.normalize());
            let indirect_color = ray_color(&bounce_ray, scene, depth + 1);
            
            // ЗАМЕНА 3: Умножаем на rr_factor
            final_color += hit.material.color * indirect_color * rr_factor;
        }

        // final_color.x = final_color.x.min(3.0);
        // final_color.y = final_color.y.min(3.0);
        // final_color.z = final_color.z.min(3.0);

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
    let mut file = File::create(filename).expect("Не удалось создать файл");
    write!(file, "P3\n{} {}\n255\n", width, height).unwrap();
    
    for &pixel in buffer {
        let r = (pixel >> 16) & 255;
        let g = (pixel >> 8) & 255;
        let b = pixel & 255;
        writeln!(file, "{} {} {}", r, g, b).unwrap();
    }
    println!("Изображение успешно сохранено в {}", filename);
}

fn main() {
    let mut window = Window::new(
        "Path Tracer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).expect("Не удалось создать окно");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera::new(aspect_ratio);
    let scene = build_scene();

    let samples_per_pixel = 128;
    let gamma = 2.2f32;
    let inv_gamma = 1.0 / gamma;

    println!("рендер: {}x{}, Сэмплов на пиксель: {}", WIDTH, HEIGHT, samples_per_pixel);

    buffer.par_chunks_mut(WIDTH).enumerate().for_each(|(y, row)| {
        let mut rng = rand::thread_rng();
        
        for x in 0..WIDTH {
            let mut pixel_color = Vec3::ZERO;

            for _ in 0..samples_per_pixel {
                let random_u: f32 = rng.gen_range(0.0..1.0);
                let random_v: f32 = rng.gen_range(0.0..1.0);

                let u = (x as f32 + random_u) / (WIDTH - 1) as f32;
                let v = ((HEIGHT - 1 - y) as f32 + random_v) / (HEIGHT - 1) as f32;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &scene, 0);
            }

            pixel_color /= samples_per_pixel as f32;
            
            // 2. ТОНАЛЬНАЯ КОМПРЕССИЯ (Отсечение больше 1.0)
            pixel_color.x = pixel_color.x.min(1.0);
            pixel_color.y = pixel_color.y.min(1.0);
            pixel_color.z = pixel_color.z.min(1.0);

            // 3. ГАММА-КОРРЕКЦИЯ (V_corr = V^(1/gamma))
            pixel_color.x = pixel_color.x.powf(inv_gamma);
            pixel_color.y = pixel_color.y.powf(inv_gamma);
            pixel_color.z = pixel_color.z.powf(inv_gamma);

            row[x] = to_u32_color(pixel_color);
        }
    });

    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

    println!("\rРендер завершен!");

    save_image_ppm(&buffer, WIDTH, HEIGHT, "render_result.ppm");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
