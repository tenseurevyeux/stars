-- University Management System Database Schema (PostgreSQL)

CREATE TABLE IF NOT EXISTS direction (
    id_direction SERIAL PRIMARY KEY,
    name_direction VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS cabinet (
    id_cabinet SERIAL PRIMARY KEY,
    num_cabinet VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS laborant (
    id_lab SERIAL PRIMARY KEY,
    fio_lab VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS student (
    id_student SERIAL PRIMARY KEY,
    fio_student VARCHAR(255) NOT NULL,
    fk_direction INT NOT NULL REFERENCES direction(id_direction) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS journal (
    fk_cabinet INT NOT NULL REFERENCES cabinet(id_cabinet) ON DELETE CASCADE,
    record_no INT NOT NULL,
    date DATE NOT NULL,
    lesson INT NOT NULL,
    fk_student INT NOT NULL REFERENCES student(id_student) ON DELETE CASCADE,
    fk_lab INT NOT NULL REFERENCES laborant(id_lab) ON DELETE CASCADE,
    PRIMARY KEY (fk_cabinet, record_no)
);

-- Optional seed sample data for testing
INSERT INTO direction (name_direction) VALUES
    ('Computer Science'),
    ('Applied Mathematics'),
    ('Information Security')
ON CONFLICT DO NOTHING;

INSERT INTO cabinet (num_cabinet) VALUES
    ('101-A'),
    ('204-B'),
    ('305-C')
ON CONFLICT DO NOTHING;

INSERT INTO laborant (fio_lab) VALUES
    ('Smith John A.'),
    ('Johnson Emily R.'),
    ('Davis Michael B.')
ON CONFLICT DO NOTHING;

INSERT INTO student (fio_student, fk_direction) VALUES
    ('Alice Williams', 1),
    ('Bob Brown', 1),
    ('Charlie Miller', 2),
    ('Diana Wilson', 3)
ON CONFLICT DO NOTHING;

INSERT INTO journal (fk_cabinet, record_no, date, lesson, fk_student, fk_lab) VALUES
    (1, 1, '2026-10-15', 1, 1, 1),
    (1, 2, '2026-10-15', 2, 2, 1),
    (2, 1, '2026-10-16', 3, 3, 2),
    (3, 1, '2026-10-17', 1, 4, 3)
ON CONFLICT DO NOTHING;
