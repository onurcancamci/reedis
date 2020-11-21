pub trait CommandResult {
    fn modified_row_count(&self) -> usize;
}
