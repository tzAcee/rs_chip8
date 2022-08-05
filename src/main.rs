mod chip8;

fn main() {
    chip8::Chip8::run("./programs/test_opcode.ch8");
}
