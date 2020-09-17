use crate::chess_move::*;
use crate::*;

use std::cmp::max;
use std::cmp::min;

fn in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < 8 && y >= 0 && y < 8
}

fn place_regular(from: Point, (x, y): Point, collection: &mut MoveCollection) {
    let to = (x as usize, y as usize);
    collection.insert(to, Box::new(RegularMove::new(from, to)));
}

fn place_enpassant(
    from: Point,
    mut x: i32,
    y: i32,
    stride: i32,
    collection: &mut MoveCollection,
    board: &Board,
) {
    if !in_bounds(x, y + stride) {
        return;
    }

    let target = (x as usize, (y + stride) as usize);
    let killpos = (x as usize, y as usize);

    if !board.is_empty(target) || !board.is_enemy(killpos) || board.history.len() == 0 {
        return;
    }

    let previous = board.history.front().unwrap().as_regular();
    if previous.is_none() {
        return;
    }

    let previous = previous.unwrap();
    if previous.to != killpos {
        return;
    }

    let piece = board.tiles[previous.to.0][previous.to.1].as_ref().unwrap();
    let distance = max(previous.from.1, previous.to.1) - min(previous.from.1, previous.to.1);

    if piece.name == "Pawn" && distance == 2 {
        collection.insert(target, Box::new(EnPassant::new(from, target, killpos)));
    }
}

fn place_if_empty(from: Point, x: i32, y: i32, collection: &mut MoveCollection, board: &Board) {
    if !in_bounds(x, y) {
        return;
    }

    let to = (x as usize, y as usize);
    if board.is_empty(to) {
        place_regular(from, to, collection);
    }
}

fn place_if_enemy(
    from: Point,
    team: Team,
    x: i32,
    y: i32,
    collection: &mut MoveCollection,
    board: &Board,
) {
    if !in_bounds(x, y) {
        return;
    }

    let to = (x as usize, y as usize);
    if board.is_opposite(to, team) {
        place_regular(from, to, collection);
    }
}

fn place_if_possible(
    from: Point,
    team: Team,
    x: i32,
    y: i32,
    collection: &mut MoveCollection,
    board: &Board,
) {
    place_if_empty(from, x, y, collection, board);
    place_if_enemy(from, team, x, y, collection, board);
}

fn place_beam(
    from: Point,
    team: Team,
    dx: i32,
    dy: i32,
    collection: &mut MoveCollection,
    board: &Board,
) {
    let mut x = from.0 as i32;
    let mut y = from.1 as i32;

    loop {
        x += dx;
        y += dy;
        if !in_bounds(x, y) {
            break;
        }

        let pos = (x as usize, y as usize);
        if !board.is_empty(pos) {
            place_if_enemy(from, team, x, y, collection, board);

            break;
        }

        place_regular(from, pos, collection);
    }
}

pub fn pawn_moves(piece: &Piece, pos: Point, board: &Board, only_lethal: bool) -> MoveCollection {
    let stride = board.config.white_stride * piece.team as i32;
    let mut tiles = MoveCollection::new();

    let x = pos.0 as i32;
    let y = pos.1 as i32;

    if board.config.pawn_en_passant {
        place_enpassant(pos, x - 1, y, stride, &mut tiles, board);
        place_enpassant(pos, x + 1, y, stride, &mut tiles, board);
    }

    let y = y + stride;

    place_if_empty(pos, x, y, &mut tiles, board);
    place_if_enemy(pos, piece.team, x - 1, y, &mut tiles, board);
    place_if_enemy(pos, piece.team, x + 1, y, &mut tiles, board);

    let y = y + stride;
    if !piece.has_moved {
        place_if_empty(pos, x, y, &mut tiles, board);
    }

    tiles
}

