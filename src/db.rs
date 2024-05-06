use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::{delete, insert_into, update};
use crate::model::{Watcher, WatcherInsert, Tracker, TrackerInsert, Monitoring, Position, };
use crate::schema::watcher::{dsl::watcher, id as w_id, login};
use crate::schema::tracker::{dsl::tracker, id as t_id, status};
use crate::schema::monitoring::{dsl::monitoring, watcher_id as mw_id, tracker_id as mt_id};
use crate::schema::position::{dsl::position, tracker_id as p_id};


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_watcher(watcher1: WatcherInsert) {
    let connection = &mut establish_connection();
    insert_into(watcher)
        .values(watcher1)
        .execute(connection)
        .expect("Erreur insertion Watcher");
}

pub fn insert_tracker(latitude: f32, longitude: f32) -> Tracker {
    let connection = &mut establish_connection();
    let new_tracker = TrackerInsert {latitude, longitude};
    insert_into(tracker)
        .values(new_tracker)
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
        .expect("Erreur insertion Position");
}

pub fn get_position(id_tracker: i32) -> Vec<Position>{
    let connection = &mut establish_connection();
    position
        .select(Position::as_select())
        .filter(p_id.eq(id_tracker))
        .load(connection)
        .expect("Erreur récupération Position")
}

pub fn monitoring_exists(id_watcher: i32, id_tracker: i32) -> bool {
    let connection = &mut establish_connection();
    monitoring
        .filter(mt_id.eq(id_tracker).and(mw_id.eq(id_watcher)))
        .count()
        .get_result::<i64>(connection)
        .expect("Erreur vérification Monitoring")
        .gt(&0)
}


pub fn watcher_exists(new_login: String) ->  bool {
    let connection = &mut establish_connection();
    watcher
        .filter(login.eq(&new_login))
        .count()
        .get_result::<i64>(connection)
        .expect("Erreur vérification Watcher")
        .gt(&0)

}

pub fn tracker_exists(id_tracker: i32) -> bool {
    let connection = &mut establish_connection();
    tracker
        .filter(t_id.eq(id_tracker))
        .count()
        .get_result::<i64>(connection)
        .expect("Erreur vérification Tracker")
        .gt(&0)
}

pub fn get_alert_trackers(id_watcher: i32) -> Vec<Tracker>{
    let connection = &mut establish_connection();
    tracker
        .inner_join(monitoring)
        .filter(mw_id.eq(id_watcher).and(status.eq(1)))
        .select(Tracker::as_select())
        .load(connection)
        .expect("Erreur récupération Trackers")
}

pub fn delete_monitoring(id_watcher: i32, id_tracker: i32) {
    let connection = &mut establish_connection();
    let _ = delete(monitoring
        .filter(mw_id.eq(id_watcher).and(mt_id.eq(id_tracker))))
        .execute(connection);
}

pub fn update_tracker_status(id_tracker: i32, new_status: i32) {
    let connection = &mut establish_connection();
    update(tracker)
        .set(status.eq(new_status))
        .filter(t_id.eq(id_tracker))
        .execute(connection)
        .expect("Erreur changement statut Tracker");
}

pub fn get_watcher(id_watcher: i32) -> Watcher {
    let connection = &mut establish_connection();
    watcher
        .select(Watcher::as_select())
        .filter(w_id.eq(id_watcher))
        .get_result::<Watcher>(connection)
        .expect("Erreur récupération Watcher")
}

pub fn get_monitorings(id_tracker: i32) -> Vec<Monitoring>{
    let connection = &mut establish_connection();
    monitoring
        .select(Monitoring::as_select())
        .filter(mt_id.eq(id_tracker))
        .load(connection)
        .expect("Erreur récupération Monitorings")
}
