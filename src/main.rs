use crate::geometry::{Point2, Point3, Vec2, Vec3};

pub mod geometry;
pub mod ray;
pub mod transformations;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

#[derive(Clone)]
struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    fn black() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    fn hadamard_product(c1: Color, c2: Color) -> Color {
        Color {
            red: c1.red * c2.red,
            green: c1.green * c2.green,
            blue: c1.blue * c2.blue,
        }
    }
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }
}

fn main() {
    // Camera Setup

    // 16/9 aspect ratio = 1.7778
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width / aspect_ratio.floor() as i32);

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height) as f32;
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    let focal_length = 1.0;

    // Viewport vectors
    let viewport_u = Vec2::new(viewport_width.into(), 0.0);
    let viewport_v = Vec2::new(0.0, viewport_height.into());

    // Pixel deltas (distance from pixel to pixel)
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    for i in 0..image_height {
        for j in 0..image_width {
            let pixel_center = Point2::new(
                camera_center.x + (i.into() * pixel_delta_u),
                camera_center.y + (j.into() * pixel_delta_v),
            );

            let ray_direction = Vec3::new(
                pixel_center.x - camera_center.x,
                pixel_center.y - camera_center.y,
                focal_length - camera_center.z,
            );
        }
    }
}
