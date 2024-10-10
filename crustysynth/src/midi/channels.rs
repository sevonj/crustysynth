#[derive(Debug)]
pub enum MidiChannel {
    Ch1 = 0x0,
    Ch2 = 0x1,
    Ch3 = 0x2,
    Ch4 = 0x3,
    Ch5 = 0x4,
    Ch6 = 0x5,
    Ch7 = 0x6,
    Ch8 = 0x7,
    Ch9 = 0x8,
    Ch10 = 0x9,
    Ch11 = 0xA,
    Ch12 = 0xB,
    Ch13 = 0xC,
    Ch14 = 0xD,
    Ch15 = 0xE,
    Ch16 = 0xF,
}

impl From<u8> for MidiChannel {
    /// Get Channel from status byte. Message type is ignored.
    fn from(v: u8) -> Self {
        match v & 0x0F {
            0x0 => Self::Ch1,
            0x1 => Self::Ch2,
            0x2 => Self::Ch3,
            0x3 => Self::Ch4,
            0x4 => Self::Ch5,
            0x5 => Self::Ch6,
            0x6 => Self::Ch7,
            0x7 => Self::Ch8,
            0x8 => Self::Ch9,
            0x9 => Self::Ch10,
            0xA => Self::Ch11,
            0xB => Self::Ch12,
            0xC => Self::Ch13,
            0xD => Self::Ch14,
            0xE => Self::Ch15,
            0xF => Self::Ch16,
            _ => unreachable!(),
        }
    }
}

impl From<MidiChannel> for u8 {
    /// Get status byte. Message type is zero.
    fn from(channel: MidiChannel) -> u8 {
        channel as u8
    }
}
