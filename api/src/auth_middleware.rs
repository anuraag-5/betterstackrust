use jsonwebtoken::{decode, DecodingKey, Validation};
use poem::{http::StatusCode, Error, FromRequest, Request, RequestBody, Result};

use crate::route::user::Claims;

pub struct UserId(pub String);

#[poem::async_trait] 
impl<'a> FromRequest<'a> for UserId {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let token = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| Error::from_string("Missing token", StatusCode::UNAUTHORIZED))?;
        

        let claims = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).map_err(|e| { println!("{}",e.to_string());Error::from_string("Error during decoding jwt", StatusCode::INTERNAL_SERVER_ERROR)})?;

        
        Ok(UserId(claims.claims.sub))
    }
}