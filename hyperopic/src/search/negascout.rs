use core::cmp;
use std::time::Instant;
use anyhow::{anyhow, Result};

use TreeNode::{All, Cut, Pv};

use crate::search::moves::MoveGenerator;
use crate::search::terminator::SearchTerminator;
use crate::search::transpositions::{Transpositions, TreeNode};
use crate::{Class, Corner, create_piece, in_board, node, Piece, union_boards};
use crate::board::board_moves;
use crate::constants::class;
use crate::moves::Move;
use crate::node::SearchNode;
use crate::position::{CASTLING_DETAILS, TerminalState};
use crate::search::quiescent;

/// Provides relevant callstack information for the search to
/// use during the traversal of the tree.
pub struct Context {
    pub start: Instant,
    pub alpha: i32,
    pub beta: i32,
    pub depth: u8,
    pub precursors: Vec<Move>,
    pub early_break_enabled: bool,
}

impl Context {
    fn next_level(&self, next_alpha: i32, next_beta: i32, mv: &Move, r: u8) -> Context {
        let mut next_precursors = self.precursors.clone();
        next_precursors.push(mv.clone());
        Context {
            start: self.start,
            alpha: next_alpha,
            beta: next_beta,
            depth: self.depth - cmp::min(r, self.depth),
            precursors: next_precursors,
            early_break_enabled: self.early_break_enabled,
        }
    }
}

#[derive(Default)]
pub struct SearchResponse {
    /// The evaluation of the position negamax was called for
    pub eval: i32,
    /// The path of optimal play which led to the eval
    pub path: Vec<Move>,
}

impl std::ops::Neg for SearchResponse {
    type Output = SearchResponse;
    fn neg(self) -> Self::Output {
        SearchResponse { eval: -self.eval, path: self.path }
    }
}

pub struct Scout<'a, T: SearchTerminator, TT: Transpositions> {
    /// The terminator is responsible for deciding when the search is complete
    pub terminator: &'a T,
    /// Transposition table containing previously computed information about nodes in the tree.
    pub transpositions: &'a mut TT,
    /// Move generator for nodes in the tree
    pub moves: MoveGenerator<'a>,
}

