use std::{env, sync::{Arc}};
use tokio::sync::{Mutex};


use crate::{
    request_input::{CreateUserInput, SignInUserInput, UpdateEmailInput, UpdatePasswordInput},
    request_output::{CreateUserOutput, UpdateEmailOutput},
};
use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{
    handler,
    http::{header, StatusCode},
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
pub async fn create_user(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<CreateUserOutput>, Error> {
    let username = data.username;
    let user_password = data.password;
    let name = data.name;

    let mut locked_s = s.lock().await;
    let result = locked_s
        .sign_up(username, user_password, name).await
        .map_err(|_| Error::from_status(StatusCode::CONFLICT))?;

    Ok(Json(CreateUserOutput {
        user_id: result,
        success: true,
    }))
}

#[handler]
pub async fn sign_in_user(
    Json(data): Json<SignInUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Response, Error> {
    let username = data.username;
    let user_password = data.password;

    let mut locked_s = s.lock().await;
    let result = locked_s.sign_in(username, user_password).await;

    match result {
        Ok(user) => {
            let my_claims = Claims {
                sub: user.id,
                exp: 1111111111111,
            };
            let token = encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret(env::var("JWT_SECRET").map_err(|_| Error::from_string("Invalid ENV Secret", StatusCode::EXPECTATION_FAILED))?.as_ref()),
            )
            .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
            
            // For localhost cross-origin, use SameSite=lax or None with proper settings
            let cookie = format!(
                "jwt={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={};",
                token,
                60 * 60 * 24 * 7
            );
            
            let mut resp = Response::builder()
                .status(StatusCode::OK)
                .header(header::SET_COOKIE, cookie)
                .header(header::CONTENT_TYPE, "application/json")
                .body(format!("{{\"jwt\":\"{}\"}}", token));

            Ok(resp)
        }
        Err(_) => Err(Error::from_status(StatusCode::NOT_FOUND)),
    }
}

#[handler]
pub async fn update_email(
    Json(data): Json<UpdateEmailInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<UpdateEmailOutput> {
    let input_user_id = data.user_id;
    let new_email = data.new_email;

    let mut locked_s = s.lock().await;
    let result = locked_s.update_email(input_user_id, new_email).await;

    match result {
        Ok(_) => {
            Json(UpdateEmailOutput { success: true })
        }
        Err(_) => Json(UpdateEmailOutput { success: false })
    }
}

#[handler]
pub async fn update_password(
    Json(data): Json<UpdatePasswordInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<UpdateEmailOutput> {
    let input_user_id = data.user_id;
    let new_password = data.new_password;

    let mut locked_s = s.lock().await;
    let result = locked_s.update_password(input_user_id, new_password).await;

    match result {
        Ok(_) => {
            Json(UpdateEmailOutput { success: true })
        }
        Err(_) => Json(UpdateEmailOutput { success: false })
    }
}

#[handler]
pub async fn logout_user() ->  Response {
    Response::builder()
    .status(StatusCode::OK)
    .header(
        header::SET_COOKIE,
        "jwt=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0",
    )
    .body("Logged out")
}