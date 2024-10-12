//! _Variable Length Quantity_ is a compression scheme for unsigned integers.
//!
//! The width is a multiple of 7 bits. Total width is unknown until last byte is read.
//! Largest number allowed in the midi spec is `0x0FFFFFFF`.

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum VlqError {
    IOError { source: std::io::Error },
    TooLarge,
}
impl Error for VlqError {}
impl Display for VlqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError { source } => write!(f, "{source}"),
            Self::TooLarge => write!(f, "Value is larger than allowed (max. 0x0FFFFFFF)"),
        }
    }
}
impl From<std::io::Error> for VlqError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError { source: e }
    }
}

/// Read a vlq from a buffer.
pub fn read_vlq<R>(file: &mut R) -> Result<usize, VlqError>
where
    R: std::io::Read,
{
    let mut value: usize = 0;
    let mut buf = [0_u8];

    loop {
        file.read_exact(&mut buf)?;
        let next_7 = (buf[0] & 0x7F) as usize;

        value <<= 7;
        value += next_7;

        if value > 0x0FFFFFFF {
            return Err(VlqError::TooLarge);
        }
        if buf[0] & 0x80 == 0 {
            break;
        }
    }

    Ok(value)
}

/// Read a vlq from a buffer. No width limitation.
pub fn read_vlq_unchecked<R>(file: &mut R) -> Result<usize, VlqError>
where
    R: std::io::Read,
{
    let mut value: usize = 0;
    let mut buf = [0_u8];

    loop {
        file.read_exact(&mut buf)?;
        let next_7 = (buf[0] & 0x7F) as usize;

        value <<= 7;
        value += next_7;

        if buf[0] & 0x80 == 0 {
            break;
        }
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This will test these examples
    /// 00          => 00000000
    /// 40          => 00000040
    /// 7F          => 0000007F
    /// 81 00       => 00000080
    /// C0 00       => 00002000
    /// FF 7F       => 00003FFF
    /// 81 80 00    => 00004000
    /// C0 80 00    => 00100000
    /// FF FF 7F    => 001FFFFF
    /// 81 80 80 00 => 00200000
    /// C0 80 80 00 => 08000000
    /// FF FF FF 7F => 0FFFFFFF

    #[test]
    fn test_read_vlq() {
        assert_eq!(read_vlq(&mut [0x00].as_slice()).unwrap(), 0);
        assert_eq!(read_vlq(&mut [0x40].as_slice()).unwrap(), 0x40);
        assert_eq!(read_vlq(&mut [0x7f].as_slice()).unwrap(), 0x7f);
        assert_eq!(read_vlq(&mut [0x81, 0x00].as_slice()).unwrap(), 0x80);
        assert_eq!(read_vlq(&mut [0xc0, 0x00].as_slice()).unwrap(), 0x2000);
        assert_eq!(read_vlq(&mut [0xff, 0x7f].as_slice()).unwrap(), 0x3fff);
        assert_eq!(
            read_vlq(&mut [0x81, 0x80, 0x00].as_slice()).unwrap(),
            0x4000
        );
        assert_eq!(
            read_vlq(&mut [0xC0, 0x80, 0x00].as_slice()).unwrap(),
            0x100000
        );
        assert_eq!(
            read_vlq(&mut [0xFF, 0xFF, 0x7F].as_slice()).unwrap(),
            0x1fffff
        );
        assert_eq!(
            read_vlq(&mut [0x81, 0x80, 0x80, 0x00].as_slice()).unwrap(),
            0x200000
        );
        assert_eq!(
            read_vlq(&mut [0xC0, 0x80, 0x80, 0x00].as_slice()).unwrap(),
            0x8000000
        );
        assert_eq!(
            read_vlq(&mut [0xFF, 0xFF, 0xFF, 0x7F].as_slice()).unwrap(),
            0xFFFFFFF
        );
    }

    #[test]
    fn test_read_vlq_toolarge() {
        assert!(read_vlq(&mut [0xFF, 0xFF, 0xFF, 0xFF, 0x7F].as_slice()).is_err());
    }

    #[test]
    fn test_read_vlq_uncheckoed() {
        assert_eq!(read_vlq_unchecked(&mut [0x00].as_slice()).unwrap(), 0);
        assert_eq!(read_vlq_unchecked(&mut [0x40].as_slice()).unwrap(), 0x40);
        assert_eq!(read_vlq_unchecked(&mut [0x7f].as_slice()).unwrap(), 0x7f);
        assert_eq!(
            read_vlq_unchecked(&mut [0x81, 0x00].as_slice()).unwrap(),
            0x80
        );
        assert_eq!(
            read_vlq_unchecked(&mut [0xc0, 0x00].as_slice()).unwrap(),
            0x2000
        );
        assert_eq!(
            read_vlq_unchecked(&mut [0xff, 0x7f].as_slice()).unwrap(),
            0x3fff
        );
        assert_eq!(
            read_vlq_unchecked(&mut [0x81, 0x80, 0x00].as_slice()).unwrap(),
            0x4000
        );
        assert_eq!(
            read_vlq_unchecked(&mut [0xC0, 0x80, 0x00].as_slice()).unwrap(),
            0x100000
        );
        assert_eq!(
            read_vlq_unchecked(&mut [0xFF, 0xFF, 0x7F].as_slice()).unwrap(),
            0x1fffff
        );
        assert_eq!(
            read_vlq_unchecked(&mut [0x81, 0x80, 0x80, 0x00].as_slice()).unwrap(),
            0x200000
        );
        assert_eq!(
            read_vlq_unchecked(&mut [0xC0, 0x80, 0x80, 0x00].as_slice()).unwrap(),
            0x8000000
        );
        assert_eq!(
            read_vlq_unchecked(&mut [0xFF, 0xFF, 0xFF, 0x7F].as_slice()).unwrap(),
            0xFFFFFFF
        );
        assert_eq!(
            read_vlq_unchecked(&mut [0xFF, 0xFF, 0xFF, 0x7F].as_slice()).unwrap(),
            0xFFFFFFF
        );
        assert_eq!(
            read_vlq_unchecked(&mut [0xFF, 0xFF, 0xFF, 0xFF, 0x7F].as_slice()).unwrap(),
            0x7FFFFFFFF
        );
    }
}
