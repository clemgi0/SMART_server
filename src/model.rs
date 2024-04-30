use diesel::prelude::*;
use serde::Serialize;

use crate::schema::{positions_history, protected, protector, protection};

#[derive(Queryable, Selectable, Insertable, Clone, Serialize)]
#[diesel(table_name = positions_history)]
pub struct PositionsHistoryRes {
    pub latitude: f32,
    pub longitude: f32,
    pub protected_id: i32,
    pub timestamp: i32,
}

#[derive(Queryable, Identifiable, Selectable, Clone, Serialize)]
#[diesel(table_name = protected)]
pub struct ProtectedRes {
    pub id: i32,
}

#[derive(Queryable, Identifiable, Selectable, Clone, Serialize)]
#[diesel(table_name = protector)]
pub struct ProtectorRes {
    pub id: i32,
    pub login: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = protector)]
pub struct Protector {
    pub login: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Insertable, Clone, Serialize)]
#[diesel(table_name = protection)]
#[diesel(belongs_to(Protected))]
#[diesel(belongs_to(Protector))]
pub struct ProtectionRes<'a> {
    pub protector_id: i32,
    pub protected_id: i32,
    pub protected_name: &'a str
}

