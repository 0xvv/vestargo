use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Deserializer, Serialize};
use serde::ser::SerializeSeq;
use serde_repr::*;
use anyhow::Result;

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Eq, PartialEq, Copy)]
#[repr(u8)]
pub enum Color {
    Red = 63,
    Orange = 64,
    Yellow = 65,
    Green = 66,
    Blue = 67,
    Violet = 68,
    White = 69,
    Black = 70,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            63 => Color::Red,
            64 => Color::Orange,
            65 => Color::Yellow,
            66 => Color::Green,
            67 => Color::Blue,
            68 => Color::Violet,
            69 => Color::White,
            70 => Color::Black,
            _ => Color::Black,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Eq, PartialEq, Copy)]
#[repr(u8)]
pub enum Letter {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
    I = 9,
    J = 10,
    K = 11,
    L = 12,
    M = 13,
    N = 14,
    O = 15,
    P = 16,
    Q = 17,
    R = 18,
    S = 19,
    T = 20,
    U = 21,
    V = 22,
    W = 23,
    X = 24,
    Y = 25,
    Z = 26,
    One = 27,
    Two = 28,
    Three = 29,
    Four = 30,
    Five = 31,
    Six = 32,
    Seven = 33,
    Eight = 34,
    Nine = 35,
    Zero = 36,
    Exclamation = 37,
    At = 38,
    Pound = 39,
    Dollar = 40,
    LeftParen = 41,
    RightParen = 42,
    Hyphen = 44,
    Plus = 46,
    Ampersand = 47,
    Equal = 48,
    SemiColon = 49,
    Colon = 50,
    Quote = 52,
    DoubleQuote = 53,
    Percent = 54,
    Comma = 55,
    Period = 56,
    Slash = 59,
    Question = 60,
    Degree = 62,
}

impl From<u8> for Letter {
    fn from(value: u8) -> Self {
        match value {
            1 => Letter::A,
            2 => Letter::B,
            3 => Letter::C,
            4 => Letter::D,
            5 => Letter::E,
            6 => Letter::F,
            7 => Letter::G,
            8 => Letter::H,
            9 => Letter::I,
            10 => Letter::J,
            11 => Letter::K,
            12 => Letter::L,
            13 => Letter::M,
            14 => Letter::N,
            15 => Letter::O,
            16 => Letter::P,
            17 => Letter::Q,
            18 => Letter::R,
            19 => Letter::S,
            20 => Letter::T,
            21 => Letter::U,
            22 => Letter::V,
            23 => Letter::W,
            24 => Letter::X,
            25 => Letter::Y,
            26 => Letter::Z,
            27 => Letter::One,
            28 => Letter::Two,
            29 => Letter::Three,
            30 => Letter::Four,
            31 => Letter::Five,
            32 => Letter::Six,
            33 => Letter::Seven,
            34 => Letter::Eight,
            35 => Letter::Nine,
            36 => Letter::Zero,
            37 => Letter::Exclamation,
            38 => Letter::At,
            39 => Letter::Pound,
            40 => Letter::Dollar,
            41 => Letter::LeftParen,
            42 => Letter::RightParen,
            44 => Letter::Hyphen,
            46 => Letter::Plus,
            47 => Letter::Ampersand,
            48 => Letter::Equal,
            49 => Letter::SemiColon,
            50 => Letter::Colon,
            52 => Letter::Quote,
            53 => Letter::DoubleQuote,
            54 => Letter::Percent,
            55 => Letter::Comma,
            56 => Letter::Period,
            59 => Letter::Slash,
            60 => Letter::Question,
            62 => Letter::Degree,
            _ => Letter::Question,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum Bit {
    Letter(Letter),
    Color(Color),
    Blank,
    Filed,
}

pub trait Code {
    fn get_code(&self) -> u8;
}

impl Display for Letter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Letter::A => "A",
            Letter::B => "B",
            Letter::C => "C",
            Letter::D => "D",
            Letter::E => "E",
            Letter::F => "F",
            Letter::G => "G",
            Letter::H => "H",
            Letter::I => "I",
            Letter::J => "J",
            Letter::K => "K",
            Letter::L => "L",
            Letter::M => "M",
            Letter::N => "N",
            Letter::O => "O",
            Letter::P => "P",
            Letter::Q => "Q",
            Letter::R => "R",
            Letter::S => "S",
            Letter::T => "T",
            Letter::U => "U",
            Letter::V => "V",
            Letter::W => "W",
            Letter::X => "X",
            Letter::Y => "Y",
            Letter::Z => "Z",
            Letter::One => "1",
            Letter::Two => "2",
            Letter::Three => "3",
            Letter::Four => "4",
            Letter::Five => "5",
            Letter::Six => "6",
            Letter::Seven => "7",
            Letter::Eight => "8",
            Letter::Nine => "9",
            Letter::Zero => "0",
            Letter::Exclamation => "!",
            Letter::At => "@",
            Letter::Pound => "#",
            Letter::Dollar => "$",
            Letter::LeftParen => "(",
            Letter::RightParen => ")",
            Letter::Hyphen => "-",
            Letter::Plus => "+",
            Letter::Ampersand => "&",
            Letter::Equal => "=",
            Letter::SemiColon => ";",
            Letter::Colon => ":",
            Letter::Quote => "'",
            Letter::DoubleQuote => "\"",
            Letter::Percent => "%",
            Letter::Comma => ",",
            Letter::Period => ".",
            Letter::Slash => "/",
            Letter::Question => "?",
            Letter::Degree => "°",
        };
        write!(f, " {} ", string)
    }
}

