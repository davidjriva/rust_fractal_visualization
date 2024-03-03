use image::{Rgba, RgbaImage};
use minifb::{Key, Window, WindowOptions};
use std::time::{Instant};

// STATIC VARIABLES
static WIDTH: usize = 1000;
static HEIGHT: usize = 800;
static MAX_ITERATIONS: usize = 1000;

// Based on optimized algorithm: https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set
// Coloring based on sources from the same link ^^^
fn generate_mandelbrot_set_naive() -> RgbaImage{
    let mut imgbuf = RgbaImage::new(WIDTH as u32, HEIGHT as u32);
    // let mut iterations = vec![vec![0; WIDTH]; HEIGHT];

    for pixel_x in 0..WIDTH {
        for pixel_y in 0..HEIGHT {
            // Calculate num iterations for convergence and store (Pass 1)
            let iteration = calculate_num_iterations(pixel_x, pixel_y);


            // Determine color based on # of iterations to converge
            let color = map_iteration_to_color(iteration);
            imgbuf.put_pixel(pixel_x as u32, pixel_y as u32, color);
        }
    }

    // Accumulate the number of iterations per pixel x (flattens columns)
    // let num_iterations_per_pixel = second_pass(iterations.clone());
    // // Aggregate total iterations performed
    // let total = third_pass(&num_iterations_per_pixel);
    // // Determine the hue for each pixel (x,y) and use that for coloring
    // let _pass_4_results = fourth_pass(&iterations, &num_iterations_per_pixel, total);

    imgbuf
}

// Colors from https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
static COLOR_MAPPING: [Rgba<u8>; 16] = [
    Rgba([66, 30, 15, 255]),
    Rgba([25, 7, 26, 255]),
    Rgba([9, 1, 47, 255]),
    Rgba([4, 4, 73, 255]),
    Rgba([0, 7, 100, 255]),
    Rgba([12, 44, 138, 255]),
    Rgba([24, 82, 177, 255]),
    Rgba([57, 125, 209, 255]),
    Rgba([134, 181, 229, 255]),
    Rgba([211, 236, 248, 255]),
    Rgba([241, 233, 191, 255]),
    Rgba([248, 201, 95, 255]),
    Rgba([255, 170, 0, 255]),
    Rgba([204, 128, 0, 255]),
    Rgba([153, 87, 0, 255]),
    Rgba([106, 52, 3, 255]),
];
fn map_iteration_to_color(n: usize) -> Rgba<u8> {
    return if (n < MAX_ITERATIONS && n > 0) {
        let idx = n % COLOR_MAPPING.len();
        COLOR_MAPPING[idx]
    }else{
        Rgba([0,0,0,255])
    }
}

fn second_pass(iterations: Vec<Vec<usize>>) -> Vec<usize> {
    let mut num_iterations_per_pixel = vec![0; MAX_ITERATIONS];

    for pixel_x in 0..HEIGHT {
        for pixel_y in 0..WIDTH {
            let idx = iterations[pixel_x][pixel_y];
            num_iterations_per_pixel[idx] += 1;
        }
    }

    num_iterations_per_pixel
}

fn third_pass(num_iterations_per_pixel: &Vec<usize>) -> usize {
    let mut total = 0;

    for item in num_iterations_per_pixel {
        total += item;
    }

    total
}

fn fourth_pass(iterations: &Vec<Vec<usize>>, num_iterations_per_pixel: &Vec<usize>, total: usize) {
    let mut hue = vec![vec![0; WIDTH]; HEIGHT];

    for x in 0..HEIGHT{
        for y in 0..WIDTH{
            let iteration = iterations[x][y];
            for i in 0..=iteration{
                hue[x][y] += num_iterations_per_pixel[i] / total;
            }
        }
    }

    // The computed value is used as an index to a color palette
}

fn calculate_num_iterations(x: usize, y: usize) -> usize {
    let x_0 = scale_coordinate(x, WIDTH, -2.0, 0.47);
    let y_0 = scale_coordinate(y, HEIGHT, -1.12, 1.12);

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

    iteration
}

fn scale_coordinate(pixel_coordinate: usize, image_dimension: usize, min_val: f64, max_val: f64) -> f64 {
    (pixel_coordinate as f64 / image_dimension as f64) * (max_val - min_val) + min_val
}

fn main() {
    let x_pixel_upper_bound = 1000;
    let y_pixel_upper_bound = 800;

    let start_time = Instant::now();
    let mandelbrot_image = generate_mandelbrot_set_naive();
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