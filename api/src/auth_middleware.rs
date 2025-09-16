use jsonwebtoken::{decode, DecodingKey, Validation};
use poem::{http::StatusCode, Error, FromRequest, Request, RequestBody, Result};

use crate::route::user::Claims;

pub struct UserId(pub String);

#[poem::async_trait]
impl<'a> FromRequest<'a> for UserId {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let cookies = req.cookie();
        let cookie = cookies.get("jwt");
        
        match cookie {
            Some(token) => {
                let value = token.value_str();
                let claims = decode::<Claims>(
                    &value,
                    &DecodingKey::from_secret("secret".as_ref()),
                    &Validation::default(),
                )
                .map_err(|e| {
                    println!("{}", e.to_string());
                    Error::from_string(
                        "Error during decoding jwt",
                        StatusCode::UNAUTHORIZED,
                    )
                })?;

                return Ok(UserId(claims.claims.sub));
            }
            None => return Ok(UserId("".to_string())),
        }
    }
}
