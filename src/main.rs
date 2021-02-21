pub mod pos;
pub mod piece;
pub mod board;

use std::io::{self, Write};
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

fn get_pos() -> Option<Pos> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    Pos::from_notation(&line.trim())
}

fn main() {
    let mut board = Board::starting_board();
  
    loop {
        print_board(&board);
        print!("Enter position of piece: ");
        io::stdout().flush().unwrap();
        let start = get_pos().expect("Expeted a position");
        print!("Enter position to move to: ");
        io::stdout().flush().unwrap();
        let end = get_pos().expect("Expected a position");
        if board.sudo_legal(start, end) {
            board.move_piece(start, end);
            board.switch_turn();
            println!("Success!");
        } else {
            println!("Illegal move");
        }
    }
}
