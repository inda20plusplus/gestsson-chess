use crate::*;
use super::MoveGenerator;

fn in_bounds((x, y) : Point) -> bool {
    x < 8 && y < 8
}

fn int_in_bounds(x : i32, y : i32) -> bool {
    x >= 0 && x < 8 && y >= 0 && y < 8
}

fn place_if_empty((x, y) : Point, grid : &mut BoolGrid, board : &Board){
    if !in_bounds((x, y)) {
        return;
    }

    grid[x][y] = board.is_empty((x, y))
}

fn place_if_enemy((x,y) : Point, grid : &mut BoolGrid, board : &Board){
    if !in_bounds((x, y)) {
        return;
    }

    let pawn = board.tiles[x][y].as_ref();
    if pawn.is_none(){
        return;
    }

    let is_enemy = pawn.unwrap().team != board.current_player;
    grid[x][y] = is_enemy;
}

fn place_if_possible(pos : Point, mut grid : &mut BoolGrid, board : &Board){
    place_if_empty(pos, &mut grid, board);
    place_if_enemy(pos, &mut grid, board);
}

pub fn pawn_moves(piece : &Piece, pos : Point, board : &Board) -> BoolGrid {
    let stride = board.config.white_stride * piece.team as i32;
    let mut tiles = EMPTY_BOOLGRID;
    
    let steps = if  piece.has_moved { 1 } else { board.config.pawn_extra_steps };
    let mut int_y = pos.1 as i32;

    for i in 0..steps{
        int_y += stride;
        
        if int_y < 0 || int_y >= 8 {
            break;
        }

        let (x, y) = (pos.0, int_y as usize);
        place_if_empty((x, y), &mut tiles, board);

        if x > 1 {
            let x = pos.0 - 1;
            place_if_enemy((x, y),&mut tiles, board);
        }

        if x < 7 {
            let x = pos.0 + 1;
            place_if_enemy((x, y), &mut tiles, board);
        }
    }

    tiles
}

pub fn king_moves(piece : &Piece, (x, y) : Point, board : &Board) -> BoolGrid {
    let mut grid = EMPTY_BOOLGRID;
    for i in 0..3 {
        for j in 0..3 {
            let int_x = 1 - i + x as i32;
            let int_y = 1 - j + y as i32;
            
            if int_in_bounds(int_x, int_y) {
                place_if_possible((int_x as usize, int_y as usize), &mut grid, board);
            }
        }
    }

    grid
}