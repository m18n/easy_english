use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, post, web};
use crate::base::{get_nowtime_str};
use crate::controllers::object_of_controller::{DictionariesInfo, RequestResult};
use crate::cookie::create_cookie_auth;
use crate::jwt::{Claims};
use crate::models::{MyError, MysqlDB};
use crate::StateDb;
// url controller: /api/userstart/***
#[get("/test")]
pub async fn m_test()->Result<HttpResponse, MyError>{

    Ok(HttpResponse::Ok().content_type("text/html").body("Hello"))
}
#[post("/setdictionaries")]
pub async fn m_set_dictionaries(req:HttpRequest,dictionaries_id:web::Json<DictionariesInfo>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    if let Some(claims) = req.extensions().get::<Claims>(){
        MysqlDB::setDictionaries(state.mysql_db.clone(),dictionaries_id.into_inner(),claims.user_id).await?;
        let user_dictionaries=MysqlDB::getUserDictionaries(state.mysql_db.clone(),claims.user_id).await?;
        let my_claims=Claims{
            user_dictionaries:user_dictionaries,
            current_lang_index:0,
            ..claims.clone()
        };
        let cookie=create_cookie_auth(my_claims.clone());
        let mut respon = HttpResponse::Ok().cookie(cookie).json(RequestResult { status: true });
        Ok(respon)
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }

}