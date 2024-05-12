use std::fmt::format;
use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use ramhorns::Template;
use sqlx::query;
use crate::base::{file_openString, get_nowtime_str};
use crate::controllers::object_of_controller::{ RequestResult};
use crate::jwt::{Claims, create_token};
use crate::models::{MyError, MysqlDB};
use crate::render_temps::{CurrentLang, LanguagesSupportedTemplate};
use crate::StateDb;

#[get("/login")]
pub async fn m_login()->Result<HttpResponse, MyError>{
    let contents = file_openString("./easy_english_web/login.html").await?;
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

#[get("/learn/main")]
pub async fn m_learn_main(req:HttpRequest,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let mut cookie=Claims::new();
    if let Some(claims) = req.extensions().get::<Claims>(){
        cookie=claims.clone();
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }
    let contents = file_openString("./easy_english_web/learn_lang_main.html").await?;
    let template=CurrentLang{
        current_lang:cookie.user_dictionaries[cookie.current_lang_index].language_name.clone(),
        languages:cookie.user_dictionaries.clone(),
    };
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&template)))
}
