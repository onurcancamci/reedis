pub trait EventCommand {
    fn is_listen(&self) -> bool;
}