impl Code for Letter {
    fn get_code(&self) -> u8 {
        match self {
            Letter::A => 1,
            Letter::B => 2,
            Letter::C => 3,
            Letter::D => 4,
            Letter::E => 5,
            Letter::F => 6,
            Letter::G => 7,
            Letter::H => 8,
            Letter::I => 9,
            Letter::J => 10,
            Letter::K => 11,
            Letter::L => 12,
            Letter::M => 13,
            Letter::N => 14,
            Letter::O => 15,
            Letter::P => 16,
            Letter::Q => 17,
            Letter::R => 18,
            Letter::S => 19,
            Letter::T => 20,
            Letter::U => 21,
            Letter::V => 22,
            Letter::W => 23,
            Letter::X => 24,
            Letter::Y => 25,
            Letter::Z => 26,
            Letter::One => 27,
            Letter::Two => 28,
            Letter::Three => 29,
            Letter::Four => 30,
            Letter::Five => 31,
            Letter::Six => 32,
            Letter::Seven => 33,
            Letter::Eight => 34,
            Letter::Nine => 35,
            Letter::Zero => 36,
            Letter::Exclamation => 37,
            Letter::At => 38,
            Letter::Pound => 39,
            Letter::Dollar => 40,
            Letter::LeftParen => 41,
            Letter::RightParen => 42,
            Letter::Hyphen => 44,
            Letter::Plus => 46,
            Letter::Ampersand => 47,
            Letter::Equal => 48,
            Letter::SemiColon => 49,
            Letter::Colon => 50,
            Letter::Quote => 52,
            Letter::DoubleQuote => 53,
            Letter::Percent => 54,
            Letter::Comma => 55,
            Letter::Period => 56,
            Letter::Slash => 59,
            Letter::Question => 60,
            Letter::Degree => 62,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Color::Red => "\x1b[41m   \x1b[0m",
            Color::Orange => "\x1b[48:5:208m   \x1b[m",
            Color::Yellow => "\x1b[103m   \x1b[0m",
            Color::Green => "\x1b[42m   \x1b[0m",
            Color::Blue => "\x1b[44m   \x1b[0m",
            Color::Violet => "\x1b[45m   \x1b[0m",
            Color::White => "\x1b[107m   \x1b[0m",
            Color::Black => "\x1b[40m   \x1b[0m",
        };
        write!(f, "{}", string)
    }
}

impl Code for Color {
    fn get_code(&self) -> u8 {
        match self {
            Color::Red => 63,
            Color::Orange => 64,
            Color::Yellow => 65,
            Color::Green => 66,
            Color::Blue => 67,
            Color::Violet => 68,
            Color::White => 69,
            Color::Black => 70,
        }
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bit::Letter(l) => Display::fmt(&l, f),
            Bit::Color(c) => Display::fmt(&c, f),
            Bit::Blank => {
                write!(f, "   ")
            }
            Bit::Filed => {
                write!(f, "\x1b[107m   \x1b[0m")
            }
        }
    }
}

