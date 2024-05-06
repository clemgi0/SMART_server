use diesel::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use crate::schema::*;

#[derive(Queryable, Selectable, Insertable, Serialize, Clone)]
#[diesel(table_name = position)]
pub struct Position {
    pub latitude: f32,
    pub longitude: f32,
    pub tracker_id: i32,
    pub timestamp: i64,
}

#[derive(Deserialize)]
pub struct PositionRequest {
    pub tracker_id: i32,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = tracker)]
pub struct Tracker {
    pub id: i32,
    pub status: i32,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = tracker)]
pub struct TrackerInsert {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = watcher)]
pub struct Watcher {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub salt: Vec<u8>
}

#[derive(Insertable)]
#[diesel(table_name = watcher)]
pub struct WatcherInsert {
    pub login: String,
    pub password: String,
    pub salt: Vec<u8>
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = monitoring)]
#[diesel(belongs_to(Watcher))]
#[diesel(belongs_to(Tracker))]
pub struct Monitoring {
    pub watcher_id: i32,
    pub tracker_id: i32,
    pub tracker_name: String
}

#[derive(Deserialize)]
pub struct MonitoringRequest {
    pub watcher_id: i32,
    pub tracker_id: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SignupRequest {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub user_id: i32,
    pub success: bool,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user_id: i32,
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i32,
    pub exp: usize
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims
}