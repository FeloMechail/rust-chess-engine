pub type Bitboard = u64;
pub type zobristKey = u64;
pub const EMPTY: u64 = 0;


#[derive(Debug)]
pub struct NrOf;
impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const SQUARES: usize = 64;
    pub const CASTLING_PERMISSIONS: usize = 16;
}
