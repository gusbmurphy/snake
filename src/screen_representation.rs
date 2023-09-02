use crate::prelude::*;

pub trait ScreenRepresentation: Position {
    fn get_glyph(&self) -> FontCharType;
    fn get_color(&self) -> (u8, u8, u8);
}
