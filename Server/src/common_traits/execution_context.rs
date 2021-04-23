use crate::{Event, EventTable};
use std::sync::mpsc::Sender;

pub trait ExecutionContext<E>
where
    E: Event,
{
    type EventTable: EventTable;

    fn tx_event(&self) -> &Sender<E>;
    fn event_table(&self) -> &Self::EventTable;
}
