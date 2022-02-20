use chrono::{DateTime, Utc};
use rocket::serde::Deserialize;
use rocket::serde::uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameUpdateData {
    #[serde(alias = "_id")]
    pub id: Uuid,
    pub top_of_inning: bool,
    pub home_bases: Option<i32>,
    pub home_balls: Option<i32>,
    pub home_strikes: Option<i32>,
    pub home_outs: Option<i32>,
    pub away_bases: Option<i32>,
    pub away_balls: Option<i32>,
    pub away_strikes: Option<i32>,
    pub away_outs: Option<i32>,
    pub bases_occupied: Vec<i32>,
    pub at_bat_balls: i32,
    pub at_bat_strikes: i32,
    pub half_inning_outs: i32,
    pub home_batter_name: Option<String>,
    pub home_pitcher_name: Option<String>,
    pub away_batter_name: Option<String>,
    pub away_pitcher_name: Option<String>,
    pub home_team_color: Option<String>,
    pub away_team_color: Option<String>,
    pub last_update: String,
}

impl GameUpdateData {
    pub fn num_bases(&self) -> i32 {
        if self.top_of_inning {
            self.away_bases.unwrap_or(4)
        } else {
            self.home_bases.unwrap_or(4)
        }
    }
    pub fn max_outs(&self) -> i32 {
        if self.top_of_inning {
            self.away_outs.unwrap_or(3)
        } else {
            self.home_outs.unwrap_or(3)
        }
    }
    pub fn max_balls(&self) -> i32 {
        if self.top_of_inning {
            self.away_balls.unwrap_or(4)
        } else {
            self.home_balls.unwrap_or(4)
        }
    }
    pub fn max_strikes(&self) -> i32 {
        if self.top_of_inning {
            self.away_strikes.unwrap_or(3)
        } else {
            self.home_strikes.unwrap_or(3)
        }
    }
    pub fn current_batter_name(&self) -> &Option<String> {
        if self.top_of_inning {
            &self.away_batter_name
        } else {
            &self.home_batter_name
        }
    }
    pub fn current_pitcher_name(&self) -> &Option<String> {
        if self.top_of_inning {
            &self.home_pitcher_name
        } else {
            &self.away_pitcher_name
        }
    }
    pub fn batting_team_color(&self) -> &Option<String> {
        if self.top_of_inning {
            &self.away_team_color
        } else {
            &self.home_team_color
        }
    }
    pub fn pitching_team_color(&self) -> &Option<String> {
        if self.top_of_inning {
            &self.home_team_color
        } else {
            &self.away_team_color
        }
    }
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