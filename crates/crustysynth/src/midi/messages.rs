use std::{error::Error, fmt::Display};

use super::{
    channels::MidiChannel,
    keys::{MidiKey, MidiKeyError},
};

#[derive(Debug)]
pub enum MidiMessageError {
    IOError { source: std::io::Error },
    UnknownCommand(u8),
    InvalidKey { source: MidiKeyError },
}
impl Error for MidiMessageError {}
impl Display for MidiMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError { source } => write!(f, "{source}"),
            Self::UnknownCommand(byte) => write!(f, "Unknown status byte: {byte:#04x}"),
            Self::InvalidKey { source } => write!(f, "{source}"),
        }
    }
}
impl From<std::io::Error> for MidiMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError { source: e }
    }
}
impl From<MidiKeyError> for MidiMessageError {
    fn from(e: MidiKeyError) -> Self {
        Self::InvalidKey { source: e }
    }
}

#[derive(Debug, Clone)]
pub enum ChannelMessage {
    NoteOff {
        channel: MidiChannel,
        key: MidiKey,
        vel: u8,
    },
    NoteOn {
        channel: MidiChannel,
        key: MidiKey,
        vel: u8,
    },
    AfterTouch {
        channel: MidiChannel,
        key: MidiKey,
        pressure: u8,
    },
    ControlChange {
        channel: MidiChannel,
        control: u8,
        value: u8,
    },
    ProgramChange {
        channel: MidiChannel,
        program: u8,
    },
    ChannelPressure {
        channel: MidiChannel,
        value: u8,
    },
    PitchBend {
        channel: MidiChannel,
        value: u16,
    },

    ChannelMode {
        channel: MidiChannel,
        control: u8,
        value: u8,
    },
}
impl ChannelMessage {
    /// Read the entire message
    pub fn read<R>(file: &mut R) -> Result<Self, MidiMessageError>
    where
        R: std::io::Read,
    {
        let mut status_byte_buf = [0_u8];
        file.read_exact(&mut status_byte_buf)?;
        let status_byte = status_byte_buf[0];
        Self::read_with_status(status_byte, file)
    }

    /// For when you have already read the status byte. This expects data bytes immediately.
    pub fn read_with_status<R>(status_byte: u8, file: &mut R) -> Result<Self, MidiMessageError>
    where
        R: std::io::Read,
    {
        match status_byte & 0xF0 {
            0x80 => {
                let mut buf = [0_u8; 2];
                file.read_exact(&mut buf)?;

                Ok(Self::NoteOff {
                    channel: MidiChannel::from(status_byte),
                    key: MidiKey::try_from(buf[0] & 0x7F)?,
                    vel: buf[1] & 0x7F,
                })
            }
            0x90 => {
                let mut buf = [0_u8; 2];
                file.read_exact(&mut buf)?;

                Ok(Self::NoteOn {
                    channel: MidiChannel::from(status_byte),
                    key: MidiKey::try_from(buf[0] & 0x7F)?,
                    vel: buf[1] & 0x7F,
                })
            }
            0xA0 => {
                let mut buf = [0_u8; 2];
                file.read_exact(&mut buf)?;

                Ok(Self::AfterTouch {
                    channel: MidiChannel::from(status_byte),
                    key: MidiKey::try_from(buf[0] & 0x7F)?,
                    pressure: buf[1] & 0x7F,
                })
            }
            0xB0 => {
                let mut buf = [0_u8; 2];
                file.read_exact(&mut buf)?;

                match buf[0] {
                    122..=127 => Ok(Self::ChannelMode {
                        channel: MidiChannel::from(status_byte),
                        control: buf[0] & 0x7F,
                        value: buf[1] & 0x7F,
                    }),

                    _ => Ok(Self::ControlChange {
                        channel: MidiChannel::from(status_byte),
                        control: buf[0] & 0x7F,
                        value: buf[1] & 0x7F,
                    }),
                }
            }
            0xC0 => {
                let mut buf = [0_u8];
                file.read_exact(&mut buf)?;

                Ok(Self::ProgramChange {
                    channel: MidiChannel::from(status_byte),
                    program: buf[0] & 0x7F,
                })
            }
            0xD0 => {
                let mut buf = [0_u8];
                file.read_exact(&mut buf)?;

                Ok(Self::ChannelPressure {
                    channel: MidiChannel::from(status_byte),
                    value: buf[0] & 0x7F,
                })
            }
            0xE0 => {
                let mut buf = [0_u8; 2];
                file.read_exact(&mut buf)?;

                Ok(Self::PitchBend {
                    channel: MidiChannel::from(status_byte),
                    value: ((buf[1] & 0x7F) << 7) as u16 + (buf[0] & 0x7F) as u16,
                })
            }

            _ => Err(MidiMessageError::UnknownCommand(status_byte)),
        }
    }

