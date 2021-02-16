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
