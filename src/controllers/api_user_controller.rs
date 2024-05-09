use actix_web::{get, HttpResponse};
use crate::base::file_openString;
use crate::models::MyError;

#[get("/test")]
pub async fn m_test()->Result<HttpResponse, MyError>{

    Ok(HttpResponse::Ok().content_type("text/html").body("Hello"))
}