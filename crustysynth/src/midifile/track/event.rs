use std::{error::Error, fmt::Display};

use crate::{
    midi::{
        channels::MidiChannel,
        keys::{MidiKey, MidiKeyError},
    },
    midifile::vlq::read_vlq,
};

#[derive(Debug)]
pub enum MidiEventError {
    IOError { source: std::io::Error },
    UnknownStatusByte(u8),
    InvalidKey { source: MidiKeyError },
}
impl Error for MidiEventError {}
impl Display for MidiEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError { source } => write!(f, "{source}"),
            Self::UnknownStatusByte(byte) => write!(f, "Unknown status byte: {byte:#04x}"),
            Self::InvalidKey { source } => write!(f, "{source}"),
        }
    }
}
impl From<std::io::Error> for MidiEventError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError { source: e }
    }
}
impl From<MidiKeyError> for MidiEventError {
    fn from(e: MidiKeyError) -> Self {
        Self::InvalidKey { source: e }
    }
}

#[derive(Debug)]

pub enum MidiEvent {
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

    SysEx {
        id: u8,
        data: Vec<u8>,
    },
    SongPositionPointer {
        position: u16,
    },
    SongSelect {
        song: u8,
    },
    TuneRequest,
    EndOfExclusive,

    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,

    Meta {
        meta_type: u8,
        data: Vec<u8>,
    },
}

impl MidiEvent {
    pub fn read<R>(file: &mut R) -> Result<Self, MidiEventError>
    where
        R: std::io::Read,
    {
        let mut status_byte_buf = [0_u8];
        file.read_exact(&mut status_byte_buf)?;
        let status_byte = status_byte_buf[0];

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

                Ok(Self::NoteOff {
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
            0xF0 => match status_byte {
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
                _ => Err(MidiEventError::UnknownStatusByte(status_byte)),
            },
            _ => Err(MidiEventError::UnknownStatusByte(status_byte)),
        }
    }
}
