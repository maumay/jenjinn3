use myopic_core::*;

use crate::imp::Board;

impl Board {
    pub fn passive_control(&self) -> BitBoard {
        let mut cache = self.cache.borrow_mut();
        match cache.passive_control {
            Some(x) => x,
            None => {
                let result = self.compute_control(self.active.reflect());
                cache.passive_control = Some(result);
                result
            }
        }
    }

    /// Computes the total area of control on the board for a given side. Note
    /// that the passive king is treated as invisible so that if it is in
    /// check it cannot create it's own escape squares by blocking the
    /// control ray of an attacking slider. TODO Improve efficiency by
    /// treated all pawns as a block
    fn compute_control(&self, side: Side) -> BitBoard {
        let pieces = &self.pieces;
        let (whites, blacks) = match side {
            Side::W => (pieces.whites(), pieces.blacks() - pieces.king_loc(Side::B)),
            Side::B => (pieces.whites() - pieces.king_loc(Side::W), pieces.blacks()),
        };
        let locs = |piece: Piece| pieces.locs(piece);
        let control = |piece: Piece, square: Square| piece.control(square, whites | blacks);
        Class::all()
            .into_iter()
            .map(|class| Piece(side, class))
            .flat_map(|p| locs(p).into_iter().map(move |sq| control(p, sq)))
            .collect()
    }
}

#[cfg(test)]
mod test {

    use crate::imp::rights::Rights;
    use crate::imp::test::TestBoard;
    use crate::imp::Board;
    use crate::{BitBoard, Side, Square::*};

    struct TestCase {
        board: TestBoard,
        side: Side,
        expected_control: BitBoard,
    }

    fn execute_test(case: TestCase) {
        assert_eq!(case.expected_control, Board::from(case.board).compute_control(case.side));
    }

    fn get_test_board() -> TestBoard {
        TestBoard {
            whites: vec![A2 | B3 | C2 | D2 | E4 | F2 | G2 | H2, !!F3, B2 | F1, !!A1, !!D1, !!E1],
            blacks: vec![A7 | B7 | C7 | D7 | E5 | F7 | G7 | H5, C6 | G8, !!C8, A8 | H8, !!F6, !!E8],
            castle_rights: Rights::all(),
            enpassant: None,
            active: Side::W,
            clock: 20,
            history_count: 20,
        }
    }

    #[test]
    fn test_white_control() {
        let expected_control: BitBoard = vec![
            A1, A2, A3, A4, A6, B1, B3, B5, C1, C2, C3, C4, D1, D2, D3, D4, D5, E1, E2, E3, E5, F1,
            F2, F3, F5, G1, G2, G3, G5, H2, H3, H4,
        ]
        .into_iter()
        .collect();

        execute_test(TestCase { board: get_test_board(), side: Side::W, expected_control })
    }

    #[test]
    fn test_black_control() {
        let expected_control: BitBoard = vec![
            A7, A6, A5, B8, B7, B6, B4, C8, C6, D8, D7, D6, D4, E7, E6, E5, F8, F7, F6, F5, F4, F3,
            G8, G7, G6, G5, G4, H7, H6, H5, H4,
        ]
        .into_iter()
        .collect();

        execute_test(TestCase { board: get_test_board(), side: Side::B, expected_control })
    }
}
