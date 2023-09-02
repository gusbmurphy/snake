use crate::prelude::*;

pub struct Controller {
    state: Board,
}

impl Controller {
    pub fn new(state: Board) -> Self {
        Controller {
            state,
        }
    }
}

impl GameState for Controller {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.state.tick(ctx);
    }
}
