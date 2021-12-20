use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
use std::fmt::{Display, Formatter, Result};

#[repr(usize)]
#[derive(Debug, Copy, Clone, IntoEnumIterator, PartialEq, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ResistorColor::Black => {
                write!(f, "Black")
            }
            ResistorColor::Blue => {
                write!(f, "Blue")
            }
            ResistorColor::Brown => {
                write!(f, "Brown")
            }
            ResistorColor::Green => {
                write!(f, "Green")
            }
            ResistorColor::Grey => {
                write!(f, "Grey")
            }
            ResistorColor::Orange => {
                write!(f, "Orange")
            }
            ResistorColor::Red => {
                write!(f, "Red")
            }
            ResistorColor::Violet => {
                write!(f, "Violet")
            }
            ResistorColor::White => {
                write!(f, "White")
            }
            ResistorColor::Yellow => {
                write!(f, "Yellow")
            }
        }
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    // match _color {
    //     ResistorColor::Black => 0,
    //     ResistorColor::Blue => 6,
    //     ResistorColor::Brown => 1,
    //     ResistorColor::Green => 5,
    //     ResistorColor::Grey => 8,
    //     ResistorColor::Orange => 3,
    //     ResistorColor::Red => 2,
    //     ResistorColor::Violet => 7,
    //     ResistorColor::White => 9,
    //     ResistorColor::Yellow => 4,
    // }
    _color.int_value()
}

pub fn color_to_value_ref(_color: &ResistorColor) -> usize {
    _color.int_value()
}
pub fn value_to_color_string(value: usize) -> String {
    let resistor = ResistorColor::from_int(value);
    match resistor {
        Ok(v) => (v).to_string(),
        Err(_e) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    // unimplemented!("return a list of all the colors ordered by resistance")
    let mut arr: Vec<ResistorColor> = ResistorColor::into_enum_iter().collect();
    arr.sort_by_key(color_to_value_ref);
    arr
}
