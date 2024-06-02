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
mod check_auth_api_middleware;
mod check_user_api_middleware;
mod gpt_module;
mod translate_module;
mod google_module;
mod cookie;

use std::env;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_files as fs;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use chat_gpt_rs::api::Api;
use deepl::DeepLApi;
use no_cache_middleware::NoCache;
use tokio::sync::Mutex;
use crate::check_auth_api_middleware::CheckAuthApi;
use crate::check_auth_middleware::CheckAuth;
use crate::check_user_middleware::CheckUser;
use crate::check_db_api_middleware::CheckDbApi;
use crate::check_db_view_middleware::CheckDbView;
use crate::check_user_api_middleware::CheckUserApi;
use crate::controllers::{api_auth_controller, api_controller, api_user_controller, main_controller, settings_controller, view_controller};
use crate::controllers::object_of_controller::RequestResult;
use crate::gpt_module::GptModule;
use crate::models::{MyError, MysqlDB, MysqlInfo};
use crate::translate_module::DeeplModule;
use dotenv::dotenv;
use actix_cors::Cors;
use crate::google_module::GoogleModule;

pub struct StateDb{
    pub mysql_db:Arc<Mutex<MysqlDB>>,
    deepl_api:DeepLApi,
    gpt_api:Arc<Client<OpenAIConfig>>,
    google_module:Arc<GoogleModule>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // let logger=LogManager::new().await;
    // logger.add_log(vec!["error".to_string(), "sqlite".to_string()], "2023".to_string(), "test1".to_string()).await;
    // logger.add_log(vec!["error".to_string(), "sqlite".to_string()], "2023".to_string(), "test2".to_string()).await;
    // logger.add_log(vec!["error".to_string()], "2023".to_string(), "test3".to_string()).await;
    // logger.add_log(vec!["error".to_string()], "2023".to_string(), "test4".to_string()).await;
    // logger.add_log(vec!["error".to_string(), "sqlite".to_string()], "2023".to_string(), "test5".to_string()).await;
    // println!("{}",logger.get_logs_json().await);
    // logger.get_log(vec!["error".to_string()]).await;

    //println!("{}",logger.get_key_json(vec!["error".to_string(),"sqlite".to_string()]).await.to_string());
    let mysql_info=MysqlInfo{ip:env::var("IP").unwrap(),login:env::var("LOGIN").unwrap(),password:env::var("PASSWORD").unwrap(),database:env::var("DATABASE").unwrap(),port:env::var("PORT").unwrap()};
    //let mysql_info=MysqlInfo{ip:"213.226.95.124".to_string(),login:"root_all".to_string(),password:"1".to_string(),database:"easy_english".to_string(),port:"6060".to_string()};
    let mut mysql_db=MysqlDB::new();
    let deepl_api_=DeeplModule::connect(env::var("DEEPL").unwrap()).await;
    let gpt_api_=GptModule::connect().await;
    let res_conn=mysql_db.connect(mysql_info).await;
    match res_conn {
        Ok(_) => {}
        Err(e) => {e.pushlog().await;}
    }
    let state=web::Data::new(StateDb{
        google_module:Arc::new(GoogleModule::init(env::var("GOOGLE_API").unwrap())),
        mysql_db:Arc::new(Mutex::new(mysql_db)),
        deepl_api:deepl_api_.clone(),
        gpt_api:gpt_api_
    });
    println!("START WEB SERVER");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::clone(&state))
            .default_service(web::route().to(settings_controller::m_none))
            .wrap(NoCache)

            .service(fs::Files::new("/public", "./easy_english_web").show_files_listing())

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
                            .service(view_controller::m_learn_main)
                            .service(view_controller::m_translate_main)
                            .service(view_controller::m_translate_history)
                            .service(view_controller::m_translate_history_pagination)
                            .service(view_controller::m_translate_history_item_show)
                    )

            )
            .service(
                web::scope("/settings")
                    .service(settings_controller::m_settings_error)
                    .service(settings_controller::m_restart_server)
            )
            .service(
                web::scope("/api")
                    .wrap(CheckDbApi)
                    .service(api_user_controller::m_text_to_audio)
                    .service(api_controller::m_auth)
                    .service(
                        web::scope("/userspace")
                            .wrap(CheckUserApi)
                            .service(api_user_controller::m_test)
                            .service(api_user_controller::m_set_dictionaries)
                            .service(api_user_controller::m_deepl_translate)
                            .service(api_user_controller::m_gpt_full_formal_translate)
                            .service(api_user_controller::m_gpt_short_formal_translate)
                            .service(api_user_controller::m_gpt_full_speak_translate)
                            .service(api_user_controller::m_gpt_short_speak_translate)
                            .service(api_user_controller::m_save_translate)
                            .service(api_user_controller::m_delete_translated)
                            .service(api_user_controller::m_outauth)

                    )
                    .service(
                        web::scope("/userstart")
                            .wrap(CheckAuthApi)
                            .service(api_auth_controller::m_test)
                            .service(api_auth_controller::m_set_dictionaries)
                    )
            )
    })
        .bind(("0.0.0.0", 3002))?
        .run()
        .await
}