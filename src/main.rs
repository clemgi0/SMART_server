mod db;
mod schema;
mod model;
mod responder;
mod mail;

#[macro_use] extern crate rocket;

use std::env;
use chrono::Utc;
use rand::random;
use dotenvy::dotenv;
use rocket::serde::json::Json;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use jsonwebtoken::{encode, EncodingKey, Algorithm, Header};
use jsonwebtoken::errors::Error;
use argon2::Config;

use crate::db::*;
use crate::model::*;
use crate::mail::send_mail;
use crate::schema::watcher::dsl::watcher;
use crate::responder::CustomResponse;

pub fn create_jwt(id: i32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let expiration = Utc::now().checked_add_signed(chrono::Duration::seconds(60)).expect("Invalid timestamp").timestamp();

    let claims = Claims {
        subject_id: id,
        exp: expiration as usize
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

#[post("/signup", data = "<signup_request>")]
fn signup(signup_request: Json<SignupRequest>) -> Json<SignupResponse> {
    if watcher_exists(signup_request.login.clone()) {
        return Json(SignupResponse {success: false});
    }

    let salt: [u8; 32] = random();
    let config = Config::default();
    let hashed_password = argon2::hash_encoded(signup_request.password.as_bytes(), &salt, &config);

    insert_watcher(WatcherInsert {login: signup_request.login.clone(), password: hashed_password.expect("Erreur hashage password"), salt: salt.to_vec()});
    Json(SignupResponse {success: true })

}

#[get("/login", data = "<login_request>")]
fn login(login_request: Json<LoginRequest>) -> Json<LoginResponse> {

    let connection = &mut establish_connection();
    let results = watcher.select(Watcher::as_select()).load(connection).expect("Erreur select watcher");
    let watcher1_opt = results.iter().find(|watcher1| watcher1.login == login_request.login.clone());
    match watcher1_opt {
        Some(watcher1) => {
            let salt = watcher1.salt.clone();
            let config = Config::default();
            let hashed_password = argon2::hash_encoded(login_request.password.as_bytes(), &salt, &config).expect("Couldn't hash login password");
            if hashed_password == watcher1.password {
                let jwt = create_jwt(watcher1.id).expect("Couldn't create jwt token");
                Json(LoginResponse{access_token: jwt })
            } else {
                Json(LoginResponse{access_token: "".to_string() })
            }
        },
        None => {
            Json(LoginResponse{access_token: "".to_string() })
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/history", data = "<data>")]
fn history(data: Json<MonitoringRequest>) -> Result<Json<Vec<Position>>, CustomResponse> {
    if monitoring_exists(data.watcher_id, data.tracker_id) {
        Ok(Json(get_position(data.tracker_id)))
    } else {
        Err(CustomResponse::Unauthorized)
    }
}

#[post("/addposition", data = "<data>")]
fn addposition(data: Json<PositionRequest>) -> CustomResponse {
    if tracker_exists(data.tracker_id) && data.latitude.abs() <= 90.0 && data.longitude.abs() <= 90.0{
        insert_position(data.latitude, data.longitude, data.tracker_id);
        CustomResponse::OK
    } else {
        CustomResponse::Forbidden
    }
}

#[post("/addmonitoring", data = "<data>")]
fn addmonitoring(data: Json<Monitoring>) -> CustomResponse {
    if tracker_exists(data.tracker_id) && !monitoring_exists(data.watcher_id, data.tracker_id) {
        insert_monitoring(data.watcher_id, data.tracker_id, data.tracker_name.clone());
        CustomResponse::OK
    } else {
        CustomResponse::Forbidden
    }
}

#[post("/deletemonitoring", data= "<data>")]
fn deletemonitoring(data: Json<MonitoringRequest>) -> CustomResponse {
    if monitoring_exists(data.watcher_id, data.tracker_id) {
        delete_monitoring(data.watcher_id, data.tracker_id);
        CustomResponse::OK
    } else {
        CustomResponse::Unauthorized
    }
}

#[post("/addtracker", data = "<data>")]
fn addtracker(data: Json<TrackerInsert>) -> Json<i32> {
    let new_tracker = insert_tracker(data.latitude, data.longitude);
    Json(new_tracker.id)
}

#[post("/setstatus", data = "<data>")]
async fn setstatus(data: Json<Tracker>) -> CustomResponse {
    if tracker_exists(data.id) {
        update_tracker_status(data.id, data.status);

        let monitorings = get_monitorings(data.id);
        for prtn in monitorings {
            let prtr = get_watcher(prtn.watcher_id);

            let name = prtn.tracker_name;
            let message = if data.status == 0 {
                format!("{name} est revenu dans la zone sûre.")
            } else {
                format!("{name} a quitté la zone sûre.")
            };
            send_mail(prtr.login, message).await.expect("Erreur envoi du mail");
        }

        CustomResponse::OK
    } else {
        CustomResponse::Forbidden
    }
}


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index])
        .mount("/", routes![history])
        .mount("/", routes![signup])
        .mount("/", routes![login])
        .mount("/", routes![addposition])
        .mount("/", routes![addmonitoring])
        .mount("/", routes![deletemonitoring])
        .mount("/", routes![addtracker])
        .mount("/", routes![setstatus])
}