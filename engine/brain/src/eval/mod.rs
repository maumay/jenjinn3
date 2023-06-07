use std::str::FromStr;

use crate::anyhow::{Error, Result};
use crate::eval::antipattern::KnightRimFacet;
use crate::eval::castling::CastlingFacet;
use crate::eval::development::DevelopmentFacet;
use crate::eval::material::{MaterialFacet, PieceValues};

use crate::eval::phase::Phase;
use crate::eval::tables::PieceSquareTablesFacet;
use crate::Square;
use myopic_board::{Board, Move, TerminalState};

mod antipattern;
mod castling;
mod development;
pub mod material;
mod phase;
mod see;
pub mod tables;

/// The evaluation upper/lower bound definition
pub const INFTY: i32 = 500_000i32;

/// The evaluation assigned to a won position.
pub const WIN_VALUE: i32 = INFTY - 1;

/// The evaluation assigned to a lost position.
pub const LOSS_VALUE: i32 = -WIN_VALUE;

/// The evaluation assigned to a drawn position.
pub const DRAW_VALUE: i32 = 0;

/// The different types of evaluation that can be generated by a facet.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Evaluation {
    /// Static evaluation independent of the game phase
    Single(i32),
    /// Evaluation that differs depending on the game phase
    Phased { mid: i32, end: i32 },
}

/// Represents some (possibly stateful) feature of a position which can be
/// evaluated.
pub trait EvalFacet {
    /// Return the static evaluation of the given position. Implementors are
    /// guaranteed that exactly the same move sequence will have been passed to
    /// this component and the given board position. I.e the internal states
    /// are aligned. It must follow the rule 'A LARGER +VE SCORE BETTER FOR
    /// WHITE, LARGER -VE SCORE BETTER FOR BLACK'.
    fn static_eval(&self, board: &Board) -> Evaluation;

    /// Update internal state by making the given move FROM the given position
    fn make(&mut self, mv: &Move, board: &Board);

    /// Update internal state by unmaking the given move which is guaranteed to
    /// have previously been passed to the "make" method.
    fn unmake(&mut self, mv: &Move);
}

/// Wrapper around a chess board which adds position evaluation capabilities.
/// The evaluation function is decomposed into orthogonal "facets". The minimal
/// evaluator looks only at material.
pub struct Evaluator {
    board: Board,
    phase: Phase,
    material: MaterialFacet,
    facets: Vec<Box<dyn EvalFacet>>,
}

impl Evaluator {
    /// Get an immutable reference to the underlying board
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Add another evaluation facet to this instance
    pub fn push_facet(&mut self, facet: Box<dyn EvalFacet>) {
        self.facets.push(facet);
    }

    /// Make the given move on the underlying board and update all the internal
    /// facets
    pub fn make(&mut self, action: Move) -> Result<()> {
        self.material.make(&action, &self.board);
        self.phase.make(&action);
        for cmp in self.facets.iter_mut() {
            cmp.make(&action, &self.board);
        }
        self.board.make(action)
    }

    /// Unmake the given move on the underlying board and update all the internal
    /// facets
    pub fn unmake(&mut self) -> Result<Move> {
        let action = self.board.unmake()?;
        self.material.unmake(&action);
        self.phase.unmake(&action);
        for cmp in self.facets.iter_mut() {
            cmp.unmake(&action);
        }
        Ok(action)
    }

    /// Parse and make the pgn encoded moves and returns the parsed moves
    pub fn play_pgn(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut state_clone = self.board.clone();
        let parsed_moves = state_clone.play_pgn(moves)?;
        for mv in parsed_moves.iter() {
            self.make(mv.clone())?;
        }
        Ok(parsed_moves)
    }

    /// Parse and make the uci encoded moves and returns the parsed moves
    pub fn play_uci(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut state_clone = self.board.clone();
        let parsed_moves = state_clone.play_uci(moves)?;
        for mv in parsed_moves.iter() {
            self.make(mv.clone())?;
        }
        Ok(parsed_moves)
    }

    /// The relative evaluation function assigns a score to this exact position
    /// at the point of time it is called. It does not take into account
    /// potential captures/recaptures etc. It must follow the rule that 'A
    /// LARGER +VE SCORE BETTER FOR ACTIVE, LARGER -VE SCORE BETTER FOR PASSIVE'.
    /// That is if it is white to move next then a high positive score indicates
    /// a favorable position for white and if it is black to move a high positive
    /// score indicates a favorable position for black. If the state it terminal
    /// it must return the LOSS_VALUE or DRAW_VALUE depending on the type of
    /// termination.
    pub fn relative_eval(&self) -> i32 {
        match self.board.terminal_state() {
            Some(TerminalState::Draw) => DRAW_VALUE,
            Some(TerminalState::Loss) => LOSS_VALUE,
            None => {
                let parity = self.board.active().parity();
                let material = self.phase.unwrap(self.material.static_eval(&self.board));
                let facets = self
                    .facets
                    .iter()
                    .map(|facet| self.phase.unwrap(facet.static_eval(&self.board)))
                    .sum::<i32>();
                parity * (material + facets)
            }
        }
    }

    /// API function for determining whether an exchange is good on this
    /// board. The board must have a piece at both the source and target square
    /// otherwise this function will panic. The pieces must be on opposing
    /// sides and the quality of the return value is in relation to the side of
    /// the attacker, higher is good for the attacker. Positive means a good
    /// exchange, negative mean a bad one. If the pieces are on the same side the
    /// result is undefined.
    pub fn see(&self, source: Square, target: Square) -> i32 {
        see::exchange_value(&self.board, source, target, self.piece_values())
    }

    // TODO For now we just use midgame values, should take into account phase
    pub fn piece_values(&self) -> &PieceValues {
        &self.material.mid_values()
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        crate::START_FEN.parse().unwrap()
    }
}

impl From<Board> for Evaluator {
    fn from(board: Board) -> Self {
        let mut facets: Vec<Box<dyn EvalFacet>> = Vec::new();
        facets.push(Box::new(PieceSquareTablesFacet::from(&board)));
        if board.to_fen().as_str() == crate::START_FEN {
            facets.push(Box::new(CastlingFacet::default()));
            facets.push(Box::new(DevelopmentFacet::default()));
            facets.push(Box::new(KnightRimFacet::default()));
        }

        Evaluator {
            material: MaterialFacet::from(&board),
            phase: Phase::from(&board),
            facets,
            board,
        }
    }
}

impl FromStr for Evaluator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Board>().map(|b| Evaluator::from(b))
    }
}

#[cfg(test)]
mod test {
    use myopic_board::Board;

    #[test]
    fn sanity() {
        assert_eq!(crate::START_FEN, crate::START_FEN.parse::<Board>().unwrap().to_fen())
    }
}
