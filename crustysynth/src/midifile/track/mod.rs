pub mod event;

use event::{MidiEvent, MidiEventError};

use crate::vlq::read_vlq;

use super::chunks::{MidiChunk, MidiChunkType};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MidiTrackError {
    IOError { source: std::io::Error },
    InvalidChunkType(MidiChunkType),
    Event { source: MidiEventError },
}
impl Error for MidiTrackError {}
impl Display for MidiTrackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError { source } => write!(f, "{source}"),
            Self::InvalidChunkType(chunk_type) => {
                write!(f, "Chunk is not a track chunk, but a {chunk_type:?}")
            }
            Self::Event { source } => write!(f, "{source}"),
        }
    }
}
impl From<std::io::Error> for MidiTrackError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError { source: e }
    }
}
impl From<MidiEventError> for MidiTrackError {
    fn from(e: MidiEventError) -> Self {
        Self::Event { source: e }
    }
}

#[derive(Debug)]
pub struct MidiTrack {
    track_events: Vec<MidiTrackEvent>,
}
impl TryFrom<MidiChunk> for MidiTrack {
    type Error = MidiTrackError;

    fn try_from(chunk: MidiChunk) -> Result<Self, Self::Error> {
        if chunk.get_type() != MidiChunkType::MTrk {
            return Err(MidiTrackError::InvalidChunkType(chunk.get_type()));
        }

        let mut slice = chunk.get_data().as_slice();
        let mut track_events = vec![];

        while !slice.is_empty() {
            let track_event = MidiTrackEvent::read(&mut slice)?;
            track_events.push(track_event);
        }

        Ok(Self { track_events })
    }
}
impl MidiTrack {
    pub fn get_events(&self) -> &Vec<MidiTrackEvent> {
        &self.track_events
    }
}

#[derive(Debug)]
pub struct MidiTrackEvent {
    delta_time: usize,
    event: MidiEvent,
}
impl MidiTrackEvent {
    pub fn read<R>(file: &mut R) -> Result<Self, MidiTrackError>
    where
        R: std::io::Read,
    {
        let delta_time = read_vlq(file)?;

        let event = MidiEvent::read(file)?;

        Ok(Self { delta_time, event })
    }

    pub fn get_delta_time(&self) -> usize {
        self.delta_time
    }
    pub fn get_event(&self) -> &MidiEvent {
        &self.event
    }
}
