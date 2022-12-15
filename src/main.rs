mod cpu;
mod memory;
mod op_codes;
mod processor_status;

use cpu::Cpu;
use op_codes::*;

fn main() {
    let mut cpu = Cpu::new().reset(None);
    cpu.memory.data[0xFFFC] = NOP;
    cpu.execute();
}
