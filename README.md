# Structure Column Performance Comparison

A project to compare database storage formats (column type vs JSON type) and application-side processing performance.

## 📊 Project Overview

This project compares the performance of two data storage methods using MariaDB:

- **Column Type Storage**: Normalized table structure with each field stored in dedicated columns
- **JSON Type Storage**: Denormalized table structure with all data stored as JSON strings

### Comparison Targets

#### Column Type Storage (users_column)
- Normalized table structure
- Each field stored in dedicated columns
- Complex nested data (preferences, social_links) stored as JSON

#### JSON Type Storage (users_json)
- Denormalized table structure
- All data stored in a single JSON column
- JSON parsing required on the application side

## 🛠️ Technology Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: MariaDB 10.11
- **ORM**: SQLx
- **Development Environment**: devenv (Nix)
- **Serialization**: serde_json
- **UUID Generation**: uuid

## 🚀 Setup

### Prerequisites
- Nix is installed
- Rust is installed

### 1. Start devenv environment
```bash
nix develop
```

### 2. Install dependencies
```bash
cargo build
```

### 3. Start application (benchmark runs automatically)
```bash
cargo run
```

## 📈 Benchmark Execution

### Automatic execution
Running `cargo run` automatically executes the following:

1. Database connection establishment
2. Generation of 100,000 test records (column type and JSON type tables)
3. Performance benchmark execution
4. Results display

### Manual execution
```bash
# Generate complex test data
curl -X POST http://localhost:3000/generate/complex/1000

# Run benchmark
curl http://localhost:3000/benchmark/complex/1000
```

## 📊 Benchmark Results Example

| Count | Column Type | JSON Type | Complex JSON Processing | Winner |
|-------|-------------|-----------|------------------------|--------|
| 1,000 | 8ms | 4ms | 44ms | JSON Type |
| 10,000 | 93ms | 247ms | 447ms | Column Type |
| 50,000 | 280ms | 638ms | 2,262ms | Column Type |
| 100,000 | 501ms | 1,289ms | 4,440ms | Column Type |

## 🔧 API Endpoints

### Data Generation
- `POST /generate/column/{count}` - Generate column type test data
- `POST /generate/json/{count}` - Generate JSON type test data
- `POST /generate/complex/{count}` - Generate complex JSON test data

### Benchmark
- `GET /benchmark/column/{count}` - Column type performance test
- `GET /benchmark/json/{count}` - JSON type performance test
- `GET /benchmark/complex/{count}` - Complex JSON processing performance test

### Data Retrieval
- `GET /users/column` - Get column type user list
- `GET /users/json` - Get JSON type user list

## 📁 Project Structure

```
structure-column-performance/
├── src/
│   ├── main.rs              # Main application
│   └── data_generator.rs    # Test data generation
├── migrations/
│   └── 001_init.sql         # Database schema
├── devenv.nix               # Development environment settings
├── Cargo.toml               # Rust dependencies
└── README.md                # This file
```

## 🔍 Database Schema

### Column Type Table
```sql
CREATE TABLE users_column (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    age INTEGER NOT NULL,
    bio TEXT,
    avatar_url VARCHAR(500),
    preferences JSON,
    social_links JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### JSON Type Table
```sql
CREATE TABLE users_json (
    id CHAR(36) PRIMARY KEY,
    data JSON NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

## 🎯 Expected Results

- **Read Performance**: Column type is faster (index efficiency)
- **Write Performance**: JSON type is faster (simple insertion)
- **Memory Usage**: JSON type uses more (parsing overhead)
- **Disk Usage**: Nearly equivalent (depends on data volume)

## 📝 Detailed Analysis Results

For detailed benchmark results and technical analysis, please see [zenn-article.md](./zenn-article.md).

## 🤝 Contribution

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Create a Pull Request

## 📄 License

This project is published under the MIT License.