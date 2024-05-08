use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::models::{MyError};
use chrono::{Local, Datelike, Timelike};
pub fn get_nowtime_str()->String{
    let current_datetime = Local::now();

    // Отримуємо значення року, місяця, дня, години та хвилини
    let year = current_datetime.year();
    let month = current_datetime.month();
    let day = current_datetime.day();
    let hour = current_datetime.hour();
    let minute = current_datetime.minute();

    // Складаємо значення у рядок
    let datetime_string = format!("{}-{:02}-{:02} {:02}:{:02}", year, month, day, hour, minute);
    datetime_string

}
pub async fn file_openString(name_file:&str) -> Result<String, MyError>{
    let mut file = File::open(name_file).await.map_err(|e|{
        let str_error = format!("FILE OPEN|| {} error: {}\n", get_nowtime_str(), e.to_string());
        MyError::SiteError(str_error)
    })?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).await.map_err(|e|{
        let str_error = format!("FILE OPEN|| {} error: {}\n", get_nowtime_str(), e.to_string());
        MyError::SiteError(str_error)
    })?;
    Ok(contents)
}
