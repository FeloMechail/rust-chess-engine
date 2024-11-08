
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    White,
    Black,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    None,
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::King, Piece::Queen, Piece::Rook,
        Piece::Bishop, Piece::Knight, Piece::Pawn,
    ];
}
