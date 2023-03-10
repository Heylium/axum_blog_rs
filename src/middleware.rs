use axum::{extract::{FromRequest, RequestParts}, async_trait};

use crate::{error::AppError, cookie};

pub struct Auth(pub String) ;
#[async_trait]
impl<B> FromRequest<B> for Auth
    where
        B: Send,
{
    type Rejection = AppError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = req.headers().unwrap();
        let cookie = cookie::get_cookie(headers);
        let auth = cookie.unwrap_or("".to_string());
        if  auth.is_empty() {
            return Err(AppError::forbidden());
        }
        Ok(Auth(auth))
    }
}