mod apple;
mod snake;
mod board;
mod screen_representation;
mod controller;

mod prelude {
    pub use crate::screen_representation::*;
    pub use crate::apple::*;
    pub use crate::snake::*;
    pub use crate::board::*;
    pub use bracket_lib::prelude::*;
}

use controller::Controller;
use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Snake").build()?;

    let board = Board::new();

    main_loop(context, Controller::new(board))
}
