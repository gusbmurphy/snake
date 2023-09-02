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

    pub fn get_glyph(&self) -> FontCharType {
        return self.glyph;
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        return self.color;
    }
}

impl Position for ScreenRepresentation {
    fn get_x_position(&self) -> i32 {
        return self.x_position;
    }

    fn get_y_position(&self) -> i32 {
        return self.y_position;
    }
}

pub trait ScreenRepresentable {
    fn get_screen_representation(&self) -> ScreenRepresentation;
}
