use actix_web::{HttpResponse, post, web};
use crate::controllers::object_of_controller::{AuthInfo, RequestResult};
use crate::cookie::create_cookie_auth;
use crate::jwt::{Claims};
use crate::models::{MyError, MysqlDB};
use crate::StateDb;
// url controller: /api/***
#[post("/auth")]
pub async fn m_auth(auth_info:web::Json<AuthInfo>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let auth_obj=auth_info.into_inner();
    let res=MysqlDB::checkAuth(state.mysql_db.clone(),auth_obj.clone()).await?;
    if res!=-1 {
        let users_dictionaries=MysqlDB::getUserDictionaries(state.mysql_db.clone(),res).await?;
        let mut claims=Claims::new();
        claims.user_id=res;
        claims.user_name=auth_obj.user_name.clone();
        claims.admin=false;
        claims.user_dictionaries=users_dictionaries;
        claims.current_lang_index=0;
        let cookie=create_cookie_auth(claims.clone());
        let mut respon = HttpResponse::Ok().cookie(cookie).json(RequestResult { status: true });
        Ok(respon)
    }else{
        let mut respon = HttpResponse::Ok().json(RequestResult { status: false });
        Ok(respon)
    }

}
