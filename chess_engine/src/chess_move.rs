use crate::*;
use std::mem::replace;

pub trait ChessMove {
    fn get_affected_tiles(&self) -> Vec<Point>;
    fn get_target_tile(&self) -> Point;

    fn perform(&mut self, tiles: &mut BoardCollection);
    fn reverse(&mut self, tiles: &mut BoardCollection);
}

pub struct RegularMove {
    from: Point,
    to: Point,
    killed: Option<Piece>,
}

impl RegularMove {
    pub fn new(from: Point, to: Point) -> Self {
        Self {
            from: from,
            to: to,
            killed: None,
        }
    }
}

impl ChessMove for RegularMove {
    fn get_affected_tiles(&self) -> Vec<Point> {
        vec![self.from, self.to]
    }

    fn get_target_tile(&self) -> Point {
        vec![self.to]
    }

    fn perform(&mut self, mut tiles: &mut BoardCollection) {
        let piece = tiles[self.from.0][self.from.1].clone();
        self.killed = replace(&mut tiles[self.to.0][self.to.1], piece);

        tiles[self.from.0][self.from.1] = None;
    }

    fn reverse(&mut self, mut tiles: &mut BoardCollection) {
        let piece = tiles[self.to.0][self.to.1].clone();
        replace(&mut tiles[self.from.0][self.from.1], piece);

        tiles[self.to.0][self.to.1] = self.killed.clone();
    }
}