impl Code for Bit {
    fn get_code(&self) -> u8 {
        match self {
            Bit::Letter(letter) => letter.get_code(),
            Bit::Color(color) => color.get_code(),
            Bit::Blank => 0,
            Bit::Filed => 71,
        }
    }
}

impl From<u8> for Bit {
    fn from(value: u8) -> Self {
        match value {
            0 => Bit::Blank,
            63..=70 => Bit::Color(Color::from(value)),
            71 => Bit::Filed,
            _ => Bit::Letter(Letter::from(value)),
        }
    }
}

#[derive(Clone)]
pub struct Row {
    pub bits: Vec<Bit>,
}

impl Serialize for Row {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.bits.len()))?;
        for bit in &self.bits {
            seq.serialize_element(&bit.get_code())?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Row {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = Vec::<u8>::deserialize(deserializer)?;
        let mut vec = Vec::with_capacity(bits.len());
        for bit in bits {
            if bit == 0 {
                vec.push(Bit::Blank);
            } else if bit == 71 {
                vec.push(Bit::Filed);
            } else {
                vec.push(bit.into());
            }
        }
        Ok(Row { bits: vec })
    }
}

impl Row {
    pub fn new() -> Row {
        Row {
            bits: vec![Bit::Blank; 22],
        }
    }

    pub fn new_filled(bit: &Bit) -> Row {
        Row {
            bits: vec![*bit; 22],
        }
    }
}

#[derive(Clone)]
pub struct Message {
    pub rows: Vec<Row>,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.rows.len()))?;
        for row in &self.rows {
            seq.serialize_element(row)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rows = Vec::<Row>::deserialize(deserializer)?;
        Ok(Message { rows })
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for row in &self.rows {
            for bit in &row.bits {
                string.push_str(&format!("{:02} ", bit.get_code()));
            }
            string.push('\n');
        }
        write!(f, "{}", string)
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;
        for _ in 0..22 {
            write!(f, "━━━━")?;
        }
        writeln!(f)?;
        for row in &self.rows {
            write!(f, "┃")?;
            for bit in &row.bits {
                Display::fmt(&bit, f)?;
                write!(f, " ")?;
            }
            write!(f, "┃")?;
            writeln!(f)?;
            write!(f, "┃")?;
            writeln!(f)?;
        }
        write!(f, " ")?;
        for _ in 0..22 {
            write!(f, "━━━━")?;
        }
        writeln!(f)?;
        Ok(())
    }
}

impl Message {
    pub fn new() -> Message {
        let mut vec = Vec::with_capacity(6);
        for _i in 0..6 {
            vec.push(Row::new());
        }
        Message { rows: vec }
    }

    pub fn new_filled(bit: Bit) -> Message {
        let mut vec = Vec::with_capacity(6);
        for _i in 0..6 {
            vec.push(Row::new_filled(&bit));
        }
        Message { rows: vec }
    }
}

// Test
#[cfg(test)]
mod tests {
    use crate::bit::*;

