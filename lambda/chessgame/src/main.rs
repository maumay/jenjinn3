use std::error::Error;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::str::FromStr;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use lambda_runtime::{Context, error::HandlerError, lambda};
use reqwest::blocking::Response;
use rusoto_core::Region;
use rusoto_lambda::{InvokeAsyncRequest, Lambda, LambdaClient};
use serde_derive::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

use game::Game;

use crate::compute::LambdaMoveComputeService;
use crate::dynamodb::{DynamoDbOpeningService, DynamoDbOpeningServiceConfig};
use crate::endgame::EndgameService;
use crate::game::{GameConfig, GameExecutionState};

mod compute;
mod dynamodb;
mod endgame;
mod events;
mod game;
mod lichess;
mod messages;
pub mod position;
mod timing;

const GAME_STREAM_ENDPOINT: &'static str = "https://lichess.org/api/bot/game/stream";
type GameImpl = Game<DynamoDbOpeningService, LambdaMoveComputeService, EndgameService>;

/// The input payload of this lambda
#[derive(Serialize, Deserialize, Clone)]
struct PlayGameEvent {
    /// The current call depth of the lambda invokation
    #[serde(rename = "functionDepthRemaining")]
    function_depth_remaining: u8,
    /// The region this lambda is deployed in
    #[serde(rename = "functionRegion")]
    function_region: String,
    /// The name of this lambda function
    #[serde(rename = "functionName")]
    function_name: String,
    /// The lichess game id this lambda will participate in
    #[serde(rename = "lichessGameId")]
    lichess_game_id: String,
    /// An auth token for the lichess bot this lambda will play as
    #[serde(rename = "lichessAuthToken")]
    lichess_auth_token: String,
    /// The id of the lichess bot this lambda will play as
    #[serde(rename = "lichessBotId")]
    lichess_bot_id: String,
    /// The name of the dynamodb table used to store opening positions
    #[serde(rename = "openingTableName")]
    opening_table_name: String,
    /// The region in which the opening table is deployed
    #[serde(rename = "openingTableRegion")]
    opening_table_region: String,
    /// The name of the position key used as a pk in the opening table
    #[serde(rename = "openingTablePositionKey")]
    opening_table_position_key: String,
    /// The name of the move key used in the opening table
    #[serde(rename = "openingTableMoveKey")]
    opening_table_move_key: String,
    /// How many half moves we expect the game to last for
    #[serde(rename = "expectedHalfMoves")]
    expected_half_moves: u32,
    /// How many seconds to wait for the first full move to take place
    /// before aborting the game
    #[serde(rename = "abortAfterSecs")]
    abort_after_secs: usize,
}

impl PlayGameEvent {
    fn increment_depth(&self) -> PlayGameEvent {
        let mut new_event = self.clone();
        new_event.function_depth_remaining = self.function_depth_remaining - 1;
        new_event
    }
}

#[derive(Serialize, Clone)]
struct PlayGameOutput {
    message: String,
}

#[derive(Debug, Copy, Clone)]
pub struct TimeConstraints {
    start_time: Instant,
    max_execution_duration: Duration,
}

impl TimeConstraints {
    pub fn lambda_end_instant(&self) -> Instant {
        self.start_time.add(self.max_execution_duration)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    lambda!(game_handler);
    Ok(())
}

fn game_handler(e: PlayGameEvent, ctx: Context) -> Result<PlayGameOutput, HandlerError> {
    log::info!("Initializing game loop");
    let mut game = init_game(&e, &ctx)?;
    game.post_introduction();

    // Enter the game loop
    let (start, wait_duration) = (
        Instant::now(),
        Duration::from_secs(e.abort_after_secs as u64),
    );
    let mut should_recurse = false;
    for read_result in open_game_stream(&e.lichess_game_id, &e.lichess_auth_token)?.lines() {
        match read_result {
            Err(error) => {
                log::warn!("Problem reading from game stream {}", error);
                break;
            }
            Ok(event) => {
                if event.trim().is_empty() {
                    if game.halfmove_count() < 2 && start.elapsed() > wait_duration {
                        match game.abort() {
                            Err(message) => log::warn!("Failed to abort game: {}", message),
                            Ok(status) => {
                                if status.is_success() {
                                    log::info!("Successfully aborted game due to inactivity!");
                                    break;
                                } else {
                                    log::warn!("Failed to abort game, lichess status: {}", status)
                                }
                            }
                        }
                    } else if Instant::now() >= game.time_constraints().lambda_end_instant() {
                        should_recurse = true;
                        break;
                    }
                } else {
                    log::info!("Received event: {}", event);
                    match game
                        .process_event(event.as_str())
                        .map_err(|err| HandlerError::from(format!("{}", err).as_str()))?
                    {
                        GameExecutionState::Running => continue,
                        GameExecutionState::Finished => break,
                        GameExecutionState::Recurse => {
                            should_recurse = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    // If we got here then there isn't enough time in this lambda to complete the game
    if should_recurse && e.function_depth_remaining > 0 {
        // Async invoke lambda here
        recurse(&e)
    } else {
        Ok(PlayGameOutput {
            message: format!("No recursion required, game execution thread terminated"),
        })
    }
}

fn init_game(e: &PlayGameEvent, ctx: &Context) -> Result<GameImpl, HandlerError> {
    Ok(Game::new(
        GameConfig {
            game_id: e.lichess_game_id.clone(),
            bot_id: e.lichess_bot_id.clone(),
            expected_half_moves: e.expected_half_moves,
            lichess_auth_token: e.lichess_auth_token.clone(),
            time_constraints: TimeConstraints {
                start_time: Instant::now(),
                // Reduce the actual max duration by a constant fraction
                // to allow a buffer of time to invoke new lambda
                max_execution_duration: (4 * Duration::from_millis(
                    ctx.deadline as u64 - timestamp_millis(),
                )) / 5,
            },
        },
        DynamoDbOpeningService::new(DynamoDbOpeningServiceConfig {
            table_name: e.opening_table_name.clone(),
            position_key: e.opening_table_position_key.clone(),
            move_key: e.opening_table_move_key.clone(),
            table_region: parse_region(e.opening_table_region.as_str())?,
        }),
        LambdaMoveComputeService::default(),
        EndgameService::default(),
    ))
}

fn parse_region(region: &str) -> Result<Region, HandlerError> {
    Region::from_str(region).map_err(|err| HandlerError::from(format!("{}", err).as_str()))
}

pub fn timestamp_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

fn open_game_stream(
    game_id: &String,
    auth_token: &String,
) -> Result<BufReader<Response>, HandlerError> {
    reqwest::blocking::Client::new()
        .get(format!("{}/{}", GAME_STREAM_ENDPOINT, game_id).as_str())
        .bearer_auth(auth_token)
        .send()
        .map(|response| BufReader::new(response))
        .map_err(|err| HandlerError::from(format!("{}", err).as_str()))
}

fn recurse(source_event: &PlayGameEvent) -> Result<PlayGameOutput, HandlerError> {
    // Inject region as part of the PlayGameEvent
    let next_event = source_event.increment_depth();
    let region = parse_region(next_event.function_region.as_str())?;
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(LambdaClient::new(region).invoke_async(InvokeAsyncRequest {
            function_name: next_event.function_name.clone(),
            invoke_args: Bytes::from(serde_json::to_string(&next_event)?),
        }))
        .map_err(|err| HandlerError::from(err.to_string().as_str()))
        .map(|response| PlayGameOutput {
            message: format!(
                "Recursively invoked lambda at depth {} with status {:?}",
                next_event.function_depth_remaining, response.status
            ),
        })
}
