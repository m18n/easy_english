use std::fmt::format;
use std::os::linux::raw::stat;
use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use futures_util::future::join_all;
use ramhorns::Template;
use sqlx::query;
use crate::base::{file_openString, get_nowtime_str};
use crate::controllers::object_of_controller::{CurrentLanguage, RequestResult};
use crate::jwt::{Claims, create_token};
use crate::models::{MyError, MysqlDB};
use crate::render_temps::{CurrentLangTemplate, LanguagesSupportedTemplate, TranslateHistoryItemTemplate, TranslateHistoryTemplate, TranslateTemplate};
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
    let template= CurrentLangTemplate {
        current_lang:cookie.user_dictionaries[cookie.current_lang_index].language_name.clone(),
        languages:cookie.user_dictionaries.clone(),
    };
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&template)))
}
#[get("/translate/main")]
pub async fn m_translate_main(req:HttpRequest,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let mut cookie=Claims::new();
    if let Some(claims) = req.extensions().get::<Claims>(){
        cookie=claims.clone();
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }
    let langs=MysqlDB::getLanguages(state.mysql_db.clone()).await?;
    let contents = file_openString("./easy_english_web/translate_main.html").await?;
    let template=TranslateTemplate{
        current_lang:cookie.user_dictionaries[cookie.current_lang_index].language_name.clone(),
        current_lang_id:cookie.user_dictionaries[cookie.current_lang_index].language_id,
        languages:cookie.user_dictionaries.clone(),
        langueges_supported:langs
    };
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&template)))
}
#[get("/translate/history")]
pub async fn m_translate_history(req:HttpRequest,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let response = HttpResponse::Found()
        .insert_header((http::header::LOCATION, "/view/userspace/translate/history/p/1"))
        .finish();
    Ok(response)
}
#[get("/translate/history/p/{number_p}")]
pub async fn m_translate_history_pagination(req:HttpRequest,state: web::Data<StateDb>,number_p:web::Path<i32>)->Result<HttpResponse, MyError>{
    let mut num=number_p.into_inner();
    num-=1;
    let mut cookie=Claims::new();
    if let Some(claims) = req.extensions().get::<Claims>(){
        cookie=claims.clone();
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }
    let trans=MysqlDB::getTranslated(state.mysql_db.clone(),num*10,10,cookie.user_id).await?;
    let contents = file_openString("./easy_english_web/translate_history.html").await?;
    let template=TranslateHistoryTemplate{
        current_lang:cookie.user_dictionaries[cookie.current_lang_index].language_name.clone(),
        languages:cookie.user_dictionaries.clone(),
        translate_history:trans
    };
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&template)))
}
#[get("/translate/history/item/{id_item}")]
pub async fn m_translate_history_item_show(req:HttpRequest,state: web::Data<StateDb>,id_item_:web::Path<i32>)->Result<HttpResponse, MyError>{
    let mut id_item=id_item_.into_inner();
    let mut cookie=Claims::new();
    if let Some(claims) = req.extensions().get::<Claims>(){
        cookie=claims.clone();
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }
    let trans=MysqlDB::getTranslatedItem(state.mysql_db.clone(),id_item,cookie.user_id).await?;
    let mut tasks=Vec::new();
    tasks.push(MysqlDB::getLanguage(state.mysql_db.clone(),trans.lang_from_translated_id));
    tasks.push(MysqlDB::getLanguage(state.mysql_db.clone(),trans.lang_into_translated_id));
    let results=join_all(tasks).await;
    let lang_from = results[0].clone()?.language_name.clone();
    let lang_into = results[1].clone()?.language_name.clone();
    let contents = file_openString("./easy_english_web/translate_item.html").await?;
    let template=TranslateHistoryItemTemplate{
        current_lang:cookie.user_dictionaries[cookie.current_lang_index].language_name.clone(),
        languages:cookie.user_dictionaries.clone(),
        translate_history:trans,
        lang_from:lang_from,
        lang_into:lang_into
    };
    let tpl = Template::new(contents).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(tpl.render(&template)))
}