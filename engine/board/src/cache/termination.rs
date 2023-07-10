use std::collections::HashMap;
use itertools::Itertools;
use myopic_core::*;

use crate::TerminalState;
use crate::{Board, Move, Moves};

const HALF_MOVE_CLOCK_LIMIT: usize = 100;

impl Board {
    pub(crate) fn terminal_state_impl(&self) -> Option<TerminalState> {
        let terminal_status = self.cache.borrow().termination_status;
        match terminal_status {
            Some(x) => x,
            None => {
                let result = self.compute_terminal_state();
                self.cache.borrow_mut().termination_status = Some(result);
                result
            }
        }
    }

    fn compute_terminal_state(&self) -> Option<TerminalState> {
        let active = self.active;
        let active_king = self.king(active);
        if active_king.is_none() {
            return Some(TerminalState::Loss)
        }
        let active_king = active_king.unwrap();
        let passive_control = self.passive_control();
        let (whites, blacks) = self.sides();
        let king_moves = Piece(active, Class::K).moves(active_king, whites, blacks);
        // If king can move somewhere which is usually the case then not terminal.
        if (king_moves - passive_control).is_populated() {
            None
        } else if passive_control.contains(active_king) {
            self.checked_termination()
        } else {
            self.unchecked_termination()
        }
        .or(self.check_clock_limit())
        .or(self.check_repetitions())
    }

    fn check_clock_limit(&self) -> Option<TerminalState> {
        if self.half_move_clock() >= HALF_MOVE_CLOCK_LIMIT {
            Some(TerminalState::Draw)
        } else {
            None
        }
    }

    fn check_repetitions(&self) -> Option<TerminalState> {
        let mut dest: HashMap<u64, usize> = HashMap::new();
        dest.insert(self.hash(), 1);

        let positions = self
            .history
            .historical_positions()
            // Exclude positions where null move was played
            .filter(|(_, m)| !matches!(m, Move::Null))
            // Only care about positions since the last unrepeatable move
            .rev()
            .take_while(|(_, m)| Board::is_repeatable(*m))
            .map(|(h, _)| h);

        for p in positions {
            if 3 == *dest.entry(p).and_modify(|v| *v += 1).or_insert(1) {
                return Some(TerminalState::Draw)
            }
        }
        None
    }

    fn is_repeatable(m: &Move) -> bool {
        match m {
            Move::Null => true,
            Move::Enpassant { .. } | Move::Promotion { .. } | Move::Castle { .. } => false,
            Move::Standard {
                moving: Piece(_, class),
                capture, ..
            } => *class != Class::P && capture.is_none()
        }
    }

    // Assumes king is in check and cannot move out of it
    fn checked_termination(&self) -> Option<TerminalState> {
        if self.moves(Moves::All).len() > 0 {
            None
        } else {
            Some(TerminalState::Loss)
        }
    }

    // Assumes king cannot move but not in check
    fn unchecked_termination(&self) -> Option<TerminalState> {
        let king = self.king(self.active).unwrap();
        let pin_rays = Piece(Side::W, Class::Q).empty_control(king);
        let (whites, blacks) = self.sides();
        let moves = |p: Piece, loc: Square| p.moves(loc, whites, blacks);
        // These pieces have no constraints since not in check and not on pin rays
        for class in [Class::Q, Class::R, Class::B, Class::N, Class::P] {
            let piece = Piece(self.active, class);
            let locations = self.locs(&[piece]) - pin_rays;
            if locations.iter().any(|loc| moves(piece, loc).is_populated()) {
                return None;
            }
        }
        if self.moves(Moves::All).len() > 0 {
            None
        } else {
            Some(TerminalState::Draw)
        }
    }
}
