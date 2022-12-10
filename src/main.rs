#![allow(unused)]

mod cpu;
mod memory;
mod op_codes;
mod processor_status;

use cpu::Cpu;
use memory::Memory;

fn main() {
    let cpu = Cpu::new().reset();
}
