pub mod chess_move;
pub mod configuration;
pub mod pieces;

use chess_move::*;
use configuration::*;
use pieces::*;

use std::collections::HashMap;
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check() {
        let mut board = Board::new(None);
        for (x, y) in
            board.enumerate_pieces(|piece, point| !piece.necessity && piece.name != "Rook")
        {
            board.tiles[x][y] = None
        }

        assert!(board.select((7, 7)));
        assert!(board.move_piece((7, 3)));

        assert!(!board.check);
        assert!(board.select((0, 0)));
        assert!(board.move_piece((0, 1)));
        assert!(!board.check);

        assert!(board.select((0, 7)));
        assert!(board.move_piece((0, 1)));
        assert!(!board.check);

        assert!(board.select((4, 0)));
        assert!(board.move_piece((4, 1)));

        assert!(board.finished);
        assert!(board.undo_last());

        assert!(board.select((4, 0)));
        assert!(board.move_piece((5, 0)));

        assert!(board.select((7, 3)));
        assert!(board.move_piece((7, 0)));

        assert!(board.check);
        assert!(board.finished);

        assert!(board.undo_last());
        assert!(board.select((0, 1)));
        assert!(board.move_piece((0, 0)));

        assert!(board.check);
        assert!(!board.finished);
    }

    #[test]
    fn castling() {
        let mut board = Board::new(None);
        for (x, y) in
            board.enumerate_pieces(|piece, point| !piece.necessity && piece.name != "Rook")
        {
            board.tiles[x][y] = None
        }

        assert!(board.select((4, 7)));
        assert!(board.move_piece((2, 7)));

        assert!(board.select((4, 0)));
        assert!(!board.move_piece((2, 0)));
        assert!(board.move_piece((6, 0)));

        assert!(board.undo_last());
        assert!(board.undo_last());

        assert!(board.select((4, 7)));
        assert!(board.move_piece((6, 7)));

        assert!(board.select((4, 0)));
        assert!(!board.move_piece((6, 0)));
        assert!(board.move_piece((2, 0)));
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
        assert!(board.move_piece((3, 5)));

        assert!(board.select((1, 4)));
        assert!(!board.move_piece((1, 2)));
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

        assert!(board.select((1, 1)));
        assert!(board.move_piece((1, 2)));

        assert!(board.select((1, 4)));
        assert!(board.move_piece((1, 3)));

        assert!(board.select((2, 1)));
        assert!(board.move_piece((2, 3)));

        assert!(board.select((1, 3)));

        assert!(board.move_piece((2, 2)));
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
    pub current_enemy: Team,
    pub winner: Option<Team>,
    pub config: BoardConfig,
    pub history: VecDeque<Box<dyn ChessMove>>,
    pub check: bool,
}

static EMPTY_BOOLGRID: BoolGrid = [[false; 8]; 8];
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
            current_enemy: Team::Black,
            config: configuration,
            history: VecDeque::new(),
            check: false,
        }
    }

    pub fn get_threatened(&self, team: Team) -> BoolGrid {
        let mut grid = EMPTY_BOOLGRID;
        let pieces = self.enumerate_pieces(|piece, _pos| piece.team != team);

        for point in pieces {
            let piece = self.tiles[point.0][point.1].as_ref().unwrap();
            let moves = piece.get_moves(point, self, true);

            for p in moves.keys() {
                grid[p.0][p.1] = true;
            }
        }

        grid
    }

    pub fn is_empty(&self, (x, y): Point) -> bool {
        self.tiles[x][y].is_none()
    }

    pub fn is_friendly(&self, (x, y): Point) -> bool {
        let piece = self.tiles[x][y].as_ref();
        if piece.is_none() {
            return false;
        }

        piece.unwrap().team == self.current_player
    }

    pub fn is_enemy(&self, (x, y): Point) -> bool {
        let piece = self.tiles[x][y].as_ref();
        if piece.is_none() {
            return false;
        }

        piece.unwrap().team != self.current_player
    }

    pub fn is_team(&self, (x, y): Point, team: Team) -> bool {
        let tile = self.tiles[x][y].as_ref();
        if tile.is_none() {
            return false;
        }

        tile.unwrap().team == team
    }

    pub fn can_promote(&self) -> bool {
        self.enumerate_pieces(|piece, pos| piece.name == "Pawn" && (pos.1 == 0 || pos.1 == 7))
            .len()
            > 0
    }

    pub fn promote(&mut self, into: Piece) {
        let piece =
            self.enumerate_pieces(|piece, pos| piece.name == "Pawn" && (pos.1 == 0 || pos.1 == 7));
        if piece.len() == 0 {
            return;
        }

        let (x, y) = piece[0];
        self.tiles[x][y] = Some(into);

        self.update_win_status();
    }

    fn is_opposite(&self, (x, y): Point, team: Team) -> bool {
        let piece = self.tiles[x][y].as_ref();
        if piece.is_none() {
            return false;
        }

        piece.unwrap().team != team
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

    pub fn get_name(&self, (x, y): Point) -> Option<String> {
        let piece = self.tiles[x][y].as_ref();
        if piece.is_none() {
            return None;
        }

        Some(piece.unwrap().name.clone())
    }

    pub fn get_selectable(&self) -> Vec<Point> {
        self.enumerate_pieces(|piece, _point| piece.team == self.current_player)
    }

    pub fn get_kings(&self) -> Vec<Point> {
        self.enumerate_pieces(|piece, _point| piece.necessity)
    }

    pub fn get_enemies(&self) -> Vec<Point> {
        self.enumerate_pieces(|piece, (_x, _y)| piece.team != self.current_player)
    }

    pub fn get_movable(&self) -> Vec<Point> {
        self.possible_moves.keys().map(|key| *key).collect()
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

        if !self.perform_move(to) {
            return false;
        }

        self.update_win_status();

        true
    }

    fn update_win_status(&mut self) {
        self.check = self.check_check(self.current_player);
        if self.check && self.check_checkmated() {
            self.finished = true;
            self.winner = Some(self.current_enemy);
        }

        if self.check_check(self.current_enemy) {
            self.finished = true;
            self.winner = Some(self.current_player);
        }
    }

    fn perform_move(&mut self, to: Point) -> bool {
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

    fn check_check(&mut self, team: Team) -> bool {
        let threat = self.get_threatened(team);

        self.get_kings().iter().any(|(x, y)| threat[*x][*y])
    }

    fn check_checkmated(&mut self) -> bool {
        let pieces = self.get_selectable();

        for piece in pieces {
            self.select(piece);

            for mv in self.get_movable() {
                self.perform_move(mv);

                let check = self.check_check(self.current_enemy);
                self.undo_last();

                if !check {
                    return false;
                }

                self.select(piece);
            }
        }

        true
    }

    pub fn undo_last(&mut self) -> bool {
        let chessmove = self.history.pop_front();
        if chessmove.is_none() {
            return false;
        }

        chessmove.unwrap().reverse(&mut self.tiles);
        self.swap_team();
        self.deselect();

        self.finished = false;
        self.winner = None;

        true
    }

    fn swap_team(&mut self) {
        std::mem::swap(&mut self.current_player, &mut self.current_enemy);
    }

    pub fn deselect(&mut self) {
        self.held_piece = None;
    }
}
