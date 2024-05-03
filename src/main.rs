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
use diesel::{delete, insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use jsonwebtoken::{encode, EncodingKey, Algorithm, Header};
use jsonwebtoken::errors::Error;
use argon2::Config;

use crate::db::{protected_exists, protection_exists, establish_connection, get_positions_history, insert_positions_history, insert_protected, insert_protection, insert_protector, delete_protection, update_protected_status, get_protector, get_protections};
use crate::mail::send_mail;
use crate::model::{PositionsHistory, Protected, Protector, ProtectorRes, Claims, SignupRequest, SignupResponse, LoginRequest, LoginResponse, ProtectionRequest, PositionRequest, Protection};
use crate::schema::positions_history::dsl::positions_history;
use crate::schema::protected::dsl::protected;
use crate::schema::protection::dsl::protection;
use crate::schema::protector::dsl::protector;
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

    let salt: [u8; 32] = random();

    insert_protector(Protector{login: "P1".to_string(), password: "P1@mail.com".to_string(), salt: salt.to_vec()});
    insert_protector(Protector{login: "P2".to_string(), password: "P2@mail.com".to_string(), salt: salt.to_vec()});
    insert_protector(Protector{login: "P3".to_string(), password: "P3@mail.com".to_string(), salt: salt.to_vec()});

    insert_protected();
    insert_protected();
    insert_protected();

    let protector_list = protector.select(ProtectorRes::as_select()).load(connection).expect("Erreur récupération Protector");
    let protected_list = protected.select(Protected::as_select()).load(connection).expect("Erreur récupération Protected");

    insert_protection(protector_list[0].id, protected_list[0].id, "Papi".to_string());
    insert_protection(protector_list[1].id, protected_list[1].id, "Mamie".to_string());
    insert_protection(protector_list[2].id, protected_list[2].id, "Bébé".to_string());

    insert_positions_history(45.2, 4.3, protected_list[0].id);
    insert_positions_history(43.9, 5.7, protected_list[1].id);
    insert_positions_history(44.6, 3.8, protected_list[2].id);
}

#[post("/history", data = "<data>")]
fn history(data: Json<ProtectionRequest>) -> Result<Json<Vec<PositionsHistory>>, CustomResponse> {
    if protection_exists(data.id_protector, data.id_protected) {
        Ok(Json(get_positions_history(data.id_protected)))
    } else {
        Err(CustomResponse::Unauthorized)
    }
}

#[post("/addposition", data = "<data>")]
fn addposition(data: Json<PositionRequest>) -> CustomResponse {
    if protected_exists(data.id_protected) && data.latitude.abs() <= 90.0 && data.longitude.abs() <= 90.0{
        insert_positions_history(data.latitude, data.longitude, data.id_protected);
        CustomResponse::OK
    } else {
        CustomResponse::Forbidden
    }
}

#[post("/addprotection", data = "<data>")]
fn addprotection(data: Json<Protection>) -> CustomResponse {
    if protected_exists(data.protected_id) && !protection_exists(data.protector_id, data.protected_id) {
        insert_protection(data.protector_id, data.protected_id, data.protected_name.clone());
        CustomResponse::OK
    } else {
        CustomResponse::Forbidden
    }
}

#[post("/deleteprotection", data= "<data>")]
fn deleteprotection(data: Json<ProtectionRequest>) -> CustomResponse {
    if protection_exists(data.id_protector, data.id_protected) {
        delete_protection(data.id_protector, data.id_protected);
        CustomResponse::OK
    } else {
        CustomResponse::Unauthorized
    }
}

#[post("/addprotected")]
fn addprotected() -> Json<i32> {
    let new_protected = insert_protected();
    Json(new_protected.id)
}

#[post("/setstatus", data = "<data>")]
async fn setstatus(data: Json<Protected>) -> CustomResponse {
    if protected_exists(data.id) {
        update_protected_status(data.id, data.status);

        let protections = get_protections(data.id);
        for prtn in protections {
            let prtr = get_protector(prtn.protector_id);

            let name = prtn.protected_name;
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
        .mount("/", routes![reset])
        .mount("/", routes![history])
        .mount("/", routes![signup])
        .mount("/", routes![login])
        .mount("/", routes![addposition])
        .mount("/", routes![addprotection])
        .mount("/", routes![deleteprotection])
        .mount("/", routes![addprotected])
        .mount("/", routes![setstatus])
}