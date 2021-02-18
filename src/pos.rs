#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos(u8);

impl Pos {
    pub const EMPTY: Self = Pos(u8::MAX);

    pub const fn new(col: u8, row: u8) -> Pos {
        Pos(row * 8 + col)
    }

    pub fn from_notation(s: &str) -> Option<Self> {
        if s.len() != 2 { return None };

        let chars: Vec<char> = s.chars().collect();

        if !('a' as u32 <= chars[0] as u32 && chars[0] as u32 <= 'h' as u32)
            || !('1' as u32 <= chars[1] as u32 && chars[1] as u32 <= '8' as u32) {
            return None;
        }

        Some(Self::new((chars[0] as u32 - 'a' as u32) as u8, (chars[1] as u32 - '1' as u32) as u8))
    }

    pub fn from_pos_index(i: usize) -> Self {
        assert!(i < 64);
        Self(i as u8)
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

    pub const fn pos_index(self) -> usize { self.0 as usize }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Pos::new(3, 5);
        assert_eq!(p.col(), 3);
        assert_eq!(p.row(), 5);
    }

    #[test]
    fn test_from_notation() {
        let p = Pos::from_notation("b5");
        assert_eq!(p, Pos::new(1, 4));
    }
}
