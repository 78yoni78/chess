use crate::pos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece(u8);


impl PieceType {
    pub fn ray_piece(self) -> bool {
        use PieceType::*;

        match self {
            Rook | Bishop | Queen => true,
            _ => false,
        }
    }
}

impl Color {
    pub fn opposite(self) -> Self {
        use Color::*;
        match self { White => Black, Black => White }
    }
}

impl Piece {
    const BLACK_START: u8 = 16;

    pub fn from_start_pos(pos: Pos) -> Self {
        const POS_MAX: u8 = 63;

        assert!(!(3 <= pos.row() && pos.row() <= 5));
     
        let i = pos.pos_index()as u8;
        if pos.row() < 3 {
            Self(i)
        } else {
            Self(Self::BLACK_START + (POS_MAX - i))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_starting_pos() {
        let piece = Piece::from_start_pos(Pos::new(5, 0));
        assert_eq!(piece.color(), Color::White);
        assert_eq!(piece.typ(), PieceType::Bishop);
        
        let piece = Piece::from_start_pos(Pos::new(5, 7));
        assert_eq!(piece.color(), Color::Black);
        assert_eq!(piece.typ(), PieceType::Bishop);
    }
}

