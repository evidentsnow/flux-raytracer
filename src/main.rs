use std::{fs::File, io::Write};

use crate::{
    geometry::{Color, Point2, Point3, Vec2, Vec3},
    ray::Ray,
};

pub mod geometry;
pub mod ray;
pub mod transformations;

// struct Canvas {
//     width: usize,
//     height: usize,
//     pixels: Vec<Color>,
// }

// impl Canvas {
//     fn new(width: usize, height: usize) -> Canvas {
//         Canvas {
//             width,
//             height,
//             pixels: vec![Color::black(); width * height],
//         }
//     }
// }

// check if light ray hits sphere
fn hit_sphere(sphere_center: Point3, sphere_radius: f64, ray: Ray) -> bool {}

fn main() {
    // Camera Setup

    // 16/9 aspect ratio = 1.7778
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width / aspect_ratio.floor() as i32);

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height) as f32;
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    // distance from camera to viewport
    let focal_length = 1.0;

    // Viewport vectors
    let viewport_u = Vec2::new(viewport_width.into(), 0.0);
    let viewport_v = Vec2::new(0.0, viewport_height.into());

    // Pixel deltas (distance from pixel to pixel)
    let pixel_delta_u = viewport_u.x / image_width as f64;
    let pixel_delta_v = viewport_v.y / image_height as f64;

    let mut output = String::from(format!("P3\n{} {}\n255\n", image_width, image_height));

    for i in 0..image_height {
        println!("Scanlines remaining: {} ", image_height - i);

        for j in 0..image_width {
            let pixel_center = Point2::new(
                camera_center.x + (i as f64 * pixel_delta_u),
                camera_center.y + (j as f64 * pixel_delta_v),
            );

            let ray_direction = Vec3::new(
                pixel_center.x - camera_center.x,
                pixel_center.y - camera_center.y,
                focal_length - camera_center.z,
            );

            let ray = Ray::new(camera_center, ray_direction);
            // white if nothing is there
            let pixel_color = Color::new(1.0, 1.0, 1.0);

            // push pixel color to output
            output.push_str(Color::write_color(pixel_color).as_str());
        }
    }

    let mut file = File::create("output.ppm").unwrap();
    file.write_all(output.as_bytes());
}
