use std::{error::Error, fmt::Display, fs::File, io::BufReader};

use chunks::{MidiChunk, MidiChunkError, MidiChunkType};
use track::{MidiTrack, MidiTrackError};

pub mod chunks;
pub mod track;
pub mod vlq;

#[derive(Debug)]
pub enum MidiFileError {
    IOError { source: std::io::Error },
    ChunkError { source: MidiChunkError },
    TrackError { source: MidiTrackError },
    NoHeader,
    MultipleHeaders,
    UnknownFormat(u16),
}
impl Error for MidiFileError {}
impl Display for MidiFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError { source } => write!(f, "{source}"),
            Self::ChunkError { source } => write!(f, "{source}"),
            Self::TrackError { source } => write!(f, "{source}"),
            Self::NoHeader => write!(f, "Midi file did not start with a header chunk."),
            Self::MultipleHeaders => write!(f, "Midi file contains multople header chunks."),
            Self::UnknownFormat(format) => write!(f, "Midi file states unknown format: {format}"),
        }
    }
}
impl From<std::io::Error> for MidiFileError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError { source: e }
    }
}
impl From<MidiChunkError> for MidiFileError {
    fn from(e: MidiChunkError) -> Self {
        Self::ChunkError { source: e }
    }
}
impl From<MidiTrackError> for MidiFileError {
    fn from(e: MidiTrackError) -> Self {
        Self::TrackError { source: e }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MidiFileFormat {
    SingleTrack = 0,
    MultiTrack = 1,
    MultiTrackAsync = 2,
}
impl TryFrom<u16> for MidiFileFormat {
    type Error = MidiFileError;

    fn try_from(buffer: u16) -> Result<Self, MidiFileError> {
        match buffer {
            0 => Ok(Self::SingleTrack),
            1 => Ok(Self::MultiTrack),
            2 => Ok(Self::MultiTrackAsync),
            _ => Err(MidiFileError::UnknownFormat(buffer)),
        }
    }
}

#[derive(Debug)]
pub struct MidiFile {
    format: MidiFileFormat,
    ntrks: u16,
    division: u16,
    tracks: Vec<MidiTrack>,
}
impl Display for MidiFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "MidiFile")?;
        writeln!(f, "  format:      {:?}", self.format)?;
        writeln!(f, "  ntrks:       {:?}", self.ntrks)?;
        writeln!(f, "  division:    {:?}", self.division)?;
        writeln!(f, "  tracks:")?;
        for track in &self.tracks {
            writeln!(f, "{:?}", track)?;
        }
        Ok(())
    }
}
impl TryFrom<File> for MidiFile {
    type Error = MidiFileError;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(file);

        let header_chunk = MidiChunk::read(&mut reader)?;
        if header_chunk.get_type() != MidiChunkType::MThd {
            return Err(MidiFileError::NoHeader);
        }
        let header_data = header_chunk.get_data();

        let format =
            MidiFileFormat::try_from(u16::from_be_bytes(header_data[0..2].try_into().unwrap()))?;
        let ntrks = u16::from_be_bytes(header_data[2..4].try_into().unwrap());
        let division = u16::from_be_bytes(header_data[4..6].try_into().unwrap());

        let mut tracks = vec![];
        for _ in 0..ntrks {
            match MidiChunk::read(&mut reader) {
                Ok(chunk) => match chunk.get_type() {
                    MidiChunkType::MThd => return Err(MidiFileError::MultipleHeaders),
                    MidiChunkType::MTrk => tracks.push(MidiTrack::try_from(chunk)?),
                },
                Err(e) => match e {
                    // The spec says unknown chunks should be expected and ignored, for potential
                    // future additions.
                    MidiChunkError::UnknownChunkType => continue,
                    _ => return Err(e.into()),
                },
            }
        }

        Ok(Self {
            format,
            ntrks,
            division,
            tracks,
        })
    }
}
impl MidiFile {
    pub fn get_format(&self) -> MidiFileFormat {
        self.format
    }
    pub fn get_division(&self) -> u16 {
        self.division
    }
    pub fn get_tracks(&self) -> &Vec<MidiTrack> {
        &self.tracks
    }
}
