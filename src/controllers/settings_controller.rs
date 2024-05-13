use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header;
use ramhorns::Template;
use crate::base::file_openString;
use crate::globals::LOGS_DB_ERROR;
use crate::models::MyError;
use crate::render_temps::{ErrorTemplate, TranslateTemplate};

#[get("/error")]
pub async fn m_settings_error()->Result<HttpResponse, MyError>{
    let contents = file_openString("./easy_english_web/error_web_site.html").await?;
    let logs=LOGS_DB_ERROR.lock().await;
    let template=ErrorTemplate{
        error:logs.clone()
    };
    drop(logs);
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&template)))
}
pub async fn m_none()-> impl Responder{
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/view/login"))
        .finish()
}