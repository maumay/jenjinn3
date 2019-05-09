use crate::base::bitboard::BitBoard;
use crate::base::square::Square;
use crate::base::Side;
use crate::board::Board;
use crate::board::Move;
use crate::pieces;
use crate::pieces::PieceRef;

#[cfg(test)]
mod control_test;
#[cfg(test)]
mod pin_test;

type PinnedPiece = (Square, BitBoard);
type PinnedSet = (BitBoard, Vec<PinnedPiece>);
type ArmyLocations = (BitBoard, BitBoard);

const WHITE_SLIDERS: [PieceRef; 3] = [pieces::WB, pieces::WR, pieces::WQ];
const BLACK_SLIDERS: [PieceRef; 3] = [pieces::BB, pieces::BR, pieces::BQ];
const FILES: [BitBoard; 8] = BitBoard::FILES;

fn compute_constraint_area(piece_loc: Square, pinned: &PinnedSet, existing: BitBoard) -> BitBoard {
    let (all_pinned_locs, pinned_pieces) = pinned;
    if all_pinned_locs.contains(piece_loc) {
        pinned_pieces
            .into_iter()
            .find(|(sq, _)| *sq == piece_loc)
            .unwrap()
            .1
            & existing
    } else {
        BitBoard::ALL & existing
    }
}

/// TODO We want to just calculate the exact two squares from which an enpassant can be made.
fn enpassant_source_squares(enpassant_target: Square) -> BitBoard {
    unimplemented!()
    //    let fi = enpassant_target.file_index() as usize;
    //    match fi % 7 {
    //        0 => {
    //            if fi == 0 {
    //                FILES[1]
    //            } else {
    //                FILES[6]
    //            }
    //        }
    //        _ => FILES[fi + 1] | FILES[fi - 1],
    //    }
}

impl Board {
    pub fn compute_moves(&self, constraint: BitBoard) -> Vec<Move> {
        let pinned = self.compute_pinned();
        let armies = (self.pieces.whites(), self.pieces.blacks());
        let mut dest: Vec<Move> = Vec::with_capacity(50);
        dest.extend(self.compute_knight_and_slider_moves(armies, &pinned, constraint));

        let passive_control = self.compute_control(self.active.other());

        unimplemented!()
    }

    fn compute_pawn_moves(
        &self,
        armies: ArmyLocations,
        pinned: &PinnedSet,
        initial_constraint: BitBoard,
    ) -> Vec<Move> {
        let (whites, blacks) = armies;
        let mut dest: Vec<Move> = Vec::with_capacity(20);
        let (standard, enpassant, promotion) = self.separate_pawn_locs();
        let active_pawn = pieces::pawn(self.active);

        // TODO Would be nice if we could inject these functions as arguments.
        let compute_constraint =
            |location: Square| compute_constraint_area(location, pinned, initial_constraint);
        let moves = |piece: PieceRef, location: Square| piece.moves(location, whites, blacks);

        // Add moves for pawns which can only produce standard moves.
        for location in standard | enpassant {
            let targets = moves(active_pawn, location) & compute_constraint(location);
            dest.extend(Move::standards(active_pawn, location, targets));
        }

        for location in enpassant {
            if compute_constraint(location).contains(self.enpassant.unwrap()) {
                dest.push(Move::Enpassant(location));
            }
        }

        for location in promotion {
            let targets = moves(active_pawn, location) & compute_constraint(location);
            dest.extend(Move::promotions(self.active, location, targets));
        }

        dest
    }

    fn separate_pawn_locs(&self) -> (BitBoard, BitBoard, BitBoard) {
        let enpassant_files = self
            .enpassant
            .map_or(BitBoard::EMPTY, |sq| enpassant_source_squares(sq));
        let promotion_rank = self.active.pawn_last_rank();
        let pawn_locs = self.pieces.locations(pieces::pawn(self.active));
        (
            pawn_locs - enpassant_files - promotion_rank,
            pawn_locs & enpassant_files,
            pawn_locs & promotion_rank,
        )
    }

    fn compute_knight_and_slider_moves(
        &self,
        armies: ArmyLocations,
        pinned: &PinnedSet,
        total_constraint: BitBoard,
    ) -> Vec<Move> {
        let (whites, blacks) = armies;
        let mut dest: Vec<Move> = Vec::with_capacity(50);
        // Add standard moves for pieces which aren't pawns or king
        for &piece in Board::knight_and_sliders(self.active).iter() {
            for location in self.pieces.locations(piece) {
                let constraint = compute_constraint_area(location, pinned, total_constraint);
                let moves = piece.moves(location, whites, blacks) & constraint;
                dest.extend(Move::standards(piece, location, moves));
            }
        }
        dest
    }

    fn knight_and_sliders(side: Side) -> [PieceRef; 4] {
        match side {
            Side::White => [pieces::WN, pieces::WB, pieces::WR, pieces::WQ],
            Side::Black => [pieces::BN, pieces::BB, pieces::BR, pieces::BQ],
        }
    }

    pub fn compute_attacks(&self) -> Vec<Move> {
        unimplemented!()
    }

    pub fn has_legal_move(&self) -> bool {
        unimplemented!()
    }

    /// Computes the total area of control on the board for a given side.
    /// TODO Improve efficiency by treated all pawns as a block
    fn compute_control(&self, side: Side) -> BitBoard {
        let (whites, blacks) = (self.pieces.whites(), self.pieces.blacks());
        let locs = |piece: PieceRef| self.pieces.locations(piece);
        let control = |piece: PieceRef, square: Square| piece.control(square, whites, blacks);
        pieces::on_side(side)
            .iter()
            .flat_map(|&p| locs(p).into_iter().map(move |sq| control(p, sq)))
            .collect()
    }

    /// Computes the set of all active pieces which are pinned to the king,
    /// i.e have their movement areas constrained so that they do not move
    /// and leave the king in check.
    ///
    fn compute_pinned(&self) -> PinnedSet {
        let locs = |side: Side| self.pieces.side_locations(side);
        let (active, passive) = (locs(self.active), locs(self.active.other()));
        let king_loc = self.pieces.king_location(self.active);
        let mut pinned: Vec<PinnedPiece> = Vec::with_capacity(2);
        let mut pinned_locs = BitBoard::EMPTY;
        for potential_pinner in self.compute_potential_pinners(king_loc) {
            let cord = BitBoard::cord(king_loc, potential_pinner);
            if (cord & active).size() == 2 && (cord & passive).size() == 1 {
                let pinned_loc = ((cord & active) - king_loc).into_iter().next().unwrap();
                pinned.push((pinned_loc, cord));
                pinned_locs |= pinned_loc;
            }
        }
        (pinned_locs, pinned)
    }

    fn compute_potential_pinners(&self, king_loc: Square) -> BitBoard {
        let passive_sliders = match self.active {
            Side::White => BLACK_SLIDERS,
            Side::Black => WHITE_SLIDERS,
        };
        let locs = |p: PieceRef| self.pieces.locations(p);
        passive_sliders
            .iter()
            .flat_map(|&p| locs(p) & p.empty_control(king_loc))
            .collect()
    }
}
