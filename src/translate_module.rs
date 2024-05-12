use deepl::{DeepLApi, Lang};
pub struct DeeplModule{
    api:Option<DeepLApi>
}
impl DeeplModule{
    pub fn new()->Self{
        Self{api:None}
    }
    pub async fn connect(&mut self,api_key:String){
        self.api=Some(DeepLApi::with(&api_key).new());
    }
    pub async fn translate(&mut self,text:String)->String{
        let api=self.api.as_mut().unwrap();
        let res=api.translate_text(text,Lang::UK).await.unwrap();
        let d=res.translations;
        return d[0].text.clone();
    }
}