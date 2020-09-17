pub mod chess_move;
pub mod configuration;
pub mod pieces;

use chess_move::*;
use configuration::*;
use pieces::*;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem::replace;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn castling() {
        let mut board = Board::new(None);
        for (x, y) in board.enumerate_pieces(|piece, point| {
            !piece.necessity && piece.name != "Rook"
        }){
            board.tiles[x][y] = None
        }

        assert!(board.select((4,7)));
        assert!(board.move_piece((2,7)));

        assert!(board.select((4,0)));
        for i in board.get_movable(){
            println!("{} {}", i.0, i.1);
        }
        assert!(!board.move_piece((2,0)));
        assert!(board.move_piece((6,0)));

        assert!(board.undo_last());
        assert!(board.undo_last());

        assert!(board.select((4,7)));
        assert!(board.move_piece((6,7)));

        assert!(board.select((4,0)));
        assert!(!board.move_piece((6,0)));
        assert!(board.move_piece((2,0)));
    }

    #[test]
    fn pawn_move() {
        let mut board = Board::new(None);

        assert!(board.select((1, 6)));
        assert!(board.move_piece((1, 5)));

        assert!(board.current_player == Team::Black);

        assert!(board.select((0, 1)));
        assert!(board.move_piece((0, 3)));

        assert!(board.select((1, 5)));
        assert!(board.move_piece((1, 4)));

        assert!(board.select((0, 3)));
        let attackable = board.get_attackable();
        assert!(attackable.contains(&(1, 4)));

        assert!(board.move_piece((1, 4)));
        assert!(board.tiles[1][4].as_ref().unwrap().team == Team::Black);
        //Ladies and gentlemen, we got em

        assert!(board.select((3, 6)));
        assert!(board.move_piece((3,5)));

        assert!(board.select((1,4)));
        assert!(!board.move_piece((1,2)));
    }

    #[test]
    fn queen_move() {
        let mut board = Board::new(None);
        assert!(board.select((3, 6)));
        assert!(board.move_piece((3, 4)));

        assert!(board.select((3, 1)));
        assert!(board.move_piece((3, 3)));

        assert!(board.select((3, 7)));
        assert!(board.move_piece((3, 5)));

        assert!(board.select((4, 1)));
        assert!(board.move_piece((4, 3)));
        assert!(board.select((3, 5)));

        assert!(board.move_piece((5, 3)));
    }

    #[test]
    fn en_passant() {
        let mut board = Board::new(None);

        assert!(board.select((1, 6)));
        assert!(board.move_piece((1, 4)));

        assert!(board.select((1,1)));
        assert!(board.move_piece((1,2)));

        assert!(board.select((1, 4)));
        assert!(board.move_piece((1, 3)));

        assert!(board.select((2, 1)));
        assert!(board.move_piece((2, 3)));

        assert!(board.select((1,3)));

        assert!(board.move_piece((2,2)));
    }

    #[test]
    fn history() {
        let mut board = Board::new(None);

        for i in 0..100 {
            assert!(board.select((3, 6)));
            assert!(board.move_piece((3, 4)));

            assert!(board.undo_last());
        }

        assert!(board.select((3, 6)));
        assert!(board.move_piece((3, 4)));

        assert!(board.select((3, 1)));
        assert!(board.move_piece((3, 3)));

        assert!(board.undo_last());
        assert!(board.undo_last());

        assert!(board.select((3, 6)));
        assert!(board.move_piece((3, 4)));
    }
}

pub type Point = (usize, usize);
pub type ChessTile = Option<Piece>;
pub type BoolGrid = [[bool; 8]; 8];
pub type BoardCollection = Vec<Vec<ChessTile>>;
pub type MoveCollection = HashMap<Point, Box<dyn ChessMove>>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Team {
    White = 1,
    Black = -1,
}

pub struct Board {
    pub tiles: BoardCollection,
    pub possible_moves: MoveCollection,
    pub finished: bool,
    pub held_piece: Option<Point>,
    pub current_player: Team,
    pub winner: Option<Team>,
    pub config: BoardConfig,
    pub history: VecDeque<Box<dyn ChessMove>>,
}

