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

    pub fn get_screen_representations(&mut self) -> Vec<ScreenRepresentation> {
        let mut representable_objects: Vec<ScreenRepresentation> = Vec::new();

        representable_objects.push(self.apple.get_screen_representation());
        representable_objects.push(self.player.get_screen_representation());

        return representable_objects;
    }

    pub fn tick(&mut self, player_direction: Direction) {
        self.player.change_facing(player_direction);

        if Self::are_at_same_position(&self.player, &self.apple) {
            self.score += 1;
            self.generate_random_apple();
        }

        self.player.move_forward();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_score_returns_current_score() {
        let mut board = Board::new();
        board.score = 32;
        assert_eq!(board.get_score(), 32);
    }
}
