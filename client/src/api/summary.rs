use crate::api::send_request;
use serde::Deserialize;
use shared::{Action, Request, Resource, Summary};

#[derive(Debug, Deserialize)]
struct SummaryResponse {
    success: bool,
    data: Vec<Summary>,
}

pub async fn fetch_summary() -> Result<Vec<Summary>, String> {
    let req = Request {
        resource: Resource::Summary,
        action: Action::GetAll,
    };

    match send_request(req) {
        Ok(value) => match serde_json::from_value::<SummaryResponse>(value) {
            Ok(resp) => {
                if resp.success {
                    Ok(resp.data)
                } else {
                    Err("Server returned success=false".into())
                }
            }
            Err(e) => Err(format!("Parse error {}", e)),
        },
        Err(e) => Err(e.to_string()),
    }
}
