use std::env;
use std::ffi::c_long;
use std::os::linux::raw::stat;
use actix_files::NamedFile;
use actix_web::{get, HttpMessage, HttpRequest, HttpResponse, post, Responder, web};
use actix_web::cookie::Cookie;
use actix_web::http::header;
use async_std::path::PathBuf;
use std::fs;
use actix_web::web::Json;
use serde::de::Unexpected::Str;
use crate::base::{file_openString, get_nowtime_str};
use crate::controllers::object_of_controller::{CurrentLanguage, DictionariesInfo, RequestResult, ResultAnkiGpt, ResultGptCheck, ResultGptPuzzle, ResultGptTranscript, ResultGptTranslate, ResultTranslate, Sentences, SentencesLang, TextToSpeach, Translate, TranslateGpt};
use crate::cookie::{create_cookie_auth, create_cookie_auth_clear};
use crate::generate_anki::generate_anki;
use crate::google_module::GoogleModule;
use crate::gpt_module::GptModule;
use crate::jwt::{Claims};
use crate::models::{Dictionary_Sentence, MyError, MysqlDB, SentenceId, Translated, TranslatedId};
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
#[get("/check")]
pub async fn m_check(state: web::Data<StateDb>)->Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok().json(RequestResult{status:true}))
}
#[post("/text")]
pub async fn m_text_to_audio(text_:web::Json<TextToSpeach>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let bytes=GoogleModule::text_to_speach(state.google_module.clone(),text_.text.clone(),text_.name_lang.clone()).await;
    let mut res_bytes=Vec::new();
    match bytes {
        Ok(bytes) => {
            res_bytes=bytes;
        }
        Err(e) => {

        }
    }
    //let bytes=GptModule:text_to_audio(state.gpt_api.clone(),"Det finns en uppfattning om att vi föds med ett stort antal hjärnceller".to_string()).await?;
    // let rspon=HttpResponse::Found()
    //     .insert_header((header::LOCATION, "/view/login"))
    //     .finish();
    Ok(HttpResponse::Ok()
        .content_type("audio/mpeg").body(web::Bytes::from(res_bytes)))

}
#[post("/text/check")]
pub async fn m_text_check(text_:web::Json<SentencesLang>,state: web::Data<StateDb>)->Result<Json<ResultAnkiGpt>, MyError>{
    let text=text_.into_inner();
    let query=format!(r#" Я тобі надам українське речення з контекстом та {}.
Українське речення: "{}"
Контекст українського речення: "{}"
{} речення: "{}"
Ти маєш надати у відповді 2 параметри.
Перший це "assessment" на скільки хорошиї переклад з українського в тому контексті на {} від 0 до 100, це звісно приблизно.
Другий це "correct_translation" відкорегований переклад мого речення на англійську мову.
        Відповідь надай в JSON. У форматі об'єкту:
        {{
            "assessment":,
            "correct_translation":"",
        }}
        "#,text.lang_name,text.sentence_from,text.sentence_from_context,text.lang_name,text.sentence_into,text.lang_name);
    let gpt_check:Result<ResultGptCheck,MyError>=GptModule::send(state.gpt_api.clone(),query).await;
    let mut res_check=ResultGptCheck{assessment:-1,correct_translation:String::new()};
    let mut res_anki=ResultAnkiGpt{assessment:-1,correct_translation:String::new(),words_puzzle:Vec::new(),words_correct:Vec::new()};
    match gpt_check {
        Ok(result) => {
            res_check=result;
        }
        Err(error) => {
            return Ok(Json(res_anki));
        }
    }
    let words: Vec<String> = res_check.correct_translation.split_whitespace().map(|s| s.to_string())
        .collect();
    let size_words=words.len()*2;
    let query=format!(r#" Я тобі надам українське речення з контекстом та {}.
Українське речення: "{}"
Контекст українського речення: "{}"
{} речення: "{}"
Ти маєш надати у відповдь 1 параметр.
Перший — "words_puzzle", я хочу зібрати {} речень як пазли, для цього мені потрібно, щоб ти згенерував масив з {} слів, які б мене заплутали, тільки не згадуйте ті, які вже є в реченні для {} речень.
        Відповідь надай в JSON. У форматі об'єкту:
        {{
            "words_puzzle":[""],
        }}
        "#,text.lang_name,text.sentence_from,text.sentence_from_context,text.lang_name,res_check.correct_translation,text.lang_name,size_words,text.lang_name);
    let gpt_puzzle:Result<ResultGptPuzzle,MyError>=GptModule::send(state.gpt_api.clone(),query).await;
    let mut res_puzzle=ResultGptPuzzle{words_puzzle:Vec::new()};
    match gpt_puzzle {
        Ok(result) => {
            res_puzzle=result;
        }
        Err(error) => {
            return Ok(Json(res_anki));
        }
    }
    res_anki=ResultAnkiGpt{assessment:res_check.assessment,correct_translation:res_check.correct_translation
        ,words_puzzle:res_puzzle.words_puzzle,words_correct:words};
    Ok(Json(res_anki))
}
#[post("/translator/deepl/translate")]
pub async fn m_deepl_translate(req:HttpRequest,translate_info:web::Json<Translate>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let text_=DeeplModule::translate(state.deepl_api.clone(),translate_info.from_lang.clone(),translate_info.into_lang.clone(),translate_info.text.clone()).await?;
    Ok(HttpResponse::Ok().json(ResultTranslate{text:text_}))
}
// /translator/gpt/full/translate
// /translator/gpt/short/translate

#[post("/translator/gpt/full/speak/translate")]
pub async fn m_gpt_full_speak_translate(req:HttpRequest,translate_info:web::Json<TranslateGpt>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
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
#[post("/translator/gpt/full/formal/translate")]
pub async fn m_gpt_full_formal_translate(req:HttpRequest,translate_info:web::Json<TranslateGpt>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let query=format!("Будь вчителем {} мови.
Відповідайте лише у форматі JSON із такими даними:
{{
      \"sentence:\"\",
      \"explanation\":\"\"
}}
Як сказати моє речення {} мовою, формальною {} мовою, щоб передати той самий зміст, та сенс. Моє речення написане {} мовою: \"{}\".
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
#[post("/translator/gpt/short/speak/translate")]
pub async fn m_gpt_short_speak_translate(req:HttpRequest,translate_info:web::Json<TranslateGpt>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let query=format!("Будь вчителем {} мови.
Відповідайте лише у форматі JSON із такими даними:
{{
      \"sentence:\"\",
      \"explanation\":\"\"
}}
Як сказати моє речення {} мовою, розмовною {} мовою, щоб передати той самий зміст, та сенс. Моє речення написане {} мовою: \"{}\".
Я також наведу вам значення речення, яке я хотів передати: \"{}\".
По можливості скороти речення, але щоб зміст та сенс не втратився.
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
#[post("/translator/gpt/short/formal/translate")]
pub async fn m_gpt_short_formal_translate(req:HttpRequest,translate_info:web::Json<TranslateGpt>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let query=format!("Будь вчителем {} мови.
Відповідайте лише у форматі JSON із такими даними:
{{
      \"sentence:\"\",
      \"explanation\":\"\"
}}
Як сказати моє речення {} мовою, формальною {} мовою, щоб передати той самий зміст, та сенс. Моє речення написане {} мовою: \"{}\".
Я також наведу вам значення речення, яке я хотів передати: \"{}\".
По можливості скороти речення, але щоб зміст та сенс не втратився.
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

#[post("/dictionary/add")]
pub async fn m_dictionary_addnewsentence(req:HttpRequest,sentences_info:web::Json<Sentences>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    if let Some(claims) = req.extensions().get::<Claims>(){
        let query=format!("Напиши дві транскрипції для цього {} речення \"{}\". \
        Одна траксрипція це звичайна IPA а друга це адаптована під укараїнську мову.\
        Відповідь надай в JSON. У форматі об'єкту:\
        {{
            \"ipa\":\"\",
            \"ipa_ukr\":\"\",
        }}
        ",claims.user_dictionaries[claims.current_lang_index].language_name,sentences_info.sentence_into);
        let user_dict=claims.user_dictionaries[claims.current_lang_index].id;
        let translate:Result<ResultGptTranscript,MyError>=GptModule::send(state.gpt_api.clone(),query).await;
        match translate {
            Ok(result) => {
                let sentences_info=sentences_info.into_inner();
                let dict=Dictionary_Sentence{id:0,user_dictionaries:user_dict,
                    sentence_from:sentences_info.sentence_from,sentence_from_context:sentences_info.sentence_from_context,sentence_into:sentences_info.sentence_into,transcription_eng:result.ipa,transcription_ukr:result.ipa_ukr};
                let index=MysqlDB::getIndexDamp(state.mysql_db.clone(),user_dict).await?;
                MysqlDB::addDictionarySentence(state.mysql_db.clone(),dict).await?;
                let sentence=MysqlDB::getDictionaries(state.mysql_db.clone(),user_dict,0,1).await?;
                if index==-1{
                    MysqlDB::addIndexDamp(state.mysql_db.clone(),user_dict,sentence[0].id).await?;
                }

            }
            Err(error) => {

            }
        }


        Ok(HttpResponse::Ok().json(RequestResult{status:true}))
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }

}
#[post("/dictionary/deleteitem")]
pub async fn m_dictionary_deleteitem(req:HttpRequest,sentences_info:web::Json<SentenceId>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    if let Some(claims) = req.extensions().get::<Claims>(){
        let sentences_info=sentences_info.into_inner();
        let user_dict=claims.user_dictionaries[claims.current_lang_index].id;
        let index=MysqlDB::getIndexDamp(state.mysql_db.clone(),user_dict).await?;
        if index==sentences_info.id{
            let mut new_id=-1;
            let next_id=MysqlDB::getNextRecordDamp(state.mysql_db.clone(),user_dict,sentences_info.id).await?;
            if next_id==-1{
                let befor_id=MysqlDB::getBeforRecordDamp(state.mysql_db.clone(),user_dict,sentences_info.id).await?;
                if befor_id!=-1{
                    new_id=befor_id;
                }
            }else{
                new_id=next_id;
            }
            if new_id==-1{
                MysqlDB::deleteIndexDamp(state.mysql_db.clone(),user_dict).await?;
            }else{
                MysqlDB::setIndexDamp(state.mysql_db.clone(), user_dict, new_id).await?;
            }
        }
        MysqlDB::deleteDictionary(state.mysql_db.clone(),sentences_info.clone()).await?;
        Ok(HttpResponse::Ok().json(RequestResult{status:true}))
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }

}
#[post("/dictionary/setindexdump")]
pub async fn m_dictionary_set_indexdump(req:HttpRequest,sentences_info:web::Json<SentenceId>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    if let Some(claims) = req.extensions().get::<Claims>(){
        let user_dict=claims.user_dictionaries[claims.current_lang_index].id;
        MysqlDB::setIndexDamp(state.mysql_db.clone(),user_dict,sentences_info.into_inner().id).await?;
        Ok(HttpResponse::Ok().json(RequestResult{status:true}))
    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }

}
struct FileToDelete(PathBuf);

impl Drop for FileToDelete {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
#[get("/dictionary/getfromindexdump")]
pub async fn m_dictionary_get_fromindexdump(req:HttpRequest,state: web::Data<StateDb>)->Result<impl Responder, MyError>{
    if let Some(claims) = req.extensions().get::<Claims>(){
        let user_dict=claims.user_dictionaries[claims.current_lang_index].id;
        let lang_name=claims.user_dictionaries[claims.current_lang_index].language_name.clone();
        let index_dump=MysqlDB::getIndexDamp(state.mysql_db.clone(),user_dict).await?;
        let sentences=MysqlDB::getDictionariesDump(state.mysql_db.clone(),user_dict,index_dump).await?;
        let string=generate_anki(user_dict,sentences,lang_name);
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();
        let file_path = std::path::PathBuf::from(env!("HOME"))
            .as_path()
            .join(exe_dir)
            .join(string.clone());

        let file = actix_files::NamedFile::open_async(file_path.clone()).await.unwrap();
        let mut response = file.into_response(&req);

        response.headers_mut().insert(
            header::CONTENT_DISPOSITION,
            header::HeaderValue::from_str(format!("attachment; filename=\"{}\"", string).as_str()).unwrap()
        );

        // Додати розширення для зберігання шляху до файлу
        response.extensions_mut().insert(FileToDelete(PathBuf::from(file_path)));
        let first=MysqlDB::getDictionaries(state.mysql_db.clone(),user_dict,0,1).await?;
        MysqlDB::setIndexDamp(state.mysql_db.clone(),user_dict,first[0].id).await?;
        Ok(response)


    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }

}
#[get("/dictionary/getdump")]
pub async fn m_dictionary_get_dump(req:HttpRequest,state: web::Data<StateDb>)->Result<impl Responder, MyError>{
    if let Some(claims) = req.extensions().get::<Claims>(){
        let user_dict=claims.user_dictionaries[claims.current_lang_index].id;
        let lang_name=claims.user_dictionaries[claims.current_lang_index].language_name.clone();
        let sentences=MysqlDB::getDictionaries(state.mysql_db.clone(),user_dict,0,0).await?;
        let string=generate_anki(user_dict,sentences,lang_name);
        let exe_path = env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();
        let file_path = std::path::PathBuf::from(env!("HOME"))
            .as_path()
            .join(exe_dir)
            .join(string.clone());

        let file = actix_files::NamedFile::open_async(file_path.clone()).await.unwrap();
        let mut response = file.into_response(&req);

        response.headers_mut().insert(
            header::CONTENT_DISPOSITION,
            header::HeaderValue::from_str(format!("attachment; filename=\"{}\"", string).as_str()).unwrap()
        );

        // Додати розширення для зберігання шляху до файлу
        response.extensions_mut().insert(FileToDelete(PathBuf::from(file_path)));
        let first=MysqlDB::getDictionaries(state.mysql_db.clone(),user_dict,0,1).await?;
        MysqlDB::setIndexDamp(state.mysql_db.clone(),user_dict,first[0].id).await?;
        Ok(response)


    }else{
        let str_error = format!("LOGIC|| {} error: IT IS NOT SITE WITH AUTH\n", get_nowtime_str());
        return Err(MyError::SiteError(str_error));
    }

}