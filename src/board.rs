use crate::pos::*;
use crate::piece::*;

pub struct Board {
    pub pieces: [Pos; 32],
    pub squares: [Option<Piece>; 64],
    pub turn: Color,
}

impl std::ops::Index<Pos> for Board {
    type Output = Option<Piece>;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.squares[pos.pos_index()]
    }
}

impl std::ops::IndexMut<Pos> for Board {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.squares[pos.pos_index()]
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
        for pos in (0..16).chain(48..64).map(Pos::from_pos_index) {
            pieces[i] = pos;
            squares[pos.pos_index()] = Some(Piece::from_start_pos(pos));
            i += 1;
        }

        Self { pieces, squares, turn: Color::White }
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

    fn sudo_legal_bishop(&self, start: Pos, end: Pos) -> bool {
        let inc_y = if end.row() > start.row() { 1 } else { -1 };
        let inc_x = if end.col() > start.col() { 1 } else { -1 };

        let mut start = start;
        loop {
            start = start.add_row(inc_x).add_col(inc_y); 

            if start == end {
                break true;
            } else if self[start] != None {
                break false;
            }
        }
    }

    fn sudo_legal_rook(&self, start: Pos, end: Pos) -> bool {
        let (on_x, dir) = if end.col() > start.col() { 
            (true, 1)
        } else if end.col() < start.col() {
            (true, -1)
        } else if end.row() > start.row() {
            (false, 1)
        } else {
            (false, -1)
        };

        let mut start = start;

        loop {
            if on_x {
                start = start.add_col(dir);
            } else {
                start = start.add_row(dir);
            }

            if start == end {
                break true;
            } else if self[start] != None {
                break false;
            }
        }
    }

    fn sudo_legal_queen(&self, start: Pos, end: Pos) -> bool {
        if start.col() == end.col() || start.row() == end.row() {
            self.sudo_legal_rook(start, end)
        } else {
            self.sudo_legal_bishop(start, end)
        }
    }

    fn sudo_legal_pawn(&self, start: Pos, end: Pos) -> bool {
        let dif = (start.row() as i32 - end.row() as i32).abs();
        if dif == 2 {
            let mid = match self[start].unwrap().color() {
                Color::White => start.add_row(1),
                Color::Black => start.add_row(-1),
            };
            self[mid] == None && self[end] == None
        } else if /* dif == 1 && */ start.col() == end.col() {
            self[end] == None
        } else /* if dif == 1 && start.col() != end.col() */ {
            self[end] != None //    Color checking is done at caller 
        }
    }

    pub fn sudo_legal(&self, start: Pos, end: Pos) -> bool {
        match self[start] {
            None => false,
            Some(piece) => {
                use PieceType::*;
                piece.color() == self.turn &&
                piece.can_move(start, end) &&
                match piece.typ() {
                    Bishop => self.sudo_legal_bishop(start, end),
                    Rook => self.sudo_legal_rook(start, end),
                    Queen => self.sudo_legal_queen(start, end),
                    Pawn => self.sudo_legal_pawn(start, end),
                    _ => true,
                }
            },
        }
    }

    pub fn switch_turn(&mut self) {
        self.turn = self.turn.opposite();
    }
}

