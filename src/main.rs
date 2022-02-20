use rltk::RGB;
use specs::prelude::*;

mod worldmap;
mod component;
mod rect;
mod gamestate;
mod system;
use gamestate::{State, RunState};
use component::*;


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{
        ecs: World::new(),
        runstate: RunState::Running
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();

    let map = worldmap::Map::new_map_rooms_and_corridors();
    let (px, py) = map.rooms[0].center();

    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => { glyph = rltk::to_cp437('g'); name = "Goblin".to_string();}
            _ => { glyph = rltk::to_cp437('o'); name = "Orc".to_string();}
        }
        gs.ecs.create_entity()
            .with(Position{x, y})
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
            .with(Monster{})
            .with(Name{ name: format!("{} #{}", name, i+1)})
            .with(BlocksTile{})
            .with(CombatStats{max_hp: 16, hp: 16, defense: 1, power: 4})
            .build();
    }

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
        .with(CombatStats {max_hp: 30, hp: 30, defense: 2, power: 5})
        .build();

    gs.ecs.insert(rltk::Point::new(px, py));

    rltk::main_loop(context, gs)
}

