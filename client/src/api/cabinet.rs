use crate::api::send_request;
use serde::Deserialize;
use shared::{Action, Cabinet, Payload, Request, Resource};

#[derive(Debug, Deserialize)]
struct CabinetsResponse {
    success: bool,
    data: Vec<Cabinet>,
}

pub async fn fetch_cabinets() -> Result<Vec<Cabinet>, String> {
    let req = Request {
        resource: Resource::Cabinet,
        action: Action::GetAll,
    };

    match send_request(req) {
        Ok(value) => match serde_json::from_value::<CabinetsResponse>(value) {
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

pub async fn create_cabinet(number: String) -> Result<(), String> {
    let req = Request {
        resource: Resource::Cabinet,
        action: Action::Create(Payload::Cabinet {
            num_cabinet: number,
        }),
    };

    send_request(req).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn delete_cabinet(id: i32) -> Result<(), String> {
    let req = Request {
        resource: Resource::Cabinet,
        action: Action::Delete(id),
    };

    send_request(req).map_err(|e| e.to_string())?;
    Ok(())
}
