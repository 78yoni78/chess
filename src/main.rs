pub mod pos;
pub mod piece;
pub mod board;

use pos::*;
use piece::*;
use board::*;

fn piece_letter(piece: Piece) -> char {
    use PieceType::*;
    use Color::*;

    let ch = match piece.typ() {
        Pawn => 'p',
        Rook => 'r',
        Knight => 'n',
        Bishop => 'b',
        Queen => 'q',
        King => 'k',
    };

    match piece.color() {
        White => ch.to_ascii_uppercase(),
        Black => ch,
    }
}

fn print_board(board: &Board) {
    const SIZE: u8 = 8;
    const FLIP_BOARD: bool = true;

    for y in 0..SIZE {
        let y = if FLIP_BOARD { SIZE - y -1 } else { y };

        print!("{}  ", y + 1);

        for x in 0..SIZE {
            let pos = Pos::new(x, y);
            let ch = match board[pos] { Some(piece) => piece_letter(piece), None => ' ' };
            print!("{} ", ch);
        }
        println!();
    }
    println!();

    print!("   ");
    for i in 0..SIZE {
        print!("{} ", ('a' as u8 + i) as char);
    }
    println!();
}

fn main() {
    let board = Board::starting_board();
   
    print_board(&board);
}
