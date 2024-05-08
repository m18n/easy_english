use actix_web::{HttpResponse, ResponseError};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Error, MySqlPool, query};
use sqlx::FromRow;
use thiserror::Error;
use crate::base::get_nowtime_str;
use crate::globals::LOGS_DB_ERROR;
#[derive(Error, Debug)]
pub enum MyError {
    #[error("")]
    DatabaseError(String), // Автоматично конвертує sqlx::Error у MyError
    // Додайте інші варіанти помилок тут
}
impl MyError{
    pub async fn pushlog(&self){
        match self {
            MyError::DatabaseError(mess_err) => {
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
            MyError::DatabaseError(mess_err) => {
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
                return Err(MyError::DatabaseError(str_error))
            },
        };
        Ok(!self.mysql.is_none())
    }
}