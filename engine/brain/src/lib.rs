#[macro_use]
#[cfg(test)]
extern crate lazy_static;

pub use eval::tables::PositionTables;
pub use eval::values::PieceValues;
pub use eval::Evaluator;
pub use myopic_board::*;
pub use search::negascout;
pub use search::search;
pub use search::terminator::SearchTerminator;
pub use search::SearchOutcome;
pub use search::SearchParameters;

mod eval;

mod quiescent;
mod search;
mod see;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod test;
