use crate::controller::SCREEN_HEIGHT;
use crate::controller::SCREEN_WIDTH;
use crate::prelude::*;

pub trait Position {
    fn get_x_position(&self) -> i32;
    fn get_y_position(&self) -> i32;
}

pub trait ComparePosition: Position {}

pub struct Board {
    score: i32,
    player: Snake,
    apple: Apple,
    turns: [[Option<Turn>; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
}

impl Board {
    pub fn new() -> Self {
        Board {
            score: 0,
            player: Snake::new(10, 10),
            apple: Apple::new(20, 20),
            turns: [[None; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
        }
    }

    pub fn get_score(&mut self) -> i32 {
        return self.score;
    }

    pub fn get_representable_objects(&mut self) -> Vec<ScreenRepresentation> {
        let mut representable_objects: Vec<ScreenRepresentation> = Vec::new();

        representable_objects.push(self.player.get_screen_representation());
        representable_objects.push(self.apple.get_screen_representation());

        return representable_objects;
    }

    pub fn tick(&mut self, pressed_key: Option<VirtualKeyCode>) {
        self.player.move_forward();

        self.handle_input(pressed_key);

        if Self::are_at_same_position(&self.player, &self.apple) {
            self.score += 1;
            self.generate_random_apple();
        }
    }

    fn are_at_same_position(a: &impl Position, b: &impl Position) -> bool {
        a.get_x_position() == b.get_x_position() && a.get_y_position() == b.get_y_position()
    }

    fn generate_random_apple(&mut self) {
        let mut random = RandomNumberGenerator::new();
        let x_position = random.range(1, SCREEN_WIDTH);
        let y_position = random.range(1, SCREEN_HEIGHT);

        self.apple = Apple::new(x_position, y_position);
    }

    fn handle_input(&mut self, pressed_key: Option<VirtualKeyCode>) {
        if let Some(direction) = Direction::from_key_code(pressed_key) {
            self.player.change_facing(direction);

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
