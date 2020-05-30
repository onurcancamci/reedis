use crate::*;

pub struct Serializer;

impl Serializer {
    pub fn serialize(cw: &dyn Command) -> AppResult<Vec<u8>> {
        let size = cw.size();
        let mut heap = vec![0u8; size + 8];
        let (size_buf, mut heap_content) = heap.split_at_mut(8);
        size_buf.copy_from_slice(&size.to_le_bytes());
        cw.read_into(&mut heap_content)?;

        Ok(heap)
    }
}
