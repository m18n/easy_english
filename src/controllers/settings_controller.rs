use actix_web::{get, HttpResponse};
use crate::base::file_openString;
use crate::models::MyError;

#[get("/error")]
pub async fn m_settings_error()->Result<HttpResponse, MyError>{
    let contents = file_openString("./easy_english_web/index.html").await?;
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}