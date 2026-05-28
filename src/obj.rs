use glam::Vec3;
use crate::material::Material;
use crate::scene::Scene;
use crate::geometry::Triangle;


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

pub fn build_scene() -> Scene {
    let mut scene = Scene::new();

    let mat_floor = Material::diffuse(Vec3::new(0.12, 0.12, 0.15));
    let mat_shoji_white = Material::diffuse(Vec3::new(0.85, 0.85, 0.85));

    let mat_sakura = Material::diffuse(Vec3::new(0.9, 0.55, 0.65));
    let mat_bamboo = Material::diffuse(Vec3::new(0.3, 0.6, 0.4));  
    let mat_mirror = Material::specular();                         

    let warm_lantern = Material::light(Vec3::new(18.0, 10.0, 4.0));
    let cool_moonlight = Material::light(Vec3::new(1.5, 2.5, 5.0));

    let x_min = -1.5; let x_max = 1.5;
    let y_min = -1.0; let y_max = 1.5;
    let z_min = -4.0; let z_max = 1.0;

    add_quad(&mut scene, Vec3::new(x_min, y_min, z_max), Vec3::new(x_min, y_min, z_min), Vec3::new(x_max, y_min, z_min), Vec3::new(x_max, y_min, z_max), mat_floor);
    add_quad(&mut scene, Vec3::new(x_min, y_max, z_max), Vec3::new(x_max, y_max, z_max), Vec3::new(x_max, y_max, z_min), Vec3::new(x_min, y_max, z_min), mat_shoji_white);

    add_quad(&mut scene, Vec3::new(x_min, y_min, z_min), Vec3::new(x_min, y_max, z_min), Vec3::new(x_max, y_max, z_min), Vec3::new(x_max, y_min, z_min), mat_shoji_white);
    add_quad(&mut scene, Vec3::new(x_min, y_min, z_max), Vec3::new(x_min, y_max, z_max), Vec3::new(x_min, y_max, z_min), Vec3::new(x_min, y_min, z_min), mat_sakura);
    add_quad(&mut scene, Vec3::new(x_max, y_min, z_min), Vec3::new(x_max, y_max, z_min), Vec3::new(x_max, y_max, z_max), Vec3::new(x_max, y_min, z_max), mat_bamboo);
    
    add_quad(&mut scene, Vec3::new(x_max, y_min, z_max), Vec3::new(x_max, y_max, z_max), Vec3::new(x_min, y_max, z_max), Vec3::new(x_min, y_min, z_max), mat_shoji_white);

    add_box(&mut scene, Vec3::new(-0.5, y_min, -2.8), Vec3::new(0.5, 0.4, -2.1), mat_mirror);

    add_sphere(&mut scene, Vec3::new(-0.65, 0.7, -2.45), 0.15, warm_lantern);

    add_sphere(&mut scene, Vec3::new(0.2, -0.6, -1.5), 0.4, mat_shoji_white);

    add_pyramid(&mut scene, Vec3::new(0.8, y_min, -3.0), 0.8, 1.2, mat_shoji_white);

    add_quad(&mut scene,
        Vec3::new(0.3, y_max - 0.01, -2.0), Vec3::new(1.0, y_max - 0.01, -2.0),
        Vec3::new(1.0, y_max - 0.01, -2.7), Vec3::new(0.3, y_max - 0.01, -2.7), warm_lantern);

    add_quad(&mut scene,
        Vec3::new(x_min + 0.01, -0.2, -1.0), Vec3::new(x_min + 0.01, 0.8, -1.0),
        Vec3::new(x_min + 0.01, 0.8, -2.5), Vec3::new(x_min + 0.01, -0.2, -2.5), cool_moonlight);

    scene.update_lights();
    scene
}