impl<T: SearchTerminator, TT: Transpositions> Scout<'_, T, TT> {
    pub fn search(&mut self, node: &mut SearchNode, mut ctx: Context) -> Result<SearchResponse> {
        if self.terminator.should_terminate(&ctx) {
            return Err(anyhow!("Terminated at depth {}", ctx.depth));
        }
        let terminal_state = node.board().compute_terminal_state();
        if ctx.depth == 0 || terminal_state.is_some() {
            return match terminal_state {
                Some(TerminalState::Loss) => Ok(node::LOSS_VALUE),
                Some(TerminalState::Draw) => Ok(node::DRAW_VALUE),
                None => quiescent::search(node, ctx.alpha, ctx.beta),
            }
            .map(|eval| SearchResponse { eval, path: vec![] });
        }

        let (hash, mut table_move) = (node.board().key, None);
        match self.transpositions.get(node.board()) {
            None => {}
            Some(Pv { depth, eval, best_path: optimal_path, .. }) => {
                table_move = optimal_path.first().cloned();
                if ctx.early_break_enabled
                    && *depth >= ctx.depth
                    && table_move.is_some()
                    && is_pseudo_legal(node, table_move.as_ref().unwrap())
                {
                    return Ok(SearchResponse { eval: *eval, path: optimal_path.clone() });
                }
            }
            Some(Cut { depth, beta, cutoff_move, .. }) => {
                table_move = Some(cutoff_move.clone());
                if ctx.early_break_enabled
                    && *depth >= ctx.depth
                    && ctx.beta <= *beta
                    && is_pseudo_legal(node, cutoff_move)
                {
                    return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
                }
            }
            Some(All { depth, eval, best_move, .. }) => {
                table_move = Some(best_move.clone());
                if ctx.early_break_enabled
                    && *depth >= ctx.depth
                    && *eval <= ctx.alpha
                    && is_pseudo_legal(node, best_move)
                {
                    return Ok(SearchResponse { eval: *eval, path: vec![] });
                }
            }
        };

        if should_try_null_move_pruning(node, &ctx) {
            node.make(Move::Null)?;
            let null_search =
                -self.search(node, ctx.next_level(-ctx.beta, -ctx.alpha, &Move::Null, 3))?;
            node.unmake()?;
            if null_search.eval > ctx.beta {
                return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
            }
        }

        let (start_alpha, mut result, mut best_path) = (ctx.alpha, -node::INFTY, vec![]);
        for (i, m) in self.moves.generate(node, &ctx, table_move.as_ref()).enumerate() {
            node.make(m.clone())?;
            #[allow(unused_assignments)]
            let mut response = SearchResponse::default();
            if i == 0 {
                // Perform a full search immediately on the first move which
                // we expect to be the best
                response = -self.search(node, ctx.next_level(-ctx.beta, -ctx.alpha, &m, 1))?;
            } else {
                // Search with null window under the assumption that the
                // previous moves are better than this
                response = -self.search(node, ctx.next_level(-ctx.alpha - 1, -ctx.alpha, &m, 1))?;
                // If there is some move which can raise alpha
                if ctx.alpha < response.eval && response.eval < ctx.beta {
                    // Then this was actually a better move and so we must
                    // perform a full search
                    response = -self.search(node, ctx.next_level(-ctx.beta, -ctx.alpha, &m, 1))?;
                }
            }
            node.unmake()?;

            if response.eval > result {
                result = response.eval;
                best_path = response.path;
                best_path.insert(0, m.clone());
            }

            ctx.alpha = cmp::max(ctx.alpha, result);
            if ctx.alpha >= ctx.beta {
                self.transpositions.put(
                    node.board(),
                    Cut { depth: ctx.depth, beta: ctx.beta, cutoff_move: m, hash },
                );
                return Ok(SearchResponse { eval: ctx.beta, path: vec![] });
            }
        }

        // Populate the table with the information from this node.
        if ctx.alpha == start_alpha {
            if let Some(m) = best_path.first() {
                self.transpositions.put(
                    node.board(),
                    All { depth: ctx.depth, eval: result, best_move: m.clone(), hash },
                );
            }
        } else {
            self.transpositions.put(
                node.board(),
                Pv { depth: ctx.depth, eval: result, best_path: best_path.clone(), hash },
            );
        }

        Ok(SearchResponse { eval: result, path: best_path })
    }
}

fn is_pseudo_legal(node: &SearchNode, m: &Move) -> bool {
    let position = node.board();
    match m {
        Move::Null => false,
        Move::Enpassant { capture, .. } => position.enpassant == Some(*capture),
        &Move::Castle { corner } => {
            position.castling_rights[corner] && {
                let details = &CASTLING_DETAILS[corner];
                let rook = create_piece(position.active, class::R);
                let king = create_piece(position.active, class::K);
                position.piece_locs[details.rook_line.0] == Some(rook) &&
                    position.piece_locs[details.king_line.0] == Some(king)
            }
        }
        &Move::Normal { moving, from, dest, capture } => {
            let (friendly, enemy) = position.friendly_enemy_boards();
            position.piece_locs[from] == Some(moving)
                && position.piece_locs[dest] == capture
                && in_board(board_moves(moving, from, friendly, enemy), dest)
        }
        &Move::Promote { from, dest, capture, .. } => {
            position.piece_locs[from] == Some(create_piece(position.active, class::P))
                && position.piece_locs[dest] == capture
        }
    }
}

fn should_try_null_move_pruning(node: &SearchNode, ctx: &Context) -> bool {
    let position = node.board();
    ctx.depth < 5 && ctx.beta < 1000 && !position.in_check() && {
        let active = position.active;
        let pawns = position.piece_boards[create_piece(active, class::P)];
        let others = position.side_boards[active] & !pawns;
        pawns.count_ones() > 2 && others.count_ones() > 1 || others.count_ones() > 2
    }
}
