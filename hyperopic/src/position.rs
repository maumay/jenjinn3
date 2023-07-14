use crate::moves::{Move, Move::*, MoveFacet, Moves};
use crate::{
    board, create_piece, first_square, in_board, intersects, is_superset, lift, piece_class,
    piece_side, reflect_piece, reflect_side, square_file, square_rank, union_boards, Board, Corner,
    CornerMap, Piece, PieceMap, Side, SideMap, Square, SquareMap,
};
use std::io::Read;

use crate::board::{board_moves, control, cord, iter};
use crate::constants::boards::{ADJACENT_FILES, RANKS};
use crate::constants::{class, piece, side};
use anyhow::{anyhow, Result};

/// Represents the possible ways a game can be terminated, we only
/// consider a game to be terminated when a side has no legal moves
/// to make or if a special draw condition is met like position
/// repetition. If a side has no legal moves and is currently in check
/// then the game is lost, if it is not in check then the game is
/// drawn.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum TerminalState {
    Draw,
    Loss,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub piece_boards: PieceMap<Board>,
    pub piece_locs: SquareMap<Option<Piece>>,
    pub side_boards: SideMap<Board>,
    pub castling_rights: CornerMap<bool>,
    pub active: Side,
    pub enpassant: Option<Square>,
    pub clock: usize,
    pub key: u64,
    pub history: Vec<(Discards, Move)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Discards {
    pub castling_rights: CornerMap<bool>,
    pub enpassant: Option<Square>,
    pub clock: usize,
    pub key: u64,
}

impl Default for Position {
    fn default() -> Self {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".parse().unwrap()
    }
}

impl Position {
    pub fn new(
        active: Side,
        enpassant: Option<Square>,
        clock: usize,
        castling_rights: CornerMap<bool>,
        piece_locs: SquareMap<Option<Piece>>,
    ) -> Position {
        let mut key = if active == side::W { 0u64 } else { crate::hash::black_move() };
        enpassant.map(|sq| key ^= crate::hash::enpassant(sq));
        (0..64).for_each(|sq| {
            piece_locs[sq].map(|p| key ^= crate::hash::piece(p, sq));
        });
        (0..4).for_each(|c| {
            if castling_rights[c] {
                key ^= crate::hash::corner(c);
            }
        });
        let piece_boards: PieceMap<_> = std::array::from_fn(|p| {
            (0..64).filter(|&sq| piece_locs[sq] == Some(p)).fold(0u64, |a, n| a | lift(n))
        });
        let side_boards: SideMap<_> = std::array::from_fn(|side| {
            (0..64)
                .filter(|&sq| piece_locs[sq].map(|p| piece_side(p)) == Some(side))
                .fold(0u64, |a, n| a | lift(n))
        });
        Position {
            active,
            enpassant,
            clock,
            key,
            history: vec![],
            piece_boards,
            side_boards,
            piece_locs,
            castling_rights,
        }
    }
}

// Implementation block for making/unmaking moves
impl Position {
    pub fn make(&mut self, m: Move) -> Result<()> {
        use crate::constants::boards::ENPASSANT_RANKS;
        self.history.push((self.create_discards(), m.clone()));
        self.enpassant.map(|sq| self.key ^= crate::hash::enpassant(sq));
        match m {
            Null => self.enpassant = None,
            Normal { moving, from, dest, capture } => {
                capture.map(|p| self.unset_piece(p, dest));
                self.unset_piece(moving, from);
                self.set_piece(moving, dest);
                self.remove_rights(rights_removed(from));
                self.remove_rights(rights_removed(dest));
                let is_pawn = piece_class(moving) == class::P;
                self.clock = if capture.is_some() || is_pawn { 0 } else { self.clock + 1 };
                self.enpassant = if is_pawn && is_superset(ENPASSANT_RANKS, board!(from, dest)) {
                    let is_white = piece_side(moving) == side::W;
                    let shifter = if is_white { from } else { dest };
                    let next_ep = shifter + 8;
                    self.key ^= crate::hash::enpassant(next_ep);
                    Some(next_ep)
                } else {
                    None
                }
            }
            Promote { from, dest, promoted, capture } => {
                capture.map(|p| self.unset_piece(p, dest));
                let moved = if piece_side(promoted) == side::W { piece::WP } else { piece::BP };
                self.remove_rights(rights_removed(dest));
                self.unset_piece(moved, from);
                self.set_piece(promoted, dest);
                self.enpassant = None;
                self.clock = 0;
            }
            Enpassant { side, from, dest, capture } => {
                let moving = if side == side::W { piece::WP } else { piece::BP };
                let taken = if side == side::W { piece::BP } else { piece::WP };
                self.unset_piece(taken, capture);
                self.unset_piece(moving, from);
                self.set_piece(moving, dest);
                self.enpassant = None;
                self.clock = 0;
            }
            Castle { corner } => {
                let details = &CASTLING_DETAILS[corner];
                let (r_source, r_target) = details.rook_line;
                let (k_source, k_target) = details.king_line;
                self.remove_rights(rights_removed(k_source));
                let side = corner / 2;
                let rook = if side == side::W { piece::WR } else { piece::BR };
                let king = if side == side::W { piece::WK } else { piece::BK };
                self.unset_piece(rook, r_source);
                self.unset_piece(king, k_source);
                self.set_piece(rook, r_target);
                self.set_piece(king, k_target);
                self.enpassant = None;
                self.clock += 1;
            }
        };
        self.key ^= crate::hash::black_move();
        self.active = if self.active == side::W { side::B } else { side::W };
        Ok(())
    }

