# DStream

A Rust-based ETL (Extract, Transform, Load) framework inspired by Singer, featuring tap (source) and target (destination) connectors with a clean streaming protocol.

## Features

- **Modular Architecture**: Separate tap and target connectors
- **Streaming Protocol**: Efficient message-based data transfer using Apache Arrow
- **State Management**: Built-in state tracking for incremental extractions
- **Catalog Discovery**: Automatic schema discovery and stream selection
- **Type-Safe**: Leverages Rust's type system for reliability
- **Async/Await**: Built on Tokio for high-performance async I/O

## Installation

### From Source

```bash
git clone <repository-url>
cd DStream
cargo build --release
```

The binary will be available at `target/release/dstreams`.

### Add to PATH

```bash
cargo install --path .
```

## Quick Start

### 1. Discover Available Streams

```bash
dstreams discover -c examples/tap-config.json -o catalog.json
```

### 2. Run a Full Sync

```bash
dstreams sync \
  --tap-config examples/tap-config.json \
  --target-config examples/target-config.json \
  --catalog catalog.json \
  --state state.json
```

### 3. Run Tap Only

```bash
dstreams tap -c examples/tap-config.json --catalog catalog.json -o data.jsonl
```

### 4. Run Target Only

```bash
dstreams target -c examples/target-config.json -i data.jsonl
```

## CLI Reference

### Global Flags

- `-v, --verbose`: Enable verbose logging

### Commands

#### `discover`

Discover available streams from a tap and generate a catalog.

```bash
dstreams discover -c <TAP_CONFIG> [-o <OUTPUT>]
```

**Options:**
- `-c, --config <FILE>`: Path to tap configuration file (required)
- `-o, --output <FILE>`: Output catalog file (default: catalog.json)

**Example:**
```bash
dstreams discover -c tap-config.json -o my-catalog.json
```

---

#### `sync`

Run a complete tap-to-target sync pipeline.

```bash
dstreams sync --tap-config <TAP> --target-config <TARGET> [OPTIONS]
```

**Options:**
- `--tap-config <FILE>`: Path to tap configuration (required)
- `--target-config <FILE>`: Path to target configuration (required)
- `--catalog <FILE>`: Path to catalog file (optional)
- `--state <FILE>`: Path to state file (optional)

**Example:**
```bash
dstreams sync \
  --tap-config tap.json \
  --target-config target.json \
  --catalog catalog.json \
  --state state.json
```

---

#### `tap`

Run a tap (source) connector to extract data.

```bash
dstreams tap -c <CONFIG> [OPTIONS]
```

**Options:**
- `-c, --config <FILE>`: Path to tap configuration (required)
- `--catalog <FILE>`: Path to catalog file (optional)
- `--state <FILE>`: Path to state file (optional)
- `-o, --output <FILE>`: Output file (default: stdout)

**Example:**
```bash
dstreams tap -c tap.json --catalog catalog.json -o output.jsonl
```

---

#### `target`

Run a target (destination) connector to load data.

```bash
dstreams target -c <CONFIG> [OPTIONS]
```

**Options:**
- `-c, --config <FILE>`: Path to target configuration (required)
- `-i, --input <FILE>`: Input file (default: stdin)
- `--state <FILE>`: Path to state file (optional)

**Example:**
```bash
dstreams target -c target.json -i data.jsonl
```

---

#### `state`

Manage state files.

**View State:**
```bash
dstreams state view <STATE_FILE>
```

**Clear State:**
```bash
dstreams state clear <STATE_FILE>
```

**Set Bookmark:**
```bash
dstreams state set <STATE_FILE> <STREAM> <VALUE>
```

**Examples:**
```bash
dstreams state view state.json
dstreams state clear state.json
dstreams state set state.json users '{"updated_at":"2024-01-01"}'
```

---

#### `catalog`

Manage catalog files.

**View Catalog:**
```bash
dstreams catalog view <CATALOG_FILE>
```

**Select Streams:**
```bash
dstreams catalog select <CATALOG_FILE> <STREAM1> [STREAM2...]
```

**Deselect Streams:**
```bash
dstreams catalog deselect <CATALOG_FILE> <STREAM1> [STREAM2...]
```

**Examples:**
```bash
dstreams catalog view catalog.json
dstreams catalog select catalog.json users orders
dstreams catalog deselect catalog.json products
```

## Configuration

### Tap Configuration

```json
{
  "name": "my-tap",
  "type": "rest-api",
  "connection": {
    "url": "https://api.example.com"
  },
  "auth": {
    "type": "api_key",
    "key": "your-key",
    "header": "X-API-Key"
  },
  "streams": ["users", "orders"]
}
```

### Target Configuration

```json
{
  "name": "my-target",
  "type": "postgres",
  "connection": {
    "host": "localhost",
    "port": 5432,
    "database": "analytics"
  },
  "auth": {
    "type": "basic",
    "username": "user",
    "password": "pass"
  },
  "batch_size": 1000
}
```

### Authentication Types

- `none`: No authentication
- `api_key`: API key authentication
- `bearer`: Bearer token authentication
- `basic`: Basic username/password authentication
- `oauth2`: OAuth2 authentication

### Connection Types

- **URL-based**: `{"url": "https://..."}`
- **Host/Port**: `{"host": "localhost", "port": 5432, "database": "db"}`
- **File Path**: `{"path": "/path/to/file"}`

## Architecture

DStream follows a modular architecture with clear separation of concerns:

- **Taps**: Extract data from sources (APIs, databases, files)
- **Targets**: Load data into destinations (databases, data warehouses, files)
- **Protocol**: Message-based communication using Apache Arrow
- **State**: Track extraction progress for incremental syncs
- **Catalog**: Define available streams and their schemas

## Development Status

This is an early-stage framework. The core infrastructure is in place, but specific tap and target connectors need to be implemented.

### Implemented

- ✅ Core protocol and message types
- ✅ Configuration management
- ✅ State management
- ✅ Catalog system
- ✅ CLI interface

### Pending

- ⏳ Tap connector implementations
- ⏳ Target connector implementations
- ⏳ Transformation engine
- ⏳ Data validation

## Contributing

Contributions are welcome! To add a new tap or target connector:

1. Implement the relevant traits from `src/core/traits.rs`
2. Add configuration support in `src/core/config.rs`
3. Register the connector in the CLI runner

## License

[Add your license here]
