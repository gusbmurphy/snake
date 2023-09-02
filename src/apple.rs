use crate::prelude::*;

pub struct Apple {
    x_position: i32,
    y_position: i32,
}

impl Apple {
    pub fn new(x_position: i32, y_position: i32) -> Self {
        Apple {
            x_position,
            y_position,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x_position, self.y_position, RED, BLACK, to_cp437('▲'))
    }
}

impl Position for Apple {
    fn get_x_position(&self) -> i32 {
        self.x_position
    }

    fn get_y_position(&self) -> i32 {
        self.y_position
    }
}

impl ScreenRepresentable for Apple {
    fn get_screen_representation(&self) -> ScreenRepresentation {
        return ScreenRepresentation::new(to_cp437('▲'), RED, self);
    }
}
