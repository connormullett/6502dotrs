#![allow(unused)]
use crate::{
    memory::{self, Memory},
    op_codes::*,
    processor_status::ProcessorStatus,
};

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    // program counter
    pc: u16,
    // stack pointer
    sp: u16,
    // accumulator
    a: u8,
    // x index register
    x: u8,
    // y index register
    y: u8,
    // processor status (bitfield)
    ps: ProcessorStatus,

    // Memory module
    pub memory: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) -> Self {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.ps.clear();

        self.to_owned()
    }

    pub fn load_program(&mut self) {
        todo!()
    }

    pub fn execute(&mut self) {
        loop {
            let instruction = self.fetch_and_increment_pc();
            match instruction {
                LDA_IM => self.lda_immediate(),
                LDA_ABS => self.lda_absolute(),
                LDA_ABS_X => self.lda_absolute_x_indexed(),
                LDA_ZP => self.lda_zp(),
                LDA_ZP_X => self.lda_zp_x(),
                NOP => break,
                _ => {
                    panic!("unrecognized instruction: {instruction:02x}");
                }
            }
        }
    }

    // print contents of registers, pc, sp, and status flags
    fn debug_print(&self) {
        println!("pc: 0x{:04x}", self.pc);
        println!("sp: 0x{:04x}", self.sp);
        println!("a : 0x{:04x}", self.a);
        println!("x : 0x{:04x}", self.x);
        println!("y : 0x{:04x}", self.y);
        println!("ps: {}", self.ps);
    }

    // fetch a single byte from the zero page
    fn fetch_zero_page(&mut self, address: usize) -> u8 {
        if self.pc as usize > memory::MAX_MEM {
            panic!("PC exceeds max memory allocated {}", memory::MAX_MEM);
        }

        self.memory.data[address]
    }

    // read a byte from memory
    fn read_byte(&mut self, address: usize) -> u8 {
        self.memory.data[address]
    }

    fn fetch_word(&mut self) -> u16 {
        let mut data = self.memory.data[self.pc as usize] as u16;
        self.pc += 1;

        data |= u16::from(self.memory.data[self.pc as usize]) << 8;
        self.pc += 1;

        data
    }

    // fetch a byte and increment the pc
    fn fetch_and_increment_pc(&mut self) -> u8 {
        if self.pc as usize > memory::MAX_MEM {
            panic!("PC exceeds max memory allocated {}", memory::MAX_MEM);
        }

        let data = self.memory.data[self.pc as usize];
        self.pc += 1;
        data
    }

    /* LOAD A INSTRUCTIONS */
    // load accumulator immediate mode
    fn lda_immediate(&mut self) {
        let value = self.fetch_and_increment_pc();
        self.a = value;
        self.lda_set_flags();
    }

    // load accumulator absolute
    fn lda_absolute(&mut self) {
        let abs_address = self.fetch_word();
        self.a = self.read_byte(abs_address as usize);
        self.lda_set_flags();
    }

    // load accumulator absolute x indexed
    fn lda_absolute_x_indexed(&mut self) {
        let abs_address = self.fetch_word();
        let value = self.read_byte(abs_address as usize) + self.x;
        self.a = value;
        self.lda_set_flags();
    }

    // load accumulator zero page
    fn lda_zp(&mut self) {
        let zero_page_address = self.fetch_and_increment_pc();
        self.a = self.fetch_zero_page(zero_page_address as usize);
        self.lda_set_flags();
    }

    // load accumulator zero page x indexed
    fn lda_zp_x(&mut self) {
        let zero_page_address = self.fetch_and_increment_pc();
        self.a = self.fetch_zero_page((zero_page_address + self.x) as usize);
        self.lda_set_flags();
    }

    // set zero and negative flags whenever an LDA instruction is executed
    fn lda_set_flags(&mut self) {
        // set zero flag
        self.ps.set(ProcessorStatus::Z, bool::from(self.a == 0));
        self.ps
            .set(ProcessorStatus::N, bool::from((self.a & 0b10000000) > 0));
    }

    // no-op
    fn nop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use crate::op_codes::*;

    #[test]
    fn new_cpu_should_initialize_defaults() {
        let cpu = Cpu::new().reset();
        assert_eq!(cpu.pc, 0xFFFC);
    }

    #[test]
    fn lda_immediate_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset();
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_IM;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x42);
    }

    #[test]
    fn lda_absolute_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset();
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDA_ABS;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4480] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x37);
    }

    #[test]
    fn lda_absolute_x_indexed_should_load_accumulator_with_correct_value() {
        let mut cpu = Cpu::new().reset();
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // set x register
        cpu.x = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDA_ABS;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4481] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;
    }

    #[test]
    fn lda_zero_should_set_zero_flag() {
        let mut cpu = Cpu::new().reset();
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_IM;
        cpu.memory.data[0xFFFD] = 0x00;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "00000010");
    }

    #[test]
    fn lda_seventh_bit_set_should_raise_negative_flag() {
        let mut cpu = Cpu::new().reset();
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_IM;
        cpu.memory.data[0xFFFD] = 0b10000001;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "10000000");
    }

    #[test]
    fn lda_zero_page_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset();
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_ZP;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x84);
    }

    #[test]
    fn lda_zero_page_x_indexed_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset();
        // set the X register to 1
        cpu.x = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_ZP_X;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0043] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x84);
    }
}
