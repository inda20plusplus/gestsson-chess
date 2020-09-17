pub mod default;
pub mod moves;

use super::*;
use std::rc::Rc;
type MoveGenerator = dyn Fn(&Piece, Point, &Board, bool) -> MoveCollection;

#[derive(Clone)]
pub struct Piece {
    pub team: Team,
    pub name: String,
    pub worth: i32,
    pub necessity: bool,
    pub has_moved: bool,
    ptr_getmoves: Rc<MoveGenerator>,
}

impl Piece {
    pub fn get_moves(&self, point: Point, board: &Board, only_lethal : bool) -> MoveCollection {
        (self.ptr_getmoves)(&self, point, &board, only_lethal)
    }
}
