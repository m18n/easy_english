use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use crate::jwt::{Claims, create_token_auth};
use crate::models::UserDictionary;

pub fn create_cookie_auth_clear() ->Cookie<'static>{
    Cookie::build("refresh_token", "")
        .path("/")
        .http_only(true)
        .finish()
}
pub fn create_cookie_auth(mut claims:Claims) ->Cookie<'static>{
    if claims.exp==0 {
        claims.exp = 10000000000;
    }
    Cookie::build("refresh_token", create_token_auth(claims))
        .path("/")
        .http_only(true)
        .max_age(Duration::days(2))
        .finish()
}