use rltk::RGB;
use specs::prelude::*;

mod worldmap;
mod component;
mod gamestate;
use component::{Position, Renderable, Player};


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = gamestate::State{
        ecs: World::new()
    };

    let (rooms, map) = worldmap::racmap::new();
    gs.ecs.insert(map);
    let (px, py) = rooms[0].center();

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs
        .create_entity()
        .with(Position {x: px, y: py})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();
    rltk::main_loop(context, gs)
}