    pub fn get_command(&self) -> u8 {
        match self {
            Self::NoteOff { .. } => 0x80,
            Self::NoteOn { .. } => 0x90,
            Self::AfterTouch { .. } => 0xA0,
            Self::ControlChange { .. } => 0xB0,
            Self::ProgramChange { .. } => 0xC0,
            Self::ChannelPressure { .. } => 0xD0,
            Self::PitchBend { .. } => 0xE0,

            Self::ChannelMode { .. } => 0xB0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SystemMessage {
    SysEx { id: u8, data: Vec<u8> },
    SongPositionPointer { position: u16 },
    SongSelect { song: u8 },
    TuneRequest,
    EndOfExclusive,

    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,

    Reset,
}
impl SystemMessage {
    /// Read the entire message
    pub fn read<R>(file: &mut R) -> Result<Self, MidiMessageError>
    where
        R: std::io::Read,
    {
        let mut status_byte_buf = [0_u8];
        file.read_exact(&mut status_byte_buf)?;
        let status_byte = status_byte_buf[0];
        Self::read_with_status(status_byte, file)
    }

    /// For when you have already read the status byte. This expects data bytes immediately.
    pub fn read_with_status<R>(status_byte: u8, file: &mut R) -> Result<Self, MidiMessageError>
    where
        R: std::io::Read,
    {
        match status_byte & 0xF0 {
            0xF0 => {
                let mut buf = [0_u8];
                file.read_exact(&mut buf)?;
                let id = buf[0] & 0x7F;
                let mut data = vec![];
                loop {
                    file.read_exact(&mut buf)?;
                    data.push(buf[0]);
                    if buf[0] == 0xF7 {
                        break;
                    }
                }
                Ok(Self::SysEx { id, data })
            }
            0xF2 => {
                let mut buf = [0_u8; 2];
                file.read_exact(&mut buf)?;

                Ok(Self::SongPositionPointer {
                    position: ((buf[1] & 0x7F) << 7) as u16 + (buf[0] & 0x7F) as u16,
                })
            }
            0xF3 => {
                let mut buf = [0_u8];
                file.read_exact(&mut buf)?;

                Ok(Self::SongSelect {
                    song: buf[0] & 0x7F,
                })
            }
            0xF6 => Ok(Self::TuneRequest),
            0xF7 => Ok(Self::EndOfExclusive),
            0xF8 => Ok(Self::TimingClock),
            0xFA => Ok(Self::Start),
            0xFB => Ok(Self::Continue),
            0xFC => Ok(Self::Stop),
            0xFE => Ok(Self::ActiveSensing),
            0xFF => Ok(Self::Reset),

            _ => Err(MidiMessageError::UnknownCommand(status_byte)),
        }
    }

    pub fn get_command(&self) -> u8 {
        match self {
            Self::SysEx { .. } => 0xF0,
            Self::SongPositionPointer { .. } => 0xF2,
            Self::SongSelect { .. } => 0xF3,
            Self::TuneRequest => 0xF6,
            Self::EndOfExclusive => 0xF7,
            Self::TimingClock => 0xF8,
            Self::Start => 0xFA,
            Self::Continue => 0xFB,
            Self::Stop => 0xFC,
            Self::ActiveSensing => 0xFE,
            Self::Reset => 0xFF,
        }
    }
}
