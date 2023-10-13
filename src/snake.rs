use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_key_code(key_code: Option<VirtualKeyCode>) -> Option<Direction> {
        match key_code {
            Some(VirtualKeyCode::Up) => Some(Direction::Up),
            Some(VirtualKeyCode::Down) => Some(Direction::Down),
            Some(VirtualKeyCode::Left) => Some(Direction::Left),
            Some(VirtualKeyCode::Right) => Some(Direction::Right),
            _ => None,
        }
    }
}

pub struct SnakeNode {
    x_position: i32,
    y_position: i32,
    facing: Direction,
}

impl SnakeNode {
    pub fn new(x_position: i32, y_position: i32) -> Self {
        SnakeNode {
            x_position,
            y_position,
            facing: Direction::Down,
        }
    }

    pub fn change_facing(&mut self, new_direction: Direction) {
        self.facing = new_direction;
    }

    pub fn get_facing(&mut self) -> Direction {
        return self.facing;
    }

    pub fn move_forward(&mut self) {
        match self.facing {
            Direction::Up => self.y_position -= 1,
            Direction::Down => self.y_position += 1,
            Direction::Left => self.x_position -= 1,
            Direction::Right => self.x_position += 1,
        }
    }
}

impl Position for SnakeNode {
    fn get_x_position(&self) -> i32 {
        self.x_position
    }

    fn get_y_position(&self) -> i32 {
        self.y_position
    }
}

impl ScreenRepresentable for SnakeNode {
    fn get_screen_representation(&self) -> ScreenRepresentation {
        return ScreenRepresentation::new(to_cp437('@'), GREEN, self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn change_facing_sets_facing() {
        let mut snake = SnakeNode::new(2, 3);
        snake.change_facing(Direction::Left);
        assert_eq!(snake.facing, Direction::Left);
    }

    #[test]
    fn get_facing_returns_facing() {
        let mut snake = SnakeNode::new(2, 3);
        snake.facing = Direction::Left;
        assert_eq!(snake.get_facing(), Direction::Left);
    }

    #[rstest]
    #[case(Direction::Up, 1, 0)]
    #[case(Direction::Down, 1, 2)]
    #[case(Direction::Left, 0, 1)]
    #[case(Direction::Right, 2, 1)]
    fn move_forward_changes_position(#[case] direction: Direction, #[case] expected_x: i32, #[case] expected_y: i32) {
        let mut snake = SnakeNode::new(1, 1);
        snake.facing = direction;
        snake.move_forward();
        assert_eq!(snake.x_position, expected_x);
        assert_eq!(snake.y_position, expected_y);
    }
}
