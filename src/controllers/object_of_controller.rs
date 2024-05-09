use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone)]
pub struct AuthInfo{
    pub user_name:String,
    pub password:String
}
#[derive(Deserialize,Serialize)]
pub struct RequestResult{
    pub status:bool,
}
#[derive(Serialize, Deserialize)]
pub struct ErrorDb{
    pub error:bool
}
#[derive(Deserialize,Serialize,Clone)]
pub struct DictionariesId{
    pub dictionaries_id:Vec<i32>
}