# STARS: University Management System

A secure, high-performance desktop-and-server management system for academic laboratories, study directions, students, and lesson logs, built in Rust.

---

## Overview

**STARS** is a client-server application designed to manage university laboratories and journal records. It combines a native desktop GUI built with Iced and an asynchronous backend server powered by Tokio and SQLx, communicating over custom encrypted TLS streams via Rustls.

### Workspace Architecture

The project is structured as a Cargo workspace with three primary crates:

```
stars/
├── client/          # Desktop GUI application (Iced, Tokio, Rustls)
├── server/          # Asynchronous backend service (Tokio, SQLx, Postgres, Tokio-Rustls)
├── shared/          # Shared domain models, protocol requests, and serialization
├── certificates/    # TLS Root Certificate Authorities (CA) and generation scripts
├── sec/             # Server TLS certificates and private keys
├── schema.sql       # Database schema and seed data for PostgreSQL
└── shell.nix        # Nix development environment specification
```

```mermaid
flowchart LR
    subgraph Client["Desktop Client (Iced GUI)"]
        UI["Iced Views / Screens"] --> API["API Layer (client/src/api)"]
        API --> TLSClient["Rustls TLS Client Stream"]
    end

    subgraph Server["Backend Server (Tokio)"]
        TLSServer["Tokio-Rustls TLS Acceptor"] --> Dispatcher["Request Dispatcher"]
        Dispatcher --> Handlers["Resource Handlers\n(Cabinet, Student, Journal, ...)"]
        Handlers --> DBConn["SQLx Postgres Pool"]
    end

    subgraph Database["PostgreSQL"]
        Tables[("university_db\n(direction, cabinet, student,\nlaborant, journal)")]
    end

    TLSClient -- "Encrypted TCP (Port 7777)" --> TLSServer
    DBConn -- "SQL Queries" --> Tables
```

---

## Features

- **Academic Entity Management**:
  - **Directions (`direction`)**: Manage study programs and academic specializations.
  - **Cabinets (`cabinet`)**: Register and monitor classrooms and university laboratory rooms.
  - **Lab Assistants (`laborant`)**: Maintain assistant personnel records.
  - **Students (`student`)**: Track enrolled students tied to specific academic directions.
- **Journal & Lesson Tracking (`journal`)**:
  - Record lab attendance, lesson numbers, assigned lab assistant, classroom, and date.
  - Compound primary key operations (`fk_cabinet`, `record_no`) with cascade integrity.
- **Consolidated Summary (`summary`)**:
  - Live relational joins aggregating cabinet numbers, student details, specializations, and supervising assistants ordered by session time.
- **End-to-End TLS Encryption**:
  - Direct TLS 1.3 socket communication using `rustls` and `tokio-rustls`.
  - Self-contained custom Root CA trust store embedded into the client binary.
- **Configurable & Scriptable**:
  - Flexible environment variables for database connections, server bindings, and certificate paths.
  - Automated mock certificate generation script for zero-setup local development.

---

## Domain Entities

| Entity | Fields | Description |
|---|---|---|
| **Direction** | `id_direction`, `name_direction` | Field of study / specialization |
| **Cabinet** | `id_cabinet`, `num_cabinet` | Lab or classroom room identifier |
| **Laborant** | `id_lab`, `fio_lab` | Lab assistant full name |
| **Student** | `id_student`, `fio_student`, `fk_direction` | Student full name and study direction foreign key |
| **Journal** | `fk_cabinet`, `record_no`, `date`, `lesson`, `fk_student`, `fk_lab` | Detailed session attendance and lab logging |
| **Summary** | Consolidated read-only view | Aggregated JOIN across all five primary tables |

---

## Security & Mock Certificates

> [!IMPORTANT]
> The repository comes with **mock / development certificates** out of the box. All real certificates and keys have been removed to protect privacy and confidentiality.

### Certificate Layout

- `certificates/rootCA.pem`: Development Root CA certificate trusted by the client.
- `certificates/rootCA.key`: Private key for the mock Root CA.
- `certificates/rootCA-loc.pem` & `rootCA-loc.key`: Local mock Root CA certificate and key pair.
- `sec/server.crt`: Server certificate signed by `certificates/rootCA.pem`. Includes SANs for `localhost`, `127.0.0.1`, `0.0.0.0`, and `217.60.2.146`.
- `sec/server.key`: Server private key in standard PKCS#8 format.

### Regenerating Certificates

You can regenerate clean mock certificates at any time with the included script:

```bash
./certificates/generate_certs.sh
```

To use custom production certificates:
1. Replace `certificates/rootCA.pem` with your organization's CA certificate.
2. Point the server to your server certificate and private key using `TLS_CERT_PATH` and `TLS_KEY_PATH` in your `.env` file.

---

## Prerequisites

- **Rust**: Rust toolchain (2024 edition compatible, Rust 1.85+ recommended).
- **PostgreSQL**: Version 14 or higher.
- **OpenSSL / LibreSSL**: Required for generating TLS certificates.
- **GUI System Dependencies** (for Linux/Nix):
  - Wayland / X11 libraries (`libX11`, `libXcursor`, `libXi`, `libXrandr`, `libxkbcommon`).
  - Font rendering libraries (`fontconfig`, `freetype`, `expat`, `libGL`).
  - *Tip for Nix users*: Enter the development shell using `nix-shell` which provides all system dependencies.

---

## Getting Started

### 1. Database Setup

1. Create a PostgreSQL database (e.g., `university_db`):

   ```bash
   createdb -U postgres university_db
   ```

2. Apply the schema and sample seed data:

   ```bash
   psql -U postgres -d university_db -f schema.sql
   ```

### 2. Configure Environment

Copy the example environment file:

```bash
cp .env.example .env
```

Adjust the variables in `.env` as needed:

```ini
# Server configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/university_db
SERVER_HOST=0.0.0.0
SERVER_PORT=7777
TLS_CERT_PATH=./sec/server.crt
TLS_KEY_PATH=./sec/server.key

# Client optional overrides
# SERVER_ADDR=127.0.0.1:7777
# SERVER_NAME=127.0.0.1
```

### 3. Run the Backend Server

Start the server from the root directory:

```bash
cargo run --bin server
```

The server loads the TLS configuration from `TLS_CERT_PATH` and `TLS_KEY_PATH`, connects to PostgreSQL, and begins listening on `0.0.0.0:7777`.

### 4. Run the Desktop Client

In a separate terminal, launch the desktop client:

```bash
cargo run --bin client
```

To point the client to a different server address or hostname, pass environment variables:

```bash
SERVER_ADDR="127.0.0.1:7777" SERVER_NAME="127.0.0.1" cargo run --bin client
```

---

## Wire Protocol

Client and server communicate via JSON messages over raw TLS streams:

- **Client Request Format**:
  ```json
  {
    "resource": "student",
    "action": {
      "type": "GetAll"
    }
  }
  ```
  Available actions:
  - `GetAll`: Retrieve all records for the resource.
  - `Create`: Insert a new record with entity-specific payload.
  - `Delete`: Remove an entity by primary key ID.
  - `DeleteRow`: Remove a journal record by composite key (`fk_cabinet`, `record_no`).

- **Server Response Format**:
  - Success:
    ```json
    {
      "success": true,
      "data": [...]
    }
    ```
  - Error:
    ```json
    {
      "success": false,
      "message": "Error description"
    }
    ```

---

## Development & Nix Shell

For reproducible environments on Linux or macOS with Nix:

```bash
nix-shell
```

This sets up `rustc`, `cargo`, OpenGL, Vulkan loader, X11/Wayland libraries, and fonts automatically.

---

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
