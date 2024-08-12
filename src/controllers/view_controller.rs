// controller for all visible pages
use actix_web::{get, HttpMessage, HttpResponse};
use crate::base::{file_openString};
use crate::models::{MyError};
// url controller: /view/***
#[get("/login")]
pub async fn m_login()->Result<HttpResponse, MyError>{
    let contents = file_openString("./easy_english_web/login.html").await?;
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}


