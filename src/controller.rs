use crate::prelude::*;

const FRAME_DURATION: f32 = 75.0;
const BG_COLOR: (u8, u8, u8) = BLACK;
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

enum GamePhase {
    Menu,
    Playing,
    GameOver,
}

pub struct Controller {
    board: Board,
    phase: GamePhase,
    frame_time: f32,
}

impl Controller {
    pub fn new(board: Board) -> Self {
        Controller {
            board,
            phase: GamePhase::Menu,
            frame_time: 0.0,
        }
    }
}

impl GameState for Controller {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.phase {
            GamePhase::GameOver => self.game_over(ctx),
            GamePhase::Playing => self.play(ctx),
            GamePhase::Menu => self.main_menu(ctx),
        }
    }
}

impl Controller {
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLACK);

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.board.tick(ctx.key);
        }

        ctx.print(0, 0, &format!("Score: {}", self.board.get_score()));

        for representable in self.board.get_screen_representations() {
            self.render_screen_representation(representable, ctx)
        }
    }

    fn render_screen_representation(&self, representation: ScreenRepresentation, ctx: &mut BTerm) {
        ctx.set(
            representation.get_x_position(),
            representation.get_y_position(),
            representation.get_color(),
            BG_COLOR,
            representation.get_glyph(),
        );
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
        ctx.print_centered(6, &format!("You earned {} points", self.board.get_score()));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        self.watch_for_start_or_quit(ctx);
    }

    fn restart(&mut self) {
        self.phase = GamePhase::Playing;
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
}
