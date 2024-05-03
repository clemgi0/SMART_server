use rocket::data::{ByteUnit, FromData, Outcome};
use rocket::{Data, Request};
use rocket::http::Status;
use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct ProtectionData {
    pub id_protector: i32,
    pub id_protected: i32,
}

#[derive(Deserialize)]
pub struct PositionData {
    pub id_protected: i32,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Deserialize)]
pub struct NewProtectionData {
    pub id_protector: i32,
    pub id_protected: i32,
    pub name_protected: String,
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

#[rocket::async_trait]
impl<'r> FromData<'r> for NewProtectionData {
    type Error = String;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let bytes = match data.open(ByteUnit::from(4096)).into_bytes().await {
            Ok(bytes) => bytes,
            Err(_) => return Outcome::Error((Status::InternalServerError, "Failed to read request body".to_string())),
        };

        match serde_json::from_slice::<NewProtectionData>(&bytes) {
            Ok(new_protection_data) => Outcome::Success(new_protection_data),
            Err(_) => Outcome::Error((Status::UnprocessableEntity, "Invalid JSON format".to_string())),
        }
    }
}