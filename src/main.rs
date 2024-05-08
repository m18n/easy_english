mod models;
mod globals;
mod base;
mod controllers;
mod no_cache_middleware;
mod check_db_view_middleware;
mod check_db_api_middleware;
mod jwt;
mod check_user_middleware;
mod check_auth_middleware;
mod render_temps;

use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_files as fs;
use no_cache_middleware::NoCache;
use tokio::sync::Mutex;
use crate::check_auth_middleware::CheckAuth;
use crate::check_user_middleware::CheckUser;
use crate::check_db_api_middleware::CheckDbApi;
use crate::check_db_view_middleware::CheckDbView;
use crate::controllers::{api_controller, main_controller, settings_controller, view_controller};
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
    let mysql_info=MysqlInfo{ip:"213.226.95.124".to_string(),login:"root_all".to_string(),password:"1".to_string(),database:"easy_english".to_string(),port:"6060".to_string()};
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

            .service(fs::Files::new("/public", "./easy_english_web").show_files_listing())
            .service(
                web::scope("/")
                    .wrap(CheckDbView)
                    .service(main_controller::m_global_main)

            )
            .service(
                web::scope("/view")
                    .wrap(CheckDbView)
                    .service(view_controller::m_login)
                    .service(
                        web::scope("/userstart")
                            .wrap(CheckAuth)
                            .service(view_controller::m_init_dictionaries)
                    )
                    .service(
                        web::scope("/userspace")
                            .wrap(CheckUser)
                            .service(view_controller::m_main)
                    )

            )
            .service(
                web::scope("/settings")
                    .service(settings_controller::m_settings_error)
            )
            .service(
                web::scope("/api")
                    .wrap(CheckDbApi)
                    .service(api_controller::m_auth)
            )
    })
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}