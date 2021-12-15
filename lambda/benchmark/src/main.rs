mod positions;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_derive::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use itertools::Itertools;
use std::time::Instant;

const LOG_GAP: usize = 2;

#[derive(Deserialize)]
struct BenchStartEvent {
    positions: usize,
    depth: usize,
}

#[derive(Serialize)]
struct BenchOutput {
    positions_searched: usize,
    depth_searched: usize,
    min_search_time_millis: u64,
    average_search_time_millis: u64,
    max_search_time_millis: u64,
    median_search_time_millis: u64,
    total_search_time_secs: u64,
    memory_allocated_mb: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    lambda!(handler);
    Ok(())
}

fn handler(e: BenchStartEvent, ctx: Context) -> Result<BenchOutput, HandlerError> {
    let roots = positions::get(e.positions)?;
    let n = roots.len();
    let start = Instant::now();
    let mut moves = vec![];
    for (i, root) in roots.into_iter().enumerate() {
        if i % LOG_GAP == 0 {
            log::info!("[Position {}, Elapsed {}ms]", i, start.elapsed().as_millis());
        }
        moves.push(myopic_brain::search(root, e.depth).map_err(h_err)?);
    }

    let execution_times = moves
        .iter()
        .map(|o| o.time.as_millis() as u64)
        .sorted()
        .collect::<Vec<_>>();

    let output = BenchOutput {
        depth_searched: e.depth,
        positions_searched: n,
        memory_allocated_mb: ctx.memory_limit_in_mb as usize,
        min_search_time_millis: execution_times[0],
        max_search_time_millis: execution_times[n - 1],
        median_search_time_millis: execution_times[n / 2],
        average_search_time_millis: execution_times.iter().sum::<u64>() / n as u64,
        total_search_time_secs: execution_times.iter().sum::<u64>() / 1000u64,
    };

    log::info!("{}", serde_json::to_string(&output)?);
    Ok(output)
}

fn h_err(s: String) -> HandlerError {
    HandlerError::from(s.as_str())
}
