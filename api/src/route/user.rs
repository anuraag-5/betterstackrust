use std::sync::{Arc, Mutex};

use crate::{
    request_input::CreateUserInput,
    request_output::{CreateUserOutput, SigninUserOutput},
};
use poem::{
    handler,
    web::{Data, Json},
};
use store::store::Store;

#[handler]
pub fn create_user(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateUserOutput> {
    let username = data.username;
    let user_password = data.password;

    let mut locked_s = s.lock().unwrap();
    let result = locked_s.sign_up(username, user_password);

    match result {
        Ok(user_id) => Json(CreateUserOutput {
            user_id: user_id,
            success: true,
        }),
        Err(e) => Json(CreateUserOutput {
            user_id: e.to_string(),
            success: false,
        }),
    }
}

#[handler]
pub fn sign_in_user(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<SigninUserOutput> {
    let username = data.username;
    let user_password = data.password;

    let mut locked_s = s.lock().unwrap();
    let result = locked_s.sign_in(username, user_password);

    match result {
        Ok(exists) => {
            if exists {
                return Json(SigninUserOutput {
                    jwt: String::from("Temporary JWT"),
                    success: true,
                });
            } else {
                return Json(SigninUserOutput {
                    jwt: String::from("Temporary JWT"),
                    success: false,
                });
            }
        }
        Err(e) => Json(SigninUserOutput {
            jwt: e.to_string(),
            success: false,
        }),
    }
}
