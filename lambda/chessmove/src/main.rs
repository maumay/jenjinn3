use std::fmt::Display;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log;
use simple_logger::SimpleLogger;

use lambda_payloads::chessmove::*;
use myopic_brain::{anyhow, ChessBoard, EvalBoard, Move, SearchParameters};

use crate::endings::LichessEndgameService;
use crate::openings::DynamoOpeningService;
use crate::timing::TimeAllocator;

mod endings;
mod openings;
mod timing;

const TABLE_SIZE: usize = 10000;

#[async_trait]
pub trait LookupMoveService<B: ChessBoard + Send>: Display {
    async fn lookup(&self, position: B) -> anyhow::Result<Option<Move>>;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()?;
    lambda_runtime::run(service_fn(move_handler)).await?;
    Ok(())
}

async fn move_handler(event: LambdaEvent<ChooseMoveEvent>) -> Result<ChooseMoveOutput, Error> {
    let choose_move = &event.payload;
    let start = Instant::now();
    // Setup the current game position
    let mut board = EvalBoard::default();
    board.play_uci(choose_move.moves_played.as_str())?;

    let lookup_services = load_lookup_services(&choose_move.features);
    match perform_lookups(board.clone_position(), lookup_services).await {
        Some(mv) => Ok(ChooseMoveOutput {
            best_move: mv.uci_format(),
            search_details: None,
        }),
        None => {
            let lookup_duration = start.elapsed();
            let search_time = TimeAllocator::default().allocate(
                board.position_count(),
                Duration::from_millis(choose_move.clock_millis.remaining) - lookup_duration,
                Duration::from_millis(choose_move.clock_millis.increment),
            );
            let search_outcome = myopic_brain::search(
                board,
                SearchParameters {
                    terminator: search_time,
                    table_size: TABLE_SIZE,
                },
            )?;
            Ok(ChooseMoveOutput {
                best_move: search_outcome.best_move.uci_format(),
                search_details: Some(SearchDetails {
                    depth_searched: search_outcome.depth,
                    search_duration_millis: search_outcome.time.as_millis() as u64,
                    eval: search_outcome.eval,
                }),
            })
        }
    }
}

fn load_lookup_services<B>(features: &Vec<ChooseMoveFeature>) -> Vec<Box<dyn LookupMoveService<B>>>
where
    B: 'static + ChessBoard + Clone + Send,
{
    let mut services: Vec<Box<dyn LookupMoveService<B>>> = vec![];
    if !features.contains(&ChooseMoveFeature::DisableOpeningsLookup) {
        services.push(Box::new(DynamoOpeningService::default()));
    }
    if !features.contains(&ChooseMoveFeature::DisableEndgameLookup) {
        services.push(Box::new(LichessEndgameService::default()));
    }
    services
}

/// Attempt to lookup precomputed moves from various sources
async fn perform_lookups<B>(
    position: B,
    services: Vec<Box<dyn LookupMoveService<B>>>,
) -> Option<Move>
where
    B: 'static + ChessBoard + Clone + Send,
{
    for service in services.iter() {
        match service.lookup(position.clone()).await {
            Ok(None) => {
                log::info!("{} could not find a move for this position", service);
            }
            Err(e) => {
                log::error!("Error during lookup for {}: {}", service, e);
            }
            Ok(Some(mv)) => {
                log::info!("{} retrieved {}", service, mv);
                return Some(mv);
            }
        }
    }
    None
}
