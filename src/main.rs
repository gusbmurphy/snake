mod apple;
mod snake;
mod state;

mod prelude {
    pub use crate::apple::*;
    pub use crate::snake::*;
    pub use crate::state::*;
    pub use bracket_lib::prelude::*;
}

use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Snake").build()?;

    main_loop(context, State::new())
}
