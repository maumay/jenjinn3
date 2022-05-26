use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use async_trait::async_trait;
use itertools::Itertools;
use log::info;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};

use crate::LookupMoveService;
use myopic_brain::anyhow as ah;
use myopic_brain::{ChessBoard, FenPart, Move};

const TABLE_ENV_KEY: &'static str = "APP_CONFIG";

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct OpeningTable {
    pub name: String,
    pub region: String,
    #[serde(rename = "positionKey")]
    pub position_key: String,
    #[serde(rename = "moveKey")]
    pub move_key: String,
    #[serde(rename = "maxDepth")]
    pub max_depth: u8,
}

pub struct DynamoOpeningService {
    params: OpeningTable,
    client: DynamoDbClient,
}

impl Default for DynamoOpeningService {
    fn default() -> Self {
        let table_var = std::env::var(TABLE_ENV_KEY)
            .expect(format!("No value found for env var {}", TABLE_ENV_KEY).as_str());
        let table = serde_json::from_str::<OpeningTable>(table_var.as_str())
            .expect(format!("Could not parse table config {}", table_var).as_str());
        let region = Region::from_str(table.region.as_str())
            .expect(format!("Could not parse {} as region", table.region).as_str());
        DynamoOpeningService {
            params: table,
            client: DynamoDbClient::new(region),
        }
    }
}

impl Display for DynamoOpeningService {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.params.name.as_str())
    }
}

#[async_trait]
impl<B: ChessBoard + Send + 'static> LookupMoveService<B> for DynamoOpeningService {
    async fn lookup(&self, position: B) -> ah::Result<Option<Move>> {
        let pos_count = position.position_count();
        if pos_count > self.params.max_depth as usize {
            info!("No lookup as {} > {}", pos_count, self.params.max_depth);
            Ok(None)
        } else {
            let fen =
                position.to_fen_parts(&[FenPart::Board, FenPart::Active, FenPart::CastlingRights]);
            info!("Querying table {} for position {}", self.params.name, fen);
            self.client
                .get_item(self.create_request(fen))
                .await
                .map_err(|err| ah::anyhow!("{}", err))
                .and_then(|response| match response.item {
                    None => {
                        info!("No match found!");
                        Ok(None)
                    }
                    Some(attributes) => self
                        .try_extract_move(attributes)
                        .and_then(|mv| position.parse_uci(mv.as_str()))
                        .map(|mv| {
                            info!("Found opening move {}", mv.uci_format());
                            Some(mv)
                        }),
                })
        }
    }
}
impl DynamoOpeningService {
    fn create_request(&self, query_position: String) -> GetItemInput {
        // Create key
        let mut av = AttributeValue::default();
        av.s = Some(query_position);
        let mut key = HashMap::new();
        key.insert(self.params.position_key.clone(), av);
        // Create request
        let mut request = GetItemInput::default();
        request.table_name = self.params.name.clone();
        request.key = key;
        request
    }

    fn try_extract_move(&self, attributes: HashMap<String, AttributeValue>) -> ah::Result<String> {
        match attributes.get(&self.params.move_key) {
            None => Err(ah::anyhow!(
                "Position exists but missing recommended move attribute"
            )),
            Some(attribute) => match &attribute.ss {
                None => Err(ah::anyhow!(
                    "Position and recommended move attribute exist but not string set type"
                )),
                Some(move_set) => {
                    info!("Found matching set {:?}!", move_set);
                    let chosen = choose_move(move_set, rand::random)?;
                    info!("Chose {} from set", &chosen);
                    Ok(chosen)
                }
            },
        }
    }
}

fn choose_move(available: &Vec<String>, f: impl Fn() -> usize) -> ah::Result<String> {
    let records = available
        .into_iter()
        .filter_map(|s| MoveRecord::from_str(s.as_str()).ok())
        .sorted_by_key(|r| r.freq)
        .collect::<Vec<_>>();

    let frequency_sum = records.iter().map(|r| r.freq).sum::<usize>();

    if frequency_sum == 0 {
        Err(ah::anyhow!("Freq is 0 for {:?}", available))
    } else {
        let record_choice = f() % frequency_sum;
        let mut sum = 0usize;
        for record in records {
            if sum <= record_choice && record_choice < sum + record.freq {
                return Ok(record.mv);
            }
            sum += record.freq;
        }
        panic!("Failed to choose move {:?}", available)
    }
}

const MOVE_FREQ_SEPARATOR: &'static str = ":";

struct MoveRecord {
    mv: String,
    freq: usize,
}

impl FromStr for MoveRecord {
    type Err = ah::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .split(MOVE_FREQ_SEPARATOR)
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Ok(MoveRecord {
            mv: split
                .get(0)
                .ok_or(ah::anyhow!("Cannot parse move from {}", s))?
                .clone(),
            freq: split
                .get(1)
                .ok_or(ah::anyhow!("Cannot parse freq from {}", s))?
                .parse::<usize>()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::choose_move;

    #[test]
    fn test_choose_move() {
        let choices = vec![
            format!("a2a3:1"),
            format!("b2b4:1"),
            format!("g8f6:3"),
            format!("e1g1:20"),
        ];

        assert_eq!(format!("a2a3"), choose_move(&choices, || { 0 }).unwrap());
        assert_eq!(format!("b2b4"), choose_move(&choices, || { 1 }).unwrap());

        for i in 2..5 {
            assert_eq!(format!("g8f6"), choose_move(&choices, || { i }).unwrap());
        }

        for i in 5..25 {
            assert_eq!(format!("e1g1"), choose_move(&choices, || { i }).unwrap());
        }

        assert_eq!(format!("a2a3"), choose_move(&choices, || { 25 }).unwrap());
    }
}
