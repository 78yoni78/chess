#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
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
struct Pos(u8);

impl Pos {
    pub const EMPTY: Self = Pos(u8::MAX);

    pub const fn new(col: u8, row: u8) -> Pos {
        Pos(row * 8 + col)
    }

    pub fn from_notation(s: &str) -> Self {
        assert_eq!(s.len(), 2);

        let chars: Vec<char> = s.chars().collect();

        assert!('a' as u32 <= chars[0] as u32 && chars[0] as u32 <= 'h' as u32);
        assert!('1' as u32 <= chars[1] as u32 && chars[1] as u32 <= '8' as u32);

        Self::new((chars[0] as u32 - 'a' as u32) as u8, (chars[1] as u32 - '1' as u32) as u8)
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
struct Piece(u8);

impl Piece {
    const BLACK_START: u8 = 16;

    pub fn from_start_pos(pos: Pos) -> Self {
        const POS_MAX: u8 = 63;

        assert!(!(3 <= pos.0 && pos.0 <= 5));
        
        if pos.row() < 3 {
            Self(pos.0)
        } else {
            Self(Self::BLACK_START + (POS_MAX - pos.0))
        }
    }

    pub const fn piece_index(self) -> usize { self.0 as usize }

    pub const fn typ(self) -> PieceType {
        use PieceType::*;

        match self.0 % 16 {
            0 | 7 => Rook,
            1 | 6 => Knight,
            2 | 5 => Bishop,
            3 => Queen,
            4 => King,
            _ => Pawn,
        }
    }

    pub const fn color(self) -> Color {
        use Color::*;
        
        if self.0 >= Self::BLACK_START { Black } else { White } 
    }

    pub fn can_move(self, start: Pos, end: Pos) -> bool {
        use PieceType::*;
        use Color::*;

        match self.typ() {
            Pawn => {
                let dir = match self.color() { White => 1, Black => -1, };
                end == start.add_row(dir) ||
                end == start.add_row(2 * dir) ||
                end == start.add_row(dir).add_col(1) ||
                end == start.add_row(dir).add_col(-1)
            }
            Rook => {
                start.row() == end.row() || start.col() == end.col()
            }
            Knight => {
                let dif1 = (start.row() as i32 - end.row() as i32).abs();
                let dif2 = (start.col() as i32 - end.col() as i32).abs();
                (dif1 == 1 && dif2 == 2) || (dif1 == 2 && dif2 == 1)
            }
            Bishop => {
                let dif1 = (start.row() as i32 - end.row() as i32).abs();
                let dif2 = (start.col() as i32 - end.col() as i32).abs();
                dif1 == dif2
            }
            Queen => start.row() == end.row() || start.col() == end.col() || {
                let dif1 = (start.row() as i32 - end.row() as i32).abs();
                let dif2 = (start.col() as i32 - end.col() as i32).abs();
                dif1 == dif2
            },
            King => {
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

impl std::ops::Index<Pos> for Board {
    type Output = Option<Piece>;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.squares[pos.0 as usize]
    }
}

impl std::ops::IndexMut<Pos> for Board {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.squares[pos.0 as usize]
    }
}

impl std::ops::Index<Piece> for Board {
    type Output = Pos;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self.pieces[piece.piece_index()]
    }
}

impl std::ops::IndexMut<Piece> for Board {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self.pieces[piece.piece_index()]
    }
}

impl Board {
    pub fn starting_board() -> Self {
        let mut pieces = [Pos::EMPTY; 32];
        let mut squares = [None; 64];

        let mut i = 0;
        for pos in (0..15).chain(48..63).map(Pos) {
            pieces[i] = pos;
            squares[pos.0 as usize] = Some(Piece(i as u8));
            i += 1;
        }

        Self { pieces, squares }
    }

    pub fn remove_piece(&mut self, pos: Pos) {
        if let Some(piece) = self[pos] {
            self[pos] = None;
            self[piece] = Pos::EMPTY;
        }
    }

    pub fn move_piece(&mut self, start: Pos, end: Pos) {
        if let Some(piece) = self[start] {
            self[piece] = end;
            self[start] = None;
            self[end] = Some(piece);
        }
    }

    pub fn sudo_legal(&self, start: Pos, end: Pos) -> bool {
        match self[start] {
            None => false,
            Some(piece) => {
                piece.can_move(start, end)
            },
        }
    }
}

fn main() {
    let board = Board::starting_board();
    
    let p = board.pieces[Piece::from_start_pos(Pos::new(0, 7)).piece_index()];

    println!("Position of black rook is {:?}", p);
}
