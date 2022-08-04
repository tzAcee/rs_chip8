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
        println!("Display cleared...");
    }
}
