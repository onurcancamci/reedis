pub trait Command {
    fn is_terminate(&self) -> bool;

    fn is_mutator(&self) -> bool;

    fn get_path(&self) -> Option<&str>;

    fn get_operation(&self) -> Operation;
}

#[derive(Clone, Debug)]
pub enum Operation {
    Get,
    Set,
}
