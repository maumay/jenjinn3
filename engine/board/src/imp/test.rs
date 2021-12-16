use myopic_core::*;

use crate::enumset::EnumSet;
use crate::imp::Board;
use crate::imp::cache::CalculationCache;
use crate::imp::rights::Rights;
use crate::imp::history::History;
use crate::imp::positions::Positions;

#[derive(Debug, Clone)]
pub struct TestBoard {
    pub whites: Vec<BitBoard>,
    pub blacks: Vec<BitBoard>,
    pub castle_rights: EnumSet<CastleZone>,
    pub active: Side,
    pub clock: usize,
    pub enpassant: Option<Square>,
    pub history_count: usize,
}

impl From<TestBoard> for Board {
    fn from(test_board: TestBoard) -> Self {
        let pieces = Positions::new(
            vec![test_board.whites, test_board.blacks]
                .iter()
                .flat_map(|x| x.into_iter())
                .map(|&x| x)
                .collect::<Vec<BitBoard>>()
                .as_slice(),
        );
        Board {
            history: History::new(test_board.history_count),
            pieces,
            rights: Rights(test_board.castle_rights),
            active: test_board.active,
            enpassant: test_board.enpassant,
            clock: test_board.clock,
            cache: CalculationCache::default(),
        }
    }
}

impl Reflectable for TestBoard {
    fn reflect(&self) -> Self {
        TestBoard {
            whites: (&self.blacks).reflect(),
            blacks: (&self.whites).reflect(),
            castle_rights: self.castle_rights.reflect(),
            active: self.active.reflect(),
            clock: self.clock,
            enpassant: self.enpassant.reflect(),
            history_count: self.history_count,
        }
    }
}
