use anyhow::{anyhow, Result};

use myopic_core::*;

use crate::imp::history::Discards;
use crate::imp::Board;
use crate::mv::Move::*;
use crate::{ChessBoard, Move};

#[cfg(test)]
mod test;

/// Implementation of board evolution/devolution via some given Move
/// instance which is assumed to be legal for this board.
impl Board {
    /// Public API for evolving a board. All that is required is a reference to
    /// a move which is assumed to be legal. The information required to reverse
    /// this same move is returned and the board is mutated to the next state.
    pub(crate) fn make(&mut self, mv: Move) -> Result<()> {
        // Check this move came from this position
        if self.hash() != mv.source() {
            return Err(anyhow!("Mismatched source hash for {}", mv));
        }

        // Preserve the current state
        self.history.push(
            mv.clone(),
            Discards {
                rights: self.rights,
                enpassant: self.enpassant(),
                half_move_clock: self.half_move_clock(),
            },
        );

        // Moves specific actions
        match mv {
            Standard {
                moving,
                from,
                dest,
                capture,
                ..
            } => self.make_standard(moving, from, dest, capture),
            Castle { zone, .. } => self.make_castle(zone),
            Enpassant {
                side,
                from,
                dest,
                capture,
                ..
            } => self.make_enpassant(side, from, dest, capture),
            Promotion {
                from,
                dest,
                promoted,
                capture,
                ..
            } => self.make_promotion(from, dest, promoted, capture),
        };

        // General actions
        self.active = self.active.reflect();
        self.clear_cache();
        Ok(())
    }

    fn make_standard(
        &mut self,
        moving: Piece,
        source: Square,
        target: Square,
        captured: Option<Piece>,
    ) {
        if let Some(p) = captured {
            self.pieces.unset_piece(p, target);
        }
        self.pieces.unset_piece(moving, source);
        self.pieces.set_piece(moving, target);
        self.rights = self.rights.remove_rights(source | target);
        self.enpassant = Board::compute_enpassant(source, target, moving);
        self.clock = if captured.is_some() || moving.is_pawn() {
            0
        } else {
            self.clock + 1
        };
    }

    fn make_castle(&mut self, zone: CastleZone) {
        self.rights = self.rights.apply_castling(zone.side());
        let (rook, r_source, r_target) = zone.rook_data();
        let (king, k_source, k_target) = zone.king_data();
        self.pieces.unset_piece(rook, r_source);
        self.pieces.unset_piece(king, k_source);
        self.pieces.set_piece(rook, r_target);
        self.pieces.set_piece(king, k_target);
        self.enpassant = None;
        self.clock += 1;
    }

    fn make_enpassant(&mut self, side: Side, from: Square, dest: Square, capture: Square) {
        let moving_pawn = Piece::pawn(side);
        self.pieces.unset_piece(moving_pawn.reflect(), capture);
        self.pieces.unset_piece(moving_pawn, from);
        self.pieces.set_piece(moving_pawn, dest);
        self.enpassant = None;
        self.clock = 0;
    }

    fn make_promotion(
        &mut self,
        from: Square,
        dest: Square,
        promoted: Piece,
        captured: Option<Piece>,
    ) {
        if let Some(p) = captured {
            self.pieces.unset_piece(p, dest);
        }
        let moved = Piece::pawn(promoted.side());
        self.pieces.unset_piece(moved, from);
        self.pieces.set_piece(promoted, dest);
        self.enpassant = None;
        self.clock = 0;
    }

    /// Public API for devolving a move, the information lost at evolve time is
    /// required as an input here to recover the lost state exactly.
    pub(crate) fn unmake(&mut self) -> Result<Move> {
        let (mv, state) = self.history.attempt_pop()?;

        match &mv {
            &Standard {
                moving,
                from,
                dest,
                capture,
                ..
            } => self.unmake_standard(moving, from, dest, capture),

            &Promotion {
                from,
                dest,
                promoted,
                capture,
                ..
            } => self.unmake_promotion(from, dest, promoted, capture),
            &Enpassant {
                side,
                from,
                dest,
                capture,
                ..
            } => self.unmake_enpassant(side, from, dest, capture),
            &Castle { zone, .. } => self.unmake_castle(zone),
        };

        self.rights = state.rights;
        self.clock = state.half_move_clock;
        self.enpassant = state.enpassant;
        self.active = self.active.reflect();
        self.clear_cache();
        Ok(mv)
    }

    fn unmake_standard(
        &mut self,
        piece: Piece,
        source: Square,
        target: Square,
        captured: Option<Piece>,
    ) {
        self.pieces.unset_piece(piece, target);
        self.pieces.set_piece(piece, source);
        if let Some(p) = captured {
            self.pieces.set_piece(p, target);
        }
    }

    fn unmake_castle(&mut self, zone: CastleZone) {
        let (rook, r_source, r_target) = zone.rook_data();
        let (king, k_source, k_target) = zone.king_data();
        self.pieces.set_piece(rook, r_source);
        self.pieces.set_piece(king, k_source);
        self.pieces.unset_piece(rook, r_target);
        self.pieces.unset_piece(king, k_target);
    }

    fn unmake_enpassant(&mut self, side: Side, from: Square, dest: Square, capture: Square) {
        let moving_pawn = Piece::pawn(side);
        self.pieces.unset_piece(moving_pawn, dest);
        self.pieces.set_piece(moving_pawn.reflect(), capture);
        self.pieces.set_piece(moving_pawn, from);
    }

    fn unmake_promotion(
        &mut self,
        from: Square,
        dest: Square,
        promoted: Piece,
        captured: Option<Piece>,
    ) {
        let moved = Piece::pawn(promoted.side());
        self.pieces.unset_piece(promoted, dest);
        self.pieces.set_piece(moved, from);
        if let Some(p) = captured {
            self.pieces.set_piece(p, dest);
        }
    }

    /// Determines the enpassant square for the next board state given a
    /// piece which has just moved from the source to the target.
    fn compute_enpassant(source: Square, target: Square, piece: Piece) -> Option<Square> {
        if piece.is_pawn() {
            let side = piece.side();
            if side.pawn_first_rank().contains(source) && side.pawn_third_rank().contains(target) {
                source.next(side.pawn_dir())
            } else {
                None
            }
        } else {
            None
        }
    }
}
