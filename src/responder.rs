use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;

#[derive(Debug)]
pub enum CustomResponse {
    OK,
    Unauthorized,
    Forbidden,
    BadRequest
}

impl CustomResponse {
    fn get_http_status(&self) -> Status {
        match self {
            CustomResponse::OK => {Status::Ok}
            CustomResponse::Unauthorized => {Status::Unauthorized}
            CustomResponse::Forbidden => {Status::Forbidden}
            CustomResponse::BadRequest => {Status::BadRequest}
        }
    }
}

impl<'r> Responder<'r, 'static> for CustomResponse {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .status(self.get_http_status())
            .ok()
    }
}