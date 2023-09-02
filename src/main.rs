mod apple;
mod snake;
mod state;
mod controller;

mod prelude {
    pub use crate::apple::*;
    pub use crate::snake::*;
    pub use crate::state::*;
    pub use bracket_lib::prelude::*;
}

use controller::Controller;
use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Snake").build()?;

    let state = State::new();

    main_loop(context, Controller::new(state))
}
