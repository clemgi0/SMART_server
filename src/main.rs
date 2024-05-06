mod db;
mod schema;
mod model;
mod responder;
mod mail;

#[macro_use] extern crate rocket;

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use dotenvy::dotenv;
use rocket::request::{self, FromRequest};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header as httpHeader;

use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::{Request, Response};
use rocket::http::Status;
use crate::db::establish_connection;
use model::{LoginRequest, LoginResponse, SignupRequest, SignupResponse, JWT};
use rocket::serde::json::Json;
use argon2::Config;
use rand::random;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::Error;

use crate::db::*;
use crate::model::*;
use crate::mail::send_mail;
use crate::schema::watcher::dsl::watcher;
use crate::responder::CustomResponse;

pub fn create_jwt(id: i32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let expiration = Utc::now().checked_add_signed(chrono::Duration::minutes(5)).expect("Invalid timestamp").timestamp();
    let claims = Claims {
        subject_id: id,
        exp: expiration as usize
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

fn decode_jwt(token: String) -> Result<Claims, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = CustomResponse;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, CustomResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                request::Outcome::Error((Status::BadRequest,CustomResponse::BadRequest))
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => request::Outcome::Success(JWT {claims}),
                Err(_err) => {
                    request::Outcome::Error((Status::BadRequest,CustomResponse::Unauthorized))
                }
            },
        }
    }
}

#[post("/signup", data = "<signup_request>")]
fn signup(signup_request: Json<SignupRequest>) -> Json<SignupResponse> {
    if watcher_exists(signup_request.login.clone()) {
        return Json(SignupResponse {user_id:0, success: false});
    }

    let salt: [u8; 32] = random();
    let config = Config::default();
    let hashed_password = argon2::hash_encoded(signup_request.password.as_bytes(), &salt, &config);

    insert_watcher(WatcherInsert {login: signup_request.login.clone(), password: hashed_password.expect("Erreur hashage password"), salt: salt.to_vec()});

    let connection = &mut establish_connection();
    let results = watcher.select(Watcher::as_select()).load(connection).expect("Erreur select watcher");
    let watcher1_opt = results.iter().find(|watcher1| watcher1.login == signup_request.login.clone()).expect("User not found");
    Json(SignupResponse {user_id: watcher1_opt.id, success: true })

}

#[post("/login", data = "<login_request>")]
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
                Json(LoginResponse{user_id:watcher1.id, access_token: jwt })
            } else {
                Json(LoginResponse{user_id:0, access_token: "".to_string() })
            }
        },
        None => {
            Json(LoginResponse{user_id: 0, access_token: "".to_string() })
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn is_user_id_admin(id: i32) -> bool {

    let connection = &mut establish_connection();
    let watcher_list = watcher.select(Watcher::as_select()).load(connection).expect("Erreur récupération watcher");
    for watcher1 in watcher_list {
        if watcher1.id == id {
            if watcher1.login == "admin" {
                return true;
            }
        }
    }
    return false;
}

#[get("/history", data = "<data>")]
fn history(data: Json<MonitoringRequest>, jwt: JWT) -> Result<Json<Vec<Position>>, CustomResponse> {
    let user_is_admin = is_user_id_admin(jwt.claims.subject_id);
    if user_is_admin || (jwt.claims.subject_id == data.watcher_id && monitoring_exists(data.watcher_id, data.tracker_id)) {
        let positions = get_position(data.tracker_id);
        let last_positions = positions
            .iter()
            .map(|pos| pos.clone())
            .filter(|pos| (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - pos.timestamp as u64) < 3600)
            .collect();
        Ok(Json(last_positions))
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
fn addmonitoring(data: Json<Monitoring>, jwt: JWT) -> CustomResponse {
    if jwt.claims.subject_id != data.watcher_id {
        return CustomResponse::Unauthorized;
    }
    if tracker_exists(data.tracker_id) && !monitoring_exists(data.watcher_id, data.tracker_id) {
        insert_monitoring(data.watcher_id, data.tracker_id, data.tracker_name.clone());
        CustomResponse::OK
    } else {
        CustomResponse::Forbidden
    }
}

#[post("/deletemonitoring", data= "<data>")]
fn deletemonitoring(data: Json<MonitoringRequest>, jwt: JWT) -> CustomResponse {
    if jwt.claims.subject_id != data.watcher_id {
        return CustomResponse::Unauthorized;
    }
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

#[get("/getalerttrackers")]
fn getalerttrackers(jwt: JWT) -> Json<Vec<Tracker>> {
    Json(get_alert_trackers(jwt.claims.subject_id))
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

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(httpHeader::new("Access-Control-Allow-Origin", "*"));
        response.set_header(httpHeader::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(httpHeader::new("Access-Control-Allow-Headers", "*"));
        response.set_header(httpHeader::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().attach(CORS).mount("/", routes![index])
        .mount("/", routes![history])
        .mount("/", routes![signup])
        .mount("/", routes![login])
        .mount("/", routes![addposition])
        .mount("/", routes![addmonitoring])
        .mount("/", routes![deletemonitoring])
        .mount("/", routes![addtracker])
        .mount("/", routes![setstatus])
        .mount("/", routes![getalerttrackers])
}
