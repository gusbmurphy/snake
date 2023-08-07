mod player;
mod state;

mod prelude {
    pub use crate::player::*;
    pub use crate::state::*;
    pub use bracket_lib::prelude::*;
}

use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Snake").build()?;

    main_loop(context, State::new())
}
