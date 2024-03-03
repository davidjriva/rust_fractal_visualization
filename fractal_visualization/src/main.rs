use image::{Rgba, RgbaImage};
use minifb::{Key, Window, WindowOptions};
use std::time::{Instant};

// Based on optimized algorithm: https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set
fn generate_mandelbrot_set_naive(x_pixel_upper_bound: usize, y_pixel_upper_bound: usize) -> RgbaImage{
    const MAX_ITERATIONS: u16 = 1000;

    let mut imgbuf = RgbaImage::new(x_pixel_upper_bound as u32, y_pixel_upper_bound as u32);

    for pixel_x in 0..x_pixel_upper_bound {
        for pixel_y in 0..y_pixel_upper_bound {
            let x_0 = scale_coordinate(pixel_x, x_pixel_upper_bound, -2.0, 0.47);
            let y_0 = scale_coordinate(pixel_y, y_pixel_upper_bound, -1.12, 1.12);

            let mut x = 0.0;
            let mut y = 0.0;
            let mut x_squared = 0.0;
            let mut y_squared = 0.0;

            let mut iteration = 0;

            while x_squared + y_squared <= 4.0  && iteration < MAX_ITERATIONS {
                y = 2.0*x*y + y_0;
                x = x_squared - y_squared + x_0;
                x_squared = x*x;
                y_squared = y*y;
                iteration += 1;
            }

            // Determine color based on # of iterations to converge
            let color_intensity = (iteration as f64).powf(1.5);
            let color = Rgba([0, color_intensity as u8, 0, 255]);

            imgbuf.put_pixel(pixel_x as u32, pixel_y as u32, color);
        }
    }

    imgbuf
}

fn scale_coordinate(pixel_coordinate: usize, image_dimension: usize, min_val: f64, max_val: f64) -> f64 {
    (pixel_coordinate as f64 / image_dimension as f64) * (max_val - min_val) + min_val
}

fn main() {
    let x_pixel_upper_bound = 1000;
    let y_pixel_upper_bound = 800;

    let start_time = Instant::now();
    let mandelbrot_image = generate_mandelbrot_set_naive(x_pixel_upper_bound, y_pixel_upper_bound);
    let end_time = Instant::now();

    // Calculate elapsed time
    let elapsed_time = end_time - start_time;
    let elapsed_ms = elapsed_time.as_secs() * 1000 + u64::from(elapsed_time.subsec_millis());
    println!("Elapsed time in milliseconds: {} ms", elapsed_ms);

    let mut window = Window::new(
        "Mandelbrot Visualization",
        x_pixel_upper_bound,
        y_pixel_upper_bound,
        WindowOptions::default()
    ).unwrap_or_else(|e| panic!("{}", e));

    let buffer: Vec<u32> = mandelbrot_image
        .pixels()
        .map(|pixel| pixel[0] as u32 | (pixel[1] as u32) << 8 | (pixel[2] as u32) << 16 | (pixel[3] as u32) << 24)
        .collect();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, x_pixel_upper_bound, y_pixel_upper_bound).unwrap();
    }
}