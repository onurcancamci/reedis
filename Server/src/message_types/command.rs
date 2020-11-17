pub trait Command {
    fn is_terminate(&self) -> bool;
}
