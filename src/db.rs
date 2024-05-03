use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::{delete, insert_into, update};
use crate::model::{PositionsHistory, ProtectedRes, Protection, Protector, ProtectorRes};
use crate::schema::positions_history::{dsl::positions_history, protected_id as hst_protected_id};
use crate::schema::protected::{dsl::protected, id as pd_id, status};
use crate::schema::protection::{dsl::protection, protector_id as pn_protector_id, protected_id as pn_protected_id};
use crate::schema::protector::{dsl::protector, id as pr_id};

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

pub fn insert_protection(id_protector: i32, id_protected: i32, name: String) {
    let connection = &mut establish_connection();
    let protection1 = Protection{
        protector_id: id_protector,
        protected_id: id_protected,
        protected_name: name,
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
        .filter(hst_protected_id.eq(id_protected))
        .load(connection)
        .expect("Erreur récupération historique des positions")
}

pub fn protection_exists(id_protector: i32, id_protected: i32) -> bool {
    let connection = &mut establish_connection();
    protection
        .filter(pn_protected_id.eq(id_protected).and(pn_protector_id.eq(id_protector)))
        .count()
        .get_result::<i64>(connection)
        .expect("Erreur vérification protection")
        .gt(&0)
}

pub fn protected_exists(id_protected: i32) -> bool {
    let connection = &mut establish_connection();
    protected
        .filter(pd_id.eq(id_protected))
        .count()
        .get_result::<i64>(connection)
        .expect("Erreur vérification protected")
        .gt(&0)
}

pub fn delete_protection(id_protector: i32, id_protected: i32) {
    let connection = &mut establish_connection();
    let _ = delete(protection.filter(pn_protector_id.eq(id_protector).and(pn_protected_id.eq(id_protected)))).execute(connection);
}

pub fn update_protected_status(id_protected: i32, new_status: i32) {
    let connection = &mut establish_connection();
    update(protected).set(status.eq(new_status)).filter(pd_id.eq(id_protected)).execute(connection).expect("Erreur changement statut protected");
}

pub fn get_protector(id_protector: i32) -> ProtectorRes {
    let connection = &mut establish_connection();
    protector
        .select(ProtectorRes::as_select())
        .filter(pr_id.eq(id_protector))
        .get_result::<ProtectorRes>(connection)
        .expect("Erreur récupération protector")
}

pub fn get_protections(id_protected: i32) -> Vec<Protection>{
    let connection = &mut establish_connection();
    protection
        .select(Protection::as_select())
        .filter(pn_protected_id.eq(id_protected))
        .load(connection)
        .expect("Erreur récupération protections")
}