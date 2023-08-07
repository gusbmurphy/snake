use crate::prelude::*;

pub struct Player {
    x_position: i32,
    y_position: i32,
}

impl Player {
    pub fn new(x_position: i32, y_position: i32) -> Self {
        Player {
            x_position,
            y_position,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            self.x_position,
            self.y_position,
            GREEN,
            BLACK,
            to_cp437('@'),
        )
    }
}
