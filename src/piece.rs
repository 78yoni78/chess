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


