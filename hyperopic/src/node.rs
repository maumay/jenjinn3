use crate::constants::side;
use crate::position::{Position, TerminalState};

use crate::material::{MaterialFacet, PieceValues};
use crate::moves::Move;
use crate::phase::Phase;
use crate::tables::PieceSquareTablesFacet;
use crate::{see, Square};
use anyhow::Result;
use crate::eval::CastlingFacet;

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
    fn static_eval(&self, board: &Position) -> Evaluation;

    /// Update internal state by making the given move FROM the given position
    fn make(&mut self, mv: &Move, board: &Position);

    /// Update internal state by unmaking the given move which is guaranteed to
    /// have previously been passed to the "make" method.
    fn unmake(&mut self, mv: &Move);
}

/// Wrapper around a chess board which adds position evaluation capabilities.
/// The evaluation function is decomposed into orthogonal "facets". The minimal
/// evaluator looks only at material.
pub struct SearchNode {
    position: Position,
    phase: Phase,
    material: MaterialFacet,
    facets: Vec<Box<dyn EvalFacet>>,
}

impl SearchNode {
    /// Get an immutable reference to the underlying position
    pub fn position(&self) -> &Position {
        &self.position
    }

    /// Add another evaluation facet to this instance
    pub fn push_facet(&mut self, facet: Box<dyn EvalFacet>) {
        self.facets.push(facet);
    }

    /// Make the given move on the underlying board and update all the internal facets
    pub fn make(&mut self, action: Move) -> Result<()> {
        self.material.make(&action, &self.position);
        self.phase.make(&action);
        for cmp in self.facets.iter_mut() {
            cmp.make(&action, &self.position);
        }
        self.position.make(action)
    }

    /// Unmake the given move on the underlying board and update all the internal facets
    pub fn unmake(&mut self) -> Result<Move> {
        let action = self.position.unmake()?;
        self.material.unmake(&action);
        self.phase.unmake(&action);
        for cmp in self.facets.iter_mut() {
            cmp.unmake(&action);
        }
        Ok(action)
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
        match self.position.compute_terminal_state() {
            Some(TerminalState::Draw) => DRAW_VALUE,
            Some(TerminalState::Loss) => LOSS_VALUE,
            None => {
                let parity = if self.position.active == side::W { 1 } else { -1 };
                let material = self.phase.unwrap(self.material.static_eval(&self.position));
                let facets = self
                    .facets
                    .iter()
                    .map(|facet| self.phase.unwrap(facet.static_eval(&self.position)))
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
        see::exchange_value(&self.position, source, target, self.piece_values())
    }

    // TODO For now we just use midgame values, should take into account phase
    pub fn piece_values(&self) -> &PieceValues {
        &self.material.mid_values()
    }

    pub fn phase_progression(&self) -> f32 {
        self.phase.phase_progression()
    }
}

impl From<Position> for SearchNode {
    fn from(board: Position) -> Self {
        let mut board_clone = board.clone();
        let mut moves = vec![];
        while let Ok(m) = board_clone.unmake() {
            moves.push(m)
        }

        if board_clone == Position::default() {
            let mut eval = SearchNode {
                position: Position::default(),
                phase: Default::default(),
                material: Default::default(),
                facets: vec![
                    Box::new(PieceSquareTablesFacet::default()),
                    Box::new(CastlingFacet::default()),
                    //Box::new(DevelopmentFacet::default()),
                    //Box::new(KnightRimFacet::default()),
                    //Box::new(PawnStructureFacet::default())
                ],
            };
            moves.into_iter().rev().for_each(|m| eval.make(m).unwrap());
            eval
        } else {
            SearchNode {
                material: MaterialFacet::from(&board),
                phase: Phase::from(&board),
                facets: vec![
                    Box::new(PieceSquareTablesFacet::from(&board)),
                    //Box::new(PawnStructureFacet::default())
                ],
                position: board,
            }
        }
    }
}

#[cfg(test)]
mod test {
    //#[test]
    //fn sanity() {
    //    assert_eq!(crate::START_FEN, crate::START_FEN.parse::<>().unwrap().to_fen())
    //}

    //#[test]
    //fn from_board_from_start() {
    //    let pgn = "1. e4 e5 2. f4 exf4 3. Nf3 g5 4. Nc3 Nc6 5. g3 g4 6. Nh4 Nd4 7. Bc4 Be7";
    //    let mut board = Board::default();
    //    board.play_pgn(pgn).unwrap();
    //    let eval = Evaluator::from(board.clone());
    //    assert_eq!(board, eval.board().clone());
    //    assert_eq!(5, eval.facets.len());
    //}

    //#[test]
    //fn from_board_from_position() {
    //    let fen = "r5k1/pb4pp/1pn1pq2/5B2/2Pr4/B7/PP3RPP/R4QK1 b - - 0 23";
    //    let board = Board::from_str(fen).unwrap();
    //    let eval = Evaluator::from(board.clone());
    //    assert_eq!(board, eval.board().clone());
    //    assert_eq!(2, eval.facets.len());
    //}
}
