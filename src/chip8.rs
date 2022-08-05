mod chip8_mods;

use chip8_mods::*;
use ggez::{event, timer};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;
use std::{
    thread::{self, current},
    time,
};

pub struct Chip8 {
    memory: memory::Memory,
    display: display::Display,
    pc: program_counter::ProgramCounter,
    i: index_register::IndexRegister,
    stack: stack::Stack,
    delay_timer: delay_timer::DelayTimer,
    sound_timer: sound_timer::SoundTimer,
    variable_registers: [variable_register::VariableRegister; 16],
    current_instruction: u16,
    current_function: fn(&mut Chip8),
}

impl Chip8 {
    pub fn run(path: &str) {
        let mut slf = Self {
            memory: memory::Memory::new(),
            display: display::Display::new(),
            pc: program_counter::ProgramCounter::new(),
            i: index_register::IndexRegister::new(),
            stack: stack::Stack::new(),
            delay_timer: delay_timer::DelayTimer::new(),
            sound_timer: sound_timer::SoundTimer::new(),
            variable_registers: [variable_register::VariableRegister::new(); 16],
            current_instruction: 0,
            current_function: { |this| () },
        };
        slf.memory.read_program(path);
        let cb = ggez::ContextBuilder::new("CHIP8_rust", "FleeXo");
        let (ctx, event_loop) = cb.build().unwrap();
        graphics::set_window_title(&ctx, "CHIP8 Emulator in Rust");
        event::run(ctx, event_loop, slf);
    }

