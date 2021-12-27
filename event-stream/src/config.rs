use serde_derive::{Serialize, Deserialize};
use crate::payload::PlayGameEvent;

const LICHESS_AUTH_TOKEN_VAR: &'static str = "MYOPIC_LICHESS_AUTH_TOKEN";
const CONFIG_VAR: &'static str = "MYOPIC_CONFIG";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(rename = "gameFunction")]
    pub game_function: GameFunctionConfig,
    #[serde(rename = "moveFunction")]
    pub move_function: AwsResourceId,
    #[serde(rename = "openingTable")]
    pub opening_table: OpeningTableConfig,
    #[serde(rename = "lichessBot")]
    pub lichess_bot: LichessConfig,
    #[serde(rename = "timeConstraints", default)]
    pub time_constraints: TimeConstraints,
    #[serde(rename = "eventLoop", default)]
    pub event_loop: EventLoopConfig,
    #[serde(rename = "challengeServerAddress", default = "default_server_address")]
    pub challenge_server_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LichessConfig {
    #[serde(rename = "botId")]
    pub bot_id: String,
    #[serde(rename = "authToken", default = "get_lichess_auth_token")]
    pub auth_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLoopConfig {
    #[serde(rename = "retryWaitDurationSecs")]
    pub retry_wait_duration_secs: u32,
    #[serde(rename = "statusPollGapSecs")]
    pub status_poll_gap_secs: u32,
    #[serde(rename = "maxStreamLifeMins")]
    pub max_stream_life_mins: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameFunctionConfig {
    pub id: AwsResourceId,
    #[serde(rename = "maxDepth")]
    pub max_depth: u8,
    #[serde(rename = "abortAfterSecs")]
    pub abort_after_secs: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningTableConfig {
    pub id: AwsResourceId,
    #[serde(rename = "positionKey", default = "default_position_key")]
    pub position_key: String,
    #[serde(rename = "moveKey", default = "default_moves_key")]
    pub move_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsResourceId {
    pub name: String,
    #[serde(default = "default_region")]
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConstraints {
    #[serde(rename = "minInitialTimeSecs")]
    pub min_initial_time_secs: u32,
    #[serde(rename = "maxInitialTimeSecs")]
    pub max_initial_time_secs: u32,
    #[serde(rename = "minIncrementSecs")]
    pub min_increment_secs: u32,
    #[serde(rename = "maxIncrementSecs")]
    pub max_increment_secs: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        let config = get_env_var(CONFIG_VAR);
        serde_json::from_str(config.as_str())
            .expect(format!("Could not parse config {}", config).as_str())
    }
}

impl Default for EventLoopConfig {
    fn default() -> Self {
        EventLoopConfig {
            retry_wait_duration_secs: 30,
            status_poll_gap_secs: 60,
            max_stream_life_mins: 300,
        }
    }
}

impl Default for TimeConstraints {
    fn default() -> Self {
        TimeConstraints {
            min_initial_time_secs: 60,
            max_initial_time_secs: 600,
            min_increment_secs: 0,
            max_increment_secs: 5,
        }
    }
}

fn get_lichess_auth_token() -> String {
    get_env_var(LICHESS_AUTH_TOKEN_VAR)
}

fn default_server_address() -> String {
    "0.0.0.0:8080".to_string()
}

fn default_region() -> String {
    "eu-west-2".to_string()
}

fn default_position_key() -> String {
    "PositionFEN".to_string()
}

fn default_moves_key() -> String {
    "Moves".to_string()
}

fn get_env_var(key: &str) -> String {
    std::env::var(key).expect(format!("Could not find env var \"{}\"", key).as_str())
}

pub fn extract_game_lambda_payload(config: &AppConfig, game_id: &str) -> String {
    serde_json::to_string(&PlayGameEvent {
        function_depth_remaining: config.game_function.max_depth,
        function_name: config.game_function.id.name.clone(),
        function_region: config.game_function.id.region.clone(),
        move_function_name: config.move_function.name.clone(),
        move_function_region: config.move_function.region.clone(),
        lichess_game_id: game_id.to_string(),
        lichess_auth_token: config.lichess_bot.auth_token.clone(),
        lichess_bot_id: config.lichess_bot.bot_id.clone(),
        opening_table_name: config.opening_table.id.name.clone(),
        opening_table_region: config.opening_table.id.region.clone(),
        opening_table_position_key: config.opening_table.position_key.clone(),
        opening_table_move_key: config.opening_table.move_key.clone(),
        abort_after_secs: config.game_function.abort_after_secs,
    }).unwrap()
}
