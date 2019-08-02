use std::fs;

mod disassembler;

use disassembler::disassemble8080op;
fn main() {
    let data = fs::read("./invaders.e")
        .expect("Unable to read file.");
    let len = data.len();
    let mut pc: usize = 0;
    while pc < len {
    pc += disassemble8080op(&data, pc);
    }
    // Disassembler8080op(buffer, pc);
}
