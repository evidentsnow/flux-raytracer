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
}
