use std::str::FromStr;

use myopic_board::anyhow::{Error, Result};
use myopic_board::{
    BitBoard, CastleZone, ChessBoard, FenPart, Move, MoveComputeType, Piece, Side, Square,
    TerminalState,
};

use crate::enumset::EnumSet;
use crate::eval::antipattern::KnightRimFacet;
use crate::eval::castling::CastlingFacet;
use crate::eval::development::DevelopmentFacet;
use crate::eval::material::MaterialFacet;
use crate::eval::{EvalChessBoard, EvalFacet};
use crate::{eval, Board, PieceValues, PositionTables};

pub struct EvalBoard<B: ChessBoard> {
    board: B,
    material: MaterialFacet,
    facets: Vec<Box<dyn EvalFacet<B>>>,
}

pub struct BoardBuilder {
    board: Board,
    piece_values: PieceValues,
    position_tables: PositionTables,
    facets: Vec<Box<dyn EvalFacet<Board>>>,
}

impl Default for EvalBoard<Board> {
    fn default() -> Self {
        crate::START_FEN.parse().unwrap()
    }
}

impl From<Board> for EvalBoard<Board> {
    fn from(board: Board) -> Self {
        BoardBuilder::from(board).build()
    }
}

impl From<Board> for BoardBuilder {
    fn from(board: Board) -> Self {
        BoardBuilder {
            position_tables: PositionTables::default(),
            piece_values: PieceValues::default(),
            facets: if board.to_fen().as_str() == crate::START_FEN {
                vec![
                    Box::new(CastlingFacet::default()),
                    Box::new(DevelopmentFacet::default()),
                    Box::new(KnightRimFacet::default()),
                ]
            } else {
                vec![]
            },
            board,
        }
    }
}

impl FromStr for BoardBuilder {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BoardBuilder::from(s.parse::<Board>()?))
    }
}

impl FromStr for EvalBoard<Board> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<BoardBuilder>().map(|b| b.build())
    }
}

impl BoardBuilder {
    pub fn set_piece_values(mut self, piece_values: PieceValues) -> BoardBuilder {
        self.piece_values = piece_values;
        self
    }

    pub fn set_position_tables(mut self, position_tables: PositionTables) -> BoardBuilder {
        self.position_tables = position_tables;
        self
    }

    pub fn add_eval_component(mut self, cmp: Box<dyn EvalFacet<Board>>) -> BoardBuilder {
        self.facets.push(cmp);
        self
    }

    pub fn build(self) -> EvalBoard<Board> {
        EvalBoard {
            material: MaterialFacet::new(&self.board, self.piece_values, self.position_tables),
            board: self.board,
            facets: self.facets,
        }
    }
}

impl<B: ChessBoard + Clone> ChessBoard for EvalBoard<B> {
    fn make(&mut self, action: Move) -> Result<()> {
        self.material.make(&action, &self.board);
        for cmp in self.facets.iter_mut() {
            cmp.make(&action, &self.board);
        }
        self.board.make(action)
    }

    fn unmake(&mut self) -> Result<Move> {
        let action = self.board.unmake()?;
        <MaterialFacet as EvalFacet<B>>::unmake(&mut self.material, &action);
        for cmp in self.facets.iter_mut() {
            cmp.unmake(&action);
        }
        Ok(action)
    }

    fn play_pgn(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut state_clone = self.board.clone();
        let parsed_moves = state_clone.play_pgn(moves)?;
        for mv in parsed_moves.iter() {
            self.make(mv.clone())?;
        }
        Ok(parsed_moves)
    }

    fn play_uci(&mut self, moves: &str) -> Result<Vec<Move>> {
        let mut state_clone = self.board.clone();
        let parsed_moves = state_clone.play_uci(moves)?;
        for mv in parsed_moves.iter() {
            self.make(mv.clone())?;
        }
        Ok(parsed_moves)
    }

    fn compute_moves(&self, computation_type: MoveComputeType) -> Vec<Move> {
        self.board.compute_moves(computation_type)
    }

    fn terminal_state(&self) -> Option<TerminalState> {
        self.board.terminal_state()
    }

    fn in_check(&self) -> bool {
        self.board.in_check()
    }

    fn side(&self, side: Side) -> BitBoard {
        self.board.side(side)
    }

    fn sides(&self) -> (BitBoard, BitBoard) {
        self.board.sides()
    }

    fn hash(&self) -> u64 {
        self.board.hash()
    }

    fn active(&self) -> Side {
        self.board.active()
    }

    fn enpassant(&self) -> Option<Square> {
        self.board.enpassant()
    }

    fn locs(&self, pieces: &[Piece]) -> BitBoard {
        self.board.locs(pieces)
    }