    pub fn unmake(&mut self) -> Result<Move> {
        if self.history.len() == 0 {
            return Err(anyhow!("No moves left to unmake!"));
        }
        let (state, mv) = self.history.remove(self.history.len() - 1);
        match &mv {
            Null => {}
            &Normal { moving, from, dest, capture } => {
                self.unset_piece(moving, dest);
                self.set_piece(moving, from);
                capture.map(|p| self.set_piece(p, dest));
            }
            &Promote { from, dest, promoted, capture } => {
                let moved = if piece_side(promoted) == side::W { piece::WP } else { piece::BP };
                self.unset_piece(promoted, dest);
                self.set_piece(moved, from);
                capture.map(|p| self.set_piece(p, dest));
            }
            &Enpassant { side, from, dest, capture } => {
                let moving = if side == side::W { piece::WP } else { piece::BP };
                let taken = if side == side::W { piece::BP } else { piece::WP };
                self.unset_piece(moving, dest);
                self.set_piece(taken, capture);
                self.set_piece(moving, from);
            }
            &Castle { corner } => {
                let details = &CASTLING_DETAILS[corner];
                let (r_source, r_target) = details.rook_line;
                let (k_source, k_target) = details.king_line;
                let side = corner / 2;
                let rook = if side == side::W { piece::WR } else { piece::BR };
                let king = if side == side::W { piece::WK } else { piece::BK };
                self.set_piece(rook, r_source);
                self.set_piece(king, k_source);
                self.unset_piece(rook, r_target);
                self.unset_piece(king, k_target);
            }
        };
        self.castling_rights = state.castling_rights;
        self.clock = state.clock;
        self.enpassant = state.enpassant;
        self.key = state.key;
        self.active = if self.active == side::W { side::B } else { side::W };
        Ok(mv)
    }

    fn set_piece(&mut self, piece: Piece, square: Square) {
        self.key ^= crate::hash::piece(piece, square);
        let lifted = lift(square);
        let side = piece_side(piece);
        self.piece_boards[piece] |= lifted;
        self.side_boards[side] |= lifted;
        self.piece_locs[square] = Some(piece);
    }

    fn unset_piece(&mut self, piece: Piece, square: Square) {
        self.key ^= crate::hash::piece(piece, square);
        let lifted = !lift(square);
        self.piece_boards[piece] &= lifted;
        self.side_boards[piece_side(piece)] &= lifted;
        self.piece_locs[square] = None;
    }

    fn remove_rights(&mut self, corners: &[Corner]) {
        corners.iter().for_each(|&c| {
            if self.castling_rights[c] {
                self.castling_rights[c] = false;
                self.key ^= crate::hash::corner(c);
            }
        })
    }

    pub fn create_discards(&self) -> Discards {
        Discards {
            castling_rights: self.castling_rights.clone(),
            enpassant: self.enpassant,
            clock: self.clock,
            key: self.key,
        }
    }
}

pub type Constraints = SquareMap<Board>;
#[derive(Debug, PartialEq)]
pub struct ConstrainedPieces(pub Board, pub SquareMap<Board>);

