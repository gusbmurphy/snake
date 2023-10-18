use crate::controller::SCREEN_HEIGHT;
use crate::controller::SCREEN_WIDTH;
use crate::prelude::*;

pub trait Position {
    fn get_x_position(&self) -> i32;
    fn get_y_position(&self) -> i32;
}

#[derive(Clone, Copy)]
pub struct Turn {
    x_position: i32,
    y_position: i32,
    pub direction: Direction,
}

impl Turn {
    pub fn new(x_position: i32, y_position: i32, direction: Direction) -> Self {
        Turn {
            x_position,
            y_position,
            direction,
        }
    }
}

impl Position for Turn {
    fn get_x_position(&self) -> i32 {
        self.x_position
    }

    fn get_y_position(&self) -> i32 {
        self.y_position
    }
}

pub struct Board {
    score: i32,
    snake_head: SnakeNode,
    snake_tail: Vec<SnakeNode>,
    should_add_to_tail: bool,
    apple: Apple,
    turns: Vec<Turn>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            score: 0,
            snake_head: SnakeNode::new(10, 10),
            snake_tail: Vec::new(),
            should_add_to_tail: false,
            apple: Apple::new(20, 20),
            turns: Vec::new(),
        }
    }

    pub fn get_score(&mut self) -> i32 {
        return self.score;
    }

    pub fn get_screen_representations(&mut self) -> Vec<ScreenRepresentation> {
        let mut representable_objects: Vec<ScreenRepresentation> = Vec::new();

        representable_objects.push(self.apple.get_screen_representation());
        representable_objects.append(&mut self.get_screen_representations_from_tail());
        representable_objects.push(self.snake_head.get_screen_representation());

        return representable_objects;
    }

    fn get_screen_representations_from_tail(&mut self) -> Vec<ScreenRepresentation> {
        self.snake_tail
            .clone()
            .into_iter()
            .map(|node| node.get_screen_representation())
            .clone()
            .collect::<Vec<ScreenRepresentation>>()
    }

    pub fn tick(&mut self, new_direction: Direction) {
        if new_direction != self.snake_head.get_facing() {
            self.snake_head.change_facing(new_direction);
            self.add_turn_at_player_position_with_player_facing();
        }

        if self.should_add_to_tail {
            self.snake_tail.push(SnakeNode::new(
                self.snake_head.get_x_position(),
                self.snake_head.get_y_position(),
            ));

            self.should_add_to_tail = false;
        } else {
            for tail_index in 0..self.snake_tail.len() {
                self.snake_tail[tail_index].move_forward();

                for turn_index in 0..self.turns.len() {
                    if Self::are_at_same_position(
                        &self.snake_tail[tail_index],
                        &self.turns[turn_index],
                    ) {
                        self.snake_tail[tail_index].change_facing(self.turns[turn_index].direction);
                    }
                }
            }
        }

        self.snake_head.move_forward();

        if Self::are_at_same_position(&self.snake_head, &self.apple) {
            self.score += 1;
            self.generate_random_apple();
            self.should_add_to_tail = true;
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

    fn add_turn_at_player_position_with_player_facing(&mut self) {
        let x_position = self.snake_head.get_x_position();
        let y_position = self.snake_head.get_y_position();

        self.turns.push(Turn::new(
            x_position,
            y_position,
            self.snake_head.get_facing(),
        ));
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
        let test_snake = SnakeNode::new(7, 10);
        let test_tail = Vec::from([SnakeNode::new(8, 10)]);

        let mut board = Board::new();
        board.apple = test_apple;
        board.snake_head = test_snake;
        board.snake_tail = test_tail;

        let representations = board.get_screen_representations();
        assert_eq!(
            representations.get(0).unwrap().get_x_position(),
            2,
            "the apple is first"
        );
        assert_eq!(
            representations.get(1).unwrap().get_x_position(),
            8,
            "the tail is second"
        );
        assert_eq!(
            representations.get(2).unwrap().get_x_position(),
            7,
            "the head is last"
        );
    }

    #[test]
    fn score_is_incremented_if_player_is_on_apple_after_tick() {
        let test_apple = Apple::new(3, 4);
        let test_snake = SnakeNode::new(3, 3);

        let mut board = Board::new();
        board.apple = test_apple;
        board.snake_head = test_snake;
        board.score = 0;

        board.tick(Direction::Down);

        assert_eq!(board.score, 1);
    }

    #[test]
    fn snake_is_moved_in_new_direction_after_tick() {
        let snake = SnakeNode::new(3, 3);

        let mut board = Board::new();
        board.snake_head = snake;

        board.tick(Direction::Right);

        assert_eq!(board.snake_head.get_x_position(), 4);
        assert_eq!(board.snake_head.get_y_position(), 3);
    }

    #[test]
    fn turn_is_added_when_player_turns() {
        let snake = SnakeNode::new(3, 3);

        let mut board = Board::new();
        board.snake_head = snake;

        board.tick(Direction::Left);

        assert_eq!(board.turns[0].direction, Direction::Left);
        assert_eq!(board.turns[0].x_position, 3);
        assert_eq!(board.turns[0].y_position, 3);
    }

    #[test]
    fn turn_is_not_added_if_direction_given_at_tick_is_same_as_snake_facing() {
        let mut snake = SnakeNode::new(3, 3);
        snake.change_facing(Direction::Up);

        let mut board = Board::new();
        board.snake_head = snake;

        board.tick(Direction::Up);

        assert!(!board_has_any_turns(board))
    }

    fn board_has_any_turns(board: Board) -> bool {
        !board.turns.is_empty()
    }

    #[test]
    fn on_tick_after_scoring_tick_node_is_added_where_apple_was() {
        let test_apple = Apple::new(3, 4);
        let test_snake = SnakeNode::new(3, 3);

        let mut board = Board::new();
        board.apple = test_apple;
        board.snake_head = test_snake;

        board.tick(Direction::Down);
        board.tick(Direction::Down);

        let added_node = &board.snake_tail[0];
        assert_eq!(
            added_node.get_x_position(),
            3,
            "the added node is at the same X position that the apple was"
        );
        assert_eq!(
            added_node.get_y_position(),
            4,
            "the added node is at the same Y position that the apple was"
        );
    }

    #[test]
    fn right_after_scoring_tick_no_node_is_added() {
        let test_apple = Apple::new(3, 4);
        let test_snake = SnakeNode::new(3, 3);

        let mut board = Board::new();
        board.apple = test_apple;
        board.snake_head = test_snake;

        board.tick(Direction::Down);

        assert_eq!(board.snake_tail.len(), 0);
    }

    #[test]
    fn snake_tail_node_moves_in_direction_of_facing() {
        let snake_head = SnakeNode::new(8, 8);
        let mut tail_node = SnakeNode::new(3, 2);
        tail_node.change_facing(Direction::Left);

        let mut board = Board::new();
        board.snake_head = snake_head;
        board.snake_tail = Vec::from([tail_node.clone()]);

        board.tick(Direction::Down);

        assert_eq!(board.snake_tail[0].get_x_position(), 2);
        assert_eq!(board.snake_tail[0].get_y_position(), 2);
    }

    #[test]
    fn tail_node_takes_direction_of_turn() {
        let snake_head = SnakeNode::new(8, 8); // Just pointing out, the snake doesn't need to be "attached"
        let mut tail_node = SnakeNode::new(3, 2);
        tail_node.change_facing(Direction::Down);

        let turn = Turn::new(3, 3, Direction::Left);

        let mut board = Board::new();
        board.snake_head = snake_head;
        board.snake_tail = Vec::from([tail_node.clone()]);
        board.turns = Vec::from([turn.clone()]);

        board.tick(Direction::Up);

        assert_eq!(board.snake_tail[0].get_facing(), Direction::Left);
    }
}
