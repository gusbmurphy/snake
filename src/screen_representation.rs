use crate::prelude::*;

pub trait ScreenRepresentation {
    fn get_x_position(&self) -> i32;
    fn get_y_position(&self) -> i32;
    fn get_glyph(&self) -> FontCharType;
    fn get_color(&self) -> (u8, u8, u8);
}
