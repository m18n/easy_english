use std::sync::Arc;
use actix_web::{HttpResponse, ResponseError};
use futures_util::future::join_all;
use http::StatusCode;
use ramhorns::Content;
use serde::{Deserialize, Serialize};
use sqlx::{Error, MySqlPool, query};
use sqlx::FromRow;
use thiserror::Error;
use tokio::sync::Mutex;
use crate::base::get_nowtime_str;
use crate::controllers::object_of_controller::AuthInfo;
use crate::globals::LOGS_DB_ERROR;
#[derive(Error, Debug)]
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
    pub language_id:i32
}
#[derive(Debug, Serialize, Deserialize,sqlx::FromRow,Clone,Content)]
pub struct LanguageSupported{
    pub id:i32,
    pub language_name:String
}
#[derive(Debug, Serialize, Deserialize, FromRow,Clone,PartialEq)]
pub struct MysqlInfo{
    pub ip:String,
    pub login:String,
    pub password:String,
    pub database:String,
    pub port:String
}

pub struct MysqlDB{
    pub mysql:Option<MySqlPool>,

}
impl MysqlDB{
    pub fn new()->MysqlDB{
        MysqlDB{mysql:None}
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
        let user_dictionary:Vec<UserDictionary>= sqlx::query_as("SELECT ud.id, ls.language_name, ls.id AS language_id
        FROM user_dictionaries AS ud
        JOIN languages_supported AS ls ON ud.language_id = ls.id
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
    pub async fn setDictionaries(mysql_db_m:Arc<Mutex<MysqlDB>>,dictionaries_id:Vec<i32>,user_id:i32)->Result<bool, MyError>{
        let mysql_db=mysql_db_m.lock().await;
        let mysqlpool=mysql_db.mysql.as_ref().unwrap().clone();
        drop(mysql_db);
        let mut tasks_array =Vec::new();
        for element in dictionaries_id{
            let query = format!("INSERT INTO user_dictionaries (user_id,language_id) VALUES ({},{});",user_id,element);
            tasks_array.push(Self::executeSql(mysql_db_m.clone(),query.to_string(),"set dictionaries".to_string()));
        }
        let results=join_all(tasks_array).await;
        for res in results{
            res?;
        }
        Ok(true)
    }
}