use std::collections::HashMap;
use genanki_rs::{Deck, Field, Model, Note, Template};
use crate::controllers::object_of_controller::Sentences;
use crate::models::Dictionary_Sentence;
pub fn get_language_world(language: String) -> Option<String> {
    let language_map: HashMap<&str, &str> = [
        ("Arabic", "ar-SA"),
        ("Dutch", "nl-NL"),
        ("English", r#""the", "be", "to", "of", "and", "a", "in", "that", "have", "I",
            "it", "for", "not", "on", "with", "he", "as", "you", "do", "at",
            "this", "but", "his", "by", "from", "they", "we", "say", "her", "she",
            "or", "an", "will", "my", "one", "all", "would", "there", "their",
            "what", "so", "up", "out", "if", "about", "who", "get", "which", "go",
            "me", "when", "make", "can", "like", "time", "no", "just", "him", "know",
            "take", "people", "into", "year", "your", "good", "some", "could", "them", "see",
            "other", "than", "then", "now", "look", "only", "come", "its", "over", "think",
            "also", "back", "after", "use", "two", "how", "our", "work", "first", "well",
            "way", "even", "new", "want", "because", "any", "these", "give", "day", "most",
            "us", "is", "are", "was", "were", "being", "been", "has", "had", "did",
            "do", "does", "doing", "having", "can", "could", "should", "would", "will", "shall",
            "may", "might", "must", "need", "ought", "dare", "used", "was", "am", "were",
            "be", "been", "being", "have", "has", "had", "do", "does", "did",
            "making", "going", "seeing", "thinking", "saying", "taking", "coming", "giving", "working", "looking",
            "finding", "trying", "asking", "using", "putting", "keeping", "leaving", "calling", "feeling", "becoming",
            "getting", "bringing", "holding", "starting", "turning", "showing", "running", "writing", "reading", "playing",
            "moving", "paying", "learning", "changing", "leading", "growing", "studying", "teaching", "building", "working",
            "man", "woman", "child", "house", "car", "water", "food", "school", "family", "friend",
            "happy", "sad", "big", "small", "long", "short", "old", "young", "high", "low",
            "right", "left", "early", "late", "hot", "cold", "hard", "easy", "fast", "slow",
            "strong", "weak", "dark", "light", "rich", "poor", "clean", "dirty", "full", "empty""#),
        ("French", "fr-FR"),
        ("German", "de-DE"),
        ("Greek", "el-GR"),
        ("Hebrew", "he-IL"),
        ("Italian", "it-IT"),
        ("Japanese", "ja-JP"),
        ("Korean", "ko-KR"),
        ("Polish", "pl-PL"),
        ("Portuguese", "pt-PT"),
        ("Russian", "ru-RU"),
        ("Spanish", "es-ES"),
        ("Swedish", r##""och", "att", "det", "i", "en", "jag", "är", "inte", "på", "som",
    "för", "du", "med", "han", "av", "den", "var", "vi", "kan", "till",
    "har", "de", "om", "ett", "så", "men", "hon", "hade", "då", "mig",
    "nu", "man", "ni", "alltid", "bara", "eller", "från", "såg", "sin",
    "få", "här", "mycket", "jag", "också", "ut", "vet", "ville", "ska",
    "kommer", "säga", "hem", "sa", "någon", "över", "igen", "alla", "måste",
    "in", "samma", "hur", "göra", "kunde", "honom", "väldigt", "ett", "sin",
    "genom", "gör", "innan", "tror", "dig", "något", "utan", "både", "kanske",
    "många", "än", "innan", "aldrig", "efter", "nu", "måste", "får", "mig",
    "går", "någon", "vilja", "han", "eller", "de", "om", "bara", "vi", "säger",
    "veta", "från", "min", "dig", "när", "efter", "kan", "ser", "år", "år",
    "där", "bara", "sådana", "före", "vill", "ser", "under", "kanske", "oss",
    "några", "de", "två", "tycker", "gör", "hela", "hela", "skulle", "vad",
    "vet", "tycker", "tyckte", "många", "nu", "jag", "från", "fram", "också",
    "gå", "här", "gå", "gått", "veta", "den", "den", "gör", "verkligen",
    "veta", "säga", "igen", "sa", "säga", "såg", "bara", "finns", "vi",
    "veta", "här", "de", "ser", "två", "vi", "de", "jag", "du", "han",
    "hon", "det", "är", "inte", "på", "att", "som", "med", "för", "den",
    "om", "ett", "har", "vi", "du", "ja", "han", "hon", "för", "jag",
    "om", "inte", "på", "med", "det", "som", "har", "han", "för", "är",
    "på", "med", "inte", "har", "för", "med", "på", "har", "för", "är""##),
        ("Thai", "th-TH"),
        ("Turkish", "tr-TR"),
        ("Ukrainian", "uk-UA"),
        ("Vietnamese", "vi-VN"),
    ].iter().cloned().collect();
    let res=language_map.get(language.as_str()).copied();
    match res {
        None => {
            None
        }
        Some(str) => {
            Some(String::from(str))
        }
    }

}
pub fn generate_anki(user_dict:i32, sentences: Vec<Dictionary_Sentence>, lang_name:String) ->String{
    //let custom_css = ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n}\n";
    // Створіть модель з HTML та JavaScript для вводу та перевірки
    let id_model=(1600000000 + user_dict) as i64;
    let model = Model::new(
        1607392319, // Model ID
        "Simple Model with Input",
        vec![
            Field::new("Front"),
            Field::new("Back"),
            Field::new("Sentence"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt(r#"{{Front}}<br> <span class="input_client">{{type:Sentence}}</span>"#)
                .afmt("{{FrontSide}}<hr id=\"answer\">{{Back}}")
        ]
    );

    // Створіть колоду
    let mut deck = Deck::new(id_model+1, format!("{} Desk {}",lang_name,user_dict).as_str(), "This is an example deck with JS");

    // Додайте нотатку
    for i in 0..sentences.len(){
        let words: Vec<&str> = sentences[i].sentence_into.split_whitespace().collect();
        let mut words_arr_str=String::from("[");
        for j in 0..words.len(){
            words_arr_str.push_str(format!(r#""{}""#,words[j]).as_str());
            if(j!=words.len()-1){
                words_arr_str.push_str(",");
            }
        }
        words_arr_str.push_str("]");
        let front_str=format!(r#"<h2>{}</h2><span style="color:rgb(70, 70, 70);padding-bottom: 5px;padding-left: 5px;">{}</span>"#,sentences[i].sentence_from,sentences[i].sentence_from_context);
        let back=format!(r##"<div class="none select_base" value="0" onclick="chose_select(this);">
    </div>
    <div class="none item_base" value="0" onclick="chose_word(this);">
    </div>
    <div class="container">

        <div class="select_grid">

        </div>
        <div class="grid">

        </div>
        <div class="buttons">
            <button class="button_check button_style" onclick="check(this);">Перевірити</button>
            <button class="button_show button_style" onclick="show(this);">Показати</button>
        </div>
        <div class="transcript none">
            <span>Твоє речення:</span>
            <span class="ukr_sentence"></span>
            <span class="space">Оцінка твого перекладу:</span>
            <span class="score">0/100</span>
            <span class="space">Відкориговане речення:</span>
            <span class="correct_translate"></span>
            <button class="button_style listen_button" onclick="listen_correct(this);">Прослухати</button>
            <span class="space">Задумане речення:</span>
            <span class="intended_sentence"></span>
            <span class="trans_eng">{}</span>
            <span class="trans_ukr">{}</span>
            <button class="button_style listen_button" onclick="listen(this);">Прослухати</button>


        </div>

    </div>
    <style>
        .space{{
            margin-top:4px;
        }}
        .score{{
            color: rgb(6, 172, 6);

        }}
        .correct_translate{{

        }}

        .input_client{{
            display: none;
        }}
        .transcript {{
            display: flex;
            flex-direction: column;
            align-items: center;
            margin-top: 5px;
        }}

        .none {{
            display: none !important;
        }}

        .chose {{
            border: solid 2px rgb(0, 132, 255);
        }}

        .container {{
            width: 100%;
            display: flex;
            align-items: center;
            flex-direction: column;
        }}

        .hr {{
            width: 100%;
            height: 2px;
            background-color: grey;
        }}

        .grid {{
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;
            width: 240px;
        }}

        .item_grid {{
            cursor: pointer;
            display: flex;
            justify-content: center;
            align-items: center;
            min-width: 70px;
            height: 30px;
            background-color: rgb(17, 187, 17);
            color: white;
            border-radius: 5px;
            margin-left: 2px;
            margin-top: 3px;
            padding: 0 3px;
        }}

        .select_grid {{
            margin-top: 5px;
            margin-bottom: 3px;
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;
        }}

        .select_item {{
            display: flex;
            color: white;
            justify-content: center;
            align-items: center;
            min-width: 70px;
            padding: 0 3px;
            height: 30px;
            background-color: rgb(185, 185, 185);
            border-radius: 5px;
            margin-left: 4px;
            margin-top: 2px;
            cursor: pointer;
        }}

        .buttons {{
            margin-top: 3px;
            display: flex;
            flex-direction: row;
            width: 240px;
            justify-content: space-between;
        }}

        .button_style {{
            min-width: 80px;
            height: 30px;
            background-color: rgb(84, 84, 219);
            color: white;
            border: none;
            outline: none;
            border-radius: 4px;
            cursor: pointer;
        }}

        .listen_button {{
            width: 90px;
            margin-top: 4px;
        }}

        .buttons button {{}}

        .button_check {{}}

        .button_show {{}}
    </style>
    <script>
        var name_lang="{}";
        var ukr_sentences="{}";
        var ukr_sentences_context="{}";
        var sentences="{}";
        var allrightword = {};
        var commonWords = [
            {}
        ];
        var ip = "0.0.0.0.0";
        // Функція для перемішування масиву
        function shuffleArray(array) {{
            for (let i = array.length - 1; i > 0; i--) {{
                const j = Math.floor(Math.random() * (i + 1));
                [array[i], array[j]] = [array[j], array[i]];
            }}
        }}
        function create_puzzle(){{
             shuffleArray(commonWords);

            let wordArray = [...allrightword];

            // Переконатися, що не перевищуємо довжину commonWords
            let limit = Math.min(wordArray.length * 2, commonWords.length);
            for (let i = 0; i < limit; i++) {{
                wordArray.push(commonWords[i]);
            }}

            shuffleArray(wordArray);
            let select_grid = document.querySelector('.select_grid');
            for (let i = 0; i < allrightword.length; i++) {{
                let select = document.querySelector('.select_base');
                let clonedBlock = select.cloneNode(true);
                clonedBlock.classList.remove("select_base");
                clonedBlock.classList.remove("none");
                clonedBlock.classList.add("select_item");
                if (i == 0) {{
                    clonedBlock.classList.add("chose");
                }}
                select_grid.appendChild(clonedBlock);
            }}
            let item_grid = document.querySelector('.grid');
            for (let i = 0; i < wordArray.length; i++) {{
                let item = document.querySelector('.item_base');
                let itemBlock = item.cloneNode(true);
                itemBlock.classList.remove("item_base");
                itemBlock.classList.remove("none");
                itemBlock.classList.add("item_grid");
                itemBlock.setAttribute("value", i + 1);
                itemBlock.textContent = wordArray[i];
                item_grid.appendChild(itemBlock);
            }}


            console.log(wordArray);
         }}
        async function gpt_check(){{

            let url = "http://"+ip+"/api/text/check";
            console.log("URL: "+url);
            var xhr = new XMLHttpRequest();


            // Встановлюємо запит як синхронний (третій параметр false)
            try {{
                xhr.open("POST", url, true);

                // Встановлюємо заголовки запиту
                xhr.setRequestHeader("Content-Type", "application/json;charset=UTF-8");

                let incl = document.querySelector('.input_client').textContent;
                let data={{
                    sentence_from:ukr_sentences,
                    sentence_from_context:ukr_sentences_context,
                    sentence_into: incl.slice(0, -6),
                    lang_name:name_lang
                }};

                let json=JSON.stringify(data);
                console.log("DATA:"+json);

                xhr.onload = function () {{
                    if (xhr.status === 200) {{
                        let obj=JSON.parse(xhr.responseText);
                        document.querySelector('.score').textContent=obj.assessment+"/100";
                        //document.querySelector('.comment').textContent=obj.translation_comment;
                        document.querySelector('.correct_translate').textContent=obj.correct_translation;
                        allrightword=obj.words_correct;
                        commonWords=obj.words_puzzle;
                        create_puzzle();
                    }} else {{
                        console.log("ERROR SEND");
                    }}
                }};

                // Обробка помилки з'єднання
                xhr.onerror = function () {{
                    console.log("ERROR SEND");
                }};

                // Обробка таймауту
                xhr.ontimeout = function () {{
                    console.log("ERROR TIMEOUT");
                }};
                // Виконання запиту
                xhr.send(json);
             }} catch (error) {{
             console.log("ERROR SEND");
            }}

        }}
        async function getDataFromJsonbin() {{
            let binId = "669b8a31e41b4d34e4148606";
            let apiKey = "$2a$10$kAo0.pfVYv5sMm2baWg9Z.6aHG8Hy3OYFC2hOA670a8uaeRVbvPEW";
            const url = `https://api.jsonbin.io/v3/b/${{binId}}/latest`;

            try {{
                const xhr = new XMLHttpRequest();

                xhr.open("GET", url, true); // Синхронний запит
                xhr.setRequestHeader("X-Master-Key", apiKey);

                xhr.onload = function () {{
                    if (xhr.status === 200) {{

                        let data=JSON.parse(xhr.responseText);
                        data=data.record;
                        ip = data["ip"];
                        gpt_check();
                    }} else {{
                        console.log("ERROR SEND");
                    }}
                }};

                // Обробка помилки з'єднання
                xhr.onerror = function () {{
                    console.log("ERROR SEND");
                }};

                // Обробка таймауту
                xhr.ontimeout = function () {{
                    console.log("ERROR TIMEOUT");
                }};

                xhr.send();
            }} catch (error) {{
                console.log("ERROR SEND");
            }}

        }}




        // Основна функція
        async function start() {{
            await getDataFromJsonbin();

        }}

        start();
        function chose_select(item) {{
            if (item.classList.contains('chose')) {{
                item.textContent = "";
                item.style.background = "rgb(185, 185, 185)";
                item.classList.remove('chose');
                let idvalue = item.getAttribute("value");
                item.setAttribute("value", "0");
                let obj = document.querySelector('.item_grid[value="' + idvalue + '"]');
                obj.classList.remove('none');
            }} else {{
                let selectItems = document.querySelectorAll('.select_item');
                selectItems.forEach(el => el.classList.remove('chose'));
                item.classList.add('chose');
            }}
        }}
        function chose_word(item) {{
            let chose_item = document.querySelector('.chose');
            if (chose_item) {{
                if (chose_item.getAttribute("value") != "0") {{
                    let idvalue = chose_item.getAttribute("value");
                    let obj = document.querySelector('.item_grid[value="' + idvalue + '"]');
                    obj.classList.remove('none');
                }}

                chose_item.style.background = "rgb(17, 187, 17)";
                chose_item.textContent = item.textContent;
                item.classList.add('none');
                chose_item.setAttribute("value", item.getAttribute("value"));

            }}
        }}
        function check(item) {{
            console.log("CHECK");
            let select_objs = document.querySelectorAll('.select_item');
            let all_good = true;
            for (let i = 0; i < select_objs.length; i++) {{
                if (select_objs[i].textContent == allrightword[i]) {{
                    select_objs[i].style.background = "rgb(17, 187, 17)";
                }} else {{
                    select_objs[i].style.background = "rgb(232, 54, 0)";
                    all_good = false;
                }}
            }}
            if (all_good == true) {{
                success();
            }}
        }}
        function show(item) {{
            let select_objs = document.querySelectorAll('.select_item');
            for (let i = 0; i < select_objs.length; i++) {{
                select_objs[i].textContent = allrightword[i];
                select_objs[i].style.background = "rgb(17, 187, 17)";
            }}
            success();
        }}

        var currentAudio = null;

        function listen_correct(item) {{
            let correct = document.querySelector('.correct_translate');
            console.log("CORRECT:"+correct.textContent);
            const requestData = {{
                name_lang: name_lang,
                text: correct.textContent
            }};
            console.log("REQ:" + JSON.stringify(requestData));
            console.log("IP: "+ip);
            fetch('http://' + ip + '/api/text', {{
                method: 'POST',
                headers: {{
                    'Content-Type': 'application/json'
                }},
                body: JSON.stringify(requestData),
                credentials: 'include' // Додає куки і аутентифікацію до запиту
            }})
                .then(response => {{
                    if (!response.ok) {{
                        throw new Error('Network response was not ok');
                    }}
                    return response.blob(); // Перетворення відповіді на Blob
                }})
                .then(blob => {{
                    if (currentAudio) {{
                        currentAudio.pause(); // Зупинити поточне аудіо
                        currentAudio.src = ""; // Очистити джерело
                    }}

                    const url = URL.createObjectURL(blob); // Створення URL для Blob
                    currentAudio = new Audio(url); // Створення аудіо елемента
                    currentAudio.play(); // Відтворення аудіо

                    currentAudio.onended = function () {{
                        URL.revokeObjectURL(url); // Видалення URL для звільнення пам'яті
                        console.log('Audio has been played and removed from memory');
                        currentAudio = null; // Очистити посилання на поточний аудіоелемент
                    }};
                }})
                .catch(error => {{
                    console.log("ERROR AUDIO");
                }});
        }}
        function listen(item) {{
            const requestData = {{
                name_lang: name_lang,
                text: sentences
            }};
            console.log("REQ:" + JSON.stringify(requestData));
            console.log("IP: "+ip);
            fetch('http://' + ip + '/api/text', {{
                method: 'POST',
                headers: {{
                    'Content-Type': 'application/json'
                }},
                body: JSON.stringify(requestData),
                credentials: 'include' // Додає куки і аутентифікацію до запиту
            }})
                .then(response => {{
                    if (!response.ok) {{
                        throw new Error('Network response was not ok');
                    }}
                    return response.blob(); // Перетворення відповіді на Blob
                }})
                .then(blob => {{
                    if (currentAudio) {{
                        currentAudio.pause(); // Зупинити поточне аудіо
                        currentAudio.src = ""; // Очистити джерело
                    }}

                    const url = URL.createObjectURL(blob); // Створення URL для Blob
                    currentAudio = new Audio(url); // Створення аудіо елемента
                    currentAudio.play(); // Відтворення аудіо

                    currentAudio.onended = function () {{
                        URL.revokeObjectURL(url); // Видалення URL для звільнення пам'яті
                        console.log('Audio has been played and removed from memory');
                        currentAudio = null; // Очистити посилання на поточний аудіоелемент
                    }};
                }})
                .catch(error => {{
                    console.log("ERROR AUDIO");
                }});
        }}

        function success() {{
            let incl = document.querySelector('.input_client').textContent;
            let ukr_sentence=incl.slice(0, -6);
            let ukr = document.querySelector(".ukr_sentence");
            let intended=document.querySelector(".intended_sentence");
            intended.textContent=sentences;
            ukr.textContent=ukr_sentence;
            let trans = document.querySelector(".transcript");
            trans.classList.remove("none");
        }}
        </script>"##,sentences[i].transcription_eng,sentences[i].transcription_ukr,lang_name,sentences[i].sentence_from,sentences[i].sentence_from_context,sentences[i].sentence_into
                         ,words_arr_str,get_language_world(lang_name.clone()).unwrap());
        let note=Note::new(model.clone(),vec![front_str.as_str(),back.as_str(),"Space"]).unwrap();
        deck.add_note(note);
    }




    // Збережіть колоду у файл
    let name_file=format!("deck{}.apkg",id_model+sentences.len() as i64);
    deck.write_to_file(name_file.clone().as_str()).unwrap();
    name_file
}