use ramhorns::{Template, Content};
use serde::{Deserialize, Serialize};
use crate::models::{LanguageSupported, Translated, UserDictionary};

#[derive(Content)]
pub struct LanguagesSupportedTemplate {
    pub languages:Vec<LanguageSupported>
}
#[derive(Content)]
pub struct ErrorTemplate {
    pub error:String
}
#[derive(Content)]
pub struct CurrentLangTemplate {
    pub current_lang:String,
    pub languages:Vec<UserDictionary>
}
#[derive(Content)]
pub struct TranslateHistoryTemplate {
    pub current_lang:String,
    pub languages:Vec<UserDictionary>,
    pub translate_history:Vec<Translated>
}
#[derive(Content)]
pub struct TranslateHistoryItemTemplate {
    pub current_lang:String,
    pub languages:Vec<UserDictionary>,
    pub translate_history:Translated,
    pub lang_from:String,
    pub lang_into:String
}
#[derive(Content)]
pub struct TranslateTemplate{
    pub current_lang:String,
    pub current_lang_id:i32,
    pub languages:Vec<UserDictionary>,
    pub langueges_supported:Vec<LanguageSupported>
}