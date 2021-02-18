pub mod pos;
pub mod piece;
pub mod board;

use pos::*;
use piece::*;
use board::*;

fn main() {
    let board = Board::starting_board();
    
    let p = board.pieces[Piece::from_start_pos(Pos::new(0, 7)).piece_index()];

    println!("Position of black rook is {:?}", p);
}
