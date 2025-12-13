use crate::api::send_request;
use chrono::NaiveDate;
use serde::Deserialize;
use shared::{Action, Journal, Payload, Request, Resource};

#[derive(Debug, Deserialize)]
struct JournalResponse {
    success: bool,
    data: Vec<Journal>,
}

pub async fn fetch_journals() -> Result<Vec<Journal>, String> {
    let req = Request {
        resource: Resource::Journal,
        action: Action::GetAll,
    };

    match send_request(req) {
        Ok(value) => match serde_json::from_value::<JournalResponse>(value) {
            Ok(resp) => {
                if resp.success {
                    Ok(resp.data)
                } else {
                    Err("Server returned success=false".into())
                }
            }
            Err(e) => Err(format!("Parse error: {}", e)),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub async fn create_journal(
    fk_cabinet: i32,
    record_no: i32,
    date: String,
    lesson: String,
    fk_student: i32,
    fk_lab: i32,
) -> Result<(), String> {
    let parsed_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return Err("Invalid date".into()),
    };
    let parsed_lesson = match lesson.parse::<i32>() {
        Ok(v) => v,
        Err(_) => return Err("Ivalid lesson".into()),
    };
    let req = Request {
        resource: Resource::Journal,
        action: Action::Create(Payload::Journal {
            fk_cabinet,
            record_no,
            date: parsed_date,
            lesson: parsed_lesson,
            fk_student,
            fk_lab,
        }),
    };

    send_request(req).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn delete_journal(fk_cabinet: i32, record_no: i32) -> Result<(), String> {
    let req = Request {
        resource: Resource::Journal,
        action: Action::DeleteRow {
            fk_cabinet,
            record_no,
        },
    };

    send_request(req).map_err(|e| e.to_string())?;
    Ok(())
}
