mod apple;
mod board;
mod controller;
mod screen_representation;
mod snake;

mod prelude {
    pub use crate::apple::*;
    pub use crate::board::*;
    pub use crate::screen_representation::*;
    pub use crate::snake::*;
    pub use bracket_lib::prelude::*;
}

use controller::Controller;
use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Snake").build()?;

    let board = Board::new();

    main_loop(context, Controller::new(board))
}
