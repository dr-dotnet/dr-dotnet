#![allow(non_upper_case_globals)]
use crate::cil::{check_flag, il_u32, Error};

bitflags! {
    pub struct MethodHeaderFlags: u8 {
        const CorILMethod_FatFormat = 0x3;
        const CorILMethod_TinyFormat = 0x2;
        const CorILMethod_MoreSects = 0x8;
        const CorILMethod_InitLocals = 0x10;
    }
}
#[derive(Debug)]
pub struct FatMethodHeader {
    pub more_sects: bool,
    pub init_locals: bool,
    pub max_stack: u16,
    pub code_size: u32,
    pub local_var_sig_tok: u32,
}
impl FatMethodHeader {
    pub const SIZE: u8 = 12;
}
#[derive(Debug)]
pub struct TinyMethodHeader {
    pub code_size: u8,
}
#[derive(Debug)]
pub enum MethodHeader {
    Fat(FatMethodHeader),
    Tiny(TinyMethodHeader),
}
impl MethodHeader {
    pub fn from_bytes(method_il: &[u8]) -> Result<Self, Error> {
        let header_flags = method_il[0];
        if Self::is_tiny(header_flags) {
            // In a tiny header, the first 6 bits encode the code size
            let code_size = method_il[0] >> 2;
            let tiny_header = TinyMethodHeader { code_size };
            Ok(MethodHeader::Tiny(tiny_header))
        } else if Self::is_fat(header_flags) {
            let more_sects = Self::more_sects(header_flags);
            let init_locals = Self::init_locals(header_flags);
            let max_stack = u16::from_le_bytes([method_il[2], method_il[3]]);
            let code_size = il_u32(method_il, 4)?;
            let local_var_sig_tok = il_u32(method_il, 8)?;
            let fat_header = FatMethodHeader {
                more_sects,
                init_locals,
                max_stack,
                code_size,
                local_var_sig_tok,
            };
            Ok(MethodHeader::Fat(fat_header))
        } else {
            Err(Error::InvalidMethodHeader)
        }
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match &self {
            MethodHeader::Fat(header) => {
                let mut flags = MethodHeaderFlags::CorILMethod_FatFormat.bits();
                if header.more_sects {
                    flags |= MethodHeaderFlags::CorILMethod_MoreSects.bits();
                }
                if header.init_locals {
                    flags |= MethodHeaderFlags::CorILMethod_InitLocals.bits();
                }
                bytes.push(flags);
                bytes.push(FatMethodHeader::SIZE.reverse_bits());
                bytes.extend_from_slice(&header.max_stack.to_le_bytes());
                bytes.extend_from_slice(&header.code_size.to_le_bytes());
                bytes.extend_from_slice(&header.local_var_sig_tok.to_le_bytes());
            }
            MethodHeader::Tiny(header) => {
                let byte = header.code_size << 2 | MethodHeaderFlags::CorILMethod_TinyFormat.bits();
                bytes.push(byte);
            }
        }
        bytes
    }
    fn more_sects(method_header_flags: u8) -> bool {
        check_flag(
            method_header_flags,
            MethodHeaderFlags::CorILMethod_MoreSects.bits(),
        )
    }
    fn init_locals(method_header_flags: u8) -> bool {
        check_flag(
            method_header_flags,
            MethodHeaderFlags::CorILMethod_InitLocals.bits(),
        )
    }
    fn is_tiny(method_header_flags: u8) -> bool {
        // Check only the 2 least significant bits
        (method_header_flags & 0b00000011) == MethodHeaderFlags::CorILMethod_TinyFormat.bits()
    }
    fn is_fat(method_header_flags: u8) -> bool {
        // Check only the 2 least significant bits
        (method_header_flags & 0b00000011) == MethodHeaderFlags::CorILMethod_FatFormat.bits()
    }
}
