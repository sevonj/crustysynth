use std::{error::Error, fmt::Display};

use crate::{
    midi::messages::{ChannelMessage, MidiMessageError, SystemMessage},
    midifile::vlq::{read_vlq, VlqError},
};

#[derive(Debug)]
pub enum MidiEventError {
    IOError { source: std::io::Error },
    VlqError { source: VlqError },
    MessageError { source: MidiMessageError },
}
impl Error for MidiEventError {}
impl Display for MidiEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError { source } => write!(f, "{source}"),
            Self::VlqError { source } => write!(f, "{source}"),
            Self::MessageError { source } => write!(f, "{source}"),
        }
    }
}
impl From<std::io::Error> for MidiEventError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError { source: e }
    }
}
impl From<VlqError> for MidiEventError {
    fn from(e: VlqError) -> Self {
        Self::VlqError { source: e }
    }
}
impl From<MidiMessageError> for MidiEventError {
    fn from(e: MidiMessageError) -> Self {
        Self::MessageError { source: e }
    }
}

#[derive(Clone, Debug)]

pub enum MidiEvent {
    Channel(ChannelMessage),
    System(SystemMessage),
    Meta { meta_type: u8, data: Vec<u8> },
}

impl MidiEvent {
    pub fn read<R>(file: &mut R) -> Result<Self, MidiEventError>
    where
        R: std::io::Read,
    {
        let mut status_byte_buf = [0_u8];
        file.read_exact(&mut status_byte_buf)?;
        let status_byte = status_byte_buf[0];

        match status_byte {
            0x80..=0xEF => Ok(Self::Channel(ChannelMessage::read_with_status(
                status_byte,
                file,
            )?)),
            0xF0..=0xFE => Ok(Self::System(SystemMessage::read_with_status(
                status_byte,
                file,
            )?)),
            0xFF => {
                let mut buf = [0_u8];
                file.read_exact(&mut buf)?;
                let meta_type = buf[0] & 0x7F;
                let len = read_vlq(file)?;
                let mut data = vec![];
                for _ in 0..len {
                    file.read_exact(&mut buf)?;
                    data.push(buf[0]);
                }
                Ok(Self::Meta { meta_type, data })
            }
            _ => Err(MidiMessageError::UnknownCommand(status_byte).into()),
        }
    }

    pub fn get_command(&self) -> u8 {
        match self {
            MidiEvent::Channel(msg) => msg.get_command(),
            MidiEvent::System(msg) => msg.get_command(),
            MidiEvent::Meta { .. } => 0xFF,
        }
    }
}
