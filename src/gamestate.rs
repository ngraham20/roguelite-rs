use rltk::{Rltk, GameState};
use specs::prelude::*;
use crate::component::{Position, Renderable, Player, player};
use crate::worldmap;
use crate::system::Visibility;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = Visibility{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player::player_input(self, ctx);
        self.run_systems();

        worldmap::draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let players = self.ecs.read_storage::<Player>();
        for (pos, _player) in (&positions, &players).join() {
            ctx.print(0, 0, format!("{}/{}", pos.x, pos.y));
        }
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}