impl Position {
    pub fn compute_discoveries_on(&self, square: Square) -> Result<ConstrainedPieces> {
        let piece = self.piece_locs[square].ok_or_else(|| anyhow!("No piece at {}", square))?;
        let target_side = piece_side(piece);
        let discoverer_side = reflect_side(target_side);
        let target_locs = self.side_boards[target_side];
        let discoverer_locs = self.side_boards[discoverer_side];
        let (mut all_discoverers, mut constraints) = (0u64, [0u64; 64]);
        for from in iter(self.compute_xrays(discoverer_side, square)) {
            let cord = cord(from, square);
            let discoverers_on_cord = cord & discoverer_locs;
            if discoverers_on_cord.count_ones() == 2 && (cord & target_locs).count_ones() == 1 {
                let discoverer = discoverers_on_cord ^ lift(from);
                all_discoverers |= discoverer;
                constraints[discoverer.trailing_zeros() as usize] = !cord;
            }
        }
        Ok(ConstrainedPieces(all_discoverers, constraints))
    }

    pub fn compute_pinned_on(&self, square: Square) -> Result<ConstrainedPieces> {
        let piece = self.piece_locs[square].ok_or_else(|| anyhow!("No piece at {}", square))?;
        let pinned_side = piece_side(piece);
        let pinner_side = reflect_side(pinned_side);
        let pinned_locs = self.side_boards[pinned_side];
        let pinner_locs = self.side_boards[pinner_side];
        let (mut all_pinned, mut constraints) = (0u64, [0u64; 64]);
        for from in iter(self.compute_xrays(pinner_side, square)) {
            let cord = cord(from, square);
            let pinned_on_cord = cord & pinned_locs;
            if pinned_on_cord.count_ones() == 2 && (cord & pinner_locs).count_ones() == 1 {
                let pinned = pinned_on_cord ^ lift(square);
                all_pinned |= pinned;
                constraints[pinned.trailing_zeros() as usize] = cord;
            }
        }
        Ok(ConstrainedPieces(all_pinned, constraints))
    }

    fn compute_xrays(&self, side: Side, target: Square) -> Board {
        [class::R, class::B, class::Q]
            .iter()
            .map(|&c| create_piece(side, c))
            .map(|p| self.piece_boards[p] & control(p, target, 0))
            .fold(0u64, |a, n| a | n)
    }

    pub fn compute_control(&self, side: Side) -> Board {
        use crate::board::control;
        use crate::constants::{piece::*, side::*};
        let invisible_king = self.piece_boards[if side == W { BK } else { WK }];
        let occupied = (self.side_boards[W] | self.side_boards[B]) & !invisible_king;
        [class::N, class::B, class::R, class::Q, class::K]
            .into_iter()
            .map(|class| side * 6 + class)
            .flat_map(|p| iter(self.piece_boards[p]).map(move |sq| control(p, sq, occupied)))
            .fold(0u64, |a, n| a | n)
            | pawn_control(side, self.piece_boards[if side == W { WP } else { BP }])
    }
}

fn pawn_control(side: Side, pawns: Board) -> Board {
    use crate::constants::boards::FILES;
    let (not_a_file, not_h_file) = (!FILES[7], !FILES[0]);
    if side == side::W {
        ((pawns & not_a_file) << 9) | ((pawns & not_h_file) << 7)
    } else {
        ((pawns & not_h_file) >> 9) | ((pawns & not_a_file) >> 7)
    }
}

fn intersect_into(left: &mut ConstrainedPieces, right: &Constraints) {
    iter(left.0).for_each(|sq| left.1[sq] &= right[sq]);
}

fn constraint_union(left: &mut ConstrainedPieces, right: &ConstrainedPieces) {
    left.0 |= right.0;
    iter(left.0).for_each(|sq| left.1[sq] |= right.1[sq]);
}

