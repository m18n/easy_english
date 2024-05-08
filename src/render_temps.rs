
use ramhorns::{Template, Content};
use crate::models::LanguageSupported;

#[derive(Content)]
pub struct LanguagesSupportedTemplate {
    pub languages:Vec<LanguageSupported>
}