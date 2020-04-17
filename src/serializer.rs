use crate::*;

pub struct Serializer;

impl Serializer {
    pub fn serialize(cw: &dyn Command) -> AppResult<Vec<u8>> {
        let mut heap = vec![0u8; cw.size()];
        cw.read_into(&mut heap.as_mut_slice())?;
        Ok(heap)
    }
}
