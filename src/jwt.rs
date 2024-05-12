use chrono::Utc;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use crate::models::{MysqlInfo, UserDictionary};
use crate::render_temps::CurrentLang;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Claims {
    pub user_id: i32,
    pub user_name: String,
    pub user_dictionaries:Vec<UserDictionary>,
    pub admin: bool,
    pub current_lang_index:usize,
    pub exp: usize,
}
impl Claims{
    pub fn new()->Self{
        Self{user_id:0,user_name:String::new(),user_dictionaries:Vec::new(),admin:false,current_lang_index:0,exp:0}
    }
}
pub fn create_token(user_id_:i32,user_name_:String,admin_:bool,user_dictionaries_:Vec<UserDictionary>,current_lang_index_:usize)->String{
    let my_claims = Claims {
        user_id:user_id_,
        user_name:user_name_,
        admin:admin_,
        current_lang_index:current_lang_index_,
        exp:10000000000,
        user_dictionaries:user_dictionaries_
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
        .unwrap();
    token
}

pub fn validate_token(token:String) -> Result<Claims,bool> {

    let decoding_key = DecodingKey::from_secret("secret".as_ref());
    let validation = Validation::default();

    match decode::<Claims>(token.as_str(), &decoding_key, &validation) {
        Ok(data) => {
            if data.claims.exp > Utc::now().timestamp() as usize {
                Ok(data.claims)
            } else {
                Err(false)
            }

        },
        Err(err) => {
            Err(false)
        }
    }
}