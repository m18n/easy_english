use std::sync::Arc;
use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs
    },
    Client,
};
use std::env;
use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionResponseFormat, ChatCompletionResponseFormatType};


use serde::de::DeserializeOwned;
use serde::Deserialize;
use sqlx::encode::IsNull::No;
use crate::base::get_nowtime_str;
use crate::models::MyError;

pub struct GptModule{
    api:Option<Client<OpenAIConfig>>,
}
// Передаємо запитт
impl GptModule {
    pub async fn connect()->Arc<Client<OpenAIConfig>>{
        let client = Client::new();
        Arc::new(client)
    }

    pub async fn send<T>(client:Arc<Client<OpenAIConfig>>, request:String) -> std::result::Result<T,MyError>
        where
            T: DeserializeOwned,{
        // if api{
        //     let str_error = format!("GPT|| {} error: DONT CONNECT WITH API\n", get_nowtime_str());
        //     return Err(MyError::SiteError(str_error));
        // }

        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-4o")
            .response_format(ChatCompletionResponseFormat{r#type:ChatCompletionResponseFormatType::JsonObject})
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(request)
                    .build().map_err(|e|{
                    let str_error = format!("GPT|| {} error: ERROR CONTENT\n", get_nowtime_str());
                    MyError::SiteError(str_error)
                })?
                    .into(),
            ])
            .build().map_err(|e|{
            let str_error = format!("GPT|| {} error: ERROR CONTENT\n", get_nowtime_str());
            MyError::SiteError(str_error)
        })?;
        let response = client.chat().create(request).await;
        if let Ok(response) = response {
            let content=response.choices[0].message.content.clone().unwrap_or(String::new());
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