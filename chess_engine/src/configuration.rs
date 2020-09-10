use crate::*;
use pieces::default::*;

pub type BoardGenerator = Box<dyn Fn(&mut Vec<Vec<ChessTile>>)>;

pub struct BoardConfig {
    pub white_stride: i32,
    pub pawn_extra_steps: i32,
    pub place_pawns: BoardGenerator,
}

impl BoardConfig {
    pub fn default() -> Self {
        Self {
            white_stride: -1,
            pawn_extra_steps: 2,
            place_pawns: Box::new(place_defaults),
        }
    }
}

macro_rules! create_piece {
    ($piece:ident, $team:expr) => {
        ChessTile::from($piece($team))
    };
}

pub fn place_defaults(tiles: &mut Vec<Vec<ChessTile>>) {
    //place pawns
    for x in 0..8 {
        tiles[x][6] = create_piece!(pawn, Team::White);
        tiles[x][1] = create_piece!(pawn, Team::Black);
    }

    tiles[4][0] = create_piece!(king, Team::White);
    tiles[4][7] = create_piece!(king, Team::White);
}
