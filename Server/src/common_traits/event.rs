use crate::common_traits::*;

pub trait Event {
    type Content;

    fn new(path: &str, op: Operation, target: usize) -> Self;

    fn get_target(&self) -> &[usize];

    fn get_content(&self) -> &Self::Content;
}

pub trait EventContent {}
