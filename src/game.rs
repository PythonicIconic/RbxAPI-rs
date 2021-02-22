use serde::{Serialize, Deserialize};
use async_trait::async_trait;

#[async_trait]
pub trait GameBuilder {
    async fn new(self, client: reqwest::Client) -> Game;
}

#[async_trait]
impl GameBuilder for i32 {
    async fn new(self, client: reqwest::Client) -> Game {
        let data = client.get(&format!("{}/games/multiget-place-details?placeIds={}", crate::api::GAMES, self))
            .send().await.expect("Failed to get game universe info")
            .json::<serde_json::Value>().await.expect("Failed to get game universe json");

        let data = client.get(&format!("{}/games?universeIds={}", crate::api::GAMES, data[0].get("universeId").expect("Failed to find game universe ID")))
                .send().await.expect("Failed to get game root info")
                .json::<serde_json::Value>().await.expect("Failed to get game root json");

        Game {
            auth: client,
            ..serde_json::from_value(data.get("data").expect("Failed to get game root data")[0].clone()).expect("Failed to parse into Game")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    #[serde(skip)]
    auth: reqwest::Client,

    #[serde(rename="id")]
    pub universe_id: u64,
    #[serde(rename="rootPlaceId")]
    pub place_id: u64,
    pub name: String,
    pub description: String,
    pub price: Option<u64>,
    #[serde(rename="allowedGearGenres")]
    pub allowed_gear_genres: Vec<String>,
    #[serde(rename="allowedGearCategories")]
    pub allowed_gear_categories: Vec<String>,
    pub playing: u32,
    pub visits: u64,
    #[serde(rename="maxPlayers")]
    pub max_players: u8,
    pub created: String,
    pub updated: String,
    #[serde(rename="studioAccessToApisAllowed")]
    pub studio_access_to_apis_allowed: bool,
    #[serde(rename="createVipServersAllowed")]
    pub create_vip_servers_allowed: bool,
    #[serde(rename="universeAvatarType")]
    pub universe_avatar_type: String,
    pub genre: String
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Game(placeid={}, name={})", self.place_id, self.name)
    }
}
