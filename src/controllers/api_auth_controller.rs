use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, post, web};
use actix_web::cookie::Cookie;
use crate::base::{file_openString, get_nowtime_str};
use crate::controllers::object_of_controller::{AuthInfo, DictionariesId, RequestResult};
use crate::jwt::{Claims, create_token};
use crate::models::{MyError, MysqlDB};
use crate::StateDb;

#[get("/test")]
pub async fn m_test()->Result<HttpResponse, MyError>{

    Ok(HttpResponse::Ok().content_type("text/html").body("Hello"))
}
#[post("/setdictionaries")]
pub async fn m_set_dictionaries(req:HttpRequest,dictionaries_id:web::Json<DictionariesId>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    if let Some(claims) = req.extensions().get::<Claims>(){
        MysqlDB::setDictionaries(state.mysql_db.clone(),dictionaries_id.into_inner().dictionaries_id,claims.user_id).await?;
        let user_dictionaries=MysqlDB::getUserDictionaries(state.mysql_db.clone(),claims.user_id).await?;
        let cookie = Cookie::build("refresh_token", create_token(claims.user_id,claims.user_name.clone(),claims.admin,user_dictionaries,0))
            .path("/")
            .http_only(true)
            .finish();
        let mut respon = HttpResponse::Ok().cookie(cookie).json(RequestResult { status: true });
        Ok(respon)
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }

}