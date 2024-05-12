use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header;
use crate::base::file_openString;
use crate::models::MyError;

#[get("/error")]
pub async fn m_settings_error()->Result<HttpResponse, MyError>{
    let contents = file_openString("./easy_english_web/error_web_site.html").await?;
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
pub async fn m_none()-> impl Responder{
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/view/login"))
        .finish()
}