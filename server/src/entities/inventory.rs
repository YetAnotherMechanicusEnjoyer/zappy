use crate::constants::INITIAL_VISIBLE_FOOD;
use crate::world::map::Resource;

#[derive(Debug, Clone)]
pub struct Inventory {
    quantities: [usize; 7],
}

impl Inventory {
    pub fn new_player_inventory() -> Self {
        let mut quantities = [0; 7];
        quantities[Resource::Food.gui_index()] = INITIAL_VISIBLE_FOOD;
        Self { quantities }
    }

    pub fn count(&self, resource: Resource) -> usize {
        self.quantities[resource.gui_index()]
    }

    pub fn add(&mut self, resource: Resource) {
        self.quantities[resource.gui_index()] += 1;
    }

    pub fn remove(&mut self, resource: Resource) -> bool {
        let quantity = &mut self.quantities[resource.gui_index()];
        if *quantity == 0 {
            return false;
        }
        *quantity -= 1;
        true
    }

    pub fn quantities(&self) -> [usize; 7] {
        self.quantities
    }

    pub fn ai_response(&self) -> String {
        format!(
            "[ food {}, linemate {}, deraumere {}, sibur {}, mendiane {}, phiras {}, thystame {} ]\n",
            self.count(Resource::Food),
            self.count(Resource::Linemate),
            self.count(Resource::Deraumere),
            self.count(Resource::Sibur),
            self.count(Resource::Mendiane),
            self.count(Resource::Phiras),
            self.count(Resource::Thystame),
        )
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new_player_inventory()
    }
}

#[cfg(test)]
mod tests {
    use super::Inventory;
    use crate::world::map::Resource;

    #[test]
    fn starts_with_reference_visible_food() {
        assert_eq!(Inventory::default().count(Resource::Food), 9);
    }
}
