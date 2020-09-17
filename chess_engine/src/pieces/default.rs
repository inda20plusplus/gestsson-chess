use super::moves::*;
use super::Piece;
use crate::*;

use std::rc::Rc;

pub fn pawn(team: Team) -> Piece {
    Piece {
        name: "Pawn".to_owned(),
        necessity: false,
        team: team,
        has_moved: false,
        worth: 1,
        ptr_getmoves: Rc::new(pawn_moves),
    }
}

pub fn king(team: Team) -> Piece {
    Piece {
        name: "King".to_owned(),
        necessity: true,
        has_moved: false,
        team: team,
        worth: 0,
        ptr_getmoves: Rc::new(king_moves),
    }
}

pub fn bishop(team: Team) -> Piece {
    Piece {
        name: "Bishop".to_owned(),
        necessity: false,
        has_moved: false,
        team: team,
        worth: 3,
        ptr_getmoves: Rc::new(bishop_moves),
    }
}

pub fn rook(team: Team) -> Piece {
    Piece {
        name: "Rook".to_owned(),
        necessity: false,
        has_moved: false,
        team: team,
        worth: 5,
        ptr_getmoves: Rc::new(rook_moves),
    }
}

pub fn knight(team: Team) -> Piece {
    Piece {
        name: "Knight".to_owned(),
        necessity: false,
        has_moved: false,
        team: team,
        worth: 3,
        ptr_getmoves: Rc::new(knight_moves),
    }
}

pub fn queen(team: Team) -> Piece {
    Piece {
        name: "Queen".to_owned(),
        necessity: false,
        has_moved: false,
        team: team,
        worth: 9,
        ptr_getmoves: Rc::new(queen_moves),
    }
}