    fn king(&self, side: Side) -> Square {
        self.board.king(side)
    }

    fn piece(&self, location: Square) -> Option<Piece> {
        self.board.piece(location)
    }

    fn half_move_clock(&self) -> usize {
        self.board.half_move_clock()
    }

    fn position_count(&self) -> usize {
        self.board.position_count()
    }

    fn remaining_rights(&self) -> EnumSet<CastleZone> {
        self.board.remaining_rights()
    }

    fn parse_uci(&self, uci_move: &str) -> Result<Move> {
        self.board.parse_uci(uci_move)
    }

    fn to_fen_parts(&self, cmps: &[FenPart]) -> String {
        self.board.to_fen_parts(cmps)
    }
}

impl EvalChessBoard for EvalBoard<Board> {
    fn relative_eval(&self) -> i32 {
        match self.terminal_state() {
            Some(TerminalState::Draw) => eval::DRAW_VALUE,
            Some(TerminalState::Loss) => eval::LOSS_VALUE,
            None => {
                let eval = self.material.static_eval(&self.board)
                    + self
                        .facets
                        .iter()
                        .map(|cmp| cmp.static_eval(&self.board))
                        .sum::<i32>();
                match self.active() {
                    Side::W => eval,
                    Side::B => -eval,
                }
            }
        }
    }

    // TODO For now we just use midgame values, should take into account phase
    fn piece_values(&self) -> &[i32; 6] {
        &self.material.values().midgame
    }

    // TODO For now we just use midgame values, should take into account phase
    fn positional_eval(&self, piece: Piece, location: Square) -> i32 {
        self.material.tables().midgame(piece, location)
    }
}

impl<B: ChessBoard + Clone> EvalBoard<B> {
    pub fn clone_position(&self) -> B {
        self.board.clone()
    }
}

#[cfg(test)]
mod test {
    use myopic_board::{ChessBoard, Reflectable, UciMove};

    use crate::eval::material;
    use crate::{Board, BoardBuilder, PieceValues, PositionTables};

    #[derive(Clone)]
    struct TestCase {
        start_position: Board,
        moves: Vec<UciMove>,
    }

    impl Reflectable for TestCase {
        fn reflect(&self) -> Self {
            TestCase {
                start_position: self.start_position.reflect(),
                moves: self.moves.reflect(),
            }
        }
    }

    fn execute_test(test_case: TestCase) {
        execute_test_impl(test_case.clone());
        execute_test_impl(test_case.reflect());
    }

    fn execute_test_impl(test_case: TestCase) {
        let (tables, values) = (PositionTables::default(), PieceValues::default());
        let mut start = BoardBuilder::from(test_case.start_position)
            .set_piece_values(values.clone())
            .set_position_tables(tables.clone())
            .build();

        for uci_move in test_case.moves {
            let move_to_make = start.parse_uci(uci_move.as_str()).unwrap();
            start.make(move_to_make).unwrap();
            assert_eq!(
                material::compute_midgame(&start, &values, &tables),
                start.material.mid_eval()
            );
            assert_eq!(
                material::compute_endgame(&start, &values, &tables),
                start.material.end_eval()
            );
            let move_made = start.unmake().unwrap();
            assert_eq!(
                material::compute_midgame(&start, &values, &tables),
                start.material.mid_eval()
            );
            assert_eq!(
                material::compute_endgame(&start, &values, &tables),
                start.material.end_eval()
            );
            start.make(move_made).unwrap();
        }
    }

    fn test(start_fen: &'static str, moves: Vec<UciMove>) {
        execute_test(TestCase {
            start_position: start_fen.parse::<Board>().unwrap(),
            moves,
        })
    }

    #[test]
    fn case_1() {
        test(
            "rnbqk1nr/pp1pppbp/6p1/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 4",
            vec![
                UciMove::new("c2c3").unwrap(),
                UciMove::new("g8f6").unwrap(),
                UciMove::new("e1g1").unwrap(),
                UciMove::new("b7b6").unwrap(),
                UciMove::new("d2d3").unwrap(),
                UciMove::new("c8b7").unwrap(),
                UciMove::new("c1g5").unwrap(),
                UciMove::new("b8c6").unwrap(),
                UciMove::new("b1d2").unwrap(),
                UciMove::new("d8c7").unwrap(),
                UciMove::new("d1c2").unwrap(),
                UciMove::new("e8c8").unwrap(),
                UciMove::new("e4e5").unwrap(),
                UciMove::new("d7d5").unwrap(),
                UciMove::new("e5d6").unwrap(),
                UciMove::new("c8b8").unwrap(),
                UciMove::new("d6e7").unwrap(),
                UciMove::new("h8g8").unwrap(),
                UciMove::new("e7d8q").unwrap(),
            ],
        );
    }
}
