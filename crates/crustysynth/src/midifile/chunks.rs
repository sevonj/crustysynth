use std::{
    error::Error,
    fmt::{Debug, Display},
    io,
};

#[derive(Debug)]
pub enum MidiChunkError {
    IOError { source: std::io::Error },
    UnknownChunkType,
    UnexpectedHeaderLength(u32),
}
impl Error for MidiChunkError {}
impl Display for MidiChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError { source } => write!(f, "{source}"),
            Self::UnknownChunkType => write!(f, "Unknown chunk type."),
            Self::UnexpectedHeaderLength(len) => {
                write!(f, "Unexpected length for MThd. Expected 6, but got {len}")
            }
        }
    }
}
impl From<std::io::Error> for MidiChunkError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError { source: e }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MidiChunkType {
    MThd,
    MTrk,
}
impl TryFrom<[u8; 4]> for MidiChunkType {
    type Error = MidiChunkError;

    fn try_from(buffer: [u8; 4]) -> Result<Self, MidiChunkError> {
        match buffer {
            [b'M', b'T', b'h', b'd'] => Ok(Self::MThd),
            [b'M', b'T', b'r', b'k'] => Ok(Self::MTrk),
            _ => Err(MidiChunkError::UnknownChunkType),
        }
    }
}

pub struct MidiChunk {
    chunk_type: MidiChunkType,
    chunk_length: u32,
    chunk_data: Vec<u8>,
}
impl Debug for MidiChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MidiChunk")
            .field("chunk_type", &self.chunk_type)
            .field("chunk_length", &self.chunk_length)
            /* Hidden to avoid excessive printing */ // .field("chunk_data", &self.chunk_data)
            .finish()
    }
}
impl Display for MidiChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    MidiChunk")?;
        writeln!(f, "      chunk_type:      {:?}", self.chunk_type)?;
        write!(f, "      chunk_length:    {:?}", self.chunk_length)
    }
}

impl MidiChunk {
    pub fn read<R>(reader: &mut R) -> Result<Self, MidiChunkError>
    where
        R: io::Read,
    {
        let mut chunk_type_buf = [0_u8; 4];
        reader.read_exact(&mut chunk_type_buf)?;
        let chunk_type = MidiChunkType::try_from(chunk_type_buf)?;

        let mut chunk_length_buf = [0_u8; 4];
        reader.read_exact(&mut chunk_length_buf)?;
        let chunk_length = u32::from_be_bytes(chunk_length_buf);

        if chunk_type == MidiChunkType::MThd && chunk_length != 6 {
            return Err(MidiChunkError::UnexpectedHeaderLength(chunk_length));
        }

        let mut chunk_data = vec![0_u8; chunk_length as usize];
        reader.read_exact(&mut chunk_data)?;

        Ok(Self {
            chunk_type,
            chunk_length,
            chunk_data,
        })
    }

    pub fn get_type(&self) -> MidiChunkType {
        self.chunk_type
    }
    pub fn get_length(&self) -> usize {
        self.chunk_length as usize
    }
    pub fn get_data(&self) -> &Vec<u8> {
        &self.chunk_data
    }
}
