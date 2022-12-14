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
/// no-op
pub const NOP: u8 = 0xEA;
/// jump subroutine
pub const JSR: u8 = 0x20;
