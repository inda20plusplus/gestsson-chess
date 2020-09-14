use crate::chess_move::*;
use crate::*;

fn in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < 8 && y >= 0 && y < 8
}

fn place_regular(from: Point, (x, y): Point, collection: &mut MoveCollection) {
    let to = (x as usize, y as usize);
    collection.insert(to, Box::new(RegularMove::new(from, to)));
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

fn place_if_enemy(from: Point, x: i32, y: i32, collection: &mut MoveCollection, board: &Board) {
    if !in_bounds(x, y) {
        return;
    }

    let to = (x as usize, y as usize);
    if board.is_enemy(to) {
        place_regular(from, to, collection);
    }
}

fn place_if_possible(from: Point, x: i32, y: i32, collection: &mut MoveCollection, board: &Board) {
    place_if_empty(from, x, y, collection, board);
    place_if_enemy(from, x, y, collection, board);
}

fn place_beam(from: Point, dx: i32, dy: i32, collection: &mut MoveCollection, board: &Board) {
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
            place_if_enemy(from, x, y, collection, board);

            break;
        }

        place_regular(from, pos, collection);
    }
}

pub fn pawn_moves(piece: &Piece, pos: Point, board: &Board) -> MoveCollection {
    let stride = board.config.white_stride * piece.team as i32;
    let mut tiles = MoveCollection::new();

    let x = pos.0 as i32;
    let y = pos.1 as i32 + stride;

    place_if_empty(pos, x, y, &mut tiles, board);
    place_if_enemy(pos, x - 1, y, &mut tiles, board);
    place_if_enemy(pos, x + 1, y, &mut tiles, board);

    let y = y + stride;
    if !piece.has_moved {
        place_if_empty(pos, x, y, &mut tiles, board);
    }

    //rusta kod, mycket för pengarna
    if board.config.pawn_en_passant {
        let previous = board.history.front();
        if previous.is_some() {
            let last_tile = previous.unwrap().get_target_tile();
            let previous = board.tiles[last_tile.0][last_tile.1];

            if previous.is_some() {
                let previous = previous.unwrap();
                if previous.name == "Pawn" && previous.team != board.current_player {
                    place_regular(pos, last_tile, &mut tiles);
                }
            }
        }
    }

    tiles
}

pub fn king_moves(piece: &Piece, pos: Point, board: &Board) -> MoveCollection {
    let mut tiles = MoveCollection::new();
    let x = pos.0 as i32;
    let y = pos.1 as i32;

    for i in 0..3 {
        for j in 0..3 {
            place_if_possible(pos, 1 - i + x, 1 - j + y, &mut tiles, board);
        }
    }

    tiles
}

pub fn rook_moves(piece: &Piece, pos: Point, board: &Board) -> MoveCollection {
    let mut tiles = MoveCollection::new();

    place_beam(pos, 1, 0, &mut tiles, board);
    place_beam(pos, -1, 0, &mut tiles, board);

    place_beam(pos, 0, 1, &mut tiles, board);
    place_beam(pos, 0, -1, &mut tiles, board);

    tiles
}

pub fn bishop_moves(piece: &Piece, pos: Point, board: &Board) -> MoveCollection {
    let mut tiles = MoveCollection::new();

    place_beam(pos, 1, 1, &mut tiles, board);
    place_beam(pos, -1, -1, &mut tiles, board);
    place_beam(pos, -1, 1, &mut tiles, board);
    place_beam(pos, 1, -1, &mut tiles, board);

    tiles
}

pub fn queen_moves(piece: &Piece, pos: Point, board: &Board) -> MoveCollection {
    let mut tiles = MoveCollection::new();
    place_beam(pos, 1, 1, &mut tiles, board);
    place_beam(pos, -1, -1, &mut tiles, board);
    place_beam(pos, -1, 1, &mut tiles, board);
    place_beam(pos, 1, -1, &mut tiles, board);

    place_beam(pos, 1, 0, &mut tiles, board);
    place_beam(pos, -1, 0, &mut tiles, board);
    place_beam(pos, 0, 1, &mut tiles, board);
    place_beam(pos, 0, -1, &mut tiles, board);

    tiles
}

pub fn knight_moves(pieces: &Piece, pos: Point, board: &Board) -> MoveCollection {
    let mut tiles = MoveCollection::new();
    let x = pos.0 as i32;
    let y = pos.1 as i32;

    place_if_possible(pos, x + 2, y + 1, &mut tiles, board);
    place_if_possible(pos, x + 1, y + 2, &mut tiles, board);

    place_if_possible(pos, x - 2, y + 1, &mut tiles, board);
    place_if_possible(pos, x - 1, y + 2, &mut tiles, board);

    place_if_possible(pos, x + 2, y - 1, &mut tiles, board);
    place_if_possible(pos, x + 1, y - 2, &mut tiles, board);

    place_if_possible(pos, x - 2, y - 1, &mut tiles, board);
    place_if_possible(pos, x - 1, y - 2, &mut tiles, board);

    tiles
}