pub fn king_moves(piece: &Piece, pos: Point, board: &Board, only_lethal: bool) -> MoveCollection {
    let mut tiles = MoveCollection::new();
    let x = pos.0 as i32;
    let y = pos.1 as i32;

    for i in 0..3 {
        for j in 0..3 {
            place_if_possible(pos, piece.team, 1 - i + x, 1 - j + y, &mut tiles, board);
        }
    }

    //castling
    if !only_lethal && pos.0 == 4 && (pos.1 == 0 || pos.1 == 7) {
        let threatened = board.get_threatened(piece.team);

        for rook in board.enumerate_pieces(|piece, point| {
            piece.name == "Rook" && piece.team == board.current_player
        }) {
            if rook.1 != pos.1 || (rook.0 != 0 && rook.0 != 7) {
                continue;
            }

            let mut x = pos.0;
            let mut can_castle = true;
            while x != rook.0 {
                if x != pos.0 && (!board.is_empty((x, rook.1)) || threatened[x][rook.1]) {
                    can_castle = false;
                    break;
                }

                if x > rook.0 {
                    x -= 1;
                } else {
                    x += 1;
                }
            }

            if can_castle {
                let mv = Box::new(Castling::new(rook, pos));
                tiles.insert(mv.king_to, mv);
            }
        }
    }

    tiles
}

pub fn rook_moves(piece: &Piece, pos: Point, board: &Board, only_lethal: bool) -> MoveCollection {
    let mut tiles = MoveCollection::new();

    place_beam(pos, piece.team, 1, 0, &mut tiles, board);
    place_beam(pos, piece.team, -1, 0, &mut tiles, board);

    place_beam(pos, piece.team, 0, 1, &mut tiles, board);
    place_beam(pos, piece.team, 0, -1, &mut tiles, board);

    tiles
}

pub fn bishop_moves(piece: &Piece, pos: Point, board: &Board, only_lethal: bool) -> MoveCollection {
    let mut tiles = MoveCollection::new();

    place_beam(pos, piece.team, 1, 1, &mut tiles, board);
    place_beam(pos, piece.team, -1, -1, &mut tiles, board);
    place_beam(pos, piece.team, -1, 1, &mut tiles, board);
    place_beam(pos, piece.team, 1, -1, &mut tiles, board);

    tiles
}

pub fn queen_moves(piece: &Piece, pos: Point, board: &Board, only_lethal: bool) -> MoveCollection {
    let mut tiles = MoveCollection::new();
    place_beam(pos, piece.team, 1, 1, &mut tiles, board);
    place_beam(pos, piece.team, -1, -1, &mut tiles, board);
    place_beam(pos, piece.team, -1, 1, &mut tiles, board);
    place_beam(pos, piece.team, 1, -1, &mut tiles, board);

    place_beam(pos, piece.team, 1, 0, &mut tiles, board);
    place_beam(pos, piece.team, -1, 0, &mut tiles, board);
    place_beam(pos, piece.team, 0, 1, &mut tiles, board);
    place_beam(pos, piece.team, 0, -1, &mut tiles, board);

    tiles
}

pub fn knight_moves(piece: &Piece, pos: Point, board: &Board, only_lethal: bool) -> MoveCollection {
    let mut tiles = MoveCollection::new();
    let x = pos.0 as i32;
    let y = pos.1 as i32;

    place_if_possible(pos, piece.team, x + 2, y + 1, &mut tiles, board);
    place_if_possible(pos, piece.team, x + 1, y + 2, &mut tiles, board);

    place_if_possible(pos, piece.team, x - 2, y + 1, &mut tiles, board);
    place_if_possible(pos, piece.team, x - 1, y + 2, &mut tiles, board);

    place_if_possible(pos, piece.team, x + 2, y - 1, &mut tiles, board);
    place_if_possible(pos, piece.team, x + 1, y - 2, &mut tiles, board);

    place_if_possible(pos, piece.team, x - 2, y - 1, &mut tiles, board);
    place_if_possible(pos, piece.team, x - 1, y - 2, &mut tiles, board);

    tiles
}
