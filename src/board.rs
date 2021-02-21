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

    pub fn sudo_legal(&self, start: Pos, end: Pos) -> bool {
        match self[start] {
            None => false,
            Some(piece) => {
                piece.color() == self.turn &&
                piece.can_move(start, end)
            },
        }
    }
}

