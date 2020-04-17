use crate::*;
pub fn usize_from(buf: &[u8]) -> usize {
    let mut len_buf = [0u8; std::mem::size_of::<usize>()];
    len_buf.copy_from_slice(&buf[0..std::mem::size_of::<usize>()]);
    usize::from_le_bytes(len_buf)
}
pub fn i64_from(buf: &[u8]) -> i64 {
    let mut len_buf = [0u8; std::mem::size_of::<i64>()];
    len_buf.copy_from_slice(&buf[0..std::mem::size_of::<i64>()]);
    i64::from_le_bytes(len_buf)
}
pub fn f64_from(buf: &[u8]) -> f64 {
    let mut len_buf = [0u8; std::mem::size_of::<f64>()];
    len_buf.copy_from_slice(&buf[0..std::mem::size_of::<f64>()]);
    f64::from_le_bytes(len_buf)
}
pub fn u16_from(buf: &[u8]) -> u16 {
    let mut len_buf = [0u8; std::mem::size_of::<u16>()];
    len_buf.copy_from_slice(&buf[0..std::mem::size_of::<u16>()]);
    u16::from_le_bytes(len_buf)
}
pub fn u32_from(buf: &[u8]) -> u32 {
    let mut len_buf = [0u8; std::mem::size_of::<u32>()];
    len_buf.copy_from_slice(&buf[0..std::mem::size_of::<u32>()]);
    u32::from_le_bytes(len_buf)
}
pub fn string_from(buf: &[u8]) -> AppResult<String> {
    Ok(std::str::from_utf8(buf)
        .map_err(|_| AppError::InvalidString)?
        .to_string())
}
