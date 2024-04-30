#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    id: u32,
    name: String,
    position: String,
}
#[derive(Responder)]
#[response(status = 418, content_type = "json")]
struct TaskResponse(Json<Task>);

#[get("/todo")]
fn todo() -> TaskResponse {
    TaskResponse(Json(Task { id: 25, name: String::from("Jean"), position: String::from("20 avenue Albert Einstein")}))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/", routes![todo])
}