impl Position {
    pub fn moves(&self, moves: Moves) -> Vec<Move> {
        let active = self.active;
        let passive_control = self.compute_control(reflect_side(active));
        let active_king = create_piece(active, class::K);
        let active_king_loc = self.piece_boards[active_king].trailing_zeros() as usize;
        if active_king_loc == 64 {
            // King not on the board -> no legal moves
            return vec![];
        }
        let pins = self.compute_pinned_on(active_king_loc).unwrap();
        let in_check = in_board(passive_control, active_king_loc);

        // The set of constraints for each piece on the board to avoid illegal moves
        let mut constraints = if in_check {
            let attacker_side = reflect_side(active);
            let occupied = self.side_boards[side::W] | self.side_boards[side::B];
            let king_attackers = (0..5)
                .map(|class| create_piece(attacker_side, class))
                // Reflect the piece when taking control or won't work for pawns
                .map(|p| (p, self.piece_boards[p] & control(reflect_piece(p), active_king_loc, 0)))
                .flat_map(|(p, b)| iter(b).map(move |sq| (p, sq)))
                .filter(|(p, sq)| in_board(control(*p, *sq, occupied), active_king_loc))
                .fold(0u64, |a, (_, n)| a | lift(n));

            if king_attackers.count_ones() == 1 {
                // We can move out of check or block the check, we still need to take pins into account
                let from = king_attackers.trailing_zeros() as Square;
                let blocking_squares = cord(from, active_king_loc);
                let mut result = [blocking_squares; 64];
                result[active_king_loc] = !passive_control;
                iter(pins.0).for_each(|sq| result[sq] &= pins.1[sq]);
                result
            } else {
                // Only legal moves are for king to move out of passive control
                let mut result = [crate::constants::boards::EMPTY; 64];
                result[active_king_loc] = !passive_control;
                result
            }
        } else {
            // We only need to care about pins + king not moving into check
            let mut result = [crate::constants::boards::ALL; 64];
            result[active_king_loc] = !passive_control;
            iter(pins.0).for_each(|sq| result[sq] &= pins.1[sq]);
            result
        };

        match moves {
            // No further constraints needed
            Moves::All => {}
            // We further constrain the piece moves based on the facets given
            Moves::AreAny(facets) => {
                // With facets set we default to no moves, then union moves allowed by each facet
                let mut updated_constraints = ConstrainedPieces(0, [0u64; 64]);
                facets.iter().for_each(|&f| {
                    constraint_union(&mut updated_constraints, &self.compute_facet_constraints(f));
                });
                intersect_into(&mut updated_constraints, &constraints);
                constraints = updated_constraints.1;
            }
        }
        self.compute_moves(&constraints, active_king_loc)
    }

    fn compute_facet_constraints(&self, facet: MoveFacet) -> ConstrainedPieces {
        let result = ConstrainedPieces(0, [0u64; 64]);
        match facet {
            MoveFacet::Checking => {}
            MoveFacet::Attacking => {}
            MoveFacet::Promoting => {}
        }
        result
    }

    fn compute_moves(&self, constraints: &Constraints, active_king: Square) -> Vec<Move> {
        let mut result = Vec::with_capacity(40);
        self.compute_pawn_moves(&constraints)
            .chain(self.compute_nbrqk_moves(&constraints))
            .chain(self.compute_castle_moves(constraints[active_king]))
            .for_each(|m| result.push(m));
        result
    }

