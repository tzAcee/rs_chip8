pub struct Display {
    pixels: [[bool; 64]; 32]
}

impl Display {
    pub fn new() -> Self {
        Self { pixels: [[false; 64]; 32]}
    }
}