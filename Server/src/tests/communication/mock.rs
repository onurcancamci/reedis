use crate::common_traits::*;
use crate::error::MyError;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub struct MockCommand {
    terminate: bool,
}

impl Command for MockCommand {
    fn is_terminate(&self) -> bool {
        self.terminate
    }
}

#[derive(Debug, Clone)]
pub struct MockCommandResult {}

impl CommandResult for MockCommandResult {}

#[derive(Debug, Clone)]
pub struct MockEvent {
    targets: Vec<usize>,
    content: MockEventContent,
}

impl Event for MockEvent {
    type Content = MockEventContent;

    fn get_target(&self) -> &[usize] {
        self.targets.as_slice()
    }

    fn get_content(&self) -> &Self::Content {
        &self.content
    }
}

#[derive(Debug, Clone)]
pub struct MockEventCommand {
    listen: bool,
}

impl EventCommand for MockEventCommand {
    fn is_listen(&self) -> bool {
        self.listen
    }
}

#[derive(Debug, Clone)]
pub struct MockEventContent {}

impl EventContent for MockEventContent {}

pub struct MockParser;

impl Parser for MockParser {
    type ParsedCommand = MockCommand;
    type ParsedEvCommand = MockEventCommand;

    fn parse_command(data: &[u8]) -> Result<Self::ParsedCommand, MyError> {
        match data[0] {
            0 => Ok(MockCommand { terminate: false }),
            1 => Ok(MockCommand { terminate: true }),
            _ => Err(MyError::TODO),
        }
    }

    fn parse_ev_command(data: &[u8]) -> Result<Self::ParsedEvCommand, MyError> {
        match data[0] {
            0 => Ok(MockEventCommand { listen: false }),
            1 => Ok(MockEventCommand { listen: true }),
            _ => Err(MyError::TODO),
        }
    }

    fn serialize_ev_content<CO>(_: CO) -> Result<Vec<u8>, MyError>
    where
        CO: EventContent,
    {
        Ok(vec![32u8])
    }

    fn serialize_command_result<CR>(_: CR) -> Result<Vec<u8>, MyError>
    where
        CR: CommandResult,
    {
        Ok(vec![33u8])
    }
}

#[derive(Debug, Clone)]
pub struct MockDatabase {}

impl Database for MockDatabase {
    type CommandResult = MockCommandResult;
    type Event = MockEvent;

    fn run<T>(&mut self, _: T) -> Result<(Self::CommandResult, Vec<Self::Event>), MyError>
    where
        T: Command,
    {
        Ok((
            MockCommandResult {},
            vec![MockEvent {
                targets: vec![0],
                content: MockEventContent {},
            }],
        ))
    }

    fn run_ev_command<EC>(&mut self, _: EC)
    where
        EC: EventCommand,
    {
        //
    }
}

#[derive(Debug, Clone)]
pub struct MockTcpStream {
    pub inp: Vec<u8>,
    pub out: Vec<u8>,
    pub limit: usize,
}

impl MockTcpStream {
    pub fn new(inp: Vec<u8>, limit: usize) -> Self {
        MockTcpStream {
            inp,
            out: vec![],
            limit,
        }
    }
}

impl Read for MockTcpStream {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0);
        }
        if self.inp.len() > buf.len() {
            let len = buf.len();
            let vec: Vec<u8> = self.inp.drain(0..buf.len()).collect();
            buf.write(vec.as_slice())?;
            Ok(len)
        } else {
            let len = self.inp.len();
            let vec: Vec<u8> = self.inp.drain(0..).collect();
            buf.write(vec.as_slice())?;
            Ok(len)
        }
    }
}

impl Write for MockTcpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let remaining = self.limit - self.out.len();
        if buf.len() <= remaining {
            self.out.extend_from_slice(buf);
            Ok(buf.len())
        } else if remaining > 0 {
            self.out.extend_from_slice(&buf[0..remaining]);
            Ok(remaining)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "Limit reached",
            ))
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
