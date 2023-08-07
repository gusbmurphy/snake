use bracket_lib::prelude::*;

struct Player {
    x_position: i32,
    y_position: i32,
}

impl Player {
    fn new(x_position: i32, y_position: i32) -> Self {
        Player {
            x_position,
            y_position,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x_position, self.y_position, GREEN, BLACK, to_cp437('@'))
    }
}

enum GameMode {
    Menu,
    Playing,
    GameOver,
}

struct State {
    mode: GameMode,
    score: i32,
    player: Player,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::GameOver => self.game_over(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Menu => self.main_menu(ctx),
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            score: 0,
            player: Player::new(10, 10),
        }
    }

    fn watch_for_start_or_quit(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Snake");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        self.watch_for_start_or_quit(ctx);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        self.watch_for_start_or_quit(ctx);
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLACK);

        self.player.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Snake").build()?;

    main_loop(context, State::new())
}
