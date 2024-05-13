use std::collections::HashMap;
use deepl::{DeepLApi, Lang};
use crate::base::get_nowtime_str;
use crate::models::MyError;

pub struct DeeplModule{
    api:Option<DeepLApi>
}
pub fn lang_from_name(name: String) -> Option<Lang> {
    let lang_map: HashMap<&str, Lang> = [
        ("Arabic", Lang::AR),
        ("Bulgarian", Lang::BG),
        ("Czech", Lang::CS),
        ("Danish", Lang::DA),
        ("German", Lang::DE),
        ("Greek", Lang::EL),
        ("English", Lang::EN),
        ("Spanish", Lang::ES),
        ("Estonian", Lang::ET),
        ("Finnish", Lang::FI),
        ("French", Lang::FR),
        ("Hungarian", Lang::HU),
        ("Indonesian", Lang::ID),
        ("Italian", Lang::IT),
        ("Japanese", Lang::JA),
        ("Korean", Lang::KO),
        ("Lithuanian", Lang::LT),
        ("Latvian", Lang::LV),
        ("Norwegian", Lang::NB),
        ("Dutch", Lang::NL),
        ("Polish", Lang::PL),
        ("Portuguese", Lang::PT),
        ("Romanian", Lang::RO),
        ("Russian", Lang::RU),
        ("Slovak", Lang::SK),
        ("Slovenian", Lang::SL),
        ("Swedish", Lang::SV),
        ("Turkish", Lang::TR),
        ("Ukrainian", Lang::UK),
        ("Chinese", Lang::ZH),
    ].iter().cloned().collect();

    lang_map.get(name.as_str()).cloned()
}
impl DeeplModule{
    pub fn new()->Self{
        Self{api:None}
    }
    pub async fn connect(api_key:String)->DeepLApi{
        DeepLApi::with(&api_key).new()
    }
    pub async fn translate(api:DeepLApi,from_lang:String,into_lang:String,text:String)->Result<String,MyError>{
        let res=api.translate_text(text,lang_from_name(into_lang).unwrap())
            .source_lang(lang_from_name(from_lang).unwrap())
            .await.map_err(|e|{
            let str_error = format!("TRANSLATOR|| {} error: {}\n", get_nowtime_str(), e.to_string());
            MyError::SiteError(str_error)
        })?;
        let d=res.translations;
        Ok(d[0].text.clone())
    }
}