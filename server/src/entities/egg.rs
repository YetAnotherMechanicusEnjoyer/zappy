#[derive(Debug, Clone)]
pub struct Egg {
    pub id: usize,
    pub parent_id: Option<usize>,
    pub team_name: String,
    pub x: usize,
    pub y: usize,
}

impl Egg {
    pub fn new(id: usize, parent_id: Option<usize>, team_name: String, x: usize, y: usize) -> Self {
        Self {
            id,
            parent_id,
            team_name,
            x,
            y,
        }
    }
}
