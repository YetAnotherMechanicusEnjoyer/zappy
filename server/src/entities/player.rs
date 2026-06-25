use crate::constants::INITIAL_PLAYER_LEVEL;
use crate::entities::inventory::Inventory;
use rand::Rng;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..4) {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            _ => Self::West,
        }
    }

    pub const fn gui_value(self) -> usize {
        match self {
            Self::North => 1,
            Self::East => 2,
            Self::South => 3,
            Self::West => 4,
        }
    }

    pub const fn forward_delta(self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }

    pub const fn right_delta(self) -> (isize, isize) {
        match self {
            Self::North => (1, 0),
            Self::East => (0, 1),
            Self::South => (-1, 0),
            Self::West => (0, -1),
        }
    }

    pub const fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub const fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    pub const fn global_sound_index(self) -> usize {
        match self {
            Self::North => 0,
            Self::West => 2,
            Self::South => 4,
            Self::East => 6,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub team_name: String,
    pub x: usize,
    pub y: usize,
    pub orientation: Orientation,
    pub level: usize,
    pub inventory: Inventory,
    pub next_food_tick: Instant,
    pub frozen_by: Option<usize>,
}

impl Player {
    pub fn new(
        id: usize,
        team_name: String,
        x: usize,
        y: usize,
        orientation: Orientation,
        next_food_tick: Instant,
    ) -> Self {
        Self {
            id,
            team_name,
            x,
            y,
            orientation,
            level: INITIAL_PLAYER_LEVEL,
            inventory: Inventory::default(),
            next_food_tick,
            frozen_by: None,
        }
    }

    pub fn move_forward(&mut self, width: usize, height: usize) {
        let (dx, dy) = self.orientation.forward_delta();
        self.x = (self.x as isize + dx).rem_euclid(width as isize) as usize;
        self.y = (self.y as isize + dy).rem_euclid(height as isize) as usize;
    }

    pub fn turn_right(&mut self) {
        self.orientation = self.orientation.turn_right();
    }

    pub fn turn_left(&mut self) {
        self.orientation = self.orientation.turn_left();
    }
}