    #[test]
    fn test_bit_get_code() {
        assert_eq!(Bit::Blank.get_code(), 0);
        assert_eq!(Bit::Letter(Letter::A).get_code(), 1);
        assert_eq!(Bit::Letter(Letter::B).get_code(), 2);
        assert_eq!(Bit::Letter(Letter::C).get_code(), 3);
        assert_eq!(Bit::Letter(Letter::D).get_code(), 4);
        assert_eq!(Bit::Letter(Letter::E).get_code(), 5);
        assert_eq!(Bit::Letter(Letter::F).get_code(), 6);
        assert_eq!(Bit::Letter(Letter::G).get_code(), 7);
        assert_eq!(Bit::Letter(Letter::H).get_code(), 8);
        assert_eq!(Bit::Letter(Letter::I).get_code(), 9);
        assert_eq!(Bit::Letter(Letter::J).get_code(), 10);
        assert_eq!(Bit::Letter(Letter::K).get_code(), 11);
        assert_eq!(Bit::Letter(Letter::L).get_code(), 12);
        assert_eq!(Bit::Letter(Letter::M).get_code(), 13);
        assert_eq!(Bit::Letter(Letter::N).get_code(), 14);
        assert_eq!(Bit::Letter(Letter::O).get_code(), 15);
        assert_eq!(Bit::Letter(Letter::P).get_code(), 16);
        assert_eq!(Bit::Letter(Letter::Q).get_code(), 17);
        assert_eq!(Bit::Letter(Letter::R).get_code(), 18);
        assert_eq!(Bit::Letter(Letter::S).get_code(), 19);
        assert_eq!(Bit::Letter(Letter::T).get_code(), 20);
        assert_eq!(Bit::Letter(Letter::U).get_code(), 21);
        assert_eq!(Bit::Letter(Letter::V).get_code(), 22);
        assert_eq!(Bit::Letter(Letter::W).get_code(), 23);
        assert_eq!(Bit::Letter(Letter::X).get_code(), 24);
        assert_eq!(Bit::Letter(Letter::Y).get_code(), 25);
        assert_eq!(Bit::Letter(Letter::Z).get_code(), 26);
        assert_eq!(Bit::Letter(Letter::One).get_code(), 27);
        assert_eq!(Bit::Letter(Letter::Two).get_code(), 28);
        assert_eq!(Bit::Letter(Letter::Three).get_code(), 29);
        assert_eq!(Bit::Letter(Letter::Four).get_code(), 30);
        assert_eq!(Bit::Letter(Letter::Five).get_code(), 31);
        assert_eq!(Bit::Letter(Letter::Six).get_code(), 32);
        assert_eq!(Bit::Letter(Letter::Seven).get_code(), 33);
        assert_eq!(Bit::Letter(Letter::Eight).get_code(), 34);
        assert_eq!(Bit::Letter(Letter::Nine).get_code(), 35);
        assert_eq!(Bit::Letter(Letter::Zero).get_code(), 36);
        assert_eq!(Bit::Letter(Letter::Exclamation).get_code(), 37);
        assert_eq!(Bit::Letter(Letter::At).get_code(), 38);
        assert_eq!(Bit::Letter(Letter::Pound).get_code(), 39);
        assert_eq!(Bit::Letter(Letter::Dollar).get_code(), 40);
        assert_eq!(Bit::Letter(Letter::LeftParen).get_code(), 41);
        assert_eq!(Bit::Letter(Letter::RightParen).get_code(), 42);
        assert_eq!(Bit::Letter(Letter::Hyphen).get_code(), 44);
        assert_eq!(Bit::Letter(Letter::Plus).get_code(), 46);
        assert_eq!(Bit::Letter(Letter::Ampersand).get_code(), 47);
        assert_eq!(Bit::Letter(Letter::Equal).get_code(), 48);
        assert_eq!(Bit::Letter(Letter::SemiColon).get_code(), 49);
        assert_eq!(Bit::Letter(Letter::Colon).get_code(), 50);
        assert_eq!(Bit::Letter(Letter::Quote).get_code(), 52);
        assert_eq!(Bit::Letter(Letter::DoubleQuote).get_code(), 53);
        assert_eq!(Bit::Letter(Letter::Percent).get_code(), 54);
        assert_eq!(Bit::Letter(Letter::Comma).get_code(), 55);
        assert_eq!(Bit::Letter(Letter::Period).get_code(), 56);
        assert_eq!(Bit::Letter(Letter::Slash).get_code(), 59);
        assert_eq!(Bit::Letter(Letter::Question).get_code(), 60);
        assert_eq!(Bit::Letter(Letter::Degree).get_code(), 62);
        assert_eq!(Bit::Color(Color::Red).get_code(), 63);
        assert_eq!(Bit::Color(Color::Orange).get_code(), 64);
        assert_eq!(Bit::Color(Color::Yellow).get_code(), 65);
        assert_eq!(Bit::Color(Color::Green).get_code(), 66);
        assert_eq!(Bit::Color(Color::Blue).get_code(), 67);
        assert_eq!(Bit::Color(Color::Violet).get_code(), 68);
        assert_eq!(Bit::Color(Color::White).get_code(), 69);
        assert_eq!(Bit::Color(Color::Black).get_code(), 70);
        assert_eq!(Bit::Filed.get_code(), 71);
    }