static EMPTY_BOOLGRID : BoolGrid = [[false; 8]; 8];
impl Board {
    pub fn new(configuration: Option<BoardConfig>) -> Board {
        let configuration = configuration.unwrap_or(BoardConfig::default());
        let mut tiles = vec![vec![None; 8]; 8];

        (configuration.place_pawns)(&mut tiles);

        Self {
            tiles: tiles,
            possible_moves: MoveCollection::new(),
            finished: false,
            held_piece: None,
            winner: None,
            current_player: Team::White,
            config: configuration,
            history: VecDeque::new(),
        }
    }

    pub fn get_threatened(&self) -> BoolGrid {
        let mut grid = EMPTY_BOOLGRID;

        for point in self.get_enemies(){
            let piece = self.tiles[point.0][point.1].as_ref().unwrap();
            let moves = piece.get_moves(point, self, true);

            for p in moves.keys() {
                grid[p.0][p.1] = true;
            }
        }

        grid
    }

    fn is_empty(&self, (x, y): Point) -> bool {
        self.tiles[x][y].is_none()
    }

    fn is_friendly(&self, (x, y): Point) -> bool {
        let piece = self.tiles[x][y].as_ref();
        if piece.is_none() {
            return false;
        }

        piece.unwrap().team == self.current_player
    }

    fn is_enemy(&self, (x, y): Point) -> bool {
        let piece = self.tiles[x][y].as_ref();
        if piece.is_none() {
            return false;
        }

        piece.unwrap().team != self.current_player
    }

    fn enumerate_pieces<F: Fn(&Piece, Point) -> bool>(&self, closure: F) -> Vec<Point> {
        let mut ret = Vec::<Point>::new();
        for i in 0..8 {
            for j in 0..8 {
                let piece = self.tiles[i][j].as_ref();
                if piece.is_none() {
                    continue;
                }

                let point = (i, j);
                if closure(piece.unwrap(), point) {
                    ret.push(point);
                }
            }
        }

        ret
    }

    pub fn get_selectable(&self) -> Vec<Point> {
        self.enumerate_pieces(|piece, point| piece.team == self.current_player)
    }

    pub fn get_enemies(&self) -> Vec<Point> {
        self.enumerate_pieces(|piece, (x, y)| piece.team != self.current_player)
    }

    pub fn get_movable(&self) -> Vec<Point> {
        let mut ret = Vec::<Point>::new();
        for i in 0..8 {
            for j in 0..8 {
                if self.possible_moves.contains_key(&(i, j)) {
                    ret.push((i, j));
                }
            }
        }

        ret
    }

    pub fn get_attackable(&self) -> Vec<Point> {
        self.enumerate_pieces(|piece, point| {
            piece.team != self.current_player && self.possible_moves.contains_key(&point)
        })
    }

    pub fn select(&mut self, point: Point) -> bool {
        self.deselect();

        let tile = &self.tiles[point.0][point.1];
        if tile.is_none() {
            return false;
        }

        let piece = tile.as_ref().unwrap();
        if piece.team != self.current_player {
            return false;
        }

        self.held_piece = Option::from(point);
        self.possible_moves = piece.get_moves(point, &self, false);
        true
    }

    pub fn move_piece(&mut self, to: Point) -> bool {
        if self.held_piece.is_none() {
            return false;
        }

        let chessmove = self.possible_moves.remove(&to);
        if chessmove.is_none() {
            return false;
        }
        
        let mut chessmove = chessmove.unwrap();
        chessmove.perform(&mut self.tiles);

        self.history.push_front(chessmove);
        self.swap_team();

        true
    }

    pub fn undo_last(&mut self) -> bool {
        let chessmove = self.history.pop_front();
        if chessmove.is_none() {
            return false;
        }

        chessmove.unwrap().reverse(&mut self.tiles);
        self.swap_team();

        true
    }

    fn swap_team(&mut self) {
        if self.current_player == Team::White {
            self.current_player = Team::Black;
        } else {
            self.current_player = Team::White;
        }
    }

    pub fn deselect(&mut self) {
        self.held_piece = None;
    }
}
