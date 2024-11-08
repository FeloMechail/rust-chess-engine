use std::sync::Arc;

use crate::defs::{Bitboard, NrOf, zobristKey};
use super::lib::{Piece, Side};
use super::zobrist::ZobristRandoms;

/// Represents a chess board using bitboards
#[derive(Debug)]
pub struct Chessboard {
    bitboard: Bitboard,
    bb_pieces: [[Bitboard; NrOf::PIECE_TYPES]; 2],
    bb_sides: [Bitboard; 2], 
    piece_list: [Piece; NrOf::SQUARES],
    zobrist_randoms: Arc<ZobristRandoms>
}

impl Default for Chessboard {
    fn default() -> Self {
        Self {
            bitboard: 0,
            bb_pieces: [[0; NrOf::PIECE_TYPES]; 2],
            bb_sides: [0; 2],
            piece_list: [Piece::None; NrOf::SQUARES],
            zobrist_randoms: Arc::new(ZobristRandoms::new())
        }
    }
}

impl Chessboard {
    const LAST_BIT: u64 = 63;

    pub fn new(bitboard: Bitboard) -> Self {
        Self {
            bitboard,
            bb_pieces: [[bitboard; NrOf::PIECE_TYPES]; 2],
            bb_sides: [bitboard; 2],
            piece_list: [Piece::None; NrOf::SQUARES],
            zobrist_randoms: Arc::new(ZobristRandoms::new())
        }
    }

    fn init_piece_list(&self) -> [Piece; NrOf::SQUARES] {
        let bb_w = self.bb_pieces[Side::White as usize];
        let bb_b = self.bb_pieces[Side::Black as usize];
        let mut piece_list : [Piece; NrOf::SQUARES] = [Piece::None; NrOf::SQUARES];
        
        for(piece_type, (w, b)) in bb_w.iter().zip(bb_b.iter()).enumerate() {
            let mut white_pieces = *w;
            let mut black_pieces = *b;
            while white_pieces > 0 {
                let square = white_pieces.trailing_zeros() as usize;
                piece_list[square] = Piece::ALL[piece_type];
                white_pieces &= white_pieces - 1;
            }
            while black_pieces > 0 {
                let square = black_pieces.trailing_zeros() as usize;
                piece_list[square] = Piece::ALL[piece_type];
                black_pieces &= black_pieces - 1;
            }
        }
        piece_list
    }

    pub fn init_zobrist_key(&self) -> zobristKey {
        let mut key: zobristKey = 0;

        let bb_w = self.bb_pieces[Side::White as usize];
        let bb_b = self.bb_pieces[Side::Black as usize];

        for (piece_type, (w, b)) in bb_w.iter().zip(bb_b.iter()).enumerate() {
            let mut white_pieces = *w;
            let mut black_pieces = *b;
            while white_pieces > 0 {
                let square = white_pieces.trailing_zeros() as usize;
                key ^= self.zobrist_randoms.piece(Side::White, Piece::ALL[piece_type], square);
                white_pieces &= white_pieces - 1;
            }
            while black_pieces > 0 {
                let square = black_pieces.trailing_zeros() as usize;
                key ^= self.zobrist_randoms.piece(Side::Black, Piece::ALL[piece_type], square);
                black_pieces &= black_pieces - 1;
            }
        }

        todo!("Game state, castling rights, en passant square, side to move");
    }

    pub fn print_board(&self) {
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (Self::LAST_BIT - (rank * 8) - file);
                let symbol = if (self.bitboard & mask) != 0 {'1'} else {'0'};
                print!("{} ", symbol);
            }
            println!();
        }
    }

    pub fn get_pieces(&self, side: Side, piece: Piece) -> Bitboard {
        self.bb_pieces[side as usize][piece as usize]
    }

    pub fn occupancy(&self) -> Bitboard {
        self.bb_sides[Side::White as usize] | self.bb_sides[Side::Black as usize]
    }
    
}