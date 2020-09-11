mod configuration;
mod pieces;

use configuration::*;
use pieces::*;

use std::mem::replace;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_eq!((2, 4), (2, 4))
    }

    #[test]
    fn pawn_move() {
        let mut board = Board::new(None);

        assert!(board.select((1, 6)));
        assert!(board.possible_moves[1][5]);
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
    }
}

pub type Point = (usize, usize);
pub type ChessTile = Option<Piece>;
pub type BoolGrid = [[bool; 8]; 8];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Team {
    White = 1,
    Black = -1,
}

pub struct Board {
    pub tiles: Vec<Vec<ChessTile>>,
    pub possible_moves: BoolGrid,
    pub finished: bool,
    pub held_piece: Option<Point>,
    pub current_player: Team,
    pub winner: Option<Team>,
    pub config: BoardConfig,
}

const EMPTY_BOOLGRID: BoolGrid = [[false; 8]; 8];
impl Board {
    pub fn new(configuration: Option<BoardConfig>) -> Board {
        let configuration = configuration.unwrap_or(BoardConfig::default());
        let mut tiles = vec![vec![None; 8]; 8];

        (configuration.place_pawns)(&mut tiles);

        Self {
            tiles: tiles,
            possible_moves: EMPTY_BOOLGRID,
            finished: false,
            held_piece: None,
            winner: None,
            current_player: Team::White,
            config: configuration,
        }
    }

    fn is_empty(&self, (x, y): Point) -> bool {
        self.tiles[x][y].is_none()
    }

    fn is_friendly(&self, (x,y): Point) -> bool {
        let piece = self.tiles[x][y].as_ref();
        if piece.is_none() {
            return false;
        }

        piece.unwrap().team == self.current_player
    }

    fn is_enemy(&self, (x,y) : Point) -> bool {
        let piece = self.tiles[x][y].as_ref();
        if piece.is_none() {
            return false;
        }

        piece.unwrap().team != self.current_player
    }

    fn enumerate_tiles<F: Fn(&Piece, Point) -> bool>(&self, closure: F) -> Vec<Point> {
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
        self.enumerate_tiles(|piece, point| piece.team == self.current_player)
    }

    pub fn get_enemies(&self) -> Vec<Point> {
        self.enumerate_tiles(|piece, (x, y)| {
            piece.team != self.current_player
        })
    }

    pub fn get_attackable(&self) -> Vec<Point> {
        self.enumerate_tiles(|piece, (x, y)| {
            piece.team != self.current_player && self.possible_moves[x][y]
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
        self.possible_moves = piece.get_moves(point, &self);
        true
    }

    pub fn move_piece(&mut self, (to_x, to_y): Point) -> bool {
        if self.held_piece.is_none() {
            return false;
        }

        if !self.possible_moves[to_x][to_y] {
            return false;
        }

        let (from_x, from_y) = self.held_piece.unwrap();

        let temp = self.tiles[from_x][from_y].to_owned();
        let killed = replace(&mut self.tiles[to_x][to_y], temp);

        self.kill_piece(&killed);
        self.tiles[from_x][from_y] = None;
        self.swap_team();

        true
    }

    fn kill_piece(&mut self, piece: &Option<Piece>) {
        if piece.is_none() {
            return;
        }

        if piece.as_ref().unwrap().necessity {
            self.finished = true;
            self.winner = Option::from(self.current_player);
        }
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
