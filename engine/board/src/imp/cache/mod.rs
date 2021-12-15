pub use constraints::MoveConstraints;
use myopic_core::BitBoard;

use crate::Board;
use crate::imp::cache::rays::RaySet;
use crate::Termination;

mod constraints;
mod control;
mod rays;
mod termination;

// TODO Can we switch to returning references from the cache?
#[derive(Debug, Clone, Default)]
pub struct CalculationCache {
    termination_status: Option<Option<Termination>>,
    passive_control: Option<BitBoard>,
    pinned_set: Option<RaySet>,
    move_constraints: Option<MoveConstraints>,
}

impl Board {
    pub fn clear_cache(&mut self) {
        self.cache.termination_status = None;
        self.cache.passive_control = None;
        self.cache.pinned_set = None;
        self.cache.move_constraints = None;
    }
}
