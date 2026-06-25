use crate::constants::{
    DERAUMERE_DENSITY, FOOD_DENSITY, LINEMATE_DENSITY, MENDIANE_DENSITY, PHIRAS_DENSITY,
    SIBUR_DENSITY, THYSTAME_DENSITY,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum Resource {
    Food = 0,
    Linemate = 1,
    Deraumere = 2,
    Sibur = 3,
    Mendiane = 4,
    Phiras = 5,
    Thystame = 6,
}

impl Resource {
    pub const ALL: [Self; 7] = [
        Self::Food,
        Self::Linemate,
        Self::Deraumere,
        Self::Sibur,
        Self::Mendiane,
        Self::Phiras,
        Self::Thystame,
    ];

    pub fn from_protocol_name(name: &str) -> Option<Self> {
        match name {
            "food" => Some(Self::Food),
            "linemate" => Some(Self::Linemate),
            "deraumere" => Some(Self::Deraumere),
            "sibur" => Some(Self::Sibur),
            "mendiane" => Some(Self::Mendiane),
            "phiras" => Some(Self::Phiras),
            "thystame" => Some(Self::Thystame),
            _ => None,
        }
    }

    pub const fn protocol_name(self) -> &'static str {
        match self {
            Self::Food => "food",
            Self::Linemate => "linemate",
            Self::Deraumere => "deraumere",
            Self::Sibur => "sibur",
            Self::Mendiane => "mendiane",
            Self::Phiras => "phiras",
            Self::Thystame => "thystame",
        }
    }

    pub const fn gui_index(self) -> usize {
        self as usize
    }

    pub const fn density(self) -> f64 {
        match self {
            Self::Food => FOOD_DENSITY,
            Self::Linemate => LINEMATE_DENSITY,
            Self::Deraumere => DERAUMERE_DENSITY,
            Self::Sibur => SIBUR_DENSITY,
            Self::Mendiane => MENDIANE_DENSITY,
            Self::Phiras => PHIRAS_DENSITY,
            Self::Thystame => THYSTAME_DENSITY,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tile {
    quantities: [usize; 7],
}

impl Tile {
    pub fn resource_count(&self, resource: Resource) -> usize {
        self.quantities[resource.gui_index()]
    }

    pub fn add_resource(&mut self, resource: Resource) {
        self.quantities[resource.gui_index()] += 1;
    }

    pub fn remove_resource(&mut self, resource: Resource) -> bool {
        let quantity = &mut self.quantities[resource.gui_index()];
        if *quantity == 0 {
            return false;
        }
        *quantity -= 1;
        true
    }

    pub fn remove_many(&mut self, resource: Resource, amount: usize) -> bool {
        let quantity = &mut self.quantities[resource.gui_index()];
        if *quantity < amount {
            return false;
        }
        *quantity -= amount;
        true
    }

    pub fn quantities(&self) -> [usize; 7] {
        self.quantities
    }
}

#[derive(Debug)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Tile>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::default(); width * height],
        }
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.index(x, y).and_then(|index| self.tiles.get(index))
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.index(x, y).and_then(|index| self.tiles.get_mut(index))
    }

    pub fn get_tile_by_index(&self, index: usize) -> Option<&Tile> {
        self.tiles.get(index)
    }

    pub fn get_tile_by_index_mut(&mut self, index: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(index)
    }

    pub fn wrap_x(&self, x: isize) -> usize {
        x.rem_euclid(self.width as isize) as usize
    }

    pub fn wrap_y(&self, y: isize) -> usize {
        y.rem_euclid(self.height as isize) as usize
    }

    pub fn wrapped_position(&self, x: isize, y: isize) -> (usize, usize) {
        (self.wrap_x(x), self.wrap_y(y))
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        (x < self.width && y < self.height).then_some(y * self.width + x)
    }
}

#[cfg(test)]
mod tests {
    use super::{GameMap, Resource};

    #[test]
    fn wraps_both_axes() {
        let map = GameMap::new(10, 10);
        assert_eq!(map.wrapped_position(-1, 10), (9, 0));
    }

    #[test]
    fn adds_and_removes_resources() {
        let mut map = GameMap::new(10, 10);
        let tile = map.get_tile_mut(0, 0).expect("tile");
        tile.add_resource(Resource::Food);
        assert!(tile.remove_resource(Resource::Food));
        assert!(!tile.remove_resource(Resource::Food));
    }
}
