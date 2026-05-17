#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Void,
    Stone,
    Dirt,
    Grass,
}

pub struct Chunk {
    pub tiles: Vec<TileType>,
}

impl Chunk {
    pub fn new(size: i32) -> Self {
        let count = (size * size) as usize;
        Self {
            tiles: vec![TileType::Void; count],
        }
    }
}
