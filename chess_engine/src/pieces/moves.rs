use super::MoveGenerator;
use crate::*;

fn in_bounds((x, y): Point) -> bool {
    x < 8 && y < 8
}

fn int_in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < 8 && y >= 0 && y < 8
}

fn place_if_empty(x : i32, y: i32, grid: &mut BoolGrid, board: &Board) {
    if !int_in_bounds(x, y) {
        return;
    }

    let x = x as usize;
    let y = y as usize;

    grid[x][y] = board.is_empty((x, y))
}

fn place_if_enemy(x : i32, y: i32, grid: &mut BoolGrid, board: &Board) {
    if !int_in_bounds(x, y) {
        return;
    }

    let x = x as usize;
    let y = y as usize;

    let pawn = board.tiles[x][y].as_ref();
    if pawn.is_none() {
        return;
    }

    let is_enemy = pawn.unwrap().team != board.current_player;
    grid[x][y] = is_enemy;
}

fn place_if_possible(x : i32, y: i32, mut grid: &mut BoolGrid, board: &Board) {
    place_if_empty(x, y, &mut grid, board);
    place_if_enemy(x, y, &mut grid, board);
}

pub fn place_beam(dx : i32, dy: i32, pos: Point, mut grid: &mut BoolGrid, board: &Board){
    let mut int_x = pos.0 as i32;
    let mut int_y = pos.1 as i32; 

    loop{
        int_x += dx;
        int_y += dy;
        if !int_in_bounds(int_x, int_y){
            break;
        }

        let x = int_x as usize;
        let y = int_y as usize;

        if !board.is_empty((x, y)){
            grid[x][y] = board.is_enemy((x,y));
            break;
        }

        grid[x][y] = true;
    }
}

pub fn pawn_moves(piece: &Piece, pos: Point, board: &Board) -> BoolGrid {
    let stride = board.config.white_stride * piece.team as i32;
    let mut tiles = EMPTY_BOOLGRID;

    let steps = if piece.has_moved {
        1
    } else {
        board.config.pawn_extra_steps
    };

    let x = pos.0 as i32;
    let mut y = pos.1 as i32;    

    for i in 0..steps {
        y += stride;

        if y < 0 || y >= 8 {
            break;
        }

        place_if_empty(x, y, &mut tiles, board);
        place_if_enemy(x - 1, y , &mut tiles, board);
        place_if_enemy(x + 1, y, &mut tiles, board);
    }

    tiles
}

pub fn king_moves(piece: &Piece, pos: Point, board: &Board) -> BoolGrid {
    let mut grid = EMPTY_BOOLGRID;
    let x = pos.0 as i32;
    let y = pos.1 as i32;

    for i in 0..3 {
        for j in 0..3 {
            place_if_possible(1 - i + x, 1 - j + y, &mut grid, board);
        }
    }

    grid
}

pub fn rook_moves(piece: &Piece, (x, y): Point, board: &Board) -> BoolGrid {
    let mut grid = EMPTY_BOOLGRID;

    place_beam(1, 0, (x, y), &mut grid, board);
    place_beam(-1, 0, (x, y), &mut grid, board);

    place_beam(0, 1, (x, y), &mut grid, board);
    place_beam(0, -1, (x, y), &mut grid, board);

    grid
}

pub fn bishop_moves(piece: &Piece, (x, y): Point, board: &Board) -> BoolGrid {
    let mut grid = EMPTY_BOOLGRID;
    place_beam(1, 1, (x, y), &mut grid, board);
    place_beam(-1, -1, (x, y), &mut grid, board);
    place_beam(-1, 1, (x, y), &mut grid, board);
    place_beam(1, -1, (x, y), &mut grid, board);

    
    place_beam(1, 0, (x, y), &mut grid, board);
    place_beam(-1, 0, (x, y), &mut grid, board);
    place_beam(0, 1, (x, y), &mut grid, board);
    place_beam(0, -1, (x, y), &mut grid, board);

    grid
}

pub fn queen_moves(piece: &Piece, (x, y): Point, board: &Board) -> BoolGrid {
    let mut grid = EMPTY_BOOLGRID;
    place_beam(1, 1, (x, y), &mut grid, board);
    place_beam(-1, -1, (x, y), &mut grid, board);

    place_beam(-1, 1, (x, y), &mut grid, board);
    place_beam(1, -1, (x, y), &mut grid, board);

    grid
}

pub fn knight_moves(pieces: &Piece, pos: Point, board: &Board) -> BoolGrid {
    let mut grid = EMPTY_BOOLGRID;
    let x = pos.0 as i32;
    let y = pos.1 as i32;

    place_if_possible(x + 2, y + 1, &mut grid, board);
    place_if_possible(x + 1, y + 2, &mut grid, board);

    place_if_possible(x - 2, y + 1, &mut grid, board);
    place_if_possible(x - 1, y + 2, &mut grid, board);

    place_if_possible(x + 2, y - 1, &mut grid, board);
    place_if_possible(x + 1, y - 2, &mut grid, board);

    place_if_possible(x - 2, y - 1, &mut grid, board);
    place_if_possible(x - 1, y - 2, &mut grid, board);


    grid
}