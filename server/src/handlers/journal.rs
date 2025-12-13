use crate::handlers::{json_err, json_ok};
use shared::{Action, Journal, Payload};
use sqlx::PgPool;

pub async fn handle_journal(pool: &PgPool, action: Action) -> serde_json::Value {
    match action {
        Action::GetAll => {
            match sqlx::query_as::<_, Journal>("SELECT * FROM journal")
                .fetch_all(pool)
                .await
            {
                Ok(data) => json_ok(data),
                Err(e) => json_err(&e.to_string()),
            }
        }

        Action::Create(Payload::Journal {
            fk_cabinet,
            record_no,
            date,
            lesson,
            fk_student,
            fk_lab,
        }) => {
            match sqlx::query("INSERT INTO journal (fk_cabinet, record_no, date, lesson, fk_student, fk_lab) VALUES ($1, $2,$3,$4,$5,$6)")
                .bind(fk_cabinet)
                .bind(record_no)
                .bind(date)
                .bind(lesson)
                .bind(fk_student)
                .bind(fk_lab)
                .execute(pool)
                .await
            {
                Ok(_) => json_ok("Create successfully"),
                Err(e) => json_err(&e.to_string()),
            }
        }

        Action::DeleteRow{fk_cabinet, record_no} => {
            match sqlx::query("DELETE FROM journal WHERE fk_cabinet = $1 AND record_no = $2")
                .bind(fk_cabinet)
                .bind(record_no)
                .execute(pool)
                .await
            {
                Ok(_) => json_ok("Deleted"),
                Err(e) => json_err(&e.to_string()),
            }
        }

        _ => json_err("invalid action or payload for Journal"),
    }
}
