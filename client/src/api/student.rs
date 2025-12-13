use crate::api::send_request;
use serde::Deserialize;
use shared::{Action, Payload, Request, Resource, Student};

#[derive(Debug, Deserialize)]
struct StudentsResponse {
    success: bool,
    data: Vec<Student>,
}

pub async fn fetch_students() -> Result<Vec<Student>, String> {
    let req = Request {
        resource: Resource::Student,
        action: Action::GetAll,
    };

    match send_request(req) {
        Ok(value) => match serde_json::from_value::<StudentsResponse>(value) {
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

pub async fn delete_student(id: i32) -> Result<(), String> {
    let req = Request {
        resource: Resource::Student,
        action: Action::Delete(id),
    };

    send_request(req).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn create_student(fio: String, direction_id: i32) -> Result<(), String> {
    let req = Request {
        resource: Resource::Student,
        action: Action::Create(Payload::Student {
            fio_student: fio,
            fk_direction: direction_id,
        }),
    };

    send_request(req).map_err(|e| e.to_string())?;
    Ok(())
}
