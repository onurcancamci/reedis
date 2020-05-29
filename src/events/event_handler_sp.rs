use crate::*;
//use tokio::runtime;
//use tokio::sync::mpsc::Sender;

pub struct EventHandlerMPSC {
    sender: Sender<CommandInto>,
}

impl EventHandlerMPSC {
    pub fn new<'a>(sender: Sender<CommandInto>) -> EventHandlerMPSC {
        EventHandlerMPSC { sender }
    }
}

//#[async_trait]
impl EventHandler<'_> for EventHandlerMPSC {
    /* async fn trigger(&mut self, value: &CommandInto) -> AppResult<()> {
        self.sender
            .send(value.clone())
            .await
            .map_err(|_| AppError::ChannelWriteError)
    } */
}
