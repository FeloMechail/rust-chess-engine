mod board;
mod defs;

use crate::board::board::Chessboard;
use crate::board::lib::{Side, Piece};

fn print_bitboard(bitboard: u64) {
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (63 - (rank * 8) - file);
            let symbol = if (bitboard & mask) != 0 {'1'} else {'0'};
            print!("{} ", symbol);
        }
        println!();
    }
}

fn main() {
    let board = Chessboard::new(268_435_456);
    board.print_board();
    let white_rooks = board.get_pieces(Side::White, Piece::Rook);
    println!("White rooks:");
    print_bitboard(white_rooks);
}