    fn compute_nbrqk_moves<'a>(
        &'a self,
        constraints: &'a Constraints,
    ) -> impl Iterator<Item = Move> + 'a {
        let friendly = self.side_boards[self.active];
        let enemy = self.side_boards[reflect_side(self.active)];
        [class::N, class::B, class::R, class::Q, class::K].into_iter().flat_map(move |class| {
            let piece = create_piece(self.active, class);
            iter(self.piece_boards[piece]).flat_map(move |sq| {
                let moves = board_moves(piece, sq, friendly, enemy) & constraints[sq];
                self.create_normal_moves(piece, sq, moves)
            })
        })
    }

    fn compute_castle_moves<'a>(
        &'a self,
        king_constraint: Board,
    ) -> impl Iterator<Item = Move> + 'a {
        self.castling_rights.iter().enumerate().filter(|(_, &allowed)| allowed).filter_map(
            move |(corner, _)| {
                let details = &CASTLING_DETAILS[corner];
                if is_superset(king_constraint, details.no_control)
                    && !intersects(union_boards(&self.side_boards), details.no_piece)
                    && self.piece_locs[details.king_line.0]
                        == Some(create_piece(self.active, class::K))
                    && self.piece_locs[details.rook_line.0]
                        == Some(create_piece(self.active, class::R))
                {
                    Some(Castle { corner })
                } else {
                    None
                }
            },
        )
    }

    fn compute_pawn_moves<'a>(
        &'a self,
        constraints: &'a Constraints,
    ) -> impl Iterator<Item = Move> + 'a {
        let active = self.active;
        let moving = create_piece(active, class::P);
        let pawns = self.piece_boards[create_piece(active, class::P)];
        let is_white = active == side::W;
        let last_rank = if is_white { RANKS[6] } else { RANKS[1] };
        let enpassant_attack = self.enpassant.map_or(0u64, |sq| {
            let attack_rank = if is_white { RANKS[4] } else { RANKS[3] };
            attack_rank & ADJACENT_FILES[square_file(sq)]
        });
        let standard = pawns & !last_rank;
        let enpassant = pawns & enpassant_attack;
        let promotion = pawns & last_rank;
        let friendly = self.side_boards[active];
        let enemy = self.side_boards[reflect_side(active)];

        iter(standard)
            .flat_map(move |from| {
                self.create_normal_moves(
                    moving,
                    from,
                    board_moves(moving, from, friendly, enemy) & constraints[from],
                )
            })
            .chain(iter(promotion).flat_map(move |from| {
                self.create_promote_moves(
                    active,
                    from,
                    board_moves(moving, from, friendly, enemy) & constraints[from],
                )
            }))
            .chain(iter(enpassant).filter_map(move |from| {
                let dest = self.enpassant.unwrap();
                let capture = if is_white { dest - 8 } else { dest + 8 };
                if in_board(constraints[from], capture)
                    && self.enpassant_doesnt_discover_attack(from, capture)
                {
                    Some(Enpassant { side: active, from, dest, capture })
                } else {
                    None
                }
            }))
    }

    fn enpassant_doesnt_discover_attack(&self, from: Square, capture: Square) -> bool {
        let (active, passive) = (self.active, reflect_side(self.active));
        let active_king_loc = first_square(self.piece_boards[create_piece(active, class::K)]);
        let from_rank = RANKS[square_rank(from)];
        if !in_board(from_rank, active_king_loc) {
            return true;
        }
        let rook = create_piece(passive, class::R);
        let queen = create_piece(passive, class::Q);
        let attackers = (self.piece_boards[rook] | self.piece_boards[queen]) & from_rank;
        let occupied = self.side_boards[side::W] | self.side_boards[side::B];
        !iter(attackers).any(|sq| {
            let cord = cord(sq, active_king_loc) & occupied;
            // Exactly 4 pieces, king, attacker, the two pawns about to vacate the rank
            cord.count_ones() == 4 && is_superset(cord, board!(from, capture))
        })
    }

    fn create_normal_moves(
        &self,
        moving: Piece,
        from: Square,
        dest: Board,
    ) -> impl Iterator<Item = Move> + '_ {
        iter(dest).map(move |dest| Normal { moving, from, dest, capture: self.piece_locs[dest] })
    }

    fn create_promote_moves(
        &self,
        side: Side,
        from: Square,
        dest: Board,
    ) -> impl Iterator<Item = Move> + '_ {
        iter(dest).flat_map(move |dest| {
            [class::N, class::B, class::R, class::Q].into_iter().map(move |promoted| Promote {
                from,
                dest,
                promoted: create_piece(side, promoted),
                capture: self.piece_locs[dest],
            })
        })
    }
}

fn rights_removed<'a>(square: Square) -> &'a [Corner] {
    use crate::constants::{corner::*, square::*};
    match square {
        H1 => &[WK],
        E1 => &[WK, WQ],
        A1 => &[WQ],
        H8 => &[BK],
        E8 => &[BK, BQ],
        A8 => &[BQ],
        _ => &[],
    }
}

#[rustfmt::skip]
const CASTLING_DETAILS: CornerMap<CastlingDetails> = {
    use crate::constants::square::*;
    [
        CastlingDetails {
            king_line: (E1, G1),
            rook_line: (H1, F1),
            no_piece: board!(F1 => G1),
            no_control: board!(E1 => G1)
        },
        CastlingDetails {
            king_line: (E1, C1),
            rook_line: (A1, D1),
            no_piece: board!(D1 => B1),
            no_control: board!(E1 => C1)
        },
        CastlingDetails {
            king_line: (E8, G8),
            rook_line: (H8, F8),
            no_piece: board!(F8 => G8),
            no_control: board!(E8 => G8)
        },
        CastlingDetails {
            king_line: (E8, C8),
            rook_line: (A8, D8),
            no_piece: board!(D8 => B8),
            no_control: board!(E8 => C8)
        },
    ]
};

struct CastlingDetails {
    king_line: (Square, Square),
    rook_line: (Square, Square),
    no_piece: Board,
    no_control: Board,
}
