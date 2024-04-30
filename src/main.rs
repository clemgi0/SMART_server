mod db;
mod schema;
mod model;

#[macro_use] extern crate rocket;

use diesel::{insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::serde::{json::Json};
use crate::db::establish_connection;
use crate::model::{Protector, ProtectorRes};
use crate::schema::protector::dsl::protector;

#[derive(Responder)]
struct ProtectorResponse(Json<ProtectorRes>);

#[get("/todo")]
fn todo() {
    let connection = &mut establish_connection();
    let my_protector = Protector{login: "Login".to_string(), password: "Password".to_string()};
    insert_into(protector).values(my_protector).execute(connection).expect("Erreur insertion protector");
}

#[get("/res")]
fn res() -> ProtectorResponse {
    let connection = &mut establish_connection();
    let results = protector.select(ProtectorRes::as_select()).load(connection).expect("Erreur select protector");
    ProtectorResponse(Json(results[0].clone()))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/", routes![todo]).mount("/", routes![res])
}