    #[test]
    fn test_message_crafting() {
        let mut message = Message::new();
        message.rows[0].bits[0] = Bit::Letter(Letter::A);
        message.rows[0].bits[1] = Bit::Letter(Letter::B);
        message.rows[0].bits[2] = Bit::Letter(Letter::C);
        message.rows[0].bits[3] = Bit::Letter(Letter::D);
        message.rows[0].bits[4] = Bit::Letter(Letter::E);
        message.rows[0].bits[5] = Bit::Letter(Letter::F);
        message.rows[0].bits[6] = Bit::Letter(Letter::G);
        message.rows[0].bits[7] = Bit::Letter(Letter::H);
        message.rows[0].bits[8] = Bit::Letter(Letter::I);
        message.rows[0].bits[9] = Bit::Letter(Letter::J);
        message.rows[0].bits[10] = Bit::Letter(Letter::K);
        message.rows[0].bits[11] = Bit::Letter(Letter::L);
        message.rows[0].bits[12] = Bit::Letter(Letter::M);
        message.rows[0].bits[13] = Bit::Letter(Letter::N);
        message.rows[0].bits[14] = Bit::Letter(Letter::O);
        message.rows[0].bits[15] = Bit::Letter(Letter::P);
        message.rows[0].bits[16] = Bit::Letter(Letter::Q);
        message.rows[0].bits[17] = Bit::Letter(Letter::R);
        message.rows[0].bits[18] = Bit::Letter(Letter::S);
        message.rows[0].bits[19] = Bit::Letter(Letter::T);
        message.rows[0].bits[20] = Bit::Letter(Letter::U);
        message.rows[0].bits[21] = Bit::Letter(Letter::V);
        message.rows[1].bits[0] = Bit::Letter(Letter::W);
        message.rows[1].bits[1] = Bit::Letter(Letter::X);
        message.rows[1].bits[2] = Bit::Letter(Letter::Y);
        message.rows[1].bits[3] = Bit::Letter(Letter::Z);
        message.rows[1].bits[4] = Bit::Letter(Letter::One);
        message.rows[1].bits[5] = Bit::Letter(Letter::Two);
        message.rows[1].bits[6] = Bit::Letter(Letter::Three);
        message.rows[1].bits[7] = Bit::Letter(Letter::Four);
        message.rows[1].bits[8] = Bit::Letter(Letter::Five);
        message.rows[1].bits[9] = Bit::Letter(Letter::Six);
        message.rows[1].bits[10] = Bit::Letter(Letter::Seven);
        message.rows[1].bits[11] = Bit::Letter(Letter::Eight);
        message.rows[1].bits[12] = Bit::Letter(Letter::Nine);
        message.rows[1].bits[13] = Bit::Letter(Letter::Zero);
        message.rows[1].bits[14] = Bit::Letter(Letter::Exclamation);
        message.rows[1].bits[15] = Bit::Letter(Letter::At);
        message.rows[1].bits[16] = Bit::Letter(Letter::Pound);
        message.rows[1].bits[17] = Bit::Letter(Letter::Dollar);
        message.rows[1].bits[18] = Bit::Letter(Letter::LeftParen);
        message.rows[1].bits[19] = Bit::Letter(Letter::RightParen);
        message.rows[1].bits[20] = Bit::Letter(Letter::Hyphen);
        message.rows[1].bits[21] = Bit::Letter(Letter::Plus);
        message.rows[2].bits[0] = Bit::Letter(Letter::Ampersand);
        message.rows[2].bits[1] = Bit::Letter(Letter::Equal);
        message.rows[2].bits[2] = Bit::Letter(Letter::SemiColon);
        message.rows[2].bits[3] = Bit::Letter(Letter::Colon);
        message.rows[2].bits[4] = Bit::Letter(Letter::Quote);
        message.rows[2].bits[5] = Bit::Letter(Letter::DoubleQuote);
        message.rows[2].bits[6] = Bit::Letter(Letter::Percent);
        message.rows[2].bits[7] = Bit::Letter(Letter::Comma);
        message.rows[2].bits[8] = Bit::Letter(Letter::Period);
        message.rows[2].bits[9] = Bit::Letter(Letter::Slash);
        message.rows[2].bits[10] = Bit::Letter(Letter::Question);
        message.rows[2].bits[11] = Bit::Letter(Letter::Degree);
        message.rows[2].bits[12] = Bit::Color(Color::Red);
        message.rows[2].bits[13] = Bit::Color(Color::Orange);
        message.rows[2].bits[14] = Bit::Color(Color::Yellow);
        message.rows[2].bits[15] = Bit::Color(Color::Green);
        message.rows[2].bits[16] = Bit::Color(Color::Blue);
        message.rows[2].bits[17] = Bit::Color(Color::Violet);
        message.rows[2].bits[18] = Bit::Color(Color::White);
        message.rows[2].bits[19] = Bit::Color(Color::Black);
        message.rows[2].bits[20] = Bit::Blank;
        message.rows[2].bits[21] = Bit::Filed;
        // fill the last rows with orange and black
        for i in 3..6 {
            for j in 0..22 {
                if i % 2 == 0 {
                    message.rows[i].bits[j] = Bit::Color(Color::Black);
                } else {
                    message.rows[i].bits[j] = Bit::Color(Color::Orange);
                }
            }
        }

        println!("{:?}", message);
        print!("{}", message);
    }

