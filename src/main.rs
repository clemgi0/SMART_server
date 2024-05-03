mod db;
mod schema;
mod model;

#[macro_use] extern crate rocket;
use rocket::data::{ByteUnit, FromData, Outcome};

use diesel::{delete, insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::{Data, Request};
use rocket::http::Status;
use serde::Deserialize;
use crate::db::{check_protection, establish_connection, get_positions_history, insert_positions_history, insert_protected, insert_protection, insert_protector};
use model::{SignupRequest, SignupResponse, LoginRequest, LoginResponse};
use rocket::serde::json::Json;
use argon2::Config;
use rand::{random, Rng};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Algorithm, Header};
use jsonwebtoken::errors::Error;
use dotenvy::dotenv;
use std::env;
use crate::model::{PositionsHistory, ProtectedRes, Protector, ProtectorRes, Claims};
use crate::schema::positions_history::dsl::positions_history;
use crate::schema::protected::dsl::protected;
use crate::schema::protection::dsl::protection;
use crate::schema::protector::dsl::protector;

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
    let connection = &mut establish_connection();
    let results = protector.select(ProtectorRes::as_select()).load(connection).expect("Erreur select protector");

    if results.iter().any(|protector1| protector1.login == signup_request.login) {
        return Json(SignupResponse {success: false});
    }

    let salt: [u8; 32] = random();

    let config = Config::default();

    let hashed_password = argon2::hash_encoded(signup_request.password.as_bytes(), &salt, &config);

    let my_protector = Protector{login: signup_request.login.clone(), password: hashed_password.expect("Erreur hashage password"), salt: salt.to_vec()};
    insert_into(protector).values(my_protector).execute(connection).expect("Erreur insertion protector");

    Json(SignupResponse {success: true })

}

#[get("/login", data = "<login_request>")]
fn login(login_request: Json<LoginRequest>) -> Json<LoginResponse> {

    let connection = &mut establish_connection();
    let results = protector.select(ProtectorRes::as_select()).load(connection).expect("Erreur select protector");
    let protector1_opt = results.iter().find(|protector1| protector1.login == login_request.login.clone());
    match protector1_opt {
        Some(protector1) => {
            let salt = protector1.salt.clone();
            let config = Config::default();
            let hashed_password = argon2::hash_encoded(login_request.password.as_bytes(), &salt, &config).expect("Couldn't hash login password");
            if hashed_password == protector1.password {
                let jwt = create_jwt(protector1.id).expect("Couldn't create jwt token");
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

#[get("/reset")]
fn reset() {
    let connection = &mut establish_connection();
    let _ = delete(protector).execute(connection);
    let _ = delete(protected).execute(connection);
    let _ = delete(protection).execute(connection);
    let _ = delete(positions_history).execute(connection);

    let salt: [u8; 32] = rand::thread_rng().gen();

    insert_protector(Protector{login: "P1".to_string(), password: "P1@mail.com".to_string(), salt: salt.to_vec()});
    insert_protector(Protector{login: "P2".to_string(), password: "P2@mail.com".to_string(), salt: salt.to_vec()});
    insert_protector(Protector{login: "P3".to_string(), password: "P3@mail.com".to_string(), salt: salt.to_vec()});

    insert_protected();
    insert_protected();
    insert_protected();

    let protector_list = protector.select(ProtectorRes::as_select()).load(connection).expect("Erreur récupération Protector");
    let protected_list = protected.select(ProtectedRes::as_select()).load(connection).expect("Erreur récupération Protected");

    insert_protection(protector_list[0].id, protected_list[0].id, "Papi");
    insert_protection(protector_list[1].id, protected_list[1].id, "Mamie");
    insert_protection(protector_list[2].id, protected_list[2].id, "Bébé");

    insert_positions_history(45.2, 4.3, protected_list[0].id);
    insert_positions_history(43.9, 5.7, protected_list[1].id);
    insert_positions_history(44.6, 3.8, protected_list[2].id);
}

#[derive(Deserialize)]
struct PostData {
    id_protector: i32,
    id_protected: i32,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for PostData {
    type Error = String;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let bytes = match data.open(ByteUnit::from(4096)).into_bytes().await {
            Ok(bytes) => bytes,
            Err(_) => return Outcome::Error((Status::InternalServerError, "Failed to read request body".to_string())),
        };

        match serde_json::from_slice::<PostData>(&bytes) {
            Ok(post_data) => Outcome::Success(post_data),
            Err(_) => Outcome::Error((Status::UnprocessableEntity, "Invalid JSON format".to_string())),
        }
    }
}

#[post("/history", data = "<post_data>")]
fn history(post_data: PostData) -> Result<Json<Vec<PositionsHistory>>, Json<String>> {
    if check_protection(post_data.id_protector, post_data.id_protected) {
        Ok(Json(get_positions_history(post_data.id_protected)))
    } else {
        Err(Json("Unauthorized".to_string()))
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index])
        .mount("/", routes![reset])
        .mount("/", routes![history])
        .mount("/", routes![signup])
        .mount("/", routes![login])
}