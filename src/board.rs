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
    snake: Snake,
    apple: Apple,
    turns: [[Option<Turn>; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
}

impl Board {
    pub fn new() -> Self {
        Board {
            score: 0,
            snake: Snake::new(10, 10),
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
        representable_objects.push(self.snake.get_screen_representation());

        return representable_objects;
    }

    pub fn tick(&mut self, player_direction: Direction) {
        self.snake.change_facing(player_direction);
        self.add_turn_at_player_position_with_player_facing();

        if Self::are_at_same_position(&self.snake, &self.apple) {
            self.score += 1;
            self.generate_random_apple();
        }

        self.snake.move_forward();
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

    fn add_turn_at_player_position_with_player_facing(&mut self) {
        let x_position = self.snake.get_x_position();
        let y_position = self.snake.get_y_position();

        self.turns[x_position as usize][y_position as usize] =
            Some(Turn::new(x_position, y_position, self.snake.get_facing()));
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

    #[test]
    fn get_screen_representations_returns_correct_order() {
        let test_apple = Apple::new(2, 3);
        let test_snake = Snake::new(7, 10);

        let mut board = Board::new();
        board.apple = test_apple;
        board.snake = test_snake;

        let representations = board.get_screen_representations();
        assert_eq!(representations.get(0).unwrap().get_x_position(), 2);
        assert_eq!(representations.get(1).unwrap().get_x_position(), 7);
    }

    #[test]
    fn score_is_incremented_if_player_is_on_apple_at_tick() {
        let test_apple = Apple::new(3, 3);
        let test_snake = Snake::new(3, 3);

        let mut board = Board::new();
        board.apple = test_apple;
        board.snake = test_snake;
        board.score = 5;

        board.tick(Direction::Down);

        assert_eq!(board.score, 6);
    }

    #[test]
    fn snake_is_moved_in_new_direction_after_tick() {
        let snake = Snake::new(3, 3);

        let mut board = Board::new();
        board.snake = snake;

        board.tick(Direction::Right);

        assert_eq!(board.snake.get_x_position(), 4);
        assert_eq!(board.snake.get_y_position(), 3);
    }

    #[test]
    fn turn_is_added_when_player_turns() {
        let snake = Snake::new(3, 3);

        let mut board = Board::new();
        board.snake = snake;

        board.tick(Direction::Down);

        assert_eq!(board.turns[3][3].unwrap().direction, Direction::Down);
    }
}
