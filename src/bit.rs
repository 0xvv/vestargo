extern crate num;
extern crate num_derive;

use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::*;

#[derive(
    Serialize_repr, Deserialize_repr, Debug, Clone, Eq, PartialEq, Copy, FromPrimitive, ToPrimitive,
)]
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

#[derive(
    Serialize_repr, Deserialize_repr, Debug, Clone, Eq, PartialEq, Copy, FromPrimitive, ToPrimitive,
)]
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

impl FromStr for Bit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Bit::Letter(Letter::A)),
            "B" => Ok(Bit::Letter(Letter::B)),
            "C" => Ok(Bit::Letter(Letter::C)),
            "D" => Ok(Bit::Letter(Letter::D)),
            "E" => Ok(Bit::Letter(Letter::E)),
            "F" => Ok(Bit::Letter(Letter::F)),
            "G" => Ok(Bit::Letter(Letter::G)),
            "H" => Ok(Bit::Letter(Letter::H)),
            "I" => Ok(Bit::Letter(Letter::I)),
            "J" => Ok(Bit::Letter(Letter::J)),
            "K" => Ok(Bit::Letter(Letter::K)),
            "L" => Ok(Bit::Letter(Letter::L)),
            "M" => Ok(Bit::Letter(Letter::M)),
            "N" => Ok(Bit::Letter(Letter::N)),
            "O" => Ok(Bit::Letter(Letter::O)),
            "P" => Ok(Bit::Letter(Letter::P)),
            "Q" => Ok(Bit::Letter(Letter::Q)),
            "R" => Ok(Bit::Letter(Letter::R)),
            "S" => Ok(Bit::Letter(Letter::S)),
            "T" => Ok(Bit::Letter(Letter::T)),
            "U" => Ok(Bit::Letter(Letter::U)),
            "V" => Ok(Bit::Letter(Letter::V)),
            "W" => Ok(Bit::Letter(Letter::W)),
            "X" => Ok(Bit::Letter(Letter::X)),
            "Y" => Ok(Bit::Letter(Letter::Y)),
            "Z" => Ok(Bit::Letter(Letter::Z)),
            "1" => Ok(Bit::Letter(Letter::One)),
            "2" => Ok(Bit::Letter(Letter::Two)),
            "3" => Ok(Bit::Letter(Letter::Three)),
            "4" => Ok(Bit::Letter(Letter::Four)),
            "5" => Ok(Bit::Letter(Letter::Five)),
            "6" => Ok(Bit::Letter(Letter::Six)),
            "7" => Ok(Bit::Letter(Letter::Seven)),
            "8" => Ok(Bit::Letter(Letter::Eight)),
            "9" => Ok(Bit::Letter(Letter::Nine)),
            "0" => Ok(Bit::Letter(Letter::Zero)),
            "!" => Ok(Bit::Letter(Letter::Exclamation)),
            "@" => Ok(Bit::Letter(Letter::At)),
            "#" => Ok(Bit::Letter(Letter::Pound)),
            "$" => Ok(Bit::Letter(Letter::Dollar)),
            "(" => Ok(Bit::Letter(Letter::LeftParen)),
            ")" => Ok(Bit::Letter(Letter::RightParen)),
            "-" => Ok(Bit::Letter(Letter::Hyphen)),
            "+" => Ok(Bit::Letter(Letter::Plus)),
            "&" => Ok(Bit::Letter(Letter::Ampersand)),
            "=" => Ok(Bit::Letter(Letter::Equal)),
            ";" => Ok(Bit::Letter(Letter::SemiColon)),
            ":" => Ok(Bit::Letter(Letter::Colon)),
            "'" => Ok(Bit::Letter(Letter::Quote)),
            "\"" => Ok(Bit::Letter(Letter::DoubleQuote)),
            "%" => Ok(Bit::Letter(Letter::Percent)),
            "," => Ok(Bit::Letter(Letter::Comma)),
            "." => Ok(Bit::Letter(Letter::Period)),
            "/" => Ok(Bit::Letter(Letter::Slash)),
            "?" => Ok(Bit::Letter(Letter::Question)),
            "°" => Ok(Bit::Letter(Letter::Degree)),
            " " => Ok(Bit::Blank),
            "█" => Ok(Bit::Filed),
            "🟥" => Ok(Bit::Color(Color::Red)),
            "🟧" => Ok(Bit::Color(Color::Orange)),
            "🟨" => Ok(Bit::Color(Color::Yellow)),
            "🟩" => Ok(Bit::Color(Color::Green)),
            "🟦" => Ok(Bit::Color(Color::Blue)),
            "🟪" => Ok(Bit::Color(Color::Violet)),
            "◽" => Ok(Bit::Color(Color::White)),
            "⬛" => Ok(Bit::Color(Color::Black)),
            _ => Ok(Bit::Letter(Letter::Question)),
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

