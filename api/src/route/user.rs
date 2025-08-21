use std::sync::{Arc, Mutex};

use crate::{
    request_input::CreateUserInput,
    request_output::{CreateUserOutput},
};
use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    Error, Response,
};
use serde::{Deserialize, Serialize};
use store::store::Store;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[handler]
pub fn create_user(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<CreateUserOutput>, Error> {
    let username = data.username;
    let user_password = data.password;

    let mut locked_s = s.lock().unwrap();
    let result = locked_s
        .sign_up(username, user_password)
        .map_err(|_| Error::from_status(StatusCode::CONFLICT))?;

    Ok(Json(CreateUserOutput {
        user_id: result,
        success: true,
    }))
}

#[handler]
pub fn sign_in_user(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Response, Error> {
    let username = data.username;
    let user_password = data.password;

    let mut locked_s = s.lock().unwrap();
    let result = locked_s.sign_in(username, user_password);

    match result {
        Ok(id) => {
            let my_claims = Claims {
                sub: id,
                exp: 1111111111111,
            };
            let token = encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret("secret".as_ref()),
            )
            .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
            let cookie = format!(
                "jwt={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age={}",
                token,
                60 * 60 * 24 * 7
            );
            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("Set-Cookie", cookie)
                .body(format!("jwt:{}", token));

            Ok(resp)
        }
        Err(_) => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
    }
}
