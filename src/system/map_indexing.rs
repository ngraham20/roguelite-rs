use specs::prelude::*;
use crate::component::{Position, BlocksTile};
use crate::worldmap::Map;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        // populate the map with all blocking items
        // - walls (map.populate_blocked())
        // - mobs
        let (mut map, position, blockers) = data;

        // block all tiles that are walls
        map.populate_blocked();

        // retrieve all the blocking entities that have a position
        for (position, _blocks) in (&position, &blockers).join() {
            // get the tile location they're on
            let idx = map.xy_idx(position.x, position.y);

            // block the tile
            map.blocked[idx] = true;
        }
    }
}