impl ToPrimitive for Bit {
    fn to_i64(&self) -> Option<i64> {
        Some(self.to_u8().unwrap() as i64)
    }

    fn to_u8(&self) -> Option<u8> {
        match self {
            Bit::Letter(letter) => letter.to_u8(),
            Bit::Color(color) => color.to_u8(),
            Bit::Blank => Some(0),
            Bit::Filed => Some(71),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        Some(self.to_u8().unwrap() as u64)
    }
}

impl From<u8> for Bit {
    fn from(value: u8) -> Self {
        match value {
            0 => Bit::Blank,
            63..=70 => Bit::Color(Color::from_u8(value).unwrap()),
            71 => Bit::Filed,
            _ => Bit::Letter(Letter::from_u8(value).unwrap()),
        }
    }
}

#[derive(Clone)]
pub struct Row {
    pub bits: Vec<Bit>,
}

impl Serialize for Row {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.bits.len()))?;
        for bit in &self.bits {
            seq.serialize_element(&bit.to_u8())?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Row {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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

impl Default for Row {
    fn default() -> Row {
        Row {
            bits: vec![Bit::Blank; 22],
        }
    }
}

impl Row {
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

impl Message {
    pub(crate) fn from_text(text: &[String]) -> Message {
        let mut message = Message::default();
        for (i, line) in text.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                message.rows[i].bits[j] = Bit::from_str(&c.to_string()).unwrap()
            }
        }
        message
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
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
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rows = Vec::<Row>::deserialize(deserializer)?;
        Ok(Message { rows })
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for row in &self.rows {
            for bit in &row.bits {
                string.push_str(&format!("{:02} ", bit.to_u8().unwrap()));
            }
            string.push('\n');
        }
        write!(f, "{}", string)
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl Default for Message {
    fn default() -> Message {
        let mut vec = Vec::with_capacity(6);
        for _i in 0..6 {
            vec.push(Row::default());
        }
        Message { rows: vec }
    }
}

impl Message {
    pub fn new_filled(bit: Bit) -> Message {
        let mut vec = Vec::with_capacity(6);
        for _i in 0..6 {
            vec.push(Row::new_filled(&bit));
        }
        Message { rows: vec }
    }
}

#[cfg(test)]
mod tests {
    use crate::bit::*;
    use anyhow::Result;

    #[test]
    fn test_bit_to_u8() {
        assert_eq!(Bit::Blank.to_u8().unwrap(), 0);
        assert_eq!(Bit::Letter(Letter::A).to_u8().unwrap(), 1);
        assert_eq!(Bit::Letter(Letter::B).to_u8().unwrap(), 2);
        assert_eq!(Bit::Letter(Letter::C).to_u8().unwrap(), 3);
        assert_eq!(Bit::Letter(Letter::D).to_u8().unwrap(), 4);
        assert_eq!(Bit::Letter(Letter::E).to_u8().unwrap(), 5);
        assert_eq!(Bit::Letter(Letter::F).to_u8().unwrap(), 6);
        assert_eq!(Bit::Letter(Letter::G).to_u8().unwrap(), 7);
        assert_eq!(Bit::Letter(Letter::H).to_u8().unwrap(), 8);
        assert_eq!(Bit::Letter(Letter::I).to_u8().unwrap(), 9);
        assert_eq!(Bit::Letter(Letter::J).to_u8().unwrap(), 10);
        assert_eq!(Bit::Letter(Letter::K).to_u8().unwrap(), 11);
        assert_eq!(Bit::Letter(Letter::L).to_u8().unwrap(), 12);
        assert_eq!(Bit::Letter(Letter::M).to_u8().unwrap(), 13);
        assert_eq!(Bit::Letter(Letter::N).to_u8().unwrap(), 14);
        assert_eq!(Bit::Letter(Letter::O).to_u8().unwrap(), 15);
        assert_eq!(Bit::Letter(Letter::P).to_u8().unwrap(), 16);
        assert_eq!(Bit::Letter(Letter::Q).to_u8().unwrap(), 17);
        assert_eq!(Bit::Letter(Letter::R).to_u8().unwrap(), 18);
        assert_eq!(Bit::Letter(Letter::S).to_u8().unwrap(), 19);
        assert_eq!(Bit::Letter(Letter::T).to_u8().unwrap(), 20);
        assert_eq!(Bit::Letter(Letter::U).to_u8().unwrap(), 21);
        assert_eq!(Bit::Letter(Letter::V).to_u8().unwrap(), 22);
        assert_eq!(Bit::Letter(Letter::W).to_u8().unwrap(), 23);
        assert_eq!(Bit::Letter(Letter::X).to_u8().unwrap(), 24);
        assert_eq!(Bit::Letter(Letter::Y).to_u8().unwrap(), 25);
        assert_eq!(Bit::Letter(Letter::Z).to_u8().unwrap(), 26);
        assert_eq!(Bit::Letter(Letter::One).to_u8().unwrap(), 27);
        assert_eq!(Bit::Letter(Letter::Two).to_u8().unwrap(), 28);
        assert_eq!(Bit::Letter(Letter::Three).to_u8().unwrap(), 29);
        assert_eq!(Bit::Letter(Letter::Four).to_u8().unwrap(), 30);
        assert_eq!(Bit::Letter(Letter::Five).to_u8().unwrap(), 31);
        assert_eq!(Bit::Letter(Letter::Six).to_u8().unwrap(), 32);
        assert_eq!(Bit::Letter(Letter::Seven).to_u8().unwrap(), 33);
        assert_eq!(Bit::Letter(Letter::Eight).to_u8().unwrap(), 34);
        assert_eq!(Bit::Letter(Letter::Nine).to_u8().unwrap(), 35);
        assert_eq!(Bit::Letter(Letter::Zero).to_u8().unwrap(), 36);
        assert_eq!(Bit::Letter(Letter::Exclamation).to_u8().unwrap(), 37);
        assert_eq!(Bit::Letter(Letter::At).to_u8().unwrap(), 38);
        assert_eq!(Bit::Letter(Letter::Pound).to_u8().unwrap(), 39);
        assert_eq!(Bit::Letter(Letter::Dollar).to_u8().unwrap(), 40);
        assert_eq!(Bit::Letter(Letter::LeftParen).to_u8().unwrap(), 41);
        assert_eq!(Bit::Letter(Letter::RightParen).to_u8().unwrap(), 42);
        assert_eq!(Bit::Letter(Letter::Hyphen).to_u8().unwrap(), 44);
        assert_eq!(Bit::Letter(Letter::Plus).to_u8().unwrap(), 46);
        assert_eq!(Bit::Letter(Letter::Ampersand).to_u8().unwrap(), 47);
        assert_eq!(Bit::Letter(Letter::Equal).to_u8().unwrap(), 48);
        assert_eq!(Bit::Letter(Letter::SemiColon).to_u8().unwrap(), 49);
        assert_eq!(Bit::Letter(Letter::Colon).to_u8().unwrap(), 50);
        assert_eq!(Bit::Letter(Letter::Quote).to_u8().unwrap(), 52);
        assert_eq!(Bit::Letter(Letter::DoubleQuote).to_u8().unwrap(), 53);
        assert_eq!(Bit::Letter(Letter::Percent).to_u8().unwrap(), 54);
        assert_eq!(Bit::Letter(Letter::Comma).to_u8().unwrap(), 55);
        assert_eq!(Bit::Letter(Letter::Period).to_u8().unwrap(), 56);
        assert_eq!(Bit::Letter(Letter::Slash).to_u8().unwrap(), 59);
        assert_eq!(Bit::Letter(Letter::Question).to_u8().unwrap(), 60);
        assert_eq!(Bit::Letter(Letter::Degree).to_u8().unwrap(), 62);
        assert_eq!(Bit::Color(Color::Red).to_u8().unwrap(), 63);
        assert_eq!(Bit::Color(Color::Orange).to_u8().unwrap(), 64);
        assert_eq!(Bit::Color(Color::Yellow).to_u8().unwrap(), 65);
        assert_eq!(Bit::Color(Color::Green).to_u8().unwrap(), 66);
        assert_eq!(Bit::Color(Color::Blue).to_u8().unwrap(), 67);
        assert_eq!(Bit::Color(Color::Violet).to_u8().unwrap(), 68);
        assert_eq!(Bit::Color(Color::White).to_u8().unwrap(), 69);
        assert_eq!(Bit::Color(Color::Black).to_u8().unwrap(), 70);
        assert_eq!(Bit::Filed.to_u8().unwrap(), 71);
    }

    #[test]
    fn test_message_crafting() {
        let mut message = Message::default();
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
        let mut message = Message::default();

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
