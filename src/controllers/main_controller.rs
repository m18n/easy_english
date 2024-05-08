use actix_web::{get, HttpResponse};
use crate::base::file_openString;
use crate::models::MyError;

#[get("")]
pub async fn m_global_main()->Result<HttpResponse, MyError>{
    let response = HttpResponse::Found()
        .body("Hello");
    Ok(response)
}
