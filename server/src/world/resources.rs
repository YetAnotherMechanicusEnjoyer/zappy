use crate::world::map::{GameMap, Resource};
use rand::Rng;
use std::collections::HashSet;

pub fn spawn_initial_resources(map: &mut GameMap) {
    let mut random = rand::thread_rng();

    for resource in Resource::ALL {
        let quantity = target_quantity(map.tile_count(), resource.density());

        for _ in 0..quantity {
            add_random_resource(map, resource, &mut random);
        }
    }
}

pub fn refill_missing_resources(map: &mut GameMap) -> HashSet<(usize, usize)> {
    let mut changed = HashSet::new();
    let mut random = rand::thread_rng();

    for resource in Resource::ALL {
        let target = target_quantity(map.tile_count(), resource.density());
        let current = count_map_resource(map, resource);

        for _ in current..target {
            let position = add_random_resource(map, resource, &mut random);

            changed.insert(position);
        }
    }

    changed
}

pub fn target_quantity(tile_count: usize, density: f64) -> usize {
    ((tile_count as f64 * density) as usize).max(1)
}

pub fn count_map_resource(map: &GameMap, resource: Resource) -> usize {
    (0..map.tile_count())
        .filter_map(|index| map.get_tile_by_index(index))
        .map(|tile| tile.resource_count(resource))
        .sum()
}

fn add_random_resource<R: Rng + ?Sized>(
    map: &mut GameMap,
    resource: Resource,
    random: &mut R,
) -> (usize, usize) {
    let index = random.gen_range(0..map.tile_count());
    let x = index % map.width;
    let y = index / map.width;

    if let Some(tile) = map.get_tile_by_index_mut(index) {
        tile.add_resource(resource);
    }

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::{count_map_resource, spawn_initial_resources, target_quantity};
    use crate::world::map::{GameMap, Resource};

    #[test]
    fn uses_subject_densities() {
        assert_eq!(target_quantity(100, 0.5), 50);
        assert_eq!(target_quantity(100, 0.05), 5);
        assert_eq!(target_quantity(110, 0.15), 16);
        assert_eq!(target_quantity(121, 0.5), 60);
    }

    #[test]
    fn initial_map_has_expected_totals() {
        let mut map = GameMap::new(10, 10);

        spawn_initial_resources(&mut map);

        assert_eq!(count_map_resource(&map, Resource::Food), 50);
        assert_eq!(count_map_resource(&map, Resource::Thystame), 5);
    }
}
