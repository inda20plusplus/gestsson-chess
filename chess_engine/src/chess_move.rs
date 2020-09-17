use crate::*;
use std::mem::replace;

pub trait ChessMove {
    fn get_affected_tiles(&self) -> Vec<Point>;
    fn get_target_tile(&self) -> Point;

    fn can_kill(&self) -> bool {
        true
    }

    fn perform(&mut self, tiles: &mut BoardCollection);
    fn reverse(&mut self, tiles: &mut BoardCollection);

    fn as_regular(&self) -> Option<&RegularMove> {
        None
    }
}

#[derive(Clone)]
pub struct RegularMove {
    pub from: Point,
    pub to: Point,
    pub prev_state: Option<Piece>,
    killed: Option<Piece>,
}

impl RegularMove {
    pub fn new(from: Point, to: Point) -> Self {
        Self {
            from: from,
            to: to,
            prev_state: None,
            killed: None,
        }
    }
}

impl ChessMove for RegularMove {
    fn as_regular(&self) -> Option<&Self> {
        Some(self)
    }

    fn get_affected_tiles(&self) -> Vec<Point> {
        vec![self.from, self.to]
    }

    fn get_target_tile(&self) -> Point {
        self.to
    }

    fn perform(&mut self, mut tiles: &mut BoardCollection) {
        self.prev_state = tiles[self.from.0][self.from.1].clone();

        let mut next = self.prev_state.clone();
        if self.prev_state.is_some() {
            let mut piece = next.unwrap();
            piece.has_moved = true;

            next = Some(piece);
        }

        self.killed = replace(&mut tiles[self.to.0][self.to.1], next);

        tiles[self.from.0][self.from.1] = None;
    }

    fn reverse(&mut self, mut tiles: &mut BoardCollection) {
        tiles[self.from.0][self.from.1] = self.prev_state.clone();

        tiles[self.to.0][self.to.1] = self.killed.clone();
    }
}

pub struct EnPassant {
    from: Point,
    to: Point,
    killpos: Point,
    killed: Option<Piece>,
}

impl EnPassant {
    pub fn new(from: Point, to: Point, kill: Point) -> Self {
        Self {
            from: from,
            to: to,
            killpos: kill,
            killed: None,
        }
    }
}

impl ChessMove for EnPassant {
    fn get_affected_tiles(&self) -> Vec<Point> {
        vec![self.from, self.to, self.killpos]
    }

    fn get_target_tile(&self) -> Point {
        self.to
    }

    fn perform(&mut self, tiles: &mut BoardCollection) {
        let piece = tiles[self.from.0][self.from.1].clone();
        self.killed = replace(&mut tiles[self.killpos.0][self.killpos.1], None);

        tiles[self.to.0][self.to.1] = piece;
        tiles[self.from.0][self.from.1] = None;
    }

    fn reverse(&mut self, tiles: &mut BoardCollection) {
        let piece = replace(&mut tiles[self.to.0][self.to.1], None);
        tiles[self.from.0][self.from.1] = piece;

        tiles[self.killpos.0][self.killpos.1] = self.killed.clone();
    }
}

pub struct Castling {
    pub rook_from: Point,
    pub rook_to: Point,
    pub king_from: Point,
    pub king_to: Point,
}

impl Castling {
    pub fn new(rook: Point, king: Point) -> Self {
        let rook_x = if rook.0 == 0 { 3 } else { 5 };
        let king_x = if rook.0 == 0 { 2 } else { 6 };

        Self {
            rook_from: rook,
            king_from: king,
            rook_to: (rook_x, rook.1),
            king_to: (king_x, king.1),
        }
    }
}

impl ChessMove for Castling {
    fn get_affected_tiles(&self) -> Vec<Point> {
        vec![self.rook_from, self.rook_to, self.king_from, self.king_to]
    }

    fn can_kill(&self) -> bool {
        false
    }

    fn get_target_tile(&self) -> Point {
        self.king_to
    }

    fn perform(&mut self, tiles: &mut BoardCollection) {
        let king = tiles[self.king_from.0][self.king_from.1].clone();
        tiles[self.king_to.0][self.king_to.1] = king;

        tiles[self.king_from.0][self.king_from.1] = None;

        let rook = tiles[self.rook_from.0][self.rook_from.1].clone();
        tiles[self.rook_to.0][self.rook_to.1] = rook;

        tiles[self.rook_from.0][self.rook_from.1] = None;
    }

    fn reverse(&mut self, tiles: &mut BoardCollection) {
        let king = tiles[self.king_to.0][self.king_to.1].clone();
        tiles[self.king_from.0][self.king_from.1] = king;

        tiles[self.king_to.0][self.king_to.1] = None;

        let rook = tiles[self.rook_to.0][self.rook_to.1].clone();
        tiles[self.rook_from.0][self.rook_from.1] = rook;

        tiles[self.rook_to.0][self.rook_to.1] = None;
    }
}
