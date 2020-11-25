pub enum EventCommand {
    Start,
    Listen(String, usize),
}

impl EventCommand {
    pub fn is_start(&self) -> bool {
        if let EventCommand::Start = self {
            true
        } else {
            false
        }
    }

    fn path(&self) -> Option<&str> {
        if let EventCommand::Listen(p, _) = self {
            Some(p)
        } else {
            None
        }
    }

    fn listener(&self) -> Option<&usize> {
        if let EventCommand::Listen(_, l) = self {
            Some(l)
        } else {
            None
        }
    }
}
