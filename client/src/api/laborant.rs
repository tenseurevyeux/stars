use crate::api::send_request;
use serde::Deserialize;
use shared::{Action, Laborant, Payload, Request, Resource};
#[derive(Debug, Deserialize)]

struct LaborantsResponse {
    success: bool,
    data: Vec<Laborant>,
}

pub async fn fetch_laborants() -> Result<Vec<Laborant>, String> {
    let req = Request {
        resource: Resource::Laborant,
        action: Action::GetAll,
    };

    match send_request(req) {
        Ok(value) => match serde_json::from_value::<LaborantsResponse>(value) {
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

pub async fn delete_laborant(id: i32) -> Result<(), String> {
    let req = Request {
        resource: Resource::Laborant,
        action: Action::Delete(id),
    };

    send_request(req).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn create_laborant(fio: String) -> Result<(), String> {
    let req = Request {
        resource: Resource::Laborant,
        action: Action::Create(Payload::Laborant { fio_lab: fio }),
    };

    send_request(req).map_err(|e| e.to_string())?;
    Ok(())
}
