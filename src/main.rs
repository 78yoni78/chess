#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos(u8);

impl Pos {
    pub const EMPTY: Self = Pos(u8::MAX);

    pub fn new(row: u8, col: u8) -> Pos {
        Pos(row * 8 + col)
    }

    pub fn row(self) -> u8 {
        self.0 / 8
    }
    
    pub fn col(self) -> u8 {
        self.0 % 8
    }

    pub fn add_col(self, x: i16) -> Pos {
        let y = self.0 as i16;
        Pos((y + x) as u8)
    }

    pub fn add_row(self, x: i16) -> Pos {
        let y = self.0 as i16;
        Pos((y + x * 8) as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Piece {
    typ: PieceType,
    color: Color,
}

impl Piece {
    pub fn can_move(self, start: Pos, end: Pos) -> bool {
        use PieceType::*;
        use Color::*;

        match self {
            Piece { typ: Pawn, color } => {
                let dir = if color == White { 1 } else { -1 };
                end == start.add_row(dir) ||
                end == start.add_row(2 * dir) ||
                end == start.add_row(dir).add_col(1) ||
                end == start.add_row(dir).add_col(-1)
            }
            Piece { typ: Rook, .. } => {
                start.row() == end.row() || start.col() == end.col()
            }
            Piece { typ: Knight, .. } => {
                let dif1 = (start.row() as i32 - end.row() as i32).abs();
                let dif2 = (start.col() as i32 - end.col() as i32).abs();
                (dif1 == 1 && dif2 == 2) || (dif1 == 2 && dif2 == 1)
            }
            Piece { typ: Bishop, .. } => {
                let dif1 = (start.row() as i32 - end.row() as i32).abs();
                let dif2 = (start.col() as i32 - end.col() as i32).abs();
                dif1 == dif2
            }
            Piece { typ: Queen, .. } => start.row() == end.row() || start.col() == end.col() || {
                let dif1 = (start.row() as i32 - end.row() as i32).abs();
                let dif2 = (start.col() as i32 - end.col() as i32).abs();
                dif1 == dif2
            },
            Piece { typ: King, .. } => {
                let dif1 = (start.row() as i32 - end.row() as i32).abs();
                let dif2 = (start.col() as i32 - end.col() as i32).abs();
                (dif1 == 0 || dif1 == 1) && (dif2 == 0 || dif2 == 1)
            }
        }
    }
}

struct Board {
    pieces: [Pos; 32],
    squares: [Option<Piece>; 64],
} 

fn main() {
    let board = Board {
        pieces: [Pos::EMPTY; 32],
        squares: [None; 64],
    };
    println!("Hello, world!");
}
