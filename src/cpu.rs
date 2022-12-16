#![allow(unused)]
use std::ops::Shr;

use crate::{
    memory::{self, Memory},
    op_codes::*,
    processor_status::ProcessorStatus,
};

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    /// program counter
    pc: u16,
    /// stack pointer
    sp: u16,
    /// accumulator
    a: u8,
    /// x index register
    x: u8,
    /// y index register
    y: u8,
    /// processor status (bitfield)
    ps: ProcessorStatus,

    /// Memory module
    pub memory: Memory,
}

impl Cpu {
    /// construct a new cpu
    pub fn new() -> Self {
        Self::default()
    }

    /// reset the cpu to initial state
    /// an optional address can be given to give the
    /// cpu a location to fetch instructions from after
    /// the reset has finished
    pub fn reset(&mut self, address: Option<u16>) -> Self {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.ps.clear();

        // read 0xFFFC and 0xFFFD and
        // jump to that address for instructions
        if let Some(address) = address {
            self.memory.write_word(self.pc as usize, address);
            self.pc = self.memory.read_word(0xFFFC);
        }

        self.to_owned()
    }

    /// load a program into the cpu's memory at a given address
    pub fn load_program(&mut self, address: usize, program: Vec<u8>) {
        todo!()
    }

    /// execute the program loaded in memory
    pub fn execute(&mut self) {
        loop {
            let instruction = self.fetch_byte();
            match instruction {
                LDA_IM => self.lda_immediate(),
                LDA_ABS => self.lda_absolute(),
                LDA_ABS_X => self.lda_absolute_x_indexed(),
                LDA_ABS_Y => self.lda_absolute_y_indexed(),
                LDA_ZP => self.lda_zp(),
                LDA_ZP_X => self.lda_zp_x(),
                LDA_ZP_XI => self.lda_x_indexed_zero_page_indirect(),
                LDA_ZP_IY => self.lda_y_zero_page_indirect_indexed(),
                LDX_IM => self.ldx_immediate(),
                LDX_ABS => self.ldx_absolute(),
                LDX_ZP => self.ldx_zp(),
                LDX_ZP_Y => self.ldx_y_indexed_zero_page(),
                LDX_ABS_Y => self.ldx_absolute_y_indexed(),
                LDY_IM => self.ldy_immediate(),
                LDY_ABS => self.ldy_absolute(),
                LDY_ZP => self.ldy_zp(),
                LDY_ZP_X => self.ldy_x_indexed_zero_page(),
                LDY_ABS_X => self.ldy_absolute_x_indexed(),
                LSR_ACC => self.lsr_acc(),
                LSR_ABS => self.lsr_abs(),
                LSR_ZP => self.lsr_zp(),
                LSR_ABS_X => self.lsr_abs_x(),
                LSR_ZP_X => self.lsr_zp_x(),
                JSR => self.jump_subroutine(),
                NOP => break,
                _ => {
                    self.debug_print();
                    panic!("reason: unrecognized instruction");
                }
            }
        }
    }

    /// print contents of registers, pc, sp, and status flags and current instruction
    /// useful when the emulator crashes, you can get a state of the machine
    pub fn debug_print(&self) {
        println!("pc: 0x{:04x}", self.pc);
        println!("sp: 0x{:04x}", self.sp);
        println!("a : 0x{:04x}", self.a);
        println!("x : 0x{:04x}", self.x);
        println!("y : 0x{:04x}", self.y);
        println!("ps: {}", self.ps);
        println!("current instruction: 0x{:02X}", self.memory.read_byte(self.pc as usize));
    }

    /// fetch a word from memory while incrememting the pc each read (2 cycles)
    fn fetch_word(&mut self) -> u16 {
        let mut data = self.memory.data[self.pc as usize] as u16;
        self.pc += 1;

        data |= u16::from(self.memory.data[self.pc as usize]) << 8;
        self.pc += 1;

        data
    }

    /// fetch a byte and increment the pc
    fn fetch_byte(&mut self) -> u8 {
        if self.pc as usize > memory::MAX_MEM {
            panic!("PC exceeds max memory allocated {}", memory::MAX_MEM);
        }

        let data = self.memory.data[self.pc as usize];
        self.pc += 1;
        data
    }

    /* LOAD A INSTRUCTIONS */
    /// load accumulator immediate mode
    fn lda_immediate(&mut self) {
        self.a = self.fetch_byte();
        self.set_negative_and_zero_flags();
    }

