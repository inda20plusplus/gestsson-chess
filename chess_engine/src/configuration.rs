use crate::*;
use pieces::default::*;

pub type BoardGenerator = Box<dyn Fn(&mut Vec<Vec<ChessTile>>)>;

pub struct BoardConfig {
    pub white_stride: i32,
    pub pawn_en_passant: bool,
    pub place_pawns: BoardGenerator,
}

impl BoardConfig {
    pub fn default() -> Self {
        Self {
            white_stride: -1,
            pawn_en_passant: true,
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

    tiles[4][7] = create_piece!(king, Team::White);
    tiles[4][0] = create_piece!(king, Team::Black);

    tiles[3][7] = create_piece!(queen, Team::White);
    tiles[3][0] = create_piece!(queen, Team::Black);

    tiles[2][7] = create_piece!(bishop, Team::White);
    tiles[5][7] = create_piece!(bishop, Team::White);

    tiles[2][0] = create_piece!(bishop, Team::Black);
    tiles[5][0] = create_piece!(bishop, Team::Black);

    tiles[1][7] = create_piece!(knight, Team::White);
    tiles[6][7] = create_piece!(knight, Team::White);

    tiles[1][0] = create_piece!(knight, Team::Black);
    tiles[6][0] = create_piece!(knight, Team::Black);

    tiles[0][7] = create_piece!(rook, Team::White);
    tiles[7][7] = create_piece!(rook, Team::White);

    tiles[0][0] = create_piece!(rook, Team::Black);
    tiles[7][0] = create_piece!(rook, Team::Black);
}
