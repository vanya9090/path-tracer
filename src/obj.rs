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

    let red = Material::diffuse(Vec3::new(0.8, 0.1, 0.1));
    let green = Material::diffuse(Vec3::new(0.1, 0.8, 0.1));
    let white = Material::diffuse(Vec3::new(0.8, 0.8, 0.8));
    
    let mirror = Material::specular();
    let yellow = Material::diffuse(Vec3::new(0.8, 0.7, 0.1));
    let light_wall = Material::light(Vec3::new(1.0, 1.0, 1.0));

    add_quad(&mut scene, Vec3::new(-1.0, -1.0, 1.0), Vec3::new(-1.0, 1.0, 1.0), Vec3::new(-1.0, 1.0, -3.0), Vec3::new(-1.0, -1.0, -3.0), red);
    add_quad(&mut scene, Vec3::new(1.0, -1.0, -3.0), Vec3::new(1.0, 1.0, -3.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, -1.0, 1.0), green);
    add_quad(&mut scene, Vec3::new(-1.0, -1.0, 1.0), Vec3::new(-1.0, -1.0, -3.0), Vec3::new(1.0, -1.0, -3.0), Vec3::new(1.0, -1.0, 1.0), white);
    add_quad(&mut scene, Vec3::new(-1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, -3.0), Vec3::new(-1.0, 1.0, -3.0), white);
    add_quad(&mut scene, Vec3::new(-1.0, -1.0, -3.0), Vec3::new(-1.0, 1.0, -3.0), Vec3::new(1.0, 1.0, -3.0), Vec3::new(1.0, -1.0, -3.0), white);
    add_quad(&mut scene, Vec3::new(1.0, -1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(-1.0, 1.0, 1.0), Vec3::new(-1.0, -1.0, 1.0), mirror);

    add_box(&mut scene, Vec3::new(0.2, -1.0, -2.5), Vec3::new(0.7, 0.2, -1.9), mirror);

    add_pyramid(&mut scene, Vec3::new(-0.5, -1.0, -2.2), 0.7, 0.9, yellow);

    let light_main = Material::light(Vec3::new(15.0, 15.0, 15.0));
    add_quad(&mut scene, 
        Vec3::new(-0.3, 0.99, -1.7), Vec3::new(0.3, 0.99, -1.7), 
        Vec3::new(0.3, 0.99, -2.3), Vec3::new(-0.3, 0.99, -2.3), light_main);

    add_quad(&mut scene, 
        Vec3::new(0.99, -0.2, -1.5), Vec3::new(0.99, 0.2, -1.5), 
        Vec3::new(0.99, 0.2, -1.9), Vec3::new(0.99, -0.2, -1.9), light_wall);

    // let light_floating = Material::light(Vec3::new(1.0, 6.0, 30.0));
    // add_sphere(&mut scene, Vec3::new(0.3, -0.1, -1.0), 0.15, green);

    scene.update_lights();

    scene
}
