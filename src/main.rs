mod models;
mod globals;
mod base;
mod controllers;
mod no_cache_middleware;
mod check_db_view_middleware;

use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_files as fs;
use no_cache_middleware::NoCache;
use tokio::sync::Mutex;
use crate::check_db_view_middleware::CheckDbView;
use crate::controllers::main_controller;
use crate::models::{MysqlDB, MysqlInfo};
struct StateDb{
    mysql_db:Arc<Mutex<MysqlDB>>,

}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let logger=LogManager::new().await;
    // logger.add_log(vec!["error".to_string(), "sqlite".to_string()], "2023".to_string(), "test1".to_string()).await;
    // logger.add_log(vec!["error".to_string(), "sqlite".to_string()], "2023".to_string(), "test2".to_string()).await;
    // logger.add_log(vec!["error".to_string()], "2023".to_string(), "test3".to_string()).await;
    // logger.add_log(vec!["error".to_string()], "2023".to_string(), "test4".to_string()).await;
    // logger.add_log(vec!["error".to_string(), "sqlite".to_string()], "2023".to_string(), "test5".to_string()).await;
    // println!("{}",logger.get_logs_json().await);
    // logger.get_log(vec!["error".to_string()]).await;

    //println!("{}",logger.get_key_json(vec!["error".to_string(),"sqlite".to_string()]).await.to_string());
    let mysql_info=MysqlInfo{ip:"213.226.95.124".to_string(),login:"root".to_string(),password:"".to_string(),database:"easy_english".to_string(),port:"6060".to_string()};
    let mut mysql_db=MysqlDB::new();
    let res_conn=mysql_db.connect(mysql_info).await;
    match res_conn {
        Ok(_) => {}
        Err(e) => {e.pushlog().await;}
    }
    let state=web::Data::new(StateDb{
        mysql_db:Arc::new(Mutex::new(mysql_db)),
    });
    println!("START WEB SERVER");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&state))
            .wrap(NoCache)
            .wrap(CheckDbView)
            .service(fs::Files::new("/public", "./azs_site/public").show_files_listing())
            .service(main_controller::m_global_main)
    })
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}