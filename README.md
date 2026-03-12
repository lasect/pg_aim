# pgAim

PostgreSQL extension developed using `pgrx`.

## Prerequisites

- Rust toolchain (latest stable)
- `cargo-pgrx` 
- PostgreSQL development headers

## Setup

1. Install `cargo-pgrx`:
   ```bash
   cargo install --locked cargo-pgrx
   ```
2. Initialize pgrx (if not already):
   ```bash
   cargo pgrx init
   ```

## Development

- **Run in-memory Postgres instance:**
  ```bash
  cargo pgrx run
  ```
- **Run tests:**
  ```bash
  cargo pgrx test
  ```
- **Install locally:**
  ```bash
  cargo pgrx install
  ```

## Structure

- `src/lib.rs`: Extension entry point and exported functions.
- `pg_aim.control`: Postgres extension control file.
- `src/pg_aim.sql`: Custom SQL initialization (if applicable).
