use crate::cil::{
    il_f32, il_f64, il_i32, il_i64, il_i8, il_u16, il_u32, il_u8, opcode::*, Error, OperandParams,
};

#[derive(Debug)]
pub enum Operand {
    InlineNone,
    ShortInlineVar(u8),
    InlineVar(u16),
    ShortInlineI(u8),
    InlineI(i32),
    InlineI8(i64),
    ShortInlineR(f32),
    InlineR(f64),
    InlineMethod(u32),
    InlineSig(u32),
    ShortInlineBrTarget(i8),
    InlineBrTarget(i32),
    InlineSwitch(u32, Vec<i32>),
    InlineType(u32),
    InlineString(u32),
    InlineField(u32),
    InlineTok(u32),
}
impl Operand {
    pub fn length(&self) -> usize {
        match self {
            Self::InlineNone => 0,
            Self::ShortInlineVar(_) => 1,
            Self::InlineVar(_) => 2,
            Self::ShortInlineI(_) => 1,
            Self::InlineI(_) => 4,
            Self::InlineI8(_) => 8,
            Self::ShortInlineR(_) => 4,
            Self::InlineR(_) => 8,
            Self::InlineMethod(_) => 4,
            Self::InlineSig(_) => 4,
            Self::ShortInlineBrTarget(_) => 1,
            Self::InlineBrTarget(_) => 4,
            Self::InlineSwitch(length, _) => ((*length + 1) * 4) as usize,
            Self::InlineType(_) => 4,
            Self::InlineString(_) => 4,
            Self::InlineField(_) => 4,
            Self::InlineTok(_) => 4,
        }
    }
}
#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: Operand,
}

