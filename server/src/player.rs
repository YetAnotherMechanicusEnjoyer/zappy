use crate::constants::INITIAL_PLAYER_LEVEL;

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub team_name: String,
    pub x: usize,
    pub y: usize,
    pub orientation: Orientation,
    pub level: usize,
}

impl Player {
    pub fn new(id: usize, team_name: String, x: usize, y: usize, orientation: Orientation) -> Self {
        Self {
            id,
            team_name,
            x,
            y,
            orientation,
            level: INITIAL_PLAYER_LEVEL,
        }
    }
}
