use myopic_core::*;

use crate::imp::cache::rays::RaySet;
use crate::{Board, ChessBoard};

impl Board {
    pub fn compute_discoveries(&self) -> RaySet {
        let active = self.side(self.active);
        let passive = self.side(self.active.reflect());
        let king_loc =
            self.pieces.locs(Piece(self.active.reflect(), Class::K)).into_iter().next().unwrap();

        self.compute_xrayers(king_loc)
            .iter()
            .map(|xrayer| (xrayer, BitBoard::cord(king_loc, xrayer)))
            .filter(|&(_, cord)| (cord & active).size() == 2 && (cord & passive).size() == 1)
            .map(|(xrayer, cord)| {
                let discov_loc = ((cord & active) - xrayer).first().unwrap();
                (discov_loc, cord)
            })
            .collect()
    }

    fn compute_xrayers(&self, king_loc: Square) -> BitBoard {
        [Class::B, Class::R, Class::Q]
            .iter()
            .map(|&class| Piece(self.active, class))
            .flat_map(|p| self.locs(&[p]) & p.empty_control(king_loc))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use myopic_core::Square::*;

    use super::*;

    fn execute_test(fen: &'static str, expected_discoveries: RaySet) {
        let board = fen.parse::<Board>().unwrap();
        assert_eq!(expected_discoveries.reflect(), board.reflect().compute_discoveries());
        assert_eq!(expected_discoveries, board.compute_discoveries());
    }

    #[test]
    fn case_one() {
        let fen = "6r1/5p1k/4pP2/4N3/3PN3/6P1/2B3PK/7R w - - 1 10";
        let expected_pinned =
            vec![(E4, C2 | D3 | E4 | F5 | G6 | H7), (H2, H1 | H2 | H3 | H4 | H5 | H6 | H7)]
                .into_iter()
                .collect();

        execute_test(fen, expected_pinned);
    }
}
