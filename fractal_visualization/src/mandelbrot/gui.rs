
use minifb::{Key, Window, WindowOptions};

pub struct Gui {
    window: Window,
    width: usize,
    height: usize,
    buffer: Vec<u32>
}

impl Gui {
    pub fn new (width: usize, height: usize) -> Gui {
        Gui {
            width,
            height,
            window: Window::new(
            "Mandelbrot Visualization",
            width,
            height,
            WindowOptions::default()
            ).unwrap_or_else(|e| panic!("{}", e)),
            buffer: Vec::new()
        }
    }

    pub(crate) fn set_buffer(&mut self, buffer: Vec<u32>) {
        self.buffer = buffer;
    }

    pub fn update_window(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap()
    }

    pub fn window_is_open(&mut self) -> bool {
        return self.window.is_open();
    }

    pub fn escape_key_pressed(&mut self) -> bool {
        return self.window.is_key_down(Key::Escape);
    }
}