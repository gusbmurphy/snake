use crate::prelude::*;

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

pub struct Snake {
    x_position: i32,
    y_position: i32,
    facing: Direction,
}

impl Snake {
    pub fn new(x_position: i32, y_position: i32) -> Self {
        Snake {
            x_position,
            y_position,
            facing: Direction::Down,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            self.x_position,
            self.y_position,
            GREEN,
            BLACK,
            to_cp437('@'),
        )
    }

    pub fn change_facing(&mut self, new_direction: Direction) {
        self.facing = new_direction;
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

impl PositionInSpace for Snake {
    fn get_x_position(&self) -> i32 {
        self.x_position
    }

    fn get_y_position(&self) -> i32 {
        self.y_position
    }
}
