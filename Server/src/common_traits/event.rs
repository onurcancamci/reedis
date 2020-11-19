pub trait Event {
    type Content;

    fn get_target(&self) -> &[usize];

    fn get_content(&self) -> &Self::Content;
}

pub trait EventContent {}
