use image::{Rgba, RgbaImage};
use crate::mandelbrot::palettes;

// STATIC VARIABLES
static MAX_ITERATIONS: usize = 1000;
// Based on optimized algorithm: https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set
// Coloring based on sources from the same link ^^^
pub fn generate_mandelbrot_set(width: usize, height: usize) -> RgbaImage{
    let mut mandelbrot_img_pixels = vec![vec![(0,0,0,0); width]; height];
    let imgbuf = RgbaImage::new(width as u32, height as u32);

    // Use continuous coloring function to fill in mandelbrot set
    color_mandelbrot_set(imgbuf, width, height)
}

/*
    Utilizes a continuous coloring algorithm to color pixels of mandelbrot set based on a certain palette
 */
struct Point {
    x: f64,
    y: f64,
}

fn color_mandelbrot_set(mut imgbuf: RgbaImage, width: usize, height: usize) -> RgbaImage {
    let reciprocal_of_ln_two = 1.0 / (2.0f64).ln();
    let mult = (0.5f64).ln() * reciprocal_of_ln_two;

    for y in 0..height {
        for x in 0..width {
            let x_scaled = scale_coordinate(x, width, -2.0, 0.47);
            let y_scaled = scale_coordinate(y, height, -1.12, 1.12);

            let c = Point { x: x_scaled, y: y_scaled };
            let (iter_ct, _, z) = compute_escape_time(c, Point{ x: 0.0, y: 0.0});

            let mut color = Rgba([0,0,0,255]);
            if iter_ct < MAX_ITERATIONS {
                color = interpolate_color(reciprocal_of_ln_two, mult, iter_ct, z);
            }

            imgbuf.put_pixel(x as u32, y as u32, color);
        }
    }

    imgbuf
}

/*
    Interpolates colors resulting in a smooth, aesthetically pleasing image.
    Algorithm based on: https://www.fractalforums.com/programming/newbie-how-to-map-colors-in-the-mandelbrot-set/msg3478/#msg3478
    and
    smoothing: https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set
 */
fn interpolate_color(reciprocal_of_ln_two: f64, mult: f64, iter_ct: usize, z: Point) -> Rgba<u8> {
    let iter_frac = 5.0 + iter_ct as f64 - ((z.x + z.y).ln().ln() * reciprocal_of_ln_two) - mult;
    let iter_floor = iter_frac.floor() as usize;
    let iter_percent = iter_frac - iter_floor as f64;

    let start_color = palettes::PALETTE_GR[iter_floor % palettes::PALETTE_GR.len()];
    let end_color = palettes::PALETTE_GR[(iter_floor + 1) % palettes::PALETTE_GR.len()];

    calculate_color(start_color, end_color, iter_percent)
}

fn calculate_color(start_color: Rgba<u8>, end_color: Rgba<u8>, iter_p: f64) -> Rgba<u8> {
    let red = (end_color[0] as f64 - start_color[0] as f64) * iter_p + start_color[0] as f64;
    let green = (end_color[1] as f64 - start_color[1] as f64) * iter_p + start_color[1] as f64;
    let blue = (end_color[2] as f64 - start_color[2] as f64) * iter_p + start_color[2] as f64;

    Rgba([red.floor() as u8, green.floor() as u8, blue.floor() as u8, 255])
}

/*
    For a point c=(x, y) this algorithm determines the number of iterations it takes for a breakout to occur
 */
fn compute_escape_time(c: Point, mut z: Point) -> (usize, Point, Point) {
    let mut x = 0.0;
    let mut y = 0.0;

    let mut iteration = 0;

    while z.x + z.y <= 4.0  && iteration < MAX_ITERATIONS {
        y = 2.0*x*y + c.y;
        x = z.x - z.y + c.x;
        z.x = x*x;
        z.y = y*y;
        iteration += 1;
    }

    (iteration, c, z)
}

/*
    Takes in an integer n representing the number of iterations to escape in the mandelbrot set.
    The algorithm then uses modulo to determine the proper color for that pixel.
    This algorithm performs faster than the interpolation based coloring, however, the image is far less beautiful.
 */
fn map_iteration_to_color_basic(n: usize) -> Rgba<u8> {
    return if n < MAX_ITERATIONS && n > 0 {
        let idx = n % palettes::BLUE_YELLOW_PALETTE.len();
        palettes::BLUE_YELLOW_PALETTE[idx]
    }else{
        Rgba([0,0,0,255])
    }
}

/*
    Scales coordinates to the appropriate range for the Mandelbrot set
 */
fn scale_coordinate(pixel_coordinate: usize, image_dimension: usize, min_val: f64, max_val: f64) -> f64 {
    (pixel_coordinate as f64 / image_dimension as f64) * (max_val - min_val) + min_val
}

