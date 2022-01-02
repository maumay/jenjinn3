use std::time::Duration;

use lambda_runtime::{handler_fn, Context, Error};
use simple_logger::SimpleLogger;

use lambda_payloads::chessmove::*;
use myopic_brain::anyhow;
use myopic_brain::negascout::SearchContext;
use myopic_brain::{Board, ChessBoard, EvalBoard, SearchParameters};

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()?;
    lambda_runtime::run(handler_fn(move_compute_handler)).await?;
    Ok(())
}

async fn move_compute_handler(
    e: ComputeMoveEvent,
    _ctx: Context,
) -> Result<ComputeMoveOutput, Error> {
    log::info!("Received input payload {}", serde_json::to_string(&e)?);
    let terminator = extract_params(&e);
    let position = extract_position(&e)?;
    let output_payload =
        myopic_brain::search(position, terminator).map(|outcome| ComputeMoveOutput {
            best_move: outcome.best_move.uci_format(),
            depth_searched: outcome.depth,
            eval: outcome.eval,
            search_duration_millis: outcome.time.as_millis() as u64,
        })?;
    log::info!(
        "Computed output payload {}",
        serde_json::to_string(&output_payload)?
    );
    Ok(output_payload)
}

fn extract_position(e: &ComputeMoveEvent) -> Result<EvalBoard<Board>, anyhow::Error> {
    match e {
        ComputeMoveEvent::Fen { position, .. } => position.as_str().parse(),
        ComputeMoveEvent::UciSequence {
            sequence,
            start_fen,
            ..
        } => {
            let mut state = start_fen
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or(myopic_brain::START_FEN)
                .parse::<EvalBoard<Board>>()?;
            state.play_uci(sequence.as_str())?;
            log::info!("Constructed state from {:?} then {}", start_fen, sequence);
            Ok(state)
        }
    }
}

struct SearchTerminatorWrapper(pub SearchTerminator);

impl myopic_brain::SearchTerminator for SearchTerminatorWrapper {
    fn should_terminate(&self, ctx: &SearchContext) -> bool {
        let timeout = Duration::from_millis(self.0.timeout_millis.0);
        (timeout, self.0.max_depth.0 as usize).should_terminate(ctx)
    }
}

fn extract_params(e: &ComputeMoveEvent) -> SearchParameters<SearchTerminatorWrapper> {
    match e {
        &ComputeMoveEvent::Fen {
            terminator,
            table_size,
            ..
        } => SearchParameters {
            terminator: SearchTerminatorWrapper(terminator),
            table_size: table_size.0 as usize,
        },
        &ComputeMoveEvent::UciSequence {
            terminator,
            table_size,
            ..
        } => SearchParameters {
            terminator: SearchTerminatorWrapper(terminator),
            table_size: table_size.0 as usize,
        },
    }
}
