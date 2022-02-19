mod chronicler;
mod chronicler_schema;
mod game_states;
mod render;

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket::State;
use rocket::http::{ContentType, Status};
use rocket::serde::uuid::Uuid;
use rocket::tokio::sync::Mutex;
use rocket::response::{content, status};

use crate::game_states::GameStates;
use crate::render::render;

type TeamMap = HashMap<Uuid, GameStates>;
type TeamMapSync = Mutex<TeamMap>;

#[get("/")]
fn index() -> &'static str {
    "Visit /<team_id> to get a game update for your team"
}

#[get("/<team_id>")]
async fn game_img(team_map: &State<TeamMapSync>, team_id: Uuid) -> Result<content::Custom<Vec<u8>>, (Status, String)> {
    let mut team_map = team_map.lock().await;
    let game_update = team_map.entry(team_id)
        .or_insert_with_key(|uuid| GameStates::new(*uuid))
        .next().await;

    match game_update {
        Some(update) => {
            match render(update) {
                Ok(buf) => Ok(content::Custom(ContentType::PNG, buf)),
                Err(err) => {
                    Err((Status::InternalServerError, format!("{:?}", err)))
                }
            }
        },
        None => {
            Err((Status::NotFound, format!("There are no games for team {}", team_id)))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(TeamMap::new()))
        .mount("/", routes![index, game_img])
}