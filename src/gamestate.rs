use rltk::{Rltk, GameState};
use specs::prelude::*;
use crate::component::{Position, Renderable, Player, player};
use crate::worldmap;

pub struct State {
    pub ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player::player_input(self, ctx);
        let map = self.ecs.fetch::<Vec<worldmap::TileType>>();
        worldmap::draw_map(&map, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let players = self.ecs.read_storage::<Player>();
        for (_n, (pos, _player)) in (&positions, &players).join().enumerate() {
            ctx.print(0, 0, format!("{}/{}", pos.x, pos.y));
        }
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}