impl Instruction {
    /// Attempts to parse the first instruction at the beginning
    /// of the given byte array. Array must be at a valid instruction
    /// boundary.
    pub fn from_bytes(il: &[u8]) -> Result<Self, Error> {
        let byte_1 = il_u8(il, 0)?;
        let opcode = if byte_1 == 0xFE {
            // In this case, we have a multibyte opcode
            let byte_2 = il_u8(il, 1)?;
            Opcode::from_byte_pair((byte_1, byte_2))
        } else {
            Ok(Opcode::from_byte(byte_1))
        }?;
        let operand_index = opcode.length as usize;
        let operand = match &opcode.operand_params {
            OperandParams::InlineNone => Operand::InlineNone,
            OperandParams::ShortInlineVar => {
                let val = il_u8(il, operand_index)?;
                Operand::ShortInlineVar(val)
            }
            OperandParams::InlineVar => {
                let val = il_u16(il, operand_index)?;
                Operand::InlineVar(val)
            }
            OperandParams::ShortInlineI => {
                let val = il_u8(il, operand_index)?;
                Operand::ShortInlineI(val)
            }
            OperandParams::InlineI => {
                let val = il_i32(il, operand_index)?;
                Operand::InlineI(val)
            }
            OperandParams::InlineI8 => {
                let val = il_i64(il, operand_index)?;
                Operand::InlineI8(val)
            }
            OperandParams::ShortInlineR => {
                let val = il_f32(il, operand_index)?;
                Operand::ShortInlineR(val)
            }
            OperandParams::InlineR => {
                let val = il_f64(il, operand_index)?;
                Operand::InlineR(val)
            }
            OperandParams::InlineMethod => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineMethod(val)
            }
            OperandParams::InlineSig => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineSig(val)
            }
            OperandParams::ShortInlineBrTarget => {
                let val = il_i8(il, operand_index)?;
                Operand::ShortInlineBrTarget(val)
            }
            OperandParams::InlineBrTarget => {
                let val = il_i32(il, operand_index)?;
                Operand::InlineBrTarget(val)
            }
            OperandParams::InlineSwitch => {
                let length = il_u32(il, operand_index)?;
                let mut val: Vec<i32> = Vec::with_capacity(length as usize);
                for i in 1..=length {
                    let target_index = operand_index + ((i * 4) as usize);
                    let target = il_i32(il, target_index)?;
                    val.push(target);
                }
                Operand::InlineSwitch(length, val)
            }
            OperandParams::InlineType => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineType(val)
            }
            OperandParams::InlineString => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineString(val)
            }
            OperandParams::InlineField => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineField(val)
            }
            OperandParams::InlineTok => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineTok(val)
            }
        };
        Ok(Instruction {
            opcode: opcode,
            operand: operand,
        })
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        if self.opcode.length == 1 {
            bytes.push(self.opcode.byte_2);
        } else if self.opcode.length == 2 {
            bytes.push(self.opcode.byte_1);
            bytes.push(self.opcode.byte_2);
        }
        match &self.operand {
            Operand::InlineNone => (),
            Operand::ShortInlineVar(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineVar(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::ShortInlineI(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineI(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineI8(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::ShortInlineR(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineR(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineMethod(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineSig(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::ShortInlineBrTarget(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineBrTarget(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineSwitch(length, val) => {
                bytes.extend_from_slice(&length.to_le_bytes());
                println!(
                    "{}!!! {}!!! {:?}!!! {:?}!!! {:?}!!! {:?}!!! {:?}!!! {:?}!!!",
                    length,
                    val.len(),
                    length.to_le_bytes(),
                    val[0].to_le_bytes(),
                    val[1].to_le_bytes(),
                    val[2].to_le_bytes(),
                    val[3].to_le_bytes(),
                    val[4].to_le_bytes(),
                );
                let mut target_bytes: Vec<u8> =
                    val.iter().flat_map(|s| s.to_le_bytes().to_vec()).collect();
                bytes.append(&mut target_bytes);
            }
            Operand::InlineType(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineString(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineField(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineTok(val) => bytes.extend_from_slice(&val.to_le_bytes()),
        }

        bytes
    }
    pub fn length(&self) -> usize {
        self.opcode.length as usize + self.operand.length()
    }
}

pub fn nop() -> Instruction {
    Instruction {
        opcode: NOP,
        operand: Operand::InlineNone,
    }
}
pub fn break_() -> Instruction {
    Instruction {
        opcode: BREAK,
        operand: Operand::InlineNone,
    }
}
pub fn ldarg_0() -> Instruction {
    Instruction {
        opcode: LDARG_0,
        operand: Operand::InlineNone,
    }
}
pub fn ldarg_1() -> Instruction {
    Instruction {
        opcode: LDARG_1,
        operand: Operand::InlineNone,
    }
}
pub fn ldarg_2() -> Instruction {
    Instruction {
        opcode: LDARG_2,
        operand: Operand::InlineNone,
    }
}
pub fn ldarg_3() -> Instruction {
    Instruction {
        opcode: LDARG_3,
        operand: Operand::InlineNone,
    }
}
pub fn ldloc_0() -> Instruction {
    Instruction {
        opcode: LDLOC_0,
        operand: Operand::InlineNone,
    }
}
pub fn ldloc_1() -> Instruction {
    Instruction {
        opcode: LDLOC_1,
        operand: Operand::InlineNone,
    }
}
pub fn ldloc_2() -> Instruction {
    Instruction {
        opcode: LDLOC_2,
        operand: Operand::InlineNone,
    }
}
pub fn ldloc_3() -> Instruction {
    Instruction {
        opcode: LDLOC_3,
        operand: Operand::InlineNone,
    }
}
pub fn stloc_0() -> Instruction {
    Instruction {
        opcode: STLOC_0,
        operand: Operand::InlineNone,
    }
}
pub fn stloc_1() -> Instruction {
    Instruction {
        opcode: STLOC_1,
        operand: Operand::InlineNone,
    }
}
pub fn stloc_2() -> Instruction {
    Instruction {
        opcode: STLOC_2,
        operand: Operand::InlineNone,
    }
}
pub fn stloc_3() -> Instruction {
    Instruction {
        opcode: STLOC_3,
        operand: Operand::InlineNone,
    }
}
pub fn ldarg_s(val: u8) -> Instruction {
    Instruction {
        opcode: LDARG_S,
        operand: Operand::ShortInlineVar(val),
    }
}
pub fn ldarga_s(val: u8) -> Instruction {
    Instruction {
        opcode: LDARGA_S,
        operand: Operand::ShortInlineVar(val),
    }
}
pub fn starg_s(val: u8) -> Instruction {
    Instruction {
        opcode: STARG_S,
        operand: Operand::ShortInlineVar(val),
    }
}
pub fn ldloc_s(val: u8) -> Instruction {
    Instruction {
        opcode: LDLOC_S,
        operand: Operand::ShortInlineVar(val),
    }
}
pub fn ldloca_s(val: u8) -> Instruction {
    Instruction {
        opcode: LDLOCA_S,
        operand: Operand::ShortInlineVar(val),
    }
}
pub fn stloc_s(val: u8) -> Instruction {
    Instruction {
        opcode: STLOC_S,
        operand: Operand::ShortInlineVar(val),
    }
}
pub fn ldnull() -> Instruction {
    Instruction {
        opcode: LDNULL,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_m1() -> Instruction {
    Instruction {
        opcode: LDC_I4_M1,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_0() -> Instruction {
    Instruction {
        opcode: LDC_I4_0,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_1() -> Instruction {
    Instruction {
        opcode: LDC_I4_1,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_2() -> Instruction {
    Instruction {
        opcode: LDC_I4_2,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_3() -> Instruction {
    Instruction {
        opcode: LDC_I4_3,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_4() -> Instruction {
    Instruction {
        opcode: LDC_I4_4,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_5() -> Instruction {
    Instruction {
        opcode: LDC_I4_5,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_6() -> Instruction {
    Instruction {
        opcode: LDC_I4_6,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_7() -> Instruction {
    Instruction {
        opcode: LDC_I4_7,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_8() -> Instruction {
    Instruction {
        opcode: LDC_I4_8,
        operand: Operand::InlineNone,
    }
}
pub fn ldc_i4_s(val: u8) -> Instruction {
    Instruction {
        opcode: LDC_I4_S,
        operand: Operand::ShortInlineI(val),
    }
}
pub fn ldc_i4(val: i32) -> Instruction {
    Instruction {
        opcode: LDC_I4,
        operand: Operand::InlineI(val),
    }
}
pub fn ldc_i8(val: i64) -> Instruction {
    Instruction {
        opcode: LDC_I8,
        operand: Operand::InlineI8(val),
    }
}
pub fn ldc_r4(val: f32) -> Instruction {
    Instruction {
        opcode: LDC_R4,
        operand: Operand::ShortInlineR(val),
    }
}
pub fn ldc_r8(val: f64) -> Instruction {
    Instruction {
        opcode: LDC_R8,
        operand: Operand::InlineR(val),
    }
}
pub fn dup() -> Instruction {
    Instruction {
        opcode: DUP,
        operand: Operand::InlineNone,
    }
}
pub fn pop() -> Instruction {
    Instruction {
        opcode: POP,
        operand: Operand::InlineNone,
    }
}
pub fn jmp(val: u32) -> Instruction {
    Instruction {
        opcode: JMP,
        operand: Operand::InlineMethod(val),
    }
}
pub fn call(val: u32) -> Instruction {
    Instruction {
        opcode: CALL,
        operand: Operand::InlineMethod(val),
    }
}
pub fn calli(val: u32) -> Instruction {
    Instruction {
        opcode: CALLI,
        operand: Operand::InlineSig(val),
    }
}
pub fn ret() -> Instruction {
    Instruction {
        opcode: RET,
        operand: Operand::InlineNone,
    }
}
pub fn br_s(val: i8) -> Instruction {
    Instruction {
        opcode: BR_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn brfalse_s(val: i8) -> Instruction {
    Instruction {
        opcode: BRFALSE_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn brtrue_s(val: i8) -> Instruction {
    Instruction {
        opcode: BRTRUE_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn beq_s(val: i8) -> Instruction {
    Instruction {
        opcode: BEQ_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn bge_s(val: i8) -> Instruction {
    Instruction {
        opcode: BGE_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn bgt_s(val: i8) -> Instruction {
    Instruction {
        opcode: BGT_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn ble_s(val: i8) -> Instruction {
    Instruction {
        opcode: BLE_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn blt_s(val: i8) -> Instruction {
    Instruction {
        opcode: BLT_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn bne_un_s(val: i8) -> Instruction {
    Instruction {
        opcode: BNE_UN_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn bge_un_s(val: i8) -> Instruction {
    Instruction {
        opcode: BGE_UN_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn bgt_un_s(val: i8) -> Instruction {
    Instruction {
        opcode: BGT_UN_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn ble_un_s(val: i8) -> Instruction {
    Instruction {
        opcode: BLE_UN_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn blt_un_s(val: i8) -> Instruction {
    Instruction {
        opcode: BLT_UN_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn br(val: i32) -> Instruction {
    Instruction {
        opcode: BR,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn brfalse(val: i32) -> Instruction {
    Instruction {
        opcode: BRFALSE,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn brtrue(val: i32) -> Instruction {
    Instruction {
        opcode: BRTRUE,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn beq(val: i32) -> Instruction {
    Instruction {
        opcode: BEQ,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn bge(val: i32) -> Instruction {
    Instruction {
        opcode: BGE,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn bgt(val: i32) -> Instruction {
    Instruction {
        opcode: BGT,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn ble(val: i32) -> Instruction {
    Instruction {
        opcode: BLE,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn blt(val: i32) -> Instruction {
    Instruction {
        opcode: BLT,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn bne_un(val: i32) -> Instruction {
    Instruction {
        opcode: BNE_UN,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn bge_un(val: i32) -> Instruction {
    Instruction {
        opcode: BGE_UN,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn bgt_un(val: i32) -> Instruction {
    Instruction {
        opcode: BGT_UN,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn ble_un(val: i32) -> Instruction {
    Instruction {
        opcode: BLE_UN,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn blt_un(val: i32) -> Instruction {
    Instruction {
        opcode: BLT_UN,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn switch(length: u32, targets: Vec<i32>) -> Instruction {
    Instruction {
        opcode: SWITCH,
        operand: Operand::InlineSwitch(length, targets),
    }
}
pub fn ldind_i1() -> Instruction {
    Instruction {
        opcode: LDIND_I1,
        operand: Operand::InlineNone,
    }
}
pub fn ldind_u1() -> Instruction {
    Instruction {
        opcode: LDIND_U1,
        operand: Operand::InlineNone,
    }
}
pub fn ldind_i2() -> Instruction {
    Instruction {
        opcode: LDIND_I2,
        operand: Operand::InlineNone,
    }
}
pub fn ldind_u2() -> Instruction {
    Instruction {
        opcode: LDIND_U2,
        operand: Operand::InlineNone,
    }
}
pub fn ldind_i4() -> Instruction {
    Instruction {
        opcode: LDIND_I4,
        operand: Operand::InlineNone,
    }
}
pub fn ldind_u4() -> Instruction {
    Instruction {
        opcode: LDIND_U4,
        operand: Operand::InlineNone,
    }
}
pub fn ldind_i8() -> Instruction {
    Instruction {
        opcode: LDIND_I8,
        operand: Operand::InlineNone,
    }
}
pub fn ldind_i() -> Instruction {
    Instruction {
        opcode: LDIND_I,
        operand: Operand::InlineNone,
    }
}
pub fn ldind_r4() -> Instruction {
    Instruction {
        opcode: LDIND_R4,
        operand: Operand::InlineNone,
    }
}
pub fn ldind_r8() -> Instruction {
    Instruction {
        opcode: LDIND_R8,
        operand: Operand::InlineNone,
    }
}
pub fn ldind_ref() -> Instruction {
    Instruction {
        opcode: LDIND_REF,
        operand: Operand::InlineNone,
    }
}
pub fn stind_ref() -> Instruction {
    Instruction {
        opcode: STIND_REF,
        operand: Operand::InlineNone,
    }
}
pub fn stind_i1() -> Instruction {
    Instruction {
        opcode: STIND_I1,
        operand: Operand::InlineNone,
    }
}
pub fn stind_i2() -> Instruction {
    Instruction {
        opcode: STIND_I2,
        operand: Operand::InlineNone,
    }
}
pub fn stind_i4() -> Instruction {
    Instruction {
        opcode: STIND_I4,
        operand: Operand::InlineNone,
    }
}
pub fn stind_i8() -> Instruction {
    Instruction {
        opcode: STIND_I8,
        operand: Operand::InlineNone,
    }
}
pub fn stind_r4() -> Instruction {
    Instruction {
        opcode: STIND_R4,
        operand: Operand::InlineNone,
    }
}
pub fn stind_r8() -> Instruction {
    Instruction {
        opcode: STIND_R8,
        operand: Operand::InlineNone,
    }
}
pub fn add() -> Instruction {
    Instruction {
        opcode: ADD,
        operand: Operand::InlineNone,
    }
}
pub fn sub() -> Instruction {
    Instruction {
        opcode: SUB,
        operand: Operand::InlineNone,
    }
}
pub fn mul() -> Instruction {
    Instruction {
        opcode: MUL,
        operand: Operand::InlineNone,
    }
}
pub fn div() -> Instruction {
    Instruction {
        opcode: DIV,
        operand: Operand::InlineNone,
    }
}
pub fn div_un() -> Instruction {
    Instruction {
        opcode: DIV_UN,
        operand: Operand::InlineNone,
    }
}
pub fn rem() -> Instruction {
    Instruction {
        opcode: REM,
        operand: Operand::InlineNone,
    }
}
pub fn rem_un() -> Instruction {
    Instruction {
        opcode: REM_UN,
        operand: Operand::InlineNone,
    }
}
pub fn and() -> Instruction {
    Instruction {
        opcode: AND,
        operand: Operand::InlineNone,
    }
}
pub fn or() -> Instruction {
    Instruction {
        opcode: OR,
        operand: Operand::InlineNone,
    }
}
pub fn xor() -> Instruction {
    Instruction {
        opcode: XOR,
        operand: Operand::InlineNone,
    }
}
pub fn shl() -> Instruction {
    Instruction {
        opcode: SHL,
        operand: Operand::InlineNone,
    }
}
pub fn shr() -> Instruction {
    Instruction {
        opcode: SHR,
        operand: Operand::InlineNone,
    }
}
pub fn shr_un() -> Instruction {
    Instruction {
        opcode: SHR_UN,
        operand: Operand::InlineNone,
    }
}
pub fn neg() -> Instruction {
    Instruction {
        opcode: NEG,
        operand: Operand::InlineNone,
    }
}
pub fn not() -> Instruction {
    Instruction {
        opcode: NOT,
        operand: Operand::InlineNone,
    }
}
pub fn conv_i1() -> Instruction {
    Instruction {
        opcode: CONV_I1,
        operand: Operand::InlineNone,
    }
}
pub fn conv_i2() -> Instruction {
    Instruction {
        opcode: CONV_I2,
        operand: Operand::InlineNone,
    }
}
pub fn conv_i4() -> Instruction {
    Instruction {
        opcode: CONV_I4,
        operand: Operand::InlineNone,
    }
}
pub fn conv_i8() -> Instruction {
    Instruction {
        opcode: CONV_I8,
        operand: Operand::InlineNone,
    }
}
pub fn conv_r4() -> Instruction {
    Instruction {
        opcode: CONV_R4,
        operand: Operand::InlineNone,
    }
}
pub fn conv_r8() -> Instruction {
    Instruction {
        opcode: CONV_R8,
        operand: Operand::InlineNone,
    }
}
pub fn conv_u4() -> Instruction {
    Instruction {
        opcode: CONV_U4,
        operand: Operand::InlineNone,
    }
}
pub fn conv_u8() -> Instruction {
    Instruction {
        opcode: CONV_U8,
        operand: Operand::InlineNone,
    }
}
pub fn callvirt(val: u32) -> Instruction {
    Instruction {
        opcode: CALLVIRT,
        operand: Operand::InlineMethod(val),
    }
}
pub fn cpobj(val: u32) -> Instruction {
    Instruction {
        opcode: CPOBJ,
        operand: Operand::InlineType(val),
    }
}
pub fn ldobj(val: u32) -> Instruction {
    Instruction {
        opcode: LDOBJ,
        operand: Operand::InlineType(val),
    }
}
pub fn ldstr(val: u32) -> Instruction {
    Instruction {
        opcode: LDSTR,
        operand: Operand::InlineString(val),
    }
}
pub fn newobj(val: u32) -> Instruction {
    Instruction {
        opcode: NEWOBJ,
        operand: Operand::InlineMethod(val),
    }
}
pub fn castclass(val: u32) -> Instruction {
    Instruction {
        opcode: CASTCLASS,
        operand: Operand::InlineType(val),
    }
}
pub fn isinst(val: u32) -> Instruction {
    Instruction {
        opcode: ISINST,
        operand: Operand::InlineType(val),
    }
}
pub fn conv_r_un() -> Instruction {
    Instruction {
        opcode: CONV_R_UN,
        operand: Operand::InlineNone,
    }
}
pub fn unbox(val: u32) -> Instruction {
    Instruction {
        opcode: UNBOX,
        operand: Operand::InlineType(val),
    }
}
pub fn throw() -> Instruction {
    Instruction {
        opcode: THROW,
        operand: Operand::InlineNone,
    }
}
pub fn ldfld(val: u32) -> Instruction {
    Instruction {
        opcode: LDFLD,
        operand: Operand::InlineField(val),
    }
}
pub fn ldflda(val: u32) -> Instruction {
    Instruction {
        opcode: LDFLDA,
        operand: Operand::InlineField(val),
    }
}
pub fn stfld(val: u32) -> Instruction {
    Instruction {
        opcode: STFLD,
        operand: Operand::InlineField(val),
    }
}
pub fn ldsfld(val: u32) -> Instruction {
    Instruction {
        opcode: LDSFLD,
        operand: Operand::InlineField(val),
    }
}
pub fn ldsflda(val: u32) -> Instruction {
    Instruction {
        opcode: LDSFLDA,
        operand: Operand::InlineField(val),
    }
}
pub fn stsfld(val: u32) -> Instruction {
    Instruction {
        opcode: STSFLD,
        operand: Operand::InlineField(val),
    }
}
pub fn stobj(val: u32) -> Instruction {
    Instruction {
        opcode: STOBJ,
        operand: Operand::InlineType(val),
    }
}
pub fn conv_ovf_i1_un() -> Instruction {
    Instruction {
        opcode: CONV_OVF_I1_UN,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_i2_un() -> Instruction {
    Instruction {
        opcode: CONV_OVF_I2_UN,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_i4_un() -> Instruction {
    Instruction {
        opcode: CONV_OVF_I4_UN,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_i8_un() -> Instruction {
    Instruction {
        opcode: CONV_OVF_I8_UN,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_u1_un() -> Instruction {
    Instruction {
        opcode: CONV_OVF_U1_UN,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_u2_un() -> Instruction {
    Instruction {
        opcode: CONV_OVF_U2_UN,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_u4_un() -> Instruction {
    Instruction {
        opcode: CONV_OVF_U4_UN,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_u8_un() -> Instruction {
    Instruction {
        opcode: CONV_OVF_U8_UN,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_i_un() -> Instruction {
    Instruction {
        opcode: CONV_OVF_I_UN,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_u_un() -> Instruction {
    Instruction {
        opcode: CONV_OVF_U_UN,
        operand: Operand::InlineNone,
    }
}
pub fn box_(val: u32) -> Instruction {
    Instruction {
        opcode: BOX,
        operand: Operand::InlineType(val),
    }
}
pub fn newarr(val: u32) -> Instruction {
    Instruction {
        opcode: NEWARR,
        operand: Operand::InlineType(val),
    }
}
pub fn ldlen() -> Instruction {
    Instruction {
        opcode: LDLEN,
        operand: Operand::InlineNone,
    }
}
pub fn ldelema(val: u32) -> Instruction {
    Instruction {
        opcode: LDELEMA,
        operand: Operand::InlineType(val),
    }
}
pub fn ldelem_i1() -> Instruction {
    Instruction {
        opcode: LDELEM_I1,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem_u1() -> Instruction {
    Instruction {
        opcode: LDELEM_U1,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem_i2() -> Instruction {
    Instruction {
        opcode: LDELEM_I2,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem_u2() -> Instruction {
    Instruction {
        opcode: LDELEM_U2,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem_i4() -> Instruction {
    Instruction {
        opcode: LDELEM_I4,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem_u4() -> Instruction {
    Instruction {
        opcode: LDELEM_U4,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem_i8() -> Instruction {
    Instruction {
        opcode: LDELEM_I8,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem_i() -> Instruction {
    Instruction {
        opcode: LDELEM_I,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem_r4() -> Instruction {
    Instruction {
        opcode: LDELEM_R4,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem_r8() -> Instruction {
    Instruction {
        opcode: LDELEM_R8,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem_ref() -> Instruction {
    Instruction {
        opcode: LDELEM_REF,
        operand: Operand::InlineNone,
    }
}
pub fn stelem_i() -> Instruction {
    Instruction {
        opcode: STELEM_I,
        operand: Operand::InlineNone,
    }
}
pub fn stelem_i1() -> Instruction {
    Instruction {
        opcode: STELEM_I1,
        operand: Operand::InlineNone,
    }
}
pub fn stelem_i2() -> Instruction {
    Instruction {
        opcode: STELEM_I2,
        operand: Operand::InlineNone,
    }
}
pub fn stelem_i4() -> Instruction {
    Instruction {
        opcode: STELEM_I4,
        operand: Operand::InlineNone,
    }
}
pub fn stelem_i8() -> Instruction {
    Instruction {
        opcode: STELEM_I8,
        operand: Operand::InlineNone,
    }
}
pub fn stelem_r4() -> Instruction {
    Instruction {
        opcode: STELEM_R4,
        operand: Operand::InlineNone,
    }
}
pub fn stelem_r8() -> Instruction {
    Instruction {
        opcode: STELEM_R8,
        operand: Operand::InlineNone,
    }
}
pub fn stelem_ref() -> Instruction {
    Instruction {
        opcode: STELEM_REF,
        operand: Operand::InlineNone,
    }
}
pub fn ldelem(val: u32) -> Instruction {
    Instruction {
        opcode: LDELEM,
        operand: Operand::InlineType(val),
    }
}
pub fn stelem(val: u32) -> Instruction {
    Instruction {
        opcode: STELEM,
        operand: Operand::InlineType(val),
    }
}
pub fn unbox_any(val: u32) -> Instruction {
    Instruction {
        opcode: UNBOX_ANY,
        operand: Operand::InlineType(val),
    }
}
pub fn conv_ovf_i1() -> Instruction {
    Instruction {
        opcode: CONV_OVF_I1,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_u1() -> Instruction {
    Instruction {
        opcode: CONV_OVF_U1,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_i2() -> Instruction {
    Instruction {
        opcode: CONV_OVF_I2,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_u2() -> Instruction {
    Instruction {
        opcode: CONV_OVF_U2,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_i4() -> Instruction {
    Instruction {
        opcode: CONV_OVF_I4,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_u4() -> Instruction {
    Instruction {
        opcode: CONV_OVF_U4,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_i8() -> Instruction {
    Instruction {
        opcode: CONV_OVF_I8,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_u8() -> Instruction {
    Instruction {
        opcode: CONV_OVF_U8,
        operand: Operand::InlineNone,
    }
}
pub fn refanyval(val: u32) -> Instruction {
    Instruction {
        opcode: REFANYVAL,
        operand: Operand::InlineType(val),
    }
}
pub fn ckfinite() -> Instruction {
    Instruction {
        opcode: CKFINITE,
        operand: Operand::InlineNone,
    }
}
pub fn mkrefany(val: u32) -> Instruction {
    Instruction {
        opcode: MKREFANY,
        operand: Operand::InlineType(val),
    }
}
pub fn ldtoken(val: u32) -> Instruction {
    Instruction {
        opcode: LDTOKEN,
        operand: Operand::InlineTok(val),
    }
}
pub fn conv_u2() -> Instruction {
    Instruction {
        opcode: CONV_U2,
        operand: Operand::InlineNone,
    }
}
pub fn conv_u1() -> Instruction {
    Instruction {
        opcode: CONV_U1,
        operand: Operand::InlineNone,
    }
}
pub fn conv_i() -> Instruction {
    Instruction {
        opcode: CONV_I,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_i() -> Instruction {
    Instruction {
        opcode: CONV_OVF_I,
        operand: Operand::InlineNone,
    }
}
pub fn conv_ovf_u() -> Instruction {
    Instruction {
        opcode: CONV_OVF_U,
        operand: Operand::InlineNone,
    }
}
pub fn add_ovf() -> Instruction {
    Instruction {
        opcode: ADD_OVF,
        operand: Operand::InlineNone,
    }
}
pub fn add_ovf_un() -> Instruction {
    Instruction {
        opcode: ADD_OVF_UN,
        operand: Operand::InlineNone,
    }
}
pub fn mul_ovf() -> Instruction {
    Instruction {
        opcode: MUL_OVF,
        operand: Operand::InlineNone,
    }
}
pub fn mul_ovf_un() -> Instruction {
    Instruction {
        opcode: MUL_OVF_UN,
        operand: Operand::InlineNone,
    }
}
pub fn sub_ovf() -> Instruction {
    Instruction {
        opcode: SUB_OVF,
        operand: Operand::InlineNone,
    }
}
pub fn sub_ovf_un() -> Instruction {
    Instruction {
        opcode: SUB_OVF_UN,
        operand: Operand::InlineNone,
    }
}
pub fn endfinally() -> Instruction {
    Instruction {
        opcode: ENDFINALLY,
        operand: Operand::InlineNone,
    }
}
pub fn leave(val: i32) -> Instruction {
    Instruction {
        opcode: LEAVE,
        operand: Operand::InlineBrTarget(val),
    }
}
pub fn leave_s(val: i8) -> Instruction {
    Instruction {
        opcode: LEAVE_S,
        operand: Operand::ShortInlineBrTarget(val),
    }
}
pub fn stind_i() -> Instruction {
    Instruction {
        opcode: STIND_I,
        operand: Operand::InlineNone,
    }
}
pub fn conv_u() -> Instruction {
    Instruction {
        opcode: CONV_U,
        operand: Operand::InlineNone,
    }
}
pub fn arglist() -> Instruction {
    Instruction {
        opcode: ARGLIST,
        operand: Operand::InlineNone,
    }
}
pub fn ceq() -> Instruction {
    Instruction {
        opcode: CEQ,
        operand: Operand::InlineNone,
    }
}
pub fn cgt() -> Instruction {
    Instruction {
        opcode: CGT,
        operand: Operand::InlineNone,
    }
}
pub fn cgt_un() -> Instruction {
    Instruction {
        opcode: CGT_UN,
        operand: Operand::InlineNone,
    }
}
pub fn clt() -> Instruction {
    Instruction {
        opcode: CLT,
        operand: Operand::InlineNone,
    }
}
pub fn clt_un() -> Instruction {
    Instruction {
        opcode: CLT_UN,
        operand: Operand::InlineNone,
    }
}
pub fn ldftn(val: u32) -> Instruction {
    Instruction {
        opcode: LDFTN,
        operand: Operand::InlineMethod(val),
    }
}
pub fn ldvirtftn(val: u32) -> Instruction {
    Instruction {
        opcode: LDVIRTFTN,
        operand: Operand::InlineMethod(val),
    }
}
pub fn ldarg(val: u16) -> Instruction {
    Instruction {
        opcode: LDARG,
        operand: Operand::InlineVar(val),
    }
}
pub fn ldarga(val: u16) -> Instruction {
    Instruction {
        opcode: LDARGA,
        operand: Operand::InlineVar(val),
    }
}
pub fn starg(val: u16) -> Instruction {
    Instruction {
        opcode: STARG,
        operand: Operand::InlineVar(val),
    }
}
pub fn ldloc(val: u16) -> Instruction {
    Instruction {
        opcode: LDLOC,
        operand: Operand::InlineVar(val),
    }
}
pub fn ldloca(val: u16) -> Instruction {
    Instruction {
        opcode: LDLOCA,
        operand: Operand::InlineVar(val),
    }
}
pub fn stloc(val: u16) -> Instruction {
    Instruction {
        opcode: STLOC,
        operand: Operand::InlineVar(val),
    }
}
pub fn localloc() -> Instruction {
    Instruction {
        opcode: LOCALLOC,
        operand: Operand::InlineNone,
    }
}
pub fn endfilter() -> Instruction {
    Instruction {
        opcode: ENDFILTER,
        operand: Operand::InlineNone,
    }
}
pub fn unaligned(val: u8) -> Instruction {
    Instruction {
        opcode: UNALIGNED,
        operand: Operand::ShortInlineI(val),
    }
}
pub fn volatile() -> Instruction {
    Instruction {
        opcode: VOLATILE,
        operand: Operand::InlineNone,
    }
}
pub fn tailcall() -> Instruction {
    Instruction {
        opcode: TAILCALL,
        operand: Operand::InlineNone,
    }
}
pub fn initobj(val: u32) -> Instruction {
    Instruction {
        opcode: INITOBJ,
        operand: Operand::InlineType(val),
    }
}
pub fn constrained(val: u32) -> Instruction {
    Instruction {
        opcode: CONSTRAINED,
        operand: Operand::InlineType(val),
    }
}
pub fn cpblk() -> Instruction {
    Instruction {
        opcode: CPBLK,
        operand: Operand::InlineNone,
    }
}
pub fn initblk() -> Instruction {
    Instruction {
        opcode: INITBLK,
        operand: Operand::InlineNone,
    }
}
pub fn rethrow() -> Instruction {
    Instruction {
        opcode: RETHROW,
        operand: Operand::InlineNone,
    }
}
pub fn sizeof(val: u32) -> Instruction {
    Instruction {
        opcode: SIZEOF,
        operand: Operand::InlineType(val),
    }
}
pub fn refanytype() -> Instruction {
    Instruction {
        opcode: REFANYTYPE,
        operand: Operand::InlineNone,
    }
}
pub fn readonly() -> Instruction {
    Instruction {
        opcode: READONLY,
        operand: Operand::InlineNone,
    }
}
