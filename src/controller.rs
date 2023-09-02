use crate::prelude::*;

pub struct Controller {
    state: State,
}

impl Controller {
    pub fn new(state: State) -> Self {
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
