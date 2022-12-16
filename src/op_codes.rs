#![allow(unused)]
/// load accumulator immediate
pub const LDA_IM: u8 = 0xA9;
/// load accumulator absolute
pub const LDA_ABS: u8 = 0xAD;
/// load accumulator absolute x indexed
pub const LDA_ABS_X: u8 = 0xBD;
/// load accumulator absolute y indexed
pub const LDA_ABS_Y: u8 = 0xB9;
/// load accumulator zero page
pub const LDA_ZP: u8 = 0xA5;
/// load accumulator zero page x indexed
pub const LDA_ZP_X: u8 = 0xB5;
/// load accumulator zero page x indexed indirect
pub const LDA_ZP_XI: u8 = 0xA1;
/// load accumulator zero page y indirect indexed
pub const LDA_ZP_IY: u8 = 0xB1;

/// load x index immediate
pub const LDX_IM: u8 = 0xA2;
/// load x index absolute
pub const LDX_ABS: u8 = 0xA3;
/// load x index y indexed absolute
pub const LDX_ABS_Y: u8 = 0xBE;
/// load x index zero page
pub const LDX_ZP: u8 = 0xA6;
/// load x index y indexed zero page
pub const LDX_ZP_Y: u8 = 0xB6;

/// load y immediate
pub const LDY_IM: u8 = 0xA0;
/// load x index absolute
pub const LDY_ABS: u8 = 0xAC;
/// load x index y indexed absolute
pub const LDY_ABS_X: u8 = 0xBC;
/// load x index zero page
pub const LDY_ZP: u8 = 0xA4;
/// load x index y indexed zero page
pub const LDY_ZP_X: u8 = 0xB4;

/// no-op
pub const NOP: u8 = 0xEA;
/// jump subroutine
pub const JSR: u8 = 0x20;
/// logical shift right accumulator
pub const LSR_ACC: u8 = 0x4A;
/// logical shift right absolute
pub const LSR_ABS: u8 = 0x4E;
/// logical shift right zero page
pub const LSR_ZP: u8 = 0x46;
/// logical shift right absolute x indexed
pub const LSR_ABS_X: u8 = 0x5E;
