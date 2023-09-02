use crate::prelude::*;

pub struct ScreenRepresentation {
    glyph: FontCharType,
    color: (u8, u8, u8),
    x_position: i32,
    y_position: i32,
}

impl ScreenRepresentation {
    pub fn new(glyph: FontCharType, color: (u8, u8, u8), position: &impl Position) -> Self {
        ScreenRepresentation {
            glyph,
            color,
            x_position: position.get_x_position(),
            y_position: position.get_y_position(),
        }
    }   
}

pub trait ScreenRepresentable {
    fn get_screen_representation(&self) -> ScreenRepresentation;
}
