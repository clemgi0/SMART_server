use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::{delete, insert_into, update};
use crate::model::{Watcher, WatcherRes, Tracker, Monitoring, Position};
use crate::schema::position::{dsl::position, tracker_id as hst_tracker_id};
use crate::schema::tracker::{dsl::tracker, id as pd_id, status};
use crate::schema::monitoring::{dsl::monitoring, watcher_id as pn_watcher_id, tracker_id as pn_tracker_id};
use crate::schema::watcher::{dsl::watcher, id as pr_id};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_watcher(watcher1: Watcher) {
    let connection = &mut establish_connection();
    insert_into(watcher).values(watcher1).execute(connection).expect("Erreur insertion Watcher");
}

pub fn insert_tracker() -> Tracker {
    let connection = &mut establish_connection();
    insert_into(tracker)
        .default_values()
        .returning(Tracker::as_select())
        .get_result(connection)
        .expect("Erreur insertion Tracker")
}

pub fn insert_monitoring(id_watcher: i32, id_tracker: i32, name: String) {
    let connection = &mut establish_connection();
    let monitoring1 = Monitoring{
        watcher_id: id_watcher,
        tracker_id: id_tracker,
        tracker_name: name,
    };
    insert_into(monitoring)
        .values(monitoring1)
        .execute(connection)
        .expect("Erreur insertion Monitoring");
}

pub fn insert_position(latitude: f32, longitude:f32, id_tracker: i32) {
    let connection = &mut establish_connection();
    let position1 = Position {
        latitude,
        longitude,
        tracker_id: id_tracker,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
    };
    insert_into(position)
        .values(position1)
        .execute(connection)
        .expect("Erreur insertion PositionsHistory");
}

pub fn get_position(id_tracker: i32) -> Vec<Position>{
    let connection = &mut establish_connection();
    position
        .select(Position::as_select())
        .filter(hst_tracker_id.eq(id_tracker))
        .load(connection)
        .expect("Erreur récupération historique des positions")
}

pub fn monitoring_exists(id_watcher: i32, id_tracker: i32) -> bool {
    let connection = &mut establish_connection();
    monitoring
        .filter(pn_tracker_id.eq(id_tracker).and(pn_watcher_id.eq(id_watcher)))
        .count()
        .get_result::<i64>(connection)
        .expect("Erreur vérification monitoring")
        .gt(&0)
}

pub fn tracker_exists(id_tracker: i32) -> bool {
    let connection = &mut establish_connection();
    tracker
        .filter(pd_id.eq(id_tracker))
        .count()
        .get_result::<i64>(connection)
        .expect("Erreur vérification tracker")
        .gt(&0)
}

pub fn delete_monitoring(id_watcher: i32, id_tracker: i32) {
    let connection = &mut establish_connection();
    let _ = delete(monitoring.filter(pn_watcher_id.eq(id_watcher).and(pn_tracker_id.eq(id_tracker)))).execute(connection);
}

pub fn update_tracker_status(id_tracker: i32, new_status: i32) {
    let connection = &mut establish_connection();
    update(tracker).set(status.eq(new_status)).filter(pd_id.eq(id_tracker)).execute(connection).expect("Erreur changement statut tracker");
}

pub fn get_watcher(id_watcher: i32) -> WatcherRes {
    let connection = &mut establish_connection();
    watcher
        .select(WatcherRes::as_select())
        .filter(pr_id.eq(id_watcher))
        .get_result::<WatcherRes>(connection)
        .expect("Erreur récupération watcher")
}

pub fn get_monitorings(id_tracker: i32) -> Vec<Monitoring>{
    let connection = &mut establish_connection();
    monitoring
        .select(Monitoring::as_select())
        .filter(pn_tracker_id.eq(id_tracker))
        .load(connection)
        .expect("Erreur récupération monitorings")
}