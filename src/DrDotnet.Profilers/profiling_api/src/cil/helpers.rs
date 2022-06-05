use crate::cil::Error;

pub fn il_u8(il: &[u8], index: usize) -> Result<u8, Error> {
    il.get(index).ok_or(Error::InvalidCil).map(|v| *v)
}
pub fn il_u16(il: &[u8], index: usize) -> Result<u16, Error> {
    let byte_1 = il_u8(il, index)?;
    let byte_2 = il_u8(il, index + 1)?;
    Ok(u16::from_le_bytes([byte_1, byte_2]))
}
pub fn il_u32(il: &[u8], index: usize) -> Result<u32, Error> {
    let byte_1 = il_u8(il, index)?;
    let byte_2 = il_u8(il, index + 1)?;
    let byte_3 = il_u8(il, index + 2)?;
    let byte_4 = il_u8(il, index + 3)?;
    Ok(u32::from_le_bytes([byte_1, byte_2, byte_3, byte_4]))
}
pub fn il_i8(il: &[u8], index: usize) -> Result<i8, Error> {
    let byte = il_u8(il, index)?;
    Ok(i8::from_le_bytes([byte]))
}
pub fn il_i16(il: &[u8], index: usize) -> Result<i16, Error> {
    let byte_1 = il_u8(il, index)?;
    let byte_2 = il_u8(il, index + 1)?;
    Ok(i16::from_le_bytes([byte_1, byte_2]))
}
pub fn il_i32(il: &[u8], index: usize) -> Result<i32, Error> {
    let byte_1 = il_u8(il, index)?;
    let byte_2 = il_u8(il, index + 1)?;
    let byte_3 = il_u8(il, index + 2)?;
    let byte_4 = il_u8(il, index + 3)?;
    Ok(i32::from_le_bytes([byte_1, byte_2, byte_3, byte_4]))
}
pub fn il_i64(il: &[u8], index: usize) -> Result<i64, Error> {
    let byte_1 = il_u8(il, index)?;
    let byte_2 = il_u8(il, index + 1)?;
    let byte_3 = il_u8(il, index + 2)?;
    let byte_4 = il_u8(il, index + 3)?;
    let byte_5 = il_u8(il, index + 4)?;
    let byte_6 = il_u8(il, index + 5)?;
    let byte_7 = il_u8(il, index + 6)?;
    let byte_8 = il_u8(il, index + 7)?;
    Ok(i64::from_le_bytes([
        byte_1, byte_2, byte_3, byte_4, byte_5, byte_6, byte_7, byte_8,
    ]))
}
pub fn il_f32(il: &[u8], index: usize) -> Result<f32, Error> {
    let byte_1 = il_u8(il, index)?;
    let byte_2 = il_u8(il, index + 1)?;
    let byte_3 = il_u8(il, index + 2)?;
    let byte_4 = il_u8(il, index + 3)?;
    Ok(f32::from_le_bytes([byte_1, byte_2, byte_3, byte_4]))
}
pub fn il_f64(il: &[u8], index: usize) -> Result<f64, Error> {
    let byte_1 = il_u8(il, index)?;
    let byte_2 = il_u8(il, index + 1)?;
    let byte_3 = il_u8(il, index + 2)?;
    let byte_4 = il_u8(il, index + 3)?;
    let byte_5 = il_u8(il, index + 4)?;
    let byte_6 = il_u8(il, index + 5)?;
    let byte_7 = il_u8(il, index + 6)?;
    let byte_8 = il_u8(il, index + 7)?;
    Ok(f64::from_le_bytes([
        byte_1, byte_2, byte_3, byte_4, byte_5, byte_6, byte_7, byte_8,
    ]))
}
pub fn check_flag(flags: u8, flag: u8) -> bool {
    (flags & flag) == flag
}
pub fn nearest_multiple(multiple: usize, value: usize) -> usize {
    value + (multiple - 1) & !(multiple - 1)
}