    #[test]
    fn test_message_ser() -> Result<()> {
        let mut message = Message::new();

        let json = serde_json::to_string(&message)?;
        assert_eq!(json, "[[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]");

        message.rows[0].bits[0] = Bit::Letter(Letter::A);
        message.rows[0].bits[1] = Bit::Letter(Letter::B);
        message.rows[0].bits[2] = Bit::Letter(Letter::C);

        let json = serde_json::to_string(&message)?;
        assert_eq!(json, "[[1,2,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]");

        message.rows[1].bits[0] = Bit::Filed;
        message.rows[2].bits[1] = Bit::Color(Color::Blue);
        message.rows[3].bits[2] = Bit::Color(Color::Green);

        let json = serde_json::to_string(&message)?;
        assert_eq!(json, "[[1,2,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[71,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,67,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,66,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]");

        message.rows[5].bits[19] = Bit::Letter(Letter::Hyphen);
        message.rows[5].bits[20] = Bit::Letter(Letter::Equal);
        message.rows[5].bits[21] = Bit::Letter(Letter::Exclamation);

        let json = serde_json::to_string(&message)?;
        assert_eq!(json, "[[1,2,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[71,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,67,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,66,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,44,48,37]]");

        message.rows[0].bits[0] = Bit::Blank;
        message.rows[0].bits[1] = Bit::Blank;
        message.rows[0].bits[2] = Bit::Blank;

        let json = serde_json::to_string(&message)?;
        assert_eq!(json, "[[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[71,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,67,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,66,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,44,48,37]]");

        Ok(())
    }

    #[test]
    fn test_message_deser() -> Result<()> {
        let data = r#"
        [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 8, 5, 12, 12, 15, 0, 23, 15, 18, 12, 4, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ]
        "#;

        let message = serde_json::from_str::<Message>(data)?;

        assert_eq!(message.rows[2].bits[5], Bit::Letter(Letter::H));
        assert_eq!(message.rows[2].bits[6], Bit::Letter(Letter::E));
        assert_eq!(message.rows[2].bits[7], Bit::Letter(Letter::L));
        assert_eq!(message.rows[2].bits[8], Bit::Letter(Letter::L));
        assert_eq!(message.rows[2].bits[9], Bit::Letter(Letter::O));
        assert_eq!(message.rows[2].bits[10], Bit::Blank);
        assert_eq!(message.rows[2].bits[11], Bit::Letter(Letter::W));
        assert_eq!(message.rows[2].bits[12], Bit::Letter(Letter::O));
        assert_eq!(message.rows[2].bits[13], Bit::Letter(Letter::R));
        assert_eq!(message.rows[2].bits[14], Bit::Letter(Letter::L));
        assert_eq!(message.rows[2].bits[15], Bit::Letter(Letter::D));

        println!("{:?}", message);
        println!("{}", message);

        Ok(())
    }
}
