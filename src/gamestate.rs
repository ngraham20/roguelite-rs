use rltk::{Rltk, GameState};
use specs::prelude::*;
use crate::component::{Position, Renderable, Player, player};
use crate::worldmap;
use crate::worldmap::{Map};
use crate::system::*;

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = Visibility{};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);

        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
        
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player::player_input(self, ctx);
        }

        worldmap::draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        let players = self.ecs.read_storage::<Player>();
        for (pos, _player) in (&positions, &players).join() {
            ctx.print(0, 0, format!("{}/{}", pos.x, pos.y));
        }
        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            };
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}