#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos(u8);

impl Pos {
    pub const EMPTY: Self = Pos(u8::MAX);

    pub const fn new(col: u8, row: u8) -> Pos {
        Pos(row * 8 + col)
    }

    pub const fn row(self) -> u8 {
        self.0 / 8
    }
    
    pub const fn col(self) -> u8 {
        self.0 % 8
    }

    pub const fn add_col(self, x: i16) -> Pos {
        let y = self.0 as i16;
        Pos((y + x) as u8)
    }

    pub const fn add_row(self, x: i16) -> Pos {
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

impl PieceType {
    pub fn ray_piece(self) -> bool {
        use PieceType::*;

        match self {
            Rook | Bishop | Queen => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Piece {
    typ: PieceType,
    color: Color,
}

impl Piece {
    pub const fn piece_index(self) -> usize {
        use PieceType::*;

        let mut ret = match self.typ {
            Pawn => 0,
            Rook => 8,
            Knight => 10,
            Bishop => 12,
            Queen => 14,
            King => 15,
        };
        if let Color::Black = self.color { ret += 16; }
        ret
    }

    pub fn can_move(self, start: Pos, end: Pos) -> bool {
        use PieceType::*;
        use Color::*;

        match self {
            Piece { typ: Pawn, color } => {
                let dir = match color { White => 1, Black => -1, };
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

impl Board {
    pub const START: Board = {
        use PieceType::*;
        const fn P(typ: PieceType) -> Option<Piece> { Some (Piece {typ, color: Color::White}) }
        const fn p(typ: PieceType) -> Option<Piece> { Some (Piece {typ, color: Color::Black}) }

        Board {
            pieces: [
                //  White pawns
                Pos::new(0, 1), Pos::new(1, 1), Pos::new(2, 1), Pos::new(3, 1), Pos::new(4, 1), Pos::new(5, 1), Pos::new(6, 1), Pos::new(7, 1),
                //  White rooks
                Pos::new(0, 0), Pos::new(7, 0),
                //  White knights
                Pos::new(1, 0), Pos::new(6, 0),
                //  White bishops
                Pos::new(2, 0), Pos::new(5, 0),
                //  White queen and king
                Pos::new(3, 0), Pos::new(4, 0),
                //  White pawns
                Pos::new(0, 7), Pos::new(1, 7), Pos::new(2, 7), Pos::new(3, 7), Pos::new(4, 7), Pos::new(5, 7), Pos::new(6, 7), Pos::new(7, 7),
                //  White rooks
                Pos::new(0, 8), Pos::new(7, 8),
                //  White knights
                Pos::new(1, 8), Pos::new(6, 8),
                //  White bishops
                Pos::new(2, 8), Pos::new(5, 8),
                //  White queen and king
                Pos::new(3, 8), Pos::new(4, 8),
            ],
            squares: [
                P(Rook), P(Knight), P(Bishop), P(Queen), P(King), P(Bishop), P(Knight), P(Rook),
                P(Pawn), P(Pawn), P(Pawn), P(Pawn), P(Pawn), P(Pawn), P(Pawn), P(Pawn), 
                None, None, None, None, None, None, None, None, 
                None, None, None, None, None, None, None, None, 
                None, None, None, None, None, None, None, None, 
                None, None, None, None, None, None, None, None, 
                p(Pawn), p(Pawn), p(Pawn), p(Pawn), p(Pawn), p(Pawn), p(Pawn), p(Pawn), 
                p(Rook), p(Knight), p(Bishop), p(Queen), p(King), p(Bishop), p(Knight), p(Rook),
            ],
        }
    };
}

fn main() {
    let board = Board::START;
    
    let p = board.pieces[Piece {typ: PieceType::Rook, color: Color::Black}.piece_index()];

    println!("Position of blakc rook is {:?}", p);
}
