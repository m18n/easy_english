use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, post, web};
use actix_web::cookie::Cookie;
use crate::base::{file_openString, get_nowtime_str};
use crate::controllers::object_of_controller::{CurrentLanguage, DictionariesId, RequestResult, ResultGptTranslate, ResultTranslate, Translate};
use crate::gpt_module::GptModule;
use crate::jwt::{Claims, create_token};
use crate::models::{MyError, MysqlDB};
use crate::render_temps::CurrentLang;
use crate::StateDb;
use crate::translate_module::DeeplModule;

#[get("/test")]
pub async fn m_test()->Result<HttpResponse, MyError>{

    Ok(HttpResponse::Ok().content_type("text/html").body("Hello"))
}
#[post("/setcurrentlang")]
pub async fn m_set_dictionaries(req:HttpRequest,current_lang:web::Json<CurrentLanguage>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    if let Some(claims) = req.extensions().get::<Claims>(){
        let mut new_index:usize=0;
        let result = claims.user_dictionaries.iter().enumerate().find(|(_, &ref x)| x.language_name==current_lang.current_lang);
        match result {
            Some((index, element)) => {
                new_index=index;
            },
            None => {

            }
        }
        let cookie = Cookie::build("refresh_token", create_token(claims.user_id,claims.user_name.clone(),claims.admin,claims.user_dictionaries.clone(),new_index))
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
#[post("/deepltranslate")]
pub async fn m_deepl_translate(req:HttpRequest,translate_info:web::Json<Translate>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let text_=DeeplModule::translate(state.deepl_api.clone(),translate_info.from_lang.clone(),translate_info.into_lang.clone(),translate_info.text.clone()).await?;
    Ok(HttpResponse::Ok().json(ResultTranslate{text:text_}))
}
#[post("/gpttranslate")]
pub async fn m_gpt_translate(req:HttpRequest,translate_info:web::Json<Translate>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let query=format!("You should provide the answer only in Json in the format of the following data:
    {{
    \"translate\":\"\",
    \"explanation\":\"\",
    }}
Adapt and translate this {} sentence to {} conversational level, and translate it exactly according to the content and its meaning: \"{}\"
There should be a translation in the \"translate\" variable. In the \"explanation\" varialbe, there should be a short explanation of your translation, in {}.",
    translate_info.from_lang.clone(),translate_info.into_lang.clone(),translate_info.text.clone(),translate_info.from_lang.clone());
    let translate:ResultGptTranslate=GptModule::send(state.gpt_api.clone(),query).await?;
    Ok(HttpResponse::Ok().json(translate))
}