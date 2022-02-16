use rltk::{Rltk, RGB};
pub mod basicmap;
pub mod racmap;
mod rect;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    for (i, tile) in map.iter().enumerate() {
        // render tile for type
        match tile {
            TileType::Floor => {
                ctx.set(i % 80, i / 80, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));

            },
            TileType::Wall => {
                ctx.set(i % 80, i / 80, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }
    }
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}