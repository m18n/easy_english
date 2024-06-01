use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, post, web};
use actix_web::cookie::Cookie;
use actix_web::http::header;
use crate::base::{file_openString, get_nowtime_str};
use crate::controllers::object_of_controller::{CurrentLanguage, DictionariesInfo, RequestResult, ResultGptTranslate, ResultTranslate, Translate, TranslateGpt};
use crate::cookie::{create_cookie_auth, create_cookie_auth_clear};
use crate::google_module::GoogleModule;
use crate::gpt_module::GptModule;
use crate::jwt::{Claims};
use crate::models::{MyError, MysqlDB, Translated, TranslatedId};
use crate::render_temps::CurrentLangTemplate;
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
        let my_claims=Claims{

            current_lang_index:new_index,
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

#[get("/outauth")]
pub async fn m_outauth(state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let cookie =create_cookie_auth_clear();
    let respon=HttpResponse::Found()
        .insert_header((header::LOCATION, "/view/login"))
        .cookie(cookie)
        .finish();
    Ok(respon)
}
#[get("/text")]
pub async fn m_text_to_audio(state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let bytes=GoogleModule::text_to_speach(state.google_module.clone(),"hej dåre".to_string()).await;
    let mut res_bytes=Vec::new();
    match bytes {
        Ok(bytes) => {
            res_bytes=bytes;
        }
        Err(e) => {

        }
    }
    //let bytes=GptModule::text_to_audio(state.gpt_api.clone(),"Det finns en uppfattning om att vi föds med ett stort antal hjärnceller".to_string()).await?;
    // let respon=HttpResponse::Found()
    //     .insert_header((header::LOCATION, "/view/login"))
    //     .finish();
    Ok(HttpResponse::Ok()
        .content_type("audio/mpeg").body(web::Bytes::from(res_bytes)))

}
#[post("/translator/deepltranslate")]
pub async fn m_deepl_translate(req:HttpRequest,translate_info:web::Json<Translate>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let text_=DeeplModule::translate(state.deepl_api.clone(),translate_info.from_lang.clone(),translate_info.into_lang.clone(),translate_info.text.clone()).await?;
    Ok(HttpResponse::Ok().json(ResultTranslate{text:text_}))
}
#[post("/translator/gpttranslate")]
pub async fn m_gpt_translate(req:HttpRequest,translate_info:web::Json<TranslateGpt>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let query=format!("Будь вчителем {} мови.
Відповідайте лише у форматі JSON із такими даними:
{{
      \"sentence:\"\",
      \"explanation\":\"\"
}}
Як сказати моє речення {} мовою, розмовною {} мовою, щоб передати той самий зміст, та сенс. Моє речення написане {} мовою: \"{}\".
Я також наведу вам значення речення, яке я хотів передати: \"{}\".
У відповідь:
Поле \"sentence\" має містити твоє речення яке ти створив. Поле \"explanation\" повинно містити коротке пояснення вашого стовреного речення, твоє поясення має бути написано {} мовою.",
   translate_info.into_lang.clone(),translate_info.into_lang.clone(),translate_info.into_lang.clone(),translate_info.from_lang.clone(),translate_info.text.clone(),
    translate_info.text_explain.clone(),translate_info.from_lang.clone());
    let translate:Result<ResultGptTranslate,MyError>=GptModule::send(state.gpt_api.clone(),query).await;
    match translate {
        Ok(result) => {
            Ok(HttpResponse::Ok().json(result))
        }
        Err(error) => {
            error.pushlog().await;

            let res_err=ResultGptTranslate{sentence:"Error, please try again".to_string(),explanation:"Error, please try again".to_string()};
            Ok(HttpResponse::Ok().json(res_err))
        }
    }
}
#[post("/translator/savetranslated")]
pub async fn m_save_translate(req:HttpRequest,translate_info:web::Json<Translated>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    if let Some(claims) = req.extensions().get::<Claims>(){
        MysqlDB::saveTranslate(state.mysql_db.clone(),translate_info.into_inner(),claims.user_id).await?;
        Ok(HttpResponse::Ok().json(RequestResult{status:true}))
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }

}
#[post("/translator/history/deleteitem")]
pub async fn m_delete_translated(req:HttpRequest,translate_info:web::Json<TranslatedId>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    if let Some(claims) = req.extensions().get::<Claims>(){
        MysqlDB::deleteTranslated(state.mysql_db.clone(),translate_info.into_inner(),claims.user_id).await?;
        Ok(HttpResponse::Ok().json(RequestResult{status:true}))
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }

}
