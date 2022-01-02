use myopic_brain::anyhow::Result;
use myopic_brain::{Board, ChessBoard, EvalBoard};

use crate::game::InitalPosition;

pub fn get(initial: &InitalPosition, uci_sequence: &str) -> Result<EvalBoard<Board>> {
    let mut position = match initial {
        InitalPosition::Start => myopic_brain::START_FEN,
        InitalPosition::CustomFen(fen) => fen.as_str(),
    }
    .parse::<EvalBoard<Board>>()?;

    position.play_uci(uci_sequence)?;
    Ok(position)
}
