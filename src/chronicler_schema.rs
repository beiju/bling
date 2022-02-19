use chrono::{DateTime, Utc};
use rocket::serde::Deserialize;
use rocket::serde::uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameUpdateData {
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerGameUpdate {
    pub game_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub data: GameUpdateData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerGameUpdatesResponse {
    pub next_page: Option<String>,
    pub data: Vec<ChroniclerGameUpdate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerGame {
    pub game_id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerGameResponse {
    pub next_page: Option<String>,
    pub data: Vec<ChroniclerGame>,
}