pub mod types {
    pub type Bitboard = u64;

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

    
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Squares {
        A1, B1, C1, D1, E1, F1, G1, H1,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A8, B8, C8, D8, E8, F8, G8, H8,
    }

    impl Squares {
        pub const ALL: [Squares; 64] = [
            Squares::A1, Squares::B1, Squares::C1, Squares::D1, Squares::E1, Squares::F1, Squares::G1, Squares::H1,
            Squares::A2, Squares::B2, Squares::C2, Squares::D2, Squares::E2, Squares::F2, Squares::G2, Squares::H2,
            Squares::A3, Squares::B3, Squares::C3, Squares::D3, Squares::E3, Squares::F3, Squares::G3, Squares::H3,
            Squares::A4, Squares::B4, Squares::C4, Squares::D4, Squares::E4, Squares::F4, Squares::G4, Squares::H4,
            Squares::A5, Squares::B5, Squares::C5, Squares::D5, Squares::E5, Squares::F5, Squares::G5, Squares::H5,
            Squares::A6, Squares::B6, Squares::C6, Squares::D6, Squares::E6, Squares::F6, Squares::G6, Squares::H6,
            Squares::A7, Squares::B7, Squares::C7, Squares::D7, Squares::E7, Squares::F7, Squares::G7, Squares::H7,
            Squares::A8, Squares::B8, Squares::C8, Squares::D8, Squares::E8, Squares::F8, Squares::G8, Squares::H8,
        ];
    }

    #[derive(Debug)]
    pub struct NrOf;
    impl NrOf {
        pub const PIECE_TYPES: usize = 6;
        pub const SQUARES: usize = 64;
    }
}