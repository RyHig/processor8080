// use std::fs;

// mod disassembler;

// use disassembler::disassemble8080op;
// fn main() {
//     let data = fs::read("./invaders")
//         .expect("Unable to read file.");
//     let len = data.len();
//     let mut pc: usize = 0;
//     while pc < len {
//     pc += disassemble8080op(&data, pc);
//     }
// }
mod emulator;

fn main()
{
    let a = emulator::ConditionCodes::new(1, 1, 1, 1, 1, 3);
    println!("{:?}", a);
}