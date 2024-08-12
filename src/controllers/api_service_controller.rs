use actix_web::{get, HttpResponse, post, web};
use actix_web::web::Json;
use crate::controllers::object_of_controller::{RequestResult, ResultAnkiGpt, ResultGptCheck, ResultGptPuzzle, SentencesLang, TextToSpeach};
use crate::google_module::GoogleModule;
use crate::gpt_module::GptModule;
use crate::models::MyError;
use crate::StateDb;
// url controller: /api/service/***

#[get("/check")]
pub async fn m_check(state: web::Data<StateDb>)->Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok().json(RequestResult{status:true}))
}
#[post("/textToAudio")]
pub async fn m_text_to_audio(text_:web::Json<TextToSpeach>,state: web::Data<StateDb>)->Result<HttpResponse, MyError>{
    let bytes=GoogleModule::text_to_speach(state.google_module.clone(),text_.text.clone(),text_.name_lang.clone()).await;
    let mut res_bytes=Vec::new();
    match bytes {
        Ok(bytes) => {
            res_bytes=bytes;
        }
        Err(e) => {

        }
    }
    Ok(HttpResponse::Ok()
        .content_type("audio/mpeg").body(web::Bytes::from(res_bytes)))

}
#[post("/checkAnswer")]
pub async fn m_check_answer(text_:web::Json<SentencesLang>,state: web::Data<StateDb>)->Result<Json<ResultAnkiGpt>, MyError>{
    let text=text_.into_inner();
    let query=format!(r#" Я тобі надам українське речення з контекстом та {}.
Українське речення: "{}"
Контекст українського речення: "{}"
{} речення: "{}"
Ти маєш надати у відповді 2 параметри.
Перший це "assessment" на скільки хорошиї переклад з українського в тому контексті на {} від 0 до 100, це звісно приблизно.
Другий це "correct_translation" відкорегований переклад мого речення на англійську мову.
        Відповідь надай в JSON. У форматі об'єкту:
        {{
            "assessment":,
            "correct_translation":"",
        }}
        "#,text.lang_name,text.sentence_from,text.sentence_from_context,text.lang_name,text.sentence_into,text.lang_name);
    let gpt_check:Result<ResultGptCheck,MyError>=GptModule::send(state.gpt_api.clone(),query).await;
    let mut res_check=ResultGptCheck{assessment:-1,correct_translation:String::new()};
    let mut res_anki=ResultAnkiGpt{assessment:-1,correct_translation:String::new(),words_puzzle:Vec::new(),words_correct:Vec::new()};
    match gpt_check {
        Ok(result) => {
            res_check=result;
        }
        Err(error) => {
            return Ok(Json(res_anki));
        }
    }
    let words: Vec<String> = res_check.correct_translation.split_whitespace().map(|s| s.to_string())
        .collect();
    let size_words=words.len()*2;
    let query=format!(r#" Я тобі надам українське речення з контекстом та {}.
Українське речення: "{}"
Контекст українського речення: "{}"
{} речення: "{}"
Ти маєш надати у відповдь 1 параметр.
Перший — "words_puzzle", я хочу зібрати {} речень як пазли, для цього мені потрібно, щоб ти згенерував масив з {} слів, які б мене заплутали, тільки не згадуйте ті, які вже є в реченні для {} речень.
        Відповідь надай в JSON. У форматі об'єкту:
        {{
            "words_puzzle":[""],
        }}
        "#,text.lang_name,text.sentence_from,text.sentence_from_context,text.lang_name,res_check.correct_translation,text.lang_name,size_words,text.lang_name);
    let gpt_puzzle:Result<ResultGptPuzzle,MyError>=GptModule::send(state.gpt_api.clone(),query).await;
    let mut res_puzzle=ResultGptPuzzle{words_puzzle:Vec::new()};
    match gpt_puzzle {
        Ok(result) => {
            res_puzzle=result;
        }
        Err(error) => {
            return Ok(Json(res_anki));
        }
    }
    res_anki=ResultAnkiGpt{assessment:res_check.assessment,correct_translation:res_check.correct_translation
        ,words_puzzle:res_puzzle.words_puzzle,words_correct:words};
    Ok(Json(res_anki))
}