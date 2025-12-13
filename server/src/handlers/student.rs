use crate::handlers::{json_err, json_ok};
use shared::{Action, Payload, Student};
use sqlx::PgPool;

pub async fn handle_student(pool: &PgPool, action: Action) -> serde_json::Value {
    match action {
        Action::GetAll => {
            match sqlx::query_as::<_, Student>("SELECT * FROM student")
                .fetch_all(pool)
                .await
            {
                Ok(data) => json_ok(data),
                Err(e) => json_err(&e.to_string()),
            }
        }

        Action::Create(Payload::Student {
            fio_student,
            fk_direction,
        }) => {
            match sqlx::query("INSERT INTO student (fio_student, fk_direction) VALUES ($1, $2)")
                .bind(fio_student)
                .bind(fk_direction)
                .execute(pool)
                .await
            {
                Ok(_) => json_ok("Create successfully"),
                Err(e) => json_err(&e.to_string()),
            }
        }

        Action::Delete(id) => {
            match sqlx::query("DELETE FROM student WHERE id_student = $1")
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

        _ => json_err("invalid action or payload for Student"),
    }
}
