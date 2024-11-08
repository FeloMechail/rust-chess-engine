use crate::defs::{NrOf, EMPTY, zobristKey};
use super::lib::{Side, Piece};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;


const SEED: u64 = 0x0bad_c0de;

#[derive(Debug)]
pub struct ZobristRandoms {
    rnd_pieces: [[[u64; NrOf::SQUARES]; NrOf::PIECE_TYPES]; 2],
    rnd_castling: [u64; NrOf::CASTLING_PERMISSIONS],
    rnd_sides: [u64; Side::Both as usize],
    rnd_en_passant: [u64; NrOf::SQUARES + 1]
}


impl ZobristRandoms {
    pub fn new() -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);
        let mut zobrist_randoms = Self {
            rnd_pieces: [[[EMPTY; NrOf::SQUARES]; NrOf::PIECE_TYPES]; 2],
            rnd_castling: [EMPTY; NrOf::CASTLING_PERMISSIONS],
            rnd_sides: [EMPTY; 2],
            rnd_en_passant: [EMPTY; NrOf::SQUARES + 1],
        };

        zobrist_randoms.rnd_pieces.iter_mut().for_each(|side| {
            side.iter_mut().for_each(|piece| {
                piece.iter_mut().for_each(|square|{
                    *square = rng.gen::<u64>();
                });
            })
        });

        zobrist_randoms.rnd_castling.iter_mut().for_each(|castling| {
            *castling = rng.gen::<u64>();
        });

        zobrist_randoms.rnd_sides.iter_mut().for_each(|side| {
            *side = rng.gen::<u64>();
        });

        zobrist_randoms.rnd_en_passant.iter_mut().for_each(|square| {
            *square = rng.gen::<u64>();
        });

        zobrist_randoms
    }

    pub fn piece(&self, side: Side, piece: Piece, square: usize) -> u64 {
        self.rnd_pieces[side as usize][piece as usize][square]
    }

    pub fn castling(&self, castling: u8) -> zobristKey {
        self.rnd_castling[castling as usize]
    }

    pub fn side(&self, side: Side) -> zobristKey {
        self.rnd_sides[side as usize]
    }

    pub fn en_passant(&self, en_passant: Option<u8>) -> zobristKey {
        match en_passant {
            Some(square) => self.rnd_en_passant[square as usize],
            None => self.rnd_en_passant[NrOf::SQUARES],
        }
    }
}