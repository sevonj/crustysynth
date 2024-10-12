//! Timing format

use core::f64;
use std::{error::Error, fmt::Display, time::Duration};

#[derive(Debug, PartialEq)]
pub enum DivisionError {
    InvalidFrameFormat(i8),
    ZeroDivision,
}
impl Error for DivisionError {}
impl Display for DivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFrameFormat(frame) => {
                write!(f, "Division has invalid frame value: {frame}")
            }
            Self::ZeroDivision => write!(f, "Division tick rate cannot be zero."),
        }
    }
}
/// Division tells how long one tick should take.
///
/// # Examples
///
/// ```
/// use crustysynth::midifile::division::Division;
/// use std::time::Duration;
/// 
/// // You most likely want to interact with Division like this, and not deal with its variants:
/// let bpm = 120.0;
/// let value: u16 = 0xE332;
/// let division = Division::try_from(value).unwrap();
/// let tick_duration = division.get_tick_duration(bpm);
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Division {
    /// Ticks per beat (quarter note).
    Metrical(usize),
    /// Tick interval, independent of BPM.
    TimeCode(Duration),
}

impl TryFrom<u16> for Division {
    type Error = DivisionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(DivisionError::ZeroDivision);
        }
        if value & 0x8000 == 0 {
            return Ok(Self::Metrical(value as usize));
        }

        let negative_smpte_format = (value >> 8) as i8;
        let frame_duration = match negative_smpte_format {
            -24 => Duration::from_secs_f64(1.0 / 24.0),
            -25 => Duration::from_secs_f64(1.0 / 25.0),
            -29 => Duration::from_secs_f64(1.0 / 29.97),
            -30 => Duration::from_secs_f64(1.0 / 30.0),
            _ => return Err(DivisionError::InvalidFrameFormat(negative_smpte_format)),
        };
        let ticks_per_frame = (value & 0xFF) as u8;
        if ticks_per_frame == 0 {
            return Err(DivisionError::ZeroDivision);
        }
        let tick_duration = frame_duration / ticks_per_frame as u32;
        Ok(Self::TimeCode(tick_duration))
    }
}

impl Division {
    /// Get an absolute duration from any kind of `Division`.
    ///
    /// Note: BPM has no effect on`Division::TimeCode`
    /// # Examples
    ///
    /// ```
    /// use crustysynth::midifile::division::Division;
    ///
    /// let bpm = 120.0;
    /// let division = Division::try_from(0xE332).unwrap();
    /// let duration = division.get_tick_duration(bpm);
    /// ```
    pub fn get_tick_duration(&self, tempo: f64) -> Duration {
        match self {
            Division::TimeCode(duration) => *duration,
            Division::Metrical(ticks_per_beat) => {
                let secs = 1.0 / f64::from(*ticks_per_beat as u32) / tempo;
                Duration::from_secs_f64(secs) * 60
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrical() {
        assert_eq!(Division::try_from(0x0080).unwrap(), Division::Metrical(128));
        assert_eq!(Division::try_from(0x0050).unwrap(), Division::Metrical(80));
        assert_eq!(
            Division::try_from(0x7FFF).unwrap(),
            Division::Metrical(32767)
        );
    }

    #[test]
    fn test_timecode() {
        let dur_a = Duration::from_secs_f64(1.0 / 24.0) / 120;
        assert_eq!(
            Division::try_from(0xE878).unwrap(),
            Division::TimeCode(dur_a)
        );
        let dur_b = Duration::from_secs_f64(1.0 / 25.0) / 100;
        assert_eq!(
            Division::try_from(0xE764).unwrap(),
            Division::TimeCode(dur_b)
        );
        let dur_c = Duration::from_secs_f64(1.0 / 29.97) / 50;
        assert_eq!(
            Division::try_from(0xE332).unwrap(),
            Division::TimeCode(dur_c)
        );
        let dur_d = Duration::from_secs_f64(1.0 / 30.0) / 50;
        assert_eq!(
            Division::try_from(0xE232).unwrap(),
            Division::TimeCode(dur_d)
        );
    }

    #[test]
    fn test_timecode_invalid_frames() {
        for i in 0x80..=0xFF {
            if let -24 | -25 | -29 | -30 = i as i8 {
                continue;
            }
            let value = (i << 8) + 0x32;
            assert_eq!(
                Division::try_from(value).unwrap_err(),
                DivisionError::InvalidFrameFormat(i as i8)
            );
        }
    }

    #[test]
    fn test_zero_division() {
        assert_eq!(
            Division::try_from(0x0000).unwrap_err(),
            DivisionError::ZeroDivision
        );

        assert_eq!(
            Division::try_from(0xE200).unwrap_err(),
            DivisionError::ZeroDivision
        );
    }

    #[test]
    fn test_get_tick_duration_metrical() {
        let bpm_a = 120.0;
        let div_a = Division::Metrical(60);
        let dur_a = Duration::from_secs_f64(1.0 / 60.0 / bpm_a) * 60;
        assert_eq!(div_a.get_tick_duration(bpm_a), dur_a);

        let bpm_b = 62.0;
        let div_b = Division::Metrical(52);
        let dur_b = Duration::from_secs_f64(1.0 / 52.0 / bpm_b) * 60;
        assert_eq!(div_b.get_tick_duration(bpm_b), dur_b);
    }

    #[test]
    fn test_get_tick_duration_timecode() {
        let dur_a = Duration::from_secs_f64(1.0 / 24.0) / 120;
        let dur_b = Duration::from_secs_f64(1.0 / 25.0) / 100;
        let dur_c = Duration::from_secs_f64(1.0 / 29.97) / 50;
        let dur_d = Duration::from_secs_f64(1.0 / 30.0) / 50;
        // Tempo should not matter with time code
        assert_eq!(Division::TimeCode(dur_a).get_tick_duration(120.0), dur_a);
        assert_eq!(Division::TimeCode(dur_b).get_tick_duration(420.69), dur_b);
        assert_eq!(Division::TimeCode(dur_c).get_tick_duration(-120.0), dur_c);
        assert_eq!(Division::TimeCode(dur_d).get_tick_duration(999.0), dur_d);
    }
}
