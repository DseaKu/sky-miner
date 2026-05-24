use crate::terrain::TileType;

pub struct TileGenerator;

impl TileGenerator {
    pub fn generate_tile(&self, x: i32, y: i32) -> TileType {
        if y < 0 {
            return TileType::default();
        }
        TileType::Stone
    }
    // fn _calc_height_penalty(&self, cur_pos: &f32) -> f32 {
    //     use consts::{isle, HEIGHT_PENALTY};
    //     let move_toward = utils::FloatExt::move_toward;
    //
    //     move_toward(isle::ISLAND_THRESHOLD, 0.0, cur_pos * HEIGHT_PENALTY)
    // }
}
