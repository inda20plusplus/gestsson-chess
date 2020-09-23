use chess_engine::Board; 

fn main() {
    let mut board = Board::new(None); 
    board.select((6, 6));

    let moves = board.get_movable();
    println!("Moves: {:?}", moves); 
    board.move_piece(*moves.first().unwrap());
    println!("Piece: {:?}", board.get_name((0, 0)).unwrap()); 
}