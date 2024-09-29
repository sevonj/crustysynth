//! This module contains all message

pub enum Status {
    NoteOff,
    NoteOn,
    PolyphonicPressure,
    ControllerChange,
    ProgramChange,
    ChannelPressure,
    PitchBend,
}

impl TryFrom<u8> for Status {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v & 0xF0 {
            0x80 => Ok(Self::NoteOff),
            0x90 => Ok(Self::NoteOn),
            0xA0 => Ok(Self::PolyphonicPressure),
            0xB0 => Ok(Self::ControllerChange),
            0xC0 => Ok(Self::ProgramChange),
            0xD0 => Ok(Self::ChannelPressure),
            0xE0 => Ok(Self::PitchBend),
            _ => Err(()),
        }
    }
}

impl Into<u8> for Status {
    fn into(self) -> u8 {
        match self {
            Self::NoteOff => 0x80,
            Self::NoteOn => 0x90,
            Self::PolyphonicPressure => 0xA0,
            Self::ControllerChange => 0xB0,
            Self::ProgramChange => 0xC0,
            Self::ChannelPressure => 0xD0,
            Self::PitchBend => 0xE0,
        }
    }
}

pub enum Channel {
    Ch1 = 0,
    Ch2 = 1,
    Ch3 = 2,
    Ch4 = 3,
    Ch5 = 4,
    Ch6 = 5,
    Ch7 = 6,
    Ch8 = 7,
    Ch9 = 8,
    Ch10 = 9,
    Ch11 = 10,
    Ch12 = 11,
    Ch13 = 12,
    Ch14 = 13,
    Ch15 = 14,
    Ch16 = 15,
}

impl TryFrom<u8> for Channel {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v & 0xF0 {
            0 => Ok(Self::Ch1),
            1 => Ok(Self::Ch2),
            2 => Ok(Self::Ch3),
            3 => Ok(Self::Ch4),
            4 => Ok(Self::Ch5),
            5 => Ok(Self::Ch6),
            6 => Ok(Self::Ch7),
            7 => Ok(Self::Ch8),
            8 => Ok(Self::Ch9),
            9 => Ok(Self::Ch10),
            10 => Ok(Self::Ch11),
            11 => Ok(Self::Ch12),
            12 => Ok(Self::Ch13),
            13 => Ok(Self::Ch14),
            14 => Ok(Self::Ch15),
            15 => Ok(Self::Ch16),
            _ => Err(()),
        }
    }
}

impl Into<u8> for Channel {
    fn into(self) -> u8 {
        let v = self as u8;
        v << 4
    }
}

pub enum Note {
    C0 = 0,
    CS0 = 1,
    D0 = 2,
    DS0 = 3,
    E0 = 4,
    F0 = 5,
    FS0 = 6,
    G0 = 7,
    GS0 = 8,
    A0 = 9,
    AS0 = 10,
    B0 = 11,
    C1 = 12,
    CS1 = 13,
    D1 = 14,
    DS1 = 15,
    E1 = 16,
    F1 = 17,
    FS1 = 18,
    G1 = 19,
    GS1 = 20,
    A1 = 21,
    AS1 = 22,
    B1 = 23,
    C2 = 24,
    CS2 = 25,
    D2 = 26,
    DS2 = 27,
    E2 = 28,
    F2 = 29,
    FS2 = 30,
    G2 = 31,
    GS2 = 32,
    A2 = 33,
    AS2 = 34,
    B2 = 35,
    C3 = 36,
    CS3 = 37,
    D3 = 38,
    DS3 = 39,
    E3 = 40,
    F3 = 41,
    FS3 = 42,
    G3 = 43,
    GS3 = 44,
    A3 = 45,
    AS3 = 46,
    B3 = 47,
    C4 = 48,
    CS4 = 49,
    D4 = 50,
    DS4 = 51,
    E4 = 52,
    F4 = 53,
    FS4 = 54,
    G4 = 55,
    GS4 = 56,
    A4 = 57,
    AS4 = 58,
    B4 = 59,
    C5 = 60,
    CS5 = 61,
    D5 = 62,
    DS5 = 63,
    E5 = 64,
    F5 = 65,
    FS5 = 66,
    G5 = 67,
    GS5 = 68,
    A5 = 69,
    AS5 = 70,
    B5 = 71,
    C6 = 72,
    CS6 = 73,
    D6 = 74,
    DS6 = 75,
    E6 = 76,
    F6 = 77,
    FS6 = 78,
    G6 = 79,
    GS6 = 80,
    A6 = 81,
    AS6 = 82,
    B6 = 83,
    C7 = 84,
    CS7 = 85,
    D7 = 86,
    DS7 = 87,
    E7 = 88,
    F7 = 89,
    FS7 = 90,
    G7 = 91,
    GS7 = 92,
    A7 = 93,
    AS7 = 94,
    B7 = 95,
    C8 = 96,
    CS8 = 97,
    D8 = 98,
    DS8 = 99,
    E8 = 100,
    F8 = 101,
    FS8 = 102,
    G8 = 103,
    GS8 = 104,
    A8 = 105,
    AS8 = 106,
    B8 = 107,
    C9 = 108,
    CS9 = 109,
    D9 = 110,
    DS9 = 111,
    E9 = 112,
    F9 = 113,
    FS9 = 114,
    G9 = 115,
    GS9 = 116,
    A9 = 117,
    AS9 = 118,
    B9 = 119,
    C21 = 120,
    CS21 = 121,
    D21 = 122,
    DS21 = 123,
    E21 = 124,
    F21 = 125,
    FS21 = 126,
    G21 = 127,
}
impl TryFrom<u8> for Note {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0..=127 => Ok(),
            128..=255 => Err(()),
        }
    }
}

impl Into<u8> for Note {
    fn into(self) -> u8 {
        self as u8
    }
}
