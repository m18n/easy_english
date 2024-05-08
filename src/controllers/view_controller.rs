use actix_web::{get, HttpResponse, web};
use ramhorns::Template;
use crate::base::file_openString;
use crate::models::{MyError, MysqlDB};
use crate::render_temps::LanguagesSupportedTemplate;
use crate::StateDb;

#[get("/login")]
pub async fn m_login()->Result<HttpResponse, MyError>{
    let contents = file_openString("./easy_english_web/login.html").await?;
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

#[get("/main")]
pub async fn m_main()->Result<HttpResponse, MyError>{
    let contents = file_openString("./easy_english_web/main.html").await?;
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

#[get("/initdictionaries")]
pub async fn m_init_dictionaries(state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let dictionaries=MysqlDB::getLanguages(state.mysql_db.clone()).await?;
    let contents = file_openString("./easy_english_web/init_dictionaries.html").await?;
    let template=LanguagesSupportedTemplate{
        languages:dictionaries
    };
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&template)))
}