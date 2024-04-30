use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::insert_into;
use crate::model::{PositionsHistory, Protection, Protector};
use crate::schema::positions_history::dsl::positions_history;
use crate::schema::protected::dsl::protected;
use crate::schema::protection::dsl::protection;
use crate::schema::protector::dsl::protector;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_protector(protector1: Protector) {
    let connection = &mut establish_connection();
    insert_into(protector).values(protector1).execute(connection).expect("Erreur insertion Protector");
}

pub fn insert_protected() {
    let connection = &mut establish_connection();
    insert_into(protected).default_values().execute(connection).expect("Erreur insertion Protected");
}

pub fn insert_protection(id_protector: i32, id_protected: i32, name: &str) {
    let connection = &mut establish_connection();
    let protection1 = Protection{
        protector_id: id_protector,
        protected_id: id_protected,
        protected_name: &*name,
    };
    insert_into(protection).values(protection1).execute(connection).expect("Erreur insertion Protection");
}

pub fn insert_positions_history(latitude: f32, longitude:f32, id_protected: i32) {
    let connection = &mut establish_connection();
    let positions_history1 = PositionsHistory{
        latitude,
        longitude,
        protected_id: id_protected,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
    };
    insert_into(positions_history).values(positions_history1).execute(connection).expect("Erreur insertion PositionsHistory");
}
