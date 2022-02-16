use rltk::RGB;
use specs::prelude::*;

mod worldmap;
mod component;
mod rect;
mod gamestate;
mod system;
use component::{Position, Renderable, Player, Viewshed};


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = gamestate::State{
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map = worldmap::Map::new_map_rooms_and_corridors();
    let (px, py) = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position {x: px, y: py})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed {visible_tiles: Vec::new(), range: 8, dirty: true})
        .build();

    rltk::main_loop(context, gs)
}

