use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Run {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub timestamp: DateTime,
    pub game_ids: Vec<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Odds {
    pub sitename: String,
    pub odds: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub game_type: String,
    #[serde(rename = "match")]
    pub match_info: String,
    pub url: String,
    pub timestamp: DateTime,
    pub odds: std::collections::HashMap<String, Odds>,
}