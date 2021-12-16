use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GameEvent {
    #[serde(rename = "gameFull")]
    GameFull {
        #[serde(flatten)]
        content: GameFull,
    },
    #[serde(rename = "gameState")]
    State {
        #[serde(flatten)]
        content: GameState,
    },
    #[serde(rename = "chatLine")]
    ChatLine {
        #[serde(flatten)]
        content: ChatLine,
    },
}

// Make all fields optional for now, don't want
// anything to break due to bad deserialization
// of a chat event
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ChatLine {
    pub username: Option<String>,
    pub text: Option<String>,
    pub room: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct GameFull {
    pub white: Player,
    pub black: Player,
    pub clock: Clock,
    pub state: GameState,
    #[serde(rename = "initialFen")]
    pub initial_fen: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct GameState {
    pub moves: String,
    pub wtime: u64,
    pub btime: u64,
    pub winc: u64,
    pub binc: u64,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Player {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Clock {
    pub initial: u64,
    pub increment: u64,
}

#[cfg(test)]
mod test {
    use serde_json;

    use super::*;

    #[test]
    fn deserialize_chat_line() {
        let json = r#"{
            "type": "chatLine",
            "username": "th0masb",
            "text": "Hello there",
            "room": null
        }"#;

        match serde_json::from_str::<GameEvent>(json) {
            Err(error) => panic!("Parse error {:?}", error),
            Ok(event) => match event {
                GameEvent::ChatLine { content } => assert_eq!(
                    ChatLine {
                        username: Some("th0masb".to_owned()),
                        text: Some("Hello there".to_owned()),
                        room: None,
                    },
                    content
                ),
                _ => panic!("Wrong event {:?}", event),
            },
        }
    }

    #[test]
    fn deserialize_state() {
        let json = r#"{
            "type": "gameState",
            "moves": "e2e4 c7c5",
            "wtime": 1000,
            "btime": 1000,
            "winc": 0,
            "binc": 0,
            "wdraw": false,
            "bdraw": false,
            "status": "started",
            "other": "value"
        }"#;

        match serde_json::from_str::<GameEvent>(json) {
            Err(error) => panic!("Parse error {:?}", error),
            Ok(event) => match event {
                GameEvent::State { content: state } => assert_eq!(
                    GameState {
                        moves: String::from("e2e4 c7c5"),
                        wtime: 1000,
                        btime: 1000,
                        winc: 0,
                        binc: 0,
                        status: String::from("started")
                    },
                    state
                ),
                _ => panic!("Wrong event {:?}", event),
            },
        }
    }

    #[test]
    fn deserialize_game_full() {
        let json = r#"{
            "type": "gameFull",
            "id": "123",
            "other": "value",
            "white": {
                "id": "th0masb",
                "name": "th0masb",
                "title": null,
                "rating": 1500,
                "provisional": true,
                "other": "value"
            },
            "black": {
                "id": "myopic-bot",
                "name": "myopic-bot",
                "title": "BOT",
                "rating": 1500,
                "provisional": true
            },
            "clock": {
                "initial": 1200000,
                "increment": 10000
            },
            "state": {
                "moves": "e2e4 e7e5",
                "wtime": 1000,
                "btime": 1000,
                "winc": 0,
                "binc": 0,
                "wdraw": false,
                "bdraw": false,
                "status": "started"
            },
            "initialFen": "startpos"
        }"#;

        match serde_json::from_str::<GameEvent>(json) {
            Err(error) => panic!("Parse error {:?}", error),
            Ok(event) => match event {
                GameEvent::GameFull { content } => {
                    assert_eq!(
                        Player {
                            id: format!("th0masb")
                        },
                        content.white
                    );
                    assert_eq!(
                        Player {
                            id: format!("myopic-bot")
                        },
                        content.black
                    );
                    assert_eq!(
                        Clock {
                            initial: 1200000,
                            increment: 10000
                        },
                        content.clock
                    );
                    assert_eq!(
                        GameState {
                            moves: String::from("e2e4 e7e5"),
                            wtime: 1000,
                            btime: 1000,
                            winc: 0,
                            binc: 0,
                            status: String::from("started")
                        },
                        content.state
                    );
                }
                _ => panic!("Wrong type {:?}", event),
            },
        }
    }
}
