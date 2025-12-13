use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Direction {
    pub id_direction: i32,
    pub name_direction: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Cabinet {
    pub id_cabinet: i32,
    pub num_cabinet: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Laborant {
    pub id_lab: i32,
    pub fio_lab: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Student {
    pub id_student: i32,
    pub fio_student: String,
    pub fk_direction: i32,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Journal {
    pub fk_cabinet: i32,
    pub record_no: i32,
    pub date: NaiveDate,
    pub lesson: i32,
    pub fk_student: i32,
    pub fk_lab: i32,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Summary {
    pub num_cabinet: String,
    pub record_no: i32,
    pub date: NaiveDate,
    pub lesson: i32,
    pub fio_student: String,
    pub name_direction: String,
    pub fio_lab: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    Direction {
        name_direction: String,
    },
    Cabinet {
        num_cabinet: String,
    },
    Laborant {
        fio_lab: String,
    },
    Student {
        fio_student: String,
        fk_direction: i32,
    },
    Journal {
        fk_cabinet: i32,
        record_no: i32,
        date: NaiveDate,
        lesson: i32,
        fk_student: i32,
        fk_lab: i32,
    },
    Sammary {
        num_cabinet: String,
        record_no: i32,
        date: NaiveDate,
        lesson: i32,
        fio_student: String,
        name_direction: String,
        fio_lab: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub resource: Resource,
    pub action: Action,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Resource {
    Direction,
    Cabinet,
    Student,
    Laborant,
    Journal,
    Summary,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Action {
    GetAll,
    Delete(i32),
    DeleteRow { fk_cabinet: i32, record_no: i32 },
    Create(Payload),
}
