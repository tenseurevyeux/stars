use crate::handlers::{json_err, json_ok};
use shared::{Action, Direction, Payload};
use sqlx::PgPool;

pub async fn handle_direction(pool: &PgPool, action: Action) -> serde_json::Value {
    match action {
        Action::GetAll => {
            match sqlx::query_as::<_, Direction>("SELECT * FROM direction")
                .fetch_all(pool)
                .await
            {
                Ok(data) => json_ok(data),
                Err(e) => json_err(&e.to_string()),
            }
        }

        Action::Create(Payload::Direction { name_direction }) => {
            match sqlx::query("INSERT INTO direction (name_direction) VALUES ($1)")
                .bind(name_direction)
                .execute(pool)
                .await
            {
                Ok(_) => json_ok("Created successfully"),
                Err(e) => json_err(&e.to_string()),
            }
        }

        Action::Delete(id) => {
            match sqlx::query("DELETE FROM direction WHERE id_direction = $1")
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

        _ => json_err("Invalid action or payload for Direction"),
    }
}
