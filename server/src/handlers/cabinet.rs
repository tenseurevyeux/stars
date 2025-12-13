use crate::handlers::{json_err, json_ok};
use shared::{Action, Cabinet, Payload};
use sqlx::PgPool;

pub async fn handle_cabinet(pool: &PgPool, action: Action) -> serde_json::Value {
    match action {
        Action::GetAll => {
            match sqlx::query_as::<_, Cabinet>("SELECT * FROM cabinet")
                .fetch_all(pool)
                .await
            {
                Ok(data) => json_ok(data),
                Err(e) => json_err(&e.to_string()),
            }
        }

        Action::Create(Payload::Cabinet { num_cabinet }) => {
            match sqlx::query("INSERT INTO cabinet (num_cabinet) VALUES ($1)")
                .bind(num_cabinet)
                .execute(pool)
                .await
            {
                Ok(_) => json_ok("Created successfully"),
                Err(e) => json_err(&e.to_string()),
            }
        }

        Action::Delete(id) => {
            match sqlx::query("DELETE FROM cabinet WHERE id_cabinet = $1")
                .bind(id)
                .execute(pool)
                .await
            {
                Ok(res) => {
                    if res.rows_affected() > 0 {
                        json_ok("Deleted")
                    } else {
                        json_err("ID not found")
                    }
                }
                Err(e) => json_err(&e.to_string()),
            }
        }

        _ => json_err("Invalid action or payload for Cabinet"),
    }
}
