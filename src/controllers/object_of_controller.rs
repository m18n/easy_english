use ramhorns::Content;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone)]
pub struct AuthInfo{
    pub user_name:String,
    pub password:String
}
#[derive(Deserialize,Serialize,Debug)]
pub struct RequestResult{
    pub status:bool,
}
#[derive(Serialize, Deserialize)]
pub struct ErrorDb{
    pub error:bool
}
#[derive(Deserialize,Serialize,Clone)]
pub struct DictionariesInfo{
    pub dictionaries_ids:Vec<i32>,
    pub dictionaries_level_ids:Vec<i32>
}
#[derive(Deserialize,Serialize,Clone,Content)]
pub struct CurrentLanguage{
    pub current_lang:String
}
#[derive(Deserialize,Serialize,Clone,Content)]
pub struct DeleteTranslate{
    pub id_record:i32
}
#[derive(Deserialize,Serialize,Clone)]
pub struct Translate{
    pub from_lang:String,
    pub into_lang:String,
    pub text:String,
}
#[derive(Deserialize,Serialize,Clone)]
pub struct TranslateGpt{
    pub from_lang:String,
    pub into_lang:String,
    pub text:String,
    pub text_explain:String,
}
#[derive(Deserialize,Serialize,Clone)]
pub struct ResultTranslate{
    pub text:String
}
#[derive(Deserialize,Serialize,Clone)]
pub struct ResultGptTranslate{
    pub sentence:String,
    pub explanation:String,
}


