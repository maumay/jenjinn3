use std::iter::repeat;

use itertools::izip;
use lazy_static::lazy_static;

use crate::bitboard::BitBoard;
use crate::magic::compute_powerset;
use crate::square::Square;
use crate::Dir;

use super::{compute_control, compute_rook_index, ROOK_MASKS};

pub fn control(loc: Square, occupied: BitBoard) -> BitBoard {
    MOVES[loc as usize][compute_rook_index(loc, occupied)]
}

/// Implementation and tests for the static magic move database.
type Moves = Vec<Vec<BitBoard>>;

lazy_static! {
    static ref MOVES: Moves = compute_move_database();
}

fn compute_move_database() -> Moves {
    let mut dest = Vec::with_capacity(64);
    let dirs = [Dir::N, Dir::E, Dir::S, Dir::W];
    for (sq, bb) in izip!(Square::iter(), ROOK_MASKS.iter().map(|&m| BitBoard(m))) {
        let dest_size = 1 << bb.size();
        let mut sq_dest: Vec<BitBoard> = repeat(BitBoard::ALL).take(dest_size).collect();
        for occ_var in compute_powerset(&bb.into_iter().collect()) {
            let index = compute_rook_index(sq, occ_var);
            if sq_dest[index] == BitBoard::ALL {
                sq_dest[index] = compute_control(sq, occ_var, &dirs);
            }
        }
        dest.push(sq_dest);
    }
    dest
}

#[cfg(test)]
mod test {
    use crate::square::Square::*;

    use super::{compute_move_database, compute_rook_index, Moves};

    #[test]
    fn test() {
        let moves = compute_move_database();
        test_case_one(&moves);
        //test_case_two(&moves);
    }

    fn test_case_one(moves: &Moves) {
        let (sq, occ) = (D3, D1 | D5 | D6 | G3 | C3 | A6 | H8);
        let expected = D2 | D1 | D4 | D5 | E3 | F3 | G3 | C3;
        assert_eq!(expected, moves[sq as usize][compute_rook_index(sq, occ)])
    }
}
