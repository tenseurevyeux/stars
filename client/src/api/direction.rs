use crate::api::send_request;
use serde::Deserialize;
use shared::{Action, Direction, Payload, Request, Resource};

#[derive(Debug, Deserialize)]
struct DirectionsResponse {
    success: bool,
    data: Vec<Direction>,
}

pub async fn fetch_directions() -> Result<Vec<Direction>, String> {
    let req = Request {
        resource: Resource::Direction,
        action: Action::GetAll,
    };

    match send_request(req) {
        Ok(value) => match serde_json::from_value::<DirectionsResponse>(value) {
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

pub async fn delete_direction(id: i32) -> Result<(), String> {
    let req = Request {
        resource: Resource::Direction,
        action: Action::Delete(id),
    };

    send_request(req).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn create_direction(name: String) -> Result<(), String> {
    let req = Request {
        resource: Resource::Direction,
        action: Action::Create(Payload::Direction {
            name_direction: name,
        }),
    };

    send_request(req).map_err(|e| e.to_string())?;
    Ok(())
}