    fn fetch(&mut self) {
        self.current_instruction = self.memory.get_instruction(self.pc.get_point_value());
        self.pc.set_point_value(self.pc.get_point_value() + 2);
    }
    fn decode(&mut self) {
        let nibble =self.current_instruction & 0xF000;
        println!("Calling instruction: {:#04x}", self.current_instruction);
        match nibble {
            0x0000 => {
                self.current_function = |this| {
                let last_bit = ((*this).current_instruction & 0x000F);
                match last_bit {
                    // 00E0 Clear Screen
                    0 => {(*this).display.clear();}
                    // 00EE Subroutines
                    _ => {
                        (*this).stack.set_pointer((*this).stack.get_pointer()-1);
                        (*this).pc.set_point_value((*this).stack.get_value_at_pointer() as u32);
                    }
                 }
            }
            }
            0x1000 => {
                // 1NNN Jump to NNN
                self.current_function = |this| {
                    let address = ((*this).current_instruction & 0x0FFF) as u32;
                    (*this).pc.set_point_value(address);
                }
            }
            0x2000 => {
                // 2NNN Subroutines
                self.current_function = |this| {
                    let NNN = ((*this).current_instruction & 0x0FFF) as u8;

                    (*this).stack.push((*this).pc.get_point_value() as u16);
                    (*this).stack.set_pointer((*this).stack.get_pointer()+1);
                    (*this).pc.set_point_value(NNN as u32);
                };
            }
            0x3000 => {
                // 3XNN Skip
                self.current_function = |this| {
                    let X = ((*this).current_instruction & 0x0F00) >> 8;
                    let current_vx = (*this).variable_registers[X as usize].get();
                    let NN = ((*this).current_instruction & 0x00FF) as u8;
                    if current_vx == NN {
                        //do the skip
                        (*this).pc.set_point_value((*this).pc.get_point_value()+2);
                    } else {
                        println!("Did not skip, because VX != NN");
                    }
                };
            }
            0x4000 => {
                // 4XNN Skip
                self.current_function = |this| {
                    let X = ((*this).current_instruction & 0x0F00) >> 8;
                    let current_vx = (*this).variable_registers[X as usize].get();
                    let NN = ((*this).current_instruction & 0x00FF) as u8;
                    if current_vx != NN {
                        //do the skip
                        (*this).pc.set_point_value((*this).pc.get_point_value()+2);
                    } else {
                        println!("Did not skip, because VX == NN");
                    }
                };
            }
            0x5000 => {
                // 5XY0 Skip
                self.current_function = |this| {
                    let X = ((*this).current_instruction & 0x0F00) >> 8;
                    let current_vx = (*this).variable_registers[X as usize].get();
                    let Y = ((*this).current_instruction & 0x00F0) >> 4;
                    let current_vy = (*this).variable_registers[Y as usize].get();
                    if current_vx == current_vy {
                        //do the skip
                        (*this).pc.set_point_value((*this).pc.get_point_value()+2);
                    } else {
                        println!("Did not skip, because VX != VY");
                    }
                };
            }
            0x6000 => { 
                // 6XNN // set register VX to NN
                self.current_function = |this| {
                    let NN = ((*this).current_instruction & 0x00FF) as u8;
                    let X = ((*this).current_instruction & 0x0F00) >> 8;
                    (*this).variable_registers[X as usize].set(NN);
                };
            }
            0x7000 => { 
                // 7XNN (add value to register VX)
                self.current_function = |this| {
                    let NN = ((*this).current_instruction & 0x00FF) as u8;
                    let X = ((*this).current_instruction & 0x0F00) >> 8;
                    let current_vx = (*this).variable_registers[X as usize].get();
                    (*this).variable_registers[X as usize].set(NN + current_vx);
                }
            }
            0x8000 => {
                // 8XY0 Set

                // 8XY1 Binary Or

                // 8XY2 Binary And

                // 8XY3 logical XOR

                // 8XY4 Add

                // 8XY5 Subtract

                // 8XY6 Shift [Ambigious]

                // 8XY7 Subtract

                // 8XYE Shift [Ambigious]
            }
            0x9000 => {
                // 9XY0 Skip
                self.current_function = |this| {
                    let X = ((*this).current_instruction & 0x0F00) >> 8;
                    let current_vx = (*this).variable_registers[X as usize].get();
                    let Y = ((*this).current_instruction & 0x00F0) >> 4;
                    let current_vy = (*this).variable_registers[Y as usize].get();
                    if current_vx != current_vy {
                        //do the skip
                        (*this).pc.set_point_value((*this).pc.get_point_value()+2);
                    } else {
                        println!("Did not skip, because VX == VY");
                    }
                };
            }
            0xA000 => { 
                // ANNN (set index register I)
                self.current_function = |this| {
                    let NN = ((*this).current_instruction & 0x0FFF) as u16;
                    this.i.set(NN);
                }
            }
            0xB000 => {
                // BNNN Jump with offset [Ambigious]
            }
            0xC000 => {
                // CXNN Random
            }
            0xD000 => {
                //DXYN display/draw
                self.current_function = |this| {
                    // get coordinates
                    let X = ((*this).current_instruction & 0x0F00) >> 8;
                    let mut current_vx = (*this).variable_registers[X as usize].get() % 64;
                    let Y = ((*this).current_instruction & 0x00F0) >> 4;
                    let mut current_vy = (*this).variable_registers[Y as usize].get() % 32;

                    // set VF to 0
                    (*this).variable_registers[15].set(0);

                    // To the stuff with N ...
                    let N = ((*this).current_instruction & 0x000F);
                    let I = (*this).i.get();
                    for nth in 0..N {
                        let nth_byte = this.memory.get_byte(I as usize, nth as usize);
                        current_vx = (*this).variable_registers[X as usize].get() % 64;
                        for i in 0..8 {
                            let is_set = (nth_byte >> (7 - i) & 1);

                            if is_set == 1 {
                                if (*this)
                                    .display
                                    .is_pixel_on(current_vx as usize, current_vy as usize)
                                {
                                    (*this)
                                        .display
                                        .un_set_pixel(current_vx as usize, current_vy as usize);
                                    // set VF to 1
                                    (*this).variable_registers[15].set(1);
                                } else {
                                    (*this)
                                        .display
                                        .set_pixel(current_vx as usize, current_vy as usize);
                                }
                            }
                            if (*this).display.is_right_edge(current_vx as usize) {
                                break;
                            }
                            current_vx += 1;
                        }

                        current_vy += 1;
                        if (*this).display.is_bottom_edge(current_vy as usize) {
                            break;
                        }
                    }
                }
            }
            0xE000 => {
                // EX9E Skip if key
                // EXA1 Skip if key
            }
            0xF000 => {
                // FX07 sets VX to the current value of the delay timer
                // FX15 sets the delay timer to the value in VX
                // FX18 sets the sound timer to the value in VX
                // FX1E: Add to index
                // FX0A: Get key
                // FX29: Font character
                // FX33: Binary-coded decimal conversion
                // FX55 and FX65: Store and load memory [Ambigious]
            }
            _ => {
                println!("Did not found Nibble of OPCODE.");
            }
        }
    }
    fn execute(&mut self) {
        (self.current_function)(self);
    }

    fn draw_display_pixels(&mut self, ctx: &mut Context) {
        let pixel_width = 800.0 / (self.display.get_pixels()[0].len() as f32);
        let pixel_height = 600.0 / (self.display.get_pixels().len() as f32);

        for (y_i, YArr) in self.display.get_pixels().iter().enumerate() {
            for (x_i, pixel_val) in YArr.iter().enumerate() {
                let rect_coords: graphics::Rect = graphics::Rect::new(
                    x_i as f32 * pixel_width,
                    y_i as f32 * pixel_height,
                    pixel_width,
                    pixel_height,
                );
                if *pixel_val == true {
                
                let mut color = graphics::Color::WHITE;

                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect_coords,
                    color,
                )
                .unwrap();
                graphics::draw(ctx, &rect, graphics::DrawParam::default());
            }
            }
        }
    }
}

impl event::EventHandler<ggez::GameError> for Chip8 {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 1;
        while timer::check_update_time(_ctx, DESIRED_FPS)
        {
            self.fetch();
            self.decode();
            self.execute();

        }
            Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.draw_display_pixels(ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}
