use std::collections::HashMap;

use anyhow::{anyhow, Result};
use reqwest::{blocking, StatusCode};

use crate::game::GameExecutionState;

const GAME_ENDPOINT: &'static str = "https://lichess.org/api/bot/game";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LichessChatRoom {
    Player,
    Spectator,
}

#[derive(Debug)]
pub struct LichessService {
    client: blocking::Client,
    auth_token: String,
    game_id: String,
}

impl LichessService {
    pub fn new(auth_token: String, game_id: String) -> LichessService {
        LichessService {
            client: blocking::Client::new(),
            auth_token,
            game_id,
        }
    }

    pub fn abort(&self) -> Result<StatusCode> {
        self.client
            .post(format!("{}/{}/abort", GAME_ENDPOINT, self.game_id).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .map_err(|error| anyhow!("Error aborting game: {}", error))
            .map(|response| response.status())
    }

    pub fn post_move(&self, mv: String) -> Result<GameExecutionState> {
        // Add timeout and retry logic
        self.client
            .post(format!("{}/{}/move/{}", GAME_ENDPOINT, self.game_id, mv).as_str())
            .bearer_auth(&self.auth_token)
            .send()
            .map_err(|error| anyhow!("Error posting move: {}", error))
            .and_then(|response| {
                if response.status().is_success() {
                    Ok(GameExecutionState::Running)
                } else {
                    Err(anyhow!(
                        "Lichess api responded with error {} during move post",
                        response.status()
                    ))
                }
            })
    }

    pub fn post_chatline(&self, text: &str, room: LichessChatRoom) -> Result<StatusCode> {
        let mut params = HashMap::new();
        params.insert("room", match room {
            LichessChatRoom::Player => "player",
            LichessChatRoom::Spectator => "spectator"
        });
        params.insert("text", text);
        self.client
            .post(format!("{}/{}/chat", GAME_ENDPOINT, self.game_id).as_str())
            .bearer_auth(&self.auth_token)
            .form(&params)
            .send()
            .map_err(|error| anyhow!("Error posting chatline: {}", error))
            .map(|response| response.status())
    }
}
