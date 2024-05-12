
use ramhorns::{Template, Content};
use serde::{Deserialize, Serialize};
use crate::models::{LanguageSupported, UserDictionary};

#[derive(Content)]
pub struct LanguagesSupportedTemplate {
    pub languages:Vec<LanguageSupported>
}
#[derive(Content)]
pub struct CurrentLang{
    pub current_lang:String,
    pub languages:Vec<UserDictionary>
}