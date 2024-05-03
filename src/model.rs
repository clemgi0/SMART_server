use diesel::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use crate::schema::{positions_history, protected, protector, protection};

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = positions_history)]
pub struct PositionsHistory {
    pub latitude: f32,
    pub longitude: f32,
    pub protected_id: i32,
    pub timestamp: i64,
}

#[derive(Deserialize)]
pub struct PositionRequest {
    pub id_protected: i32,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = protected)]
pub struct Protected {
    pub id: i32,
    pub status: i32,
}

#[derive(Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = protector)]
pub struct ProtectorRes {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub salt: Vec<u8>
}

#[derive(Insertable)]
#[diesel(table_name = protector)]
pub struct Protector {
    pub login: String,
    pub password: String,
    pub salt: Vec<u8>
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = protection)]
#[diesel(belongs_to(Protected))]
#[diesel(belongs_to(Protector))]
pub struct Protection {
    pub protector_id: i32,
    pub protected_id: i32,
    pub protected_name: String
}

#[derive(Deserialize)]
pub struct ProtectionRequest {
    pub id_protector: i32,
    pub id_protected: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SignupRequest {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
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