use crate::handlers::{json_err, json_ok};
use shared::{Action, Laborant, Payload};
use sqlx::PgPool;

pub async fn handle_laborant(pool: &PgPool, action: Action) -> serde_json::Value {
    match action {
        Action::GetAll => {
            match sqlx::query_as::<_, Laborant>("SELECT * FROM laborant")
                .fetch_all(pool)
                .await
            {
                Ok(data) => json_ok(data),
                Err(e) => json_err(&e.to_string()),
            }
        }

        Action::Create(Payload::Laborant { fio_lab }) => {
            match sqlx::query("INSERT INTO laborant (fio_lab) VALUES ($1)")
                .bind(fio_lab)
                .execute(pool)
                .await
            {
                Ok(_) => json_ok("Create successfully"),
                Err(e) => {
                    println!("X");
                    json_err(&e.to_string())
                }
            }
        }

        Action::Delete(id) => {
            match sqlx::query("DELETE FROM laborant WHERE id_lab = $1")
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

        _ => json_err("Invalid action or payload for Laborant"),
    }
}
