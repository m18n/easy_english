use std::sync::Arc;
use chat_gpt_rs::prelude::*;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use sqlx::encode::IsNull::No;
use crate::base::get_nowtime_str;
use crate::models::MyError;

pub struct GptModule{
    api:Option<Api>,
}
// Передаємо запитт
impl GptModule {
    pub async fn connect(&mut self,token:String){
        let token = Token::new(token);
        self.api = Some(Api::new(token));
    }
    pub fn new()->Self{
        Self{api:None}
    }
    pub async fn send<T>(&mut self, request:String) -> std::result::Result<T,MyError>
        where
            T: DeserializeOwned,{
        if self.api.is_none(){
            let str_error = format!("GPT|| {} error: DONT CONNECT WITH API\n", get_nowtime_str());
            return Err(MyError::SiteError(str_error));
        }
        let api=self.api.as_mut().unwrap();
        let request = Request {
            model: Model::Gpt4,
            messages: vec![Message {
                role: "user".to_string(),
                content: request,
            }],
            ..Default::default()
        };

        let response = api.chat(request).await;
        if let Ok(response) = response {
            let content=response.choices[0].message.content.clone();

            match serde_json::from_str::<T>(content.as_str()) {
                Ok(data) => Ok(data),
                Err(e) => {
                    // Тут можна здійснювати додаткову обробку помилок
                    let str_error = format!("GPT|| {} error: PARSE GPT\n", get_nowtime_str());
                    Err(MyError::SiteError(str_error))
                }
            }
        } else {
            let str_error = format!("GPT|| {} error: GET RESPONSE GPT\n", get_nowtime_str());
            Err(MyError::SiteError(str_error))
        }

    }
}