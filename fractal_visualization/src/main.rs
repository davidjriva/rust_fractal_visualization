mod mandelbrot;

use std::time::{Instant};
use mandelbrot::{generate_mandelbrot_set};
use mandelbrot::gui;

fn main() {
    let x_pixel_upper_bound = 1000;
    let y_pixel_upper_bound = 800;

    let start_time = Instant::now();
    let mandelbrot_image = generate_mandelbrot_set(x_pixel_upper_bound, y_pixel_upper_bound);
    let end_time = Instant::now();

    // Calculate elapsed time
    let elapsed_time = end_time - start_time;
    let elapsed_ms = elapsed_time.as_secs() * 1000 + u64::from(elapsed_time.subsec_millis());
    println!("Elapsed time in milliseconds: {} ms", elapsed_ms);

    let mut gui = gui::Gui::new(x_pixel_upper_bound, y_pixel_upper_bound);

    gui.set_buffer(mandelbrot_image
        .pixels()
        .map(|pixel| pixel[0] as u32 | (pixel[1] as u32) << 8 | (pixel[2] as u32) << 16 | (pixel[3] as u32) << 24)
        .collect()
    );

    while gui.window_is_open() && !gui.escape_key_pressed() {
        gui.update_window();
    }
}