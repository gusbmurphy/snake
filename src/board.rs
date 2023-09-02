use crate::prelude::*;

const FRAME_DURATION: f32 = 75.0;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

enum GamePhase {
    Menu,
    Playing,
    GameOver,
}

pub trait Position {
    fn get_x_position(&self) -> i32;

    fn get_y_position(&self) -> i32;

    fn is_at_same_position_as(&self, other: &impl Position) -> bool {
        self.get_x_position() == other.get_x_position()
            && self.get_y_position() == other.get_y_position()
    }
}

pub struct Board {
    phase: GamePhase,
    score: i32,
    frame_time: f32,
    player: Snake,
    apple: Apple,
    turns: [[Option<Turn>; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
}

impl GameState for Board {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.phase {
            GamePhase::GameOver => self.game_over(ctx),
            GamePhase::Playing => self.play(ctx),
            GamePhase::Menu => self.main_menu(ctx),
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            phase: GamePhase::Menu,
            score: 0,
            frame_time: 0.0,
            player: Snake::new(10, 10),
            apple: Apple::new(20, 20),
            turns: [[None; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
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
        self.phase = GamePhase::Playing;
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

    fn generate_random_apple(&mut self) {
        let mut random = RandomNumberGenerator::new();
        let x_position = random.range(1, SCREEN_WIDTH);
        let y_position = random.range(1, SCREEN_HEIGHT);

        self.apple = Apple::new(x_position, y_position);
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

        if self.player.is_at_same_position_as(&self.apple) {
            self.score += 1;
            self.generate_random_apple();
        }

        ctx.print(0, 0, &format!("Score: {}", self.score));

        self.apple.render(ctx);
        self.player.render(ctx);
    }

    fn handle_input(&mut self, pressed_key: Option<VirtualKeyCode>) {
        if let Some(direction) = Direction::from_key_code(pressed_key) {
            let player_x = self.player.get_x_position();
            let player_y = self.player.get_y_position();

            let new_turn = Turn::new(player_x, player_y, direction);

            self.turns[player_x as usize][player_y as usize] = Some(new_turn);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrow_key_places_turn_at_snake_head() {
        let snake = Snake::new(5, 7);
        let mut state = Board::new();
        state.player = snake;

        state.handle_input(Some(VirtualKeyCode::Up));

        let expected_turn = state.turns[5][7];

        assert!(expected_turn.is_some());
        assert!(expected_turn.unwrap().direction == Direction::Up);
    }
}
