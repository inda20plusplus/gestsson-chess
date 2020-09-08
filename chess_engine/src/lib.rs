mod pieces;

use pieces::*;
use std::mem::swap;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_eq!((2, 4), (2, 4))
    }
}

pub type Point = (usize, usize);
pub type ChessTile = Option<Piece>;
pub type BoolGrid = [[bool; 8]; 8];

#[derive(Clone, Copy)]
pub enum Team{
    While,
    Black
}

pub struct Board {
    pub tiles: Vec<Vec<ChessTile>>,
    pub possible_moves : BoolGrid,
    held_piece: Option<Point>
}

const empty_boolgrid : BoolGrid = [[false; 8]; 8];
impl Board {
    pub fn new() -> Board{
        Self {
            tiles: vec![vec![None; 8]; 8],
            possible_moves: empty_boolgrid,
            held_piece: None
        }
    }

    fn select(&mut self, point : Point) -> bool {
        self.deselect();

        let tile = &self.tiles[point.0][point.1];
        if tile.is_none() {
            return false;
        }

        self.held_piece = Option::from(point);
        self.possible_moves = tile.as_ref().unwrap().get_moves(point, &self);
        true
    }

    fn move_piece(&mut self, (to_x, to_y) : Point){
        if self.held_piece.is_none() {
            return;
        }

        if !self.possible_moves[to_x][to_y] {
            return;
        }

        let (from_x, from_y) = self.held_piece.unwrap();
        
        let temp = self.tiles[from_x][from_y].to_owned();
        self.tiles[to_x][to_y] = temp;

        self.tiles[from_x][from_y] = None;
    }

    fn deselect(&mut self){
        self.held_piece = None;
    }
}