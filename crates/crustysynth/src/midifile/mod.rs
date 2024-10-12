//! MIDI file specific definitions

use std::{error::Error, fmt::Display, fs::File, io::BufReader};

use chunks::{MidiChunk, MidiChunkError, MidiChunkType};
use division::{Division, DivisionError};
use miditrack::{MidiTrack, MidiTrackError};

pub mod chunks;
pub mod division;
pub mod miditrack;
pub mod vlq;

#[derive(Debug)]
pub enum MidiFileError {
    IOError { source: std::io::Error },
    ChunkError { source: MidiChunkError },
    TrackError { source: MidiTrackError },
    DivisionError { source: DivisionError },
    NoHeader,
    NoTracks,
    Type0TooManyTracks,
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
            Self::DivisionError { source } => write!(f, "{source}"),
            Self::NoHeader => write!(f, "Midi file did not start with a header chunk."),
            Self::NoTracks => write!(f, "Midi file has no track chunks."),
            Self::Type0TooManyTracks => {
                write!(f, "Midi file is type 0, but has multiple track chunks.")
            }
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
impl From<DivisionError> for MidiFileError {
    fn from(e: DivisionError) -> Self {
        Self::DivisionError { source: e }
    }
}

/// # Examples
///
/// ```
/// #[macro_use]
///
/// use crustysynth::midifile::MidiFileFormat;
///
/// assert_eq!(
///     MidiFileFormat::try_from(0_u16).unwrap(),
///     MidiFileFormat::SingleTrack
/// );
/// assert_eq!(
///     MidiFileFormat::try_from(1_u16).unwrap(),
///     MidiFileFormat::MultiTrack
/// );
/// assert_eq!(
///     MidiFileFormat::try_from(2_u16).unwrap(),
///     MidiFileFormat::MultiTrackAsync
/// );
///
/// assert_eq!(MidiFileFormat::SingleTrack as u16, 0);
/// assert_eq!(MidiFileFormat::MultiTrack as u16, 1);
/// assert_eq!(MidiFileFormat::MultiTrackAsync as u16, 2);
///
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MidiFileFormat {
    /// Type 0: Single track
    SingleTrack = 0,
    /// Type 1: Multiple tracks that play simultaneously
    MultiTrack = 1,
    /// Type 2: Multiple independent tracks (separate songs)
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

/// Represents the contents of a MIDI file.
///
/// # Examples
///
/// ```
/// use crustysynth::midifile::MidiFile;
/// use std::fs::File;
///
/// let file = File::open("../../samples/salsa.mid").unwrap();
/// let midi = MidiFile::try_from(file).unwrap();
/// ```
#[derive(Debug)]
pub struct MidiFile {
    format: MidiFileFormat,
    ntrks: u16,
    division: Division,
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
        if format == MidiFileFormat::SingleTrack && ntrks > 1 {
            return Err(MidiFileError::Type0TooManyTracks);
        }
        let division =
            Division::try_from(u16::from_be_bytes(header_data[4..6].try_into().unwrap()))?;

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
        if tracks.is_empty() {
            return Err(MidiFileError::NoTracks);
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
    pub fn get_division(&self) -> Division {
        self.division
    }
    pub fn get_tracks(&self) -> &Vec<MidiTrack> {
        &self.tracks
    }
}
