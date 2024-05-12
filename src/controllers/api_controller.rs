use actix_web::{get, HttpResponse, post, web};
use actix_web::cookie::Cookie;
use crate::base::file_openString;
use crate::controllers::object_of_controller::{AuthInfo, RequestResult};
use crate::jwt::create_token;
use crate::models::{MyError, MysqlDB};
use crate::StateDb;

// URL=/api/*
#[post("/auth")]
pub async fn m_auth(auth_info:web::Json<AuthInfo>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let auth_obj=auth_info.into_inner();
    let res=MysqlDB::checkAuth(state.mysql_db.clone(),auth_obj.clone()).await?;
    if res!=-1 {
        let users_dictionaries=MysqlDB::getUserDictionaries(state.mysql_db.clone(),res).await?;
        let cookie = Cookie::build("refresh_token", create_token(res,auth_obj.user_name.clone(),false,users_dictionaries,0))
            .path("/")
            .http_only(true)
            .finish();
        let mut respon = HttpResponse::Ok().cookie(cookie).json(RequestResult { status: true });
        Ok(respon)
    }else{
        let mut respon = HttpResponse::Ok().json(RequestResult { status: false });
        Ok(respon)
    }

}