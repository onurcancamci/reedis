pub trait EventTable {
    fn listen(&mut self, path: &str, listener: usize);
    fn unlisten(&mut self, path: &str, listener: usize);
    fn unlisten_listener(&mut self, listener: usize);
    //fn unlisten_path(&mut self, path: &str); //TODO: might not be needed

    fn lookup<'a>(&'a self, path: &str) -> Box<dyn Iterator<Item = usize> + 'a>;
}
