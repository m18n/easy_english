use actix_web::{get, HttpResponse, web};
use ramhorns::Template;
use crate::base::file_openString;
use crate::models::{MyError, MysqlDB};
use crate::render_temps::InitTemplate;
use crate::StateDb;
// url controller: /view/userstart/***

#[get("/initdictionaries")]
pub async fn m_init_dictionaries(state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let dictionaries=MysqlDB::getLanguages(state.mysql_db.clone()).await?;
    let lang_levels=MysqlDB::getLanguagesLevels(state.mysql_db.clone()).await?;
    let contents = file_openString("./easy_english_web/init_dictionaries.html").await?;
    let template=InitTemplate{
        languages:dictionaries,
        languages_levels:lang_levels
    };
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&template)))
}