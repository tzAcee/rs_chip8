pub struct Display {
    pixels: [[bool; 64]; 32],
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [[false; 64]; 32],
        }
    }

    pub fn clear(&mut self) {
        for Y in self.pixels.iter_mut() {
            for pixel in Y.iter_mut() {
                *pixel = false;
            }
        }
    }

    pub fn is_pixel_on(&self, x: usize, y: usize) -> bool {
        self.pixels[y][x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize) {
        self.pixels[y][x] = true;
    }

    pub fn un_set_pixel(&mut self, x: usize, y: usize) {
        self.pixels[y][x] = false;
    }

    pub fn is_right_edge(&self, x: usize) -> bool {
        x >= 63
    }

    pub fn is_bottom_edge(&self, y: usize) -> bool {
        y >= 32
    }

    pub fn get_pixels(&self) -> [[bool; 64]; 32] {
        self.pixels
    }
}