    /// load accumulator absolute
    fn lda_absolute(&mut self) {
        let abs_address = self.fetch_word();
        self.a = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load accumulator absolute x indexed
    fn lda_absolute_x_indexed(&mut self) {
        let abs_address = self.fetch_word() + self.x as u16;
        self.a = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load accumulator absolute y indexed
    fn lda_absolute_y_indexed(&mut self) {
        let abs_address = self.fetch_word() + self.y as u16;
        self.a = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load accumulator zero page
    fn lda_zp(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.a = self.memory.read_byte(zero_page_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load accumulator zero page x indexed
    fn lda_zp_x(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.a = self.memory.read_byte((zero_page_address) as usize) + self.x;
        self.set_negative_and_zero_flags();
    }

    /// load accumulator indexed zero page indirect
    fn lda_x_indexed_zero_page_indirect(&mut self) {
        let indirect_address = self.fetch_byte() + self.x;
        self.a = self.memory.read_byte(indirect_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load accumulator zero page indirect y indexed
    fn lda_y_zero_page_indirect_indexed(&mut self) {
        let zero_page_address = self.fetch_byte();
        let effective_address = self.memory.read_word(zero_page_address as usize);
        let effective_address_y = effective_address + self.y as u16;
        self.a = self.memory.read_byte(effective_address_y as usize);
        self.set_negative_and_zero_flags();
    }

    /// set zero and negative flags whenever an LDA instruction is executed
    fn set_negative_and_zero_flags(&mut self) {
        // set zero flag
        self.ps.set(ProcessorStatus::Z, self.a == 0);
        self.ps
            .set(ProcessorStatus::N, (self.a & 0b10000000) > 0);
    }

    /* LOAD X INSTRUCTIONS */
    /// load x index immediate mode
    fn ldx_immediate(&mut self) {
        self.x = self.fetch_byte();
        self.set_negative_and_zero_flags();
    }

    /// load x index absolute mode
    fn ldx_absolute(&mut self) {
        let abs_address = self.fetch_word();
        self.x = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load x index from zero page
    fn ldx_zp(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.x = self.memory.read_byte(zero_page_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load x index y indexed absolute
    fn ldx_absolute_y_indexed(&mut self) {
        let abs_address = self.fetch_word() + self.y as u16;
        self.x = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load x index y indexed zero page
    fn ldx_y_indexed_zero_page(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.x = self.memory.read_byte((zero_page_address) as usize) + self.y;
        self.set_negative_and_zero_flags();
    }

    /* LOAD Y INSTRUCTIONS */
    /// load y index immediate mode
    fn ldy_immediate(&mut self) {
        self.y = self.fetch_byte();
        self.set_negative_and_zero_flags();
    }

    /// load y index absolute mode
    fn ldy_absolute(&mut self) {
        let abs_address = self.fetch_word();
        self.y = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load y index from zero page
    fn ldy_zp(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.y = self.memory.read_byte(zero_page_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load y index x indexed absolute
    fn ldy_absolute_x_indexed(&mut self) {
        let abs_address = self.fetch_word() + self.x as u16;
        self.y = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load x index y indexed zero page
    fn ldy_x_indexed_zero_page(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.y = self.memory.read_byte((zero_page_address) as usize) + self.x;
        self.set_negative_and_zero_flags();
    }

    /// jump to a subroutine by pushing the pc onto the stack and modifying the pc
    fn jump_subroutine(&mut self) {
        let sub_address = self.fetch_word();
        self.memory.write_word(self.sp as usize, (self.pc - 1));
        self.sp -= 2;
        self.pc = sub_address;
    }

    /* logical shift right instructions */
    /// logical shift right accumulator mode
    fn lsr_acc(&mut self) {
        let carry = self.a & 1;
        self.a >>= 1;
        self.set_negative_and_zero_flags();
        self.set_carry_flag(carry > 0);
    }
    
    /// logical shift right absolute mode
    fn lsr_abs(&mut self) {
        let abs_address = self.fetch_word() as usize;
        let data = self.memory.read_byte(abs_address);

        self.memory.write_byte(abs_address, data >> 1);

        self.set_negative_and_zero_flags();
        self.set_carry_flag((data & 1) > 0);
    }

    /// logical shift right zero page
    fn lsr_zp(&mut self) {
        let zero_page_address = self.fetch_byte() as usize;
        let data = self.memory.read_byte(zero_page_address);
        self.memory.write_byte(zero_page_address, data >> 1);
        self.set_carry_flag((data & 1) > 0);
        self.set_negative_and_zero_flags();
    }

    /// logical shift right absolute x indexed
    fn lsr_abs_x(&mut self) {
        let abs_address = self.fetch_word() as usize;
        let effective_address = abs_address + self.x as usize;
        let data = self.memory.read_byte(effective_address);

        self.memory.write_byte(effective_address, data >> 1);

        self.set_negative_and_zero_flags();
        self.set_carry_flag((data & 1) > 0);
    }

    /// logical shift right zero page x indexed
    fn lsr_zp_x(&mut self) {
        let zero_page_address = self.fetch_byte() as usize;
        let effective_address = zero_page_address + self.x as usize;
        let data = self.memory.read_byte(effective_address);

        self.memory.write_byte(effective_address, data >> 1);

        self.set_negative_and_zero_flags();
        self.set_carry_flag((data & 1) > 0);
    }

    /// sets the carry bit if flag is true in processor status register
    fn set_carry_flag(&mut self, flag: bool) {
        self.ps.set(ProcessorStatus::C, flag);
    }

    /// no-op (do nothing)
    fn nop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use crate::op_codes::*;
    use crate::processor_status::ProcessorStatus;

    #[test]
    fn new_cpu_should_initialize_defaults() {
        let cpu = Cpu::new().reset(None);
        assert_eq!(cpu.pc, 0xFFFC);
    }

    #[test]
    fn reset_cpu_with_address_should_fetch_from_correct_address() {
        let cpu = Cpu::new().reset(0x0010.into());
        assert_eq!(cpu.pc, 0x0010);
    }

    #[test]
    fn set_carry_flag_should_set_correct_bit() {
        let mut cpu = Cpu::new().reset(None);
        cpu.set_carry_flag(true);
        assert_eq!(cpu.ps, ProcessorStatus::C)
    }

    #[test]
    fn write_word_should_write_correct_data_to_memory() {
        let data: u16 = 0b1111111100000000;
        let mut cpu = Cpu::new().reset(None);
        cpu.memory.write_word(0xFFFC, data);
        let word = cpu.memory.read_word(0xFFFC);
        assert_eq!(word, data);
    }

    #[test]
    fn logical_shift_right_absolute_x_indexed_should_shift_value_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0011] = 0x02;
        cpu.memory.data[0x0001] = LDX_IM;
        cpu.memory.data[0x0002] = 0x01;
        cpu.memory.data[0x0003] = LSR_ABS_X;
        cpu.memory.data[0x0004] = 0x10;
        cpu.memory.data[0x0005] = 0x00; // 0x0010
        cpu.memory.data[0x0006] = NOP;

        cpu.execute();
        let address = cpu.memory.read_byte(0x011); //0x10 + 1
        assert_eq!(address, 0x01);
    }

    #[test]
    fn logical_shift_right_zero_page_x_indexed_should_shift_value_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0011] = 0x02;
        cpu.memory.data[0x0001] = LDX_IM;
        cpu.memory.data[0x0002] = 0x01;
        cpu.memory.data[0x0003] = LSR_ZP_X;
        cpu.memory.data[0x0004] = 0x10;
        cpu.memory.data[0x0005] = NOP;

        cpu.execute();
        let address = cpu.memory.read_byte(0x011); //0x10 + 1
        assert_eq!(address, 0x01);
    }

    #[test]
    fn logical_shift_right_zero_page_should_shift_value_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0010] = 0x02;
        cpu.memory.data[0x0001] = LSR_ZP;
        cpu.memory.data[0x0002] = 0x10;
        cpu.memory.data[0x0003] = NOP;

        cpu.execute();
        let address = cpu.memory.read_byte(0x010);
        assert_eq!(address, 0x01);
    }

    #[test]
    fn logical_shift_right_absolute_should_shift_value_at_address_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0100] = 0x02;
        cpu.memory.data[0x0001] = LSR_ABS;
        cpu.memory.data[0x0002] = 0x00;
        cpu.memory.data[0x0003] = 0x01; // 0x0100
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        let address = cpu.memory.read_byte(0x0100);
        assert_eq!(address, 0x01);
    }

    #[test]
    fn logical_shift_right_accumulator_should_shift_value_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0x02;
        cpu.memory.data[0x0003] = LSR_ACC;
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x01);
    }

    #[test]
    fn logical_shift_right_should_set_carry_flag() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0x02;
        cpu.memory.data[0x0003] = LSR_ACC;
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "00000000");
    }

    #[test]
    fn logical_shift_right_should_reset_negative_flag() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0b1000;
        cpu.memory.data[0x0003] = LSR_ACC;
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "00000000");
    }

    #[test]
    fn logical_shift_right_should_set_carry_and_zero_flags() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0b0001;
        cpu.memory.data[0x0003] = LSR_ACC;
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "00000011");
    }

    #[test]
    fn jump_subroutine_should_jump_to_new_address() {
        let mut cpu = Cpu::new().reset(None);

        // load a dummy program into memory
        cpu.memory.data[0xFFFC] = JSR;
        cpu.memory.data[0xFFFD] = 0x10;
        cpu.memory.data[0xFFFE] = 0x00; // JSR 0x0010
        cpu.memory.data[0x0010] = NOP;

        cpu.execute();
        // stack pointer should be 0xFF 0xFD (high byte first)
        let expected_return_address = (cpu.sp + 2) as usize;
        let stack_address = cpu.memory.read_word(expected_return_address);
        // should get to no-op
        assert_eq!(cpu.pc, 0x0011);
        // return to last byte of last instruction
        assert_eq!(stack_address, 0xFFFE);
    }

    #[test]
    fn ldy_immediate_should_load_y_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDY_IM;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0x42);
    }

    #[test]
    fn ldy_absolute_should_load_y_register() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDY_ABS;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4480] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0x37);
    }

    #[test]
    fn ldy_absolute_x_indexed_should_load_y_register_with_correct_value() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // set y register
        cpu.x = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDY_ABS_X;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4481] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0x37);
    }

    #[test]
    fn ldy_zero_page_should_load_y_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDY_ZP;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0x84);
    }

    #[test]
    fn ldy_zero_page_x_indexed_should_load_y_register() {
        let mut cpu = Cpu::new().reset(None);
        // set the X register to 1
        cpu.x = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDY_ZP_X;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0x85);
    }

    #[test]
    fn ldx_immediate_should_load_x_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDX_IM;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x42);
    }

    #[test]
    fn ldx_absolute_should_load_x_register() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDX_ABS;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4480] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x37);
    }

    #[test]
    fn ldx_absolute_y_indexed_should_load_x_register_with_correct_value() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // set y register
        cpu.y = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDX_ABS_Y;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4481] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x37);
    }

    #[test]
    fn ldx_zero_page_should_load_x_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDX_ZP;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x84);
    }

    #[test]
    fn ldx_zero_page_y_indexed_should_load_x_register() {
        let mut cpu = Cpu::new().reset(None);
        // set the X register to 1
        cpu.y = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDX_ZP_Y;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x85);
    }

    #[test]
    fn lda_immediate_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_IM;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x42);
    }

    #[test]
    fn lda_absolute_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
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
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // set x register
        cpu.x = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDA_ABS_X;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4481] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x37);
    }

    #[test]
    fn lda_absolute_y_indexed_should_load_accumulator_with_correct_value() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // set y register
        cpu.y = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDA_ABS_Y;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4481] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x37);
    }

    #[test]
    fn lda_zero_should_set_zero_flag() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_IM;
        cpu.memory.data[0xFFFD] = 0x00;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "00000010");
    }

    #[test]
    fn lda_seventh_bit_set_should_raise_negative_flag() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_IM;
        cpu.memory.data[0xFFFD] = 0b10000001;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "10000000");
    }

    #[test]
    fn lda_zero_page_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
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
        let mut cpu = Cpu::new().reset(None);
        // set the X register to 1
        cpu.x = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_ZP_X;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x85);
    }

    #[test]
    fn lda_zero_page_x_indexed_indirect_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
        // set the X register to 1
        cpu.x = 0x04;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_ZP_XI;
        cpu.memory.data[0xFFFD] = 0x20;
        cpu.memory.data[0x0024] = 0x20;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x20);
    }

    #[test]
    fn lda_zero_page_indirect_y_indexed_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
        // set the Y register to 10
        cpu.y = 0x04;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_ZP_IY;
        cpu.memory.data[0xFFFD] = 0x02;
        cpu.memory.data[0x0002] = 0x00;
        cpu.memory.data[0x0003] = 0x80;
        cpu.memory.data[0x8004] = 0x37;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x37);
    }

    #[test]
    fn read_word() {
        let mut cpu = Cpu::new().reset(None);
        cpu.memory.data[0x44] = 0x20;
        cpu.memory.data[0x45] = 0x20;

        let word = cpu.memory.read_word(0x44);

        assert_eq!(word, 0x2020);
    }
}
