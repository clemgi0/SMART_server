use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::{delete, insert_into};
use crate::model::{PositionsHistory, ProtectedRes, Protection, Protector};
use crate::schema::positions_history::{dsl::positions_history, protected_id as history_protected_id};
use crate::schema::protected::dsl::protected;
use crate::schema::protected::id;
use crate::schema::protection::{dsl::protection, protector_id as protection_protector_id, protected_id as protection_protected_id};
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

pub fn insert_protected() -> ProtectedRes {
    let connection = &mut establish_connection();
    insert_into(protected)
        .default_values()
        .returning(ProtectedRes::as_select())
        .get_result(connection)
        .expect("Erreur insertion Protected")
}

pub fn insert_protection(id_protector: i32, id_protected: i32, name: &str) {
    let connection = &mut establish_connection();
    let protection1 = Protection{
        protector_id: id_protector,
        protected_id: id_protected,
        protected_name: &*name,
    };
    insert_into(protection)
        .values(protection1)
        .execute(connection)
        .expect("Erreur insertion Protection");
}

pub fn insert_positions_history(latitude: f32, longitude:f32, id_protected: i32) {
    let connection = &mut establish_connection();
    let positions_history1 = PositionsHistory{
        latitude,
        longitude,
        protected_id: id_protected,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
    };
    insert_into(positions_history)
        .values(positions_history1)
        .execute(connection)
        .expect("Erreur insertion PositionsHistory");
}

pub fn get_positions_history(id_protected: i32) -> Vec<PositionsHistory>{
    let connection = &mut establish_connection();
    positions_history
        .select(PositionsHistory::as_select())
        .filter(history_protected_id.eq(id_protected))
        .load(connection)
        .expect("Erreur récupération historique des positions")
}

pub fn protection_exists(id_protector: i32, id_protected: i32) -> bool {
    let connection = &mut establish_connection();
    protection
        .filter(protection_protected_id.eq(id_protected).and(protection_protector_id.eq(id_protector)))
        .count()
        .get_result::<i64>(connection)
        .expect("Erreur vérification protection")
        .gt(&0)
}

pub fn protected_exists(id_protected: i32) -> bool {
    let connection = &mut establish_connection();
    protected
        .filter(id.eq(id_protected))
        .count()
        .get_result::<i64>(connection)
        .expect("Erreur vérification protected")
        .gt(&0)
}

pub fn delete_protection(id_protector: i32, id_protected: i32) {
    let connection = &mut establish_connection();
    let _ = delete(protection.filter(protection_protector_id.eq(id_protector).and(protection_protected_id.eq(id_protected)))).execute(connection);
}