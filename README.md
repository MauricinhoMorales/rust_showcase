# Rust SurrealDB API

This is a simple Rust API that interacts with a SurrealDB database. It demonstrates how to create a RESTful API using Actix-web and store data in SurrealDB.

## Prerequisites

Before running the project, ensure you have the following installed:

1. **Rust**: The programming language used for this project.
2. **SurrealDB**: The database used to store data.

## Installation

### 1. Install Rust

To install Rust, run the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, restart your terminal or run the following to apply the changes:

```bash
source $HOME/.cargo/env
```

Verify the installation:

```bash
rustc --version
```

### 2. Install SurrealDB

Linux:

```bash
curl -sSf https://install.surrealdb.com | sh
```

Windows:

```bash
iwr https://windows.surrealdb.com -useb | iex
```

or download the appropriate binary for your operating system from the [SurrealDB download](https://surrealdb.com/install) page.

### 3. Clone the Repository

```bash
git clone https://github.com/CodeCultist/hsr-calculator-rust.git
cd hsr-calculator-rust
```

Run the project:

```bash
cargo run
```

> Note: The API will be available at localhost:8080+

### Test Endpoints

```bash
curl -X POST -H "Content-Type: application/json" -d '{"name": "Test", "email": "test@example.com"}' http://localhost:8080/users
```

```bash
curl http://localhost:8080/users
```

### Surrealist

To have a visual handle of the database you can install [Surrealist](https://surrealdb-com.translate.goog/surrealist?_x_tr_sl=en&_x_tr_tl=es&_x_tr_hl=es&_x_tr_pto=tc)
