use image::Rgba;

// Colors from https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
// Despite using RGBA, the Window appears to read in as BGR, so I had to reverse the colors here.
pub const BLUE_YELLOW_PALETTE: [Rgba<u8>; 16] = [
    Rgba([15, 30, 66, 255]),
    Rgba([26, 7, 25, 255]),
    Rgba([47, 1, 9, 255]),
    Rgba([73, 4, 4, 255]),
    Rgba([100, 7, 0, 255]),
    Rgba([138, 44, 12, 255]),
    Rgba([177, 82, 24, 255]),
    Rgba([209, 125, 57, 255]),
    Rgba([229, 181, 134, 255]),
    Rgba([248, 236, 211, 255]),
    Rgba([191, 233, 241, 255]),
    Rgba([95, 201, 248, 255]),
    Rgba([0, 170, 255, 255]),
    Rgba([0, 128, 204, 255]),
    Rgba([0, 87, 153, 255]),
    Rgba([3, 52, 106, 255]),
];

// Palettes based on C program: https://github.com/elmomoilanen/Mandelbrot/blob/main/src/palette.c
pub const PALETTE_UF: [Rgba<u8>; 16] = [
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

pub const PALETTE_OR: [Rgba<u8>; 12] = [
    Rgba([180, 64, 16, 255]),
    Rgba([229, 83, 0, 255]),
    Rgba([255, 105, 0, 255]),
    Rgba([255, 43, 0, 255]),
    Rgba([255, 64, 35, 255]),
    Rgba([255, 92, 0, 255]),
    Rgba([255, 129, 0, 255]),
    Rgba([255, 159, 0, 255]),
    Rgba([255, 195, 77, 255]),
    Rgba([235, 190, 15, 255]),
    Rgba([220, 195, 15, 255]),
    Rgba([215, 252, 0, 255]),
];

pub const PALETTE_LB: [Rgba<u8>; 12] = [
    Rgba([47, 86, 233, 255]),
    Rgba([45, 100, 245, 255]),
    Rgba([47, 141, 255, 255]),
    Rgba([51, 171, 249, 255]),
    Rgba([52, 204, 255, 255]),
    Rgba([82, 219, 255, 255]),
    Rgba([23, 236, 236, 255]),
    Rgba([110, 255, 255, 255]),
    Rgba([168, 255, 255, 255]),
    Rgba([149, 212, 122, 255]),
    Rgba([169, 255, 47, 255]),
    Rgba([255, 173, 47, 255]),
];

pub const PALETTE_RE: [Rgba<u8>; 9] = [
    Rgba([139, 0, 0, 255]),
    Rgba([178, 34, 34, 255]),
    Rgba([255, 99, 71, 255]),
    Rgba([255, 127, 80, 255]),
    Rgba([233, 150, 122, 255]),
    Rgba([255, 160, 122, 255]),
    Rgba([184, 134, 11, 255]),
    Rgba([218, 165, 32, 255]),
    Rgba([240, 230, 140, 255]),
];

pub const PALETTE_GR: [Rgba<u8>; 10] = [
    Rgba([154, 205, 50, 255]),
    Rgba([128, 128, 0, 255]),
    Rgba([85, 107, 47, 255]),
    Rgba([107, 142, 35, 255]),
    Rgba([0, 100, 0, 255]),
    Rgba([0, 128, 0, 255]),
    Rgba([34, 139, 34, 255]),
    Rgba([0, 255, 0, 255]),
    Rgba([50, 205, 50, 255]),
    Rgba([143, 188, 143, 255]),
];