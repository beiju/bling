use rocket::futures::{Stream, stream, StreamExt};
use rocket::serde::uuid::Uuid;

use crate::chronicler_schema::{ChroniclerGame, ChroniclerGameResponse, ChroniclerGameUpdate, ChroniclerGameUpdatesResponse, GameUpdateData};

pub fn game_updates_for_team(_team: Uuid) -> impl Stream<Item=GameUpdateData> {
    // For testing
    game_updates_for_game(Uuid::parse_str("4cc78f38-36b7-4282-baa6-21ff46a41e56").unwrap())
    // games_for_team(team).flat_map(game_updates_for_game)
}

pub fn games_for_team(team: Uuid) -> impl Stream<Item=Uuid> {
    games_for_team_pages(team)
        .flat_map(|vec| stream::iter(vec.into_iter()))
        .map(|game| game.game_id)
}

pub fn game_updates_for_game(game_id: Uuid) -> impl Stream<Item=GameUpdateData> {
    game_update_pages(game_id)
        .flat_map(|vec| stream::iter(vec.into_iter()))
        .map(|game| game.data)
}

struct ChronState {
    pub page: Option<String>,
    pub stop: bool,
    pub client: reqwest::Client,
}

fn games_for_team_pages(team: Uuid) -> impl Stream<Item=Vec<ChroniclerGame>> {
    let start_state = ChronState {
        page: None,
        stop: false,
        client: reqwest::Client::new(),
    };


    stream::unfold(start_state, move |state| async move {
        if state.stop {
            None
        } else {
            Some(games_for_team_page(team, state).await)
        }
    })
}

async fn games_for_team_page(team_id: Uuid, state: ChronState) -> (Vec<ChroniclerGame>, ChronState) {
    let request = state.client
        .get("https://api.sibr.dev/chronicler/v1/games")
        .query(&[("team", &team_id)])
        .query(&[("order", "asc")]);

    let request = match state.page {
        Some(page) => {
            info!("Fetching page {} of games for team {}", page, team_id);
            request.query(&[("page", &page)])
        },
        None => {
            info!("Fetching first page of games for team {}", team_id);
            request }
    };

    let request = request.build().unwrap();

    let response = state.client
                .execute(request).await
                .expect("Chronicler API call failed")
                .text().await
                .expect("Chronicler text decode failed");

    let (response_data, next_page) =  {
        let response: ChroniclerGameResponse = serde_json::from_str(&response).unwrap();
        (response.data, response.next_page)
    };

    let stop = next_page.is_none();
    (response_data, ChronState {
        page: next_page,
        stop,
        client: state.client
    })
}

fn game_update_pages(game_id: Uuid) -> impl Stream<Item=Vec<ChroniclerGameUpdate>> {
    let start_state = ChronState {
        page: None,
        stop: false,
        client: reqwest::Client::new(),
    };


    stream::unfold(start_state, move |state| async move {
        if state.stop {
            None
        } else {
            Some(game_update_page(game_id, state).await)
        }
    })
}

async fn game_update_page(game_id: Uuid, state: ChronState) -> (Vec<ChroniclerGameUpdate>, ChronState) {
    let request = state.client
        .get("https://api.sibr.dev/chronicler/v1/games/updates")
        .query(&[("id", &game_id)])
        .query(&[("order", "asc")]);

    let request = match state.page {
        Some(page) => {
            info!("Fetching page {} of updates in game {}", page, game_id);
            request.query(&[("page", &page)])
        },
        None => {
            info!("Fetching first page of updates in game {}", game_id);
            request }
    };

    let request = request.build().unwrap();

    let response = state.client
                .execute(request).await
                .expect("Chronicler API call failed")
                .text().await
                .expect("Chronicler text decode failed");

    let (response_data, next_page) =  {
        let response: ChroniclerGameUpdatesResponse = serde_json::from_str(&response).unwrap();
        (response.data, response.next_page)
    };

    let stop = next_page.is_none();
    (response_data, ChronState {
        page: next_page,
        stop,
        client: state.client
    })
}
