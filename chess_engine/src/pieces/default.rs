use crate::*;
use super::Piece;
use super::moves::*;

use std::rc::Rc;

pub fn pawn(team : Team) -> Piece {
    Piece {
        name: "Pawn".to_owned(),
        necessity: false,
        team: team,
        has_moved: false,
        worth: 1,
        ptr_getmoves: Rc::new(pawn_moves)
    }
}

pub fn king(team : Team) -> Piece {
    Piece {
        name: "King".to_owned(),
        necessity: true,
        has_moved: false,
        team: team,
        worth: 1,
        ptr_getmoves: Rc::new(king_moves)
    }
}