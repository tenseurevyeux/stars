use crate::handlers::{json_err, json_ok};
use shared::{Action, Summary};
use sqlx::PgPool;

pub async fn handle_summary(pool: &PgPool, action: Action) -> serde_json::Value {
    match action {
        Action::GetAll => {
            match sqlx::query_as::<_, Summary>(
                "SELECT
                    c.num_cabinet,
                    j.record_no,
                    j.date,
                    j.lesson,
                    s.fio_student,
                    d.name_direction,
                    l.fio_lab
                    FROM journal j
                    INNER JOIN cabinet c ON j.fk_cabinet = c.id_cabinet
                    INNER JOIN student s ON j.fk_student = s.id_student
                    INNER JOIN direction d ON s.fk_direction = d.id_direction
                    INNER JOIN laborant l ON j.fk_lab = l.id_lab
                    ORDER BY c.num_cabinet, j.record_no, j.date",
            )
            .fetch_all(pool)
            .await
            {
                Ok(data) => json_ok(data),
                Err(e) => json_err(&e.to_string()),
            }
        }

        _ => json_err("invalid action or payload for Summary"),
    }
}
