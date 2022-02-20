use std::pin::Pin;
use rocket::futures::{Stream, StreamExt};
use rocket::serde::uuid::Uuid;
use crate::chronicler;
use crate::chronicler_schema::GameUpdateData;

pub struct GameStates {
    team: Uuid,
    stream: Pin<Box<dyn Stream<Item=GameUpdateData> + Send + Sync>>,
    started: bool,
}

impl GameStates {
    pub fn new(team: Uuid) -> GameStates {
        GameStates {
            team,
            stream: Box::pin(chronicler::game_updates_for_team(team)),
            started: false,
        }
    }

    pub async fn next(&mut self) -> Option<GameUpdateData> {
        while let Some(update) = self.next_inner().await {
            if update.last_update != "Game over." {
                return Some(update);
            }
            info!("Skipping Game Over");
        }

        None
    }

    pub async fn next_inner(&mut self) -> Option<GameUpdateData> {
        match self.stream.next().await {
            Some(update) => {
                self.started = true;
                Some(update)
            }
            None => {
                if !self.started {
                    None
                } else {
                    self.stream = Box::pin(chronicler::game_updates_for_team(self.team));
                    self.stream.next().await
                }
            }
        }
    }
}