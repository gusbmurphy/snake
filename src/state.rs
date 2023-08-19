use crate::prelude::*;

const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    GameOver,
}

pub struct State {
    mode: GameMode,
    score: i32,
    frame_time: f32,
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
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
            score: 0,
            frame_time: 0.0,
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

        if let Some(direction) = Direction::from_key_code(ctx.key) {
            self.player.change_facing(direction);
        }

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.move_forward();
        }

        self.player.render(ctx);
    }
}
