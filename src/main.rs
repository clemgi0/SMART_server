mod db;
mod schema;
mod model;
mod responder;

#[macro_use] extern crate rocket;

use rocket::data::{ByteUnit, FromData, Outcome};

use diesel::{delete, insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::{Data, Request};
use rocket::http::Status;
use rocket::serde::{json::Json};
use serde::Deserialize;
use crate::db::{protected_exists, protection_exists, establish_connection, get_positions_history, insert_positions_history, insert_protected, insert_protection, insert_protector};
use model::{SignupRequest, SignupResponse};
use argon2::Config;
use rand::random;
use crate::model::{PositionsHistory, ProtectedRes, Protector, ProtectorRes};
use crate::responder::CustomResponse;
use crate::schema::positions_history::dsl::positions_history;
use crate::schema::protected::dsl::protected;
use crate::schema::protection::dsl::protection;
use crate::schema::protector::dsl::protector;

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

    let my_protector = Protector{login: signup_request.login.clone(), password: hashed_password.expect("Erreur hashage password")};
    insert_into(protector).values(my_protector).execute(connection).expect("Erreur insertion protector");

    Json(SignupResponse {success: true })

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

    insert_protector(Protector{login: "P1".to_string(), password: "P1@mail.com".to_string()});
    insert_protector(Protector{login: "P2".to_string(), password: "P2@mail.com".to_string()});
    insert_protector(Protector{login: "P3".to_string(), password: "P3@mail.com".to_string()});

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
struct ProtectionData {
    id_protector: i32,
    id_protected: i32,
}

#[derive(Deserialize)]
struct PositionData {
    id_protected: i32,
    latitude: f32,
    longitude: f32,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for ProtectionData {
    type Error = String;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let bytes = match data.open(ByteUnit::from(4096)).into_bytes().await {
            Ok(bytes) => bytes,
            Err(_) => return Outcome::Error((Status::InternalServerError, "Failed to read request body".to_string())),
        };

        match serde_json::from_slice::<ProtectionData>(&bytes) {
            Ok(post_data) => Outcome::Success(post_data),
            Err(_) => Outcome::Error((Status::UnprocessableEntity, "Invalid JSON format".to_string())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for PositionData {
    type Error = String;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let bytes = match data.open(ByteUnit::from(4096)).into_bytes().await {
            Ok(bytes) => bytes,
            Err(_) => return Outcome::Error((Status::InternalServerError, "Failed to read request body".to_string())),
        };

        match serde_json::from_slice::<PositionData>(&bytes) {
            Ok(position_data) => Outcome::Success(position_data),
            Err(_) => Outcome::Error((Status::UnprocessableEntity, "Invalid JSON format".to_string())),
        }
    }
}

#[post("/history", data = "<post_data>")]
fn history(post_data: ProtectionData) -> Result<Json<Vec<PositionsHistory>>, CustomResponse> {
    if protection_exists(post_data.id_protector, post_data.id_protected) {
        Ok(Json(get_positions_history(post_data.id_protected)))
    } else {
        Err(CustomResponse::Unauthorized)
    }
}

#[post("/addposition", data = "<position_data>")]
fn addposition(position_data: PositionData) -> Result<CustomResponse, CustomResponse> {
    if protected_exists(position_data.id_protected) && position_data.latitude.abs() <= 90.0 && position_data.longitude.abs() <= 90.0{
        insert_positions_history(position_data.latitude, position_data.longitude, position_data.id_protected);
        Ok(CustomResponse::OK)
    } else {
        Err(CustomResponse::Forbidden)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/", routes![reset])
        .mount("/", routes![history])
        .mount("/", routes![signup])
        .mount("/", routes![addposition])
}