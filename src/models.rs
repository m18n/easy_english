use std::sync::Arc;
use actix_web::{HttpResponse, ResponseError};
use futures_util::future::join_all;
use http::StatusCode;
use ramhorns::Content;
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Str;
use sqlx::{Error, MySqlPool, query};
use sqlx::FromRow;
use thiserror::Error;
use tokio::sync::Mutex;
use crate::base::get_nowtime_str;
use crate::controllers::object_of_controller::{AuthInfo, DictionariesInfo};
use crate::globals::LOGS_DB_ERROR;
#[derive(Error, Debug,Clone)]
pub enum MyError {
    #[error("")]
    SiteError(String), // Автоматично конвертує sqlx::Error у MyError
    // Додайте інші варіанти помилок тут
}
impl MyError{
    pub async fn pushlog(&self){
        match self {
            MyError::SiteError(mess_err) => {
                let mess_err = mess_err.clone();
                let mut log = LOGS_DB_ERROR.lock().await;
                log.push_str(&mess_err);
            }
        }
    }
}
impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {

        return StatusCode::BAD_REQUEST;
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::SiteError(mess_err) => {
                let mess_err = mess_err.clone();
                tokio::spawn(async move{
                    let mut log = LOGS_DB_ERROR.lock().await;
                    log.push_str(&mess_err);
                });

                HttpResponse::Found()
                    .insert_header((http::header::LOCATION, "/settings/error"))
                    .finish()
            }

            // Обробіть інші варіанти помилок тут
        }
    }
}
#[derive(Debug, Serialize, Deserialize,sqlx::FromRow)]
pub struct User{
    id:i32,
    user_name:String,
    password:String,
}
#[derive(Debug, Serialize, Deserialize,sqlx::FromRow,Clone,Content)]
pub struct UserDictionary{
    pub id:i32,
    pub language_name:String,
    pub language_id:i32,
    pub language_level:String,
    pub language_level_id:i32
}
#[derive(Debug, Serialize, Deserialize,sqlx::FromRow,Clone,Content)]
pub struct LanguageSupported{
    pub id:i32,
    pub language_name:String
}
#[derive(Debug, Serialize, Deserialize,sqlx::FromRow,Clone,Content)]
pub struct LanguagesLevels{
    pub id:i32,
    pub level_name:String
}
#[derive(Debug, Serialize, Deserialize, FromRow,Clone,PartialEq)]
pub struct MysqlInfo{
    pub ip:String,
    pub login:String,
    pub password:String,
    pub database:String,
    pub port:String
}
#[derive(Debug, Serialize, Deserialize, FromRow,Clone,PartialEq,Content)]
pub struct Translated{
    pub id:i32,
    pub lang_from_translated_id:i32,
    pub lang_into_translated_id:i32,
    pub translated_text:String,
    pub context_text:String,
    pub deepl_translated:String,
    pub deepl_check_deepl:String,
    pub speak_gpt_translated:String,
    pub speak_deepl_check_gpt_translated:String,
    pub speak_explanation_gpt:String,
    pub formal_gpt_translated:String,
    pub formal_deepl_check_gpt_translated:String,
    pub formal_explanation_gpt:String,
    pub is_full:bool
}
#[derive(Debug, Serialize, Deserialize, FromRow,Clone,PartialEq,Content)]
pub struct TranslatedId{
    pub id:i32,
}
#[derive(Debug, Serialize, Deserialize, FromRow,Clone,PartialEq,Content)]
pub struct SentenceId{
    pub id:i32,
}
#[derive(Debug, Serialize, Deserialize, FromRow,Clone,PartialEq,Content)]
pub struct Dictionary_Sentence{
    pub id:i32,
    pub user_dictionaries:i32,
    pub sentence_from:String,
    pub sentence_from_context:String,
    pub sentence_into:String,
    pub transcription_eng:String,
    pub transcription_ukr:String
}
impl Translated {
    pub fn new()->Self{
        Self{id:-1,lang_from_translated_id:-1,lang_into_translated_id:-1,translated_text:String::new()
            ,context_text:String::new(),deepl_translated:String::new(),deepl_check_deepl:String::new(),speak_gpt_translated:String::new(),
            speak_deepl_check_gpt_translated:String::new(),speak_explanation_gpt:String::new(),formal_gpt_translated:String::new(),
            formal_deepl_check_gpt_translated:String::new(),formal_explanation_gpt:String::new(),is_full:false}
    }
}
impl MysqlInfo {
    fn new()->Self{
        Self{ip:String::new(),login:String::new(),password:String::new(),database:String::new(),port:String::new()}
    }
}
//test
pub struct MysqlDB{
    pub mysql:Option<MySqlPool>,
    pub last_info:MysqlInfo
}
impl MysqlDB{
    pub fn new()->MysqlDB{
        MysqlDB{mysql:None,last_info:MysqlInfo::new()}
    }
    pub async fn disconnect(&mut self){
        self.mysql=None;
    }
    pub async fn connect(&mut self,mysql_info:MysqlInfo)->Result<bool, MyError>{
        let database_url = format!("mysql://{}:{}@{}:{}/{}",mysql_info.login,mysql_info.password,mysql_info.ip,mysql_info.port,mysql_info.database);
        println!("CONNECT INFO: {}",database_url);

        self.mysql=None;
        self.mysql=match MySqlPool::connect(&database_url).await{
            Ok(pool)=>{
                println!("CONNECTION to mysql db successfully");
                let mut log = LOGS_DB_ERROR.lock().await;
                log.clear();
                self.last_info=mysql_info;
                Some(pool)},
            Err(e)=>{
                self.disconnect().await;
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                return Err(MyError::SiteError(str_error))
            },
        };
        Ok(!self.mysql.is_none())
    }
    pub async fn executeSql(mysql_db_m:Arc<Mutex<MysqlDB>>,query:String,error_mess:String)->Result<bool, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let res= sqlx::query(query.as_str())
            .execute(&mysqlpool)
            .await.map_err(|e|{
            println!("ERROR: {}",e);
            let str_error = format!("MYSQL|| {} error: {} \n", get_nowtime_str(),error_mess);
            MyError::SiteError(str_error)
        })?;
        Ok(true)
    }
    pub async fn checkAuth(mysql_db_m:Arc<Mutex<MysqlDB>>,auth_info:AuthInfo)->Result<i32, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let users:Vec<User>= sqlx::query_as("SELECT * FROM users WHERE user_name=?;")
            .bind(auth_info.user_name)
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        if !users.is_empty() && users[0].password==auth_info.password{
            Ok(users[0].id)
        }else{
            Ok(-1)
        }
    }
    pub async fn getUserDictionaries(mysql_db_m:Arc<Mutex<MysqlDB>>,user_id:i32)->Result<Vec<UserDictionary>, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);

        let user_dictionary:Vec<UserDictionary>= sqlx::query_as("SELECT ud.id, ls.language_name, ls.id AS language_id, lv.id AS language_level_id , lv.level_name AS language_level
        FROM user_dictionaries AS ud
        JOIN languages_supported AS ls ON ud.language_id = ls.id
        JOIN language_levels AS lv ON ud.language_level = lv.id
        WHERE ud.user_id = ?")
            .bind(user_id)
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
       Ok(user_dictionary)
    }
    pub async fn getLanguages(mysql_db_m:Arc<Mutex<MysqlDB>>)->Result<Vec<LanguageSupported>, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let languages_supported:Vec<LanguageSupported>= sqlx::query_as("SELECT * FROM languages_supported")
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        Ok(languages_supported)
    }

    pub async fn getLanguagesLevels(mysql_db_m:Arc<Mutex<MysqlDB>>)->Result<Vec<LanguagesLevels>, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let languages_supported:Vec<LanguagesLevels>= sqlx::query_as("SELECT * FROM language_levels")
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        Ok(languages_supported)
    }
    pub async fn getLanguage(mysql_db_m:Arc<Mutex<MysqlDB>>,id_lang:i32)->Result<LanguageSupported, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let languages_supported:Vec<LanguageSupported>= sqlx::query_as("SELECT * FROM languages_supported WHERE id=?")
            .bind(id_lang)
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        if languages_supported.is_empty(){
            let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), "DONT FOUND LANG".to_string());
            return Err(MyError::SiteError(str_error));
        }
        Ok(languages_supported[0].clone())
    }
    pub async fn getLanguageByName(mysql_db_m:Arc<Mutex<MysqlDB>>,lang_name:String)->Result<LanguageSupported, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let languages_supported:Vec<LanguageSupported>= sqlx::query_as("SELECT * FROM languages_supported WHERE language_name=?")
            .bind(lang_name)
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        if languages_supported.is_empty(){
            let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), "DONT FOUND LANG".to_string());
            return Err(MyError::SiteError(str_error));
        }
        Ok(languages_supported[0].clone())
    }
    pub async fn getTranslated(mysql_db_m:Arc<Mutex<MysqlDB>>,start_element:i32,size_element:i32,user_id:i32,lang_id:i32)->Result<Vec<Translated>, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let mut query=String::new();
        if lang_id!=-1 {
            query = format!("SELECT * FROM translation_history WHERE user_id={} AND lang_into_translated_id={} ORDER BY id DESC LIMIT {} OFFSET {} ;"
                                , user_id, lang_id, size_element, start_element);
        }else{
            query = format!("SELECT * FROM translation_history WHERE user_id={} ORDER BY id DESC LIMIT {} OFFSET {} ;"
                                , user_id, size_element, start_element);
        }
        let translated:Vec<Translated>= sqlx::query_as(query.as_str())
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        Ok(translated)
    }
    pub async fn getDictionaries(mysql_db_m:Arc<Mutex<MysqlDB>>,dict_id:i32,start_element:i32,size_element:i32)->Result<Vec<Dictionary_Sentence>, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let mut query=String::new();
        if size_element==0&&start_element==0{
            query = format!("SELECT * FROM anki_sentences WHERE user_dictionaries={} ORDER BY id DESC;"
                            , dict_id);
        }
        else if size_element==0{
            query = format!("SELECT * FROM anki_sentences WHERE user_dictionaries={} ORDER BY id DESC OFFSET {};"
                                , dict_id,start_element);
        }else if start_element==0{
            query = format!("SELECT * FROM anki_sentences WHERE user_dictionaries={} ORDER BY id DESC LIMIT {};"
                            , dict_id,size_element);
        } else {
            query = format!("SELECT * FROM anki_sentences WHERE user_dictionaries={} ORDER BY id DESC LIMIT {} OFFSET {} ;"
                                , dict_id, size_element, start_element);
        }
        let dict:Vec<Dictionary_Sentence>= sqlx::query_as(query.as_str())
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        Ok(dict)
    }
    pub async fn getDictionariesDump(mysql_db_m:Arc<Mutex<MysqlDB>>,dict_id:i32,index_dump:i32)->Result<Vec<Dictionary_Sentence>, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);

        let query = format!("SELECT * FROM anki_sentences WHERE user_dictionaries={} AND id>{} ORDER BY id ASC;"
                        , dict_id, index_dump);
        let dict:Vec<Dictionary_Sentence>= sqlx::query_as(query.as_str())
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        Ok(dict)
    }
    pub async fn getTranslatedItem(mysql_db_m:Arc<Mutex<MysqlDB>>,id_item:i32,user_id:i32)->Result<Translated, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let query=format!("SELECT * FROM translation_history WHERE user_id={} AND id={};",user_id,id_item);
        let translated:Vec<Translated>= sqlx::query_as(query.as_str())
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        if translated.is_empty(){
            let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), "DONT FOUND");
            return Err(MyError::SiteError(str_error));
        }
        Ok(translated[0].clone())
    }

    pub async fn setDictionaries(mysql_db_m:Arc<Mutex<MysqlDB>>,dictionaries_info:DictionariesInfo,user_id:i32)->Result<bool, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let mut tasks_array =Vec::new();
        for i in 0..dictionaries_info.dictionaries_ids.len(){
            let query = format!("INSERT INTO user_dictionaries (user_id,language_id,language_level) VALUES ({},{},{});", user_id,
                                dictionaries_info.dictionaries_ids[i], dictionaries_info.dictionaries_level_ids[i]);
            tasks_array.push(Self::executeSql(mysql_db_m.clone(),query.to_string(),"set dictionaries".to_string()));
        }
        let results=join_all(tasks_array).await;
        for res in results{
            res?;
        }
        Ok(true)
    }
    pub async fn saveTranslate(mysql_db_m:Arc<Mutex<MysqlDB>>,translated_info:Translated,user_id:i32)->Result<bool, MyError>{
        let query=format!("INSERT INTO translation_history (lang_from_translated_id,lang_into_translated_id,translated_text,\
        context_text,deepl_translated,deepl_check_deepl,speak_gpt_translated,speak_deepl_check_gpt_translated,speak_explanation_gpt,\
        formal_gpt_translated,formal_deepl_check_gpt_translated,formal_explanation_gpt,is_full,user_id) VALUES ({},{},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",{},{})",
        translated_info.lang_from_translated_id,translated_info.lang_into_translated_id,translated_info.translated_text,
        translated_info.context_text,translated_info.deepl_translated,translated_info.deepl_check_deepl
                          ,translated_info.speak_gpt_translated,translated_info.speak_deepl_check_gpt_translated,
        translated_info.speak_explanation_gpt,translated_info.formal_gpt_translated,translated_info.formal_deepl_check_gpt_translated,
        translated_info.formal_explanation_gpt,translated_info.is_full,user_id);
        Self::executeSql(mysql_db_m.clone(),query,"save translated".to_string()).await?;
        Ok(true)
    }
    pub async fn deleteTranslated(mysql_db_m:Arc<Mutex<MysqlDB>>,translated_info:TranslatedId,user_id:i32)->Result<bool, MyError>{
        let query=format!("DELETE FROM translation_history WHERE user_id={} AND id={}",user_id,translated_info.id);
        Self::executeSql(mysql_db_m.clone(),query,"delete translated".to_string()).await?;
        Ok(true)
    }
    pub async fn deleteDictionary(mysql_db_m:Arc<Mutex<MysqlDB>>,sentence_id:SentenceId)->Result<bool, MyError>{
        let query=format!("DELETE FROM anki_sentences WHERE id={}",sentence_id.id);
        Self::executeSql(mysql_db_m.clone(),query,"Delete Dictionary".to_string()).await?;
        Ok(true)
    }
    pub async fn setIndexDamp(mysql_db_m:Arc<Mutex<MysqlDB>>,user_dictionary:i32,index:i32)->Result<bool, MyError>{
        let query=format!("UPDATE anki_dump SET sentences_id={} WHERE user_dictionaries={};",index,user_dictionary);
        Self::executeSql(mysql_db_m.clone(),query,"set index dump".to_string()).await?;
        Ok(true)
    }
    pub async fn deleteIndexDamp(mysql_db_m:Arc<Mutex<MysqlDB>>,user_dictionary:i32)->Result<bool, MyError>{
        let query=format!("DELETE FROM anki_dump WHERE user_dictionaries={};",user_dictionary);
        Self::executeSql(mysql_db_m.clone(),query,"delete index dump".to_string()).await?;
        Ok(true)
    }
    pub async fn getNextRecordDamp(mysql_db_m:Arc<Mutex<MysqlDB>>,user_dictionary:i32,id_dump:i32)->Result<i32, MyError> {
        let mysql_db = mysql_db_m.lock().await;
        let mysqlpool = mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let query = format!("SELECT id FROM anki_sentences WHERE id<{} AND user_dictionaries={} ORDER BY id DESC LIMIT 1;", id_dump, user_dictionary);
        let sentences_ids: Vec<i32> = sqlx::query_scalar(query.as_str())
            .fetch_all(&mysqlpool)
            .await
            .map_err(|e| {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        if sentences_ids.is_empty() {
            Ok(-1)
        } else {
            Ok(sentences_ids[0])
        }
    }
    pub async fn getBeforRecordDamp(mysql_db_m:Arc<Mutex<MysqlDB>>,user_dictionary:i32,id_dump:i32)->Result<i32, MyError> {
        let mysql_db = mysql_db_m.lock().await;
        let mysqlpool = mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let query = format!("SELECT id FROM anki_sentences WHERE id>{} AND user_dictionaries={} ORDER BY id ASC LIMIT 1;", id_dump, user_dictionary);
        let sentences_ids: Vec<i32> = sqlx::query_scalar(query.as_str())
            .fetch_all(&mysqlpool)
            .await
            .map_err(|e| {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        if sentences_ids.is_empty() {
            Ok(-1)
        } else {
            Ok(sentences_ids[0])
        }
    }
    pub async fn getIndexDamp(mysql_db_m:Arc<Mutex<MysqlDB>>,user_dictionary:i32)->Result<i32, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let query=format!("SELECT sentences_id FROM anki_dump WHERE user_dictionaries={};",user_dictionary);
        let sentences_ids:Vec<i32>= sqlx::query_scalar(query.as_str())
            .fetch_all(&mysqlpool)
            .await
            .map_err( |e|  {
                let str_error = format!("MYSQL|| {} error: {}\n", get_nowtime_str(), e.to_string());
                MyError::SiteError(str_error)
            })?;
        if sentences_ids.is_empty(){
            Ok(-1)
        }else{
            Ok(sentences_ids[0])
        }
    }
    pub async fn addIndexDamp(mysql_db_m:Arc<Mutex<MysqlDB>>,user_dictionary:i32,sentence_id:i32)->Result<bool, MyError>{
        let query=format!("INSERT INTO anki_dump (user_dictionaries,sentences_id)\
        VALUES ({},{});",user_dictionary,sentence_id);
        Self::executeSql(mysql_db_m.clone(),query,"add index dump".to_string()).await?;
        Ok(true)
    }
    pub async fn addDictionarySentence(mysql_db_m:Arc<Mutex<MysqlDB>>,sentence_info:Dictionary_Sentence)->Result<bool, MyError>{
        let query=format!("INSERT INTO anki_sentences (user_dictionaries,sentence_from,sentence_from_context,sentence_into,transcription_eng,transcription_ukr)\
        VALUES ({},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\")",sentence_info.user_dictionaries,sentence_info.sentence_from,sentence_info.sentence_from_context,sentence_info.sentence_into,sentence_info.transcription_eng,
        sentence_info.transcription_ukr);
        Self::executeSql(mysql_db_m.clone(),query,"delete dictionary".to_string()).await?;
        Ok(true)
    }
}