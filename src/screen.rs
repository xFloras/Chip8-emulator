use pixels::Pixels;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Screen {
    pub grid: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Screen {
    pub fn new() -> Self {
        Self {
            grid: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.grid = [0; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
       if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
           self.grid[y * SCREEN_WIDTH + x] ^= value;
       }
    }

    pub fn render(&self, pixels: &mut Pixels) {
        let frame = pixels.frame_mut();

        for (i, &pixel) in self.grid.iter().enumerate() {
            /*
            every pixel corresponds to 4 bytes (R, G, B, A)
            */
            let pixel_index = i*4;

            let color = if pixel == 1 { 255 } else { 0 };

            frame[pixel_index]     = color; // R
            frame[pixel_index + 1] = color; // G
            frame[pixel_index + 2] = color; // B
            frame[pixel_index + 3] = 255;   // A (transparency always 255)
        }
    }

}
