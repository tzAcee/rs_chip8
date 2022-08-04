use byteorder::{ByteOrder, LittleEndian};
use std::fs;
use std::path::Path; // 1.3.4

fn read_font() -> Option<[u8; 80]> {
    let filenpath = Path::new("./font/font");
    println!("In file {}", filenpath.display());

    let mut contents =
        fs::read_to_string(filenpath).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    contents.retain(|c| !c.is_whitespace());
    let split_content: Vec<&str> = contents.split(',').collect();
    if split_content.len() != 80 {
        return None;
    }
    let mut result_array: [u8; 80] = [0; 80];
    for (i, font_data) in split_content.iter().enumerate() {
        // convert string to i8
        let without_prefix = font_data.trim_start_matches("0x");
        result_array[i] = u8::from_str_radix(without_prefix, 16).unwrap();
    }

    let abc: u8 = 0xf0;

    return Some(result_array);
}

pub struct Memory {
    RAM: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let font_bit_array = read_font();
        let mut ram: [u8; 4096] = [0; 4096];
        if font_bit_array == None {
            panic!("Could not read font.");
        } else {
            for (i, font_byte) in font_bit_array.unwrap().iter().enumerate() {
                ram[i] = *font_byte;
            }
        }
        Self { RAM: ram }
    }

    pub fn get_instruction(&self, pointer_value: u32) -> u16 {
        let memory_point = self.RAM.get(pointer_value as usize).unwrap();
        let memory_point2 = self.RAM.get((pointer_value + 1) as usize).unwrap();

        return ((*memory_point as u16) << 8) | *memory_point2 as u16;
    }
}
