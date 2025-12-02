# DStream Execution Guide

## Building the Project

### Debug Build
```bash
cargo build
```

### Release Build (Optimized)
```bash
cargo build --release
```

The binary will be located at:
- Debug: `target/debug/dstreams`
- Release: `target/release/dstreams`

## Running Commands

### Using Cargo
```bash
cargo run -- [COMMAND] [OPTIONS]
```

### Using the Binary Directly
```bash
./target/release/dstreams [COMMAND] [OPTIONS]
```

### Install Globally
```bash
cargo install --path .
dstreams [COMMAND] [OPTIONS]
```

## Example Workflows

### 1. Discovery Workflow

Discover available streams from a data source:

```bash
# Create a tap configuration
cat > tap-config.json << EOF
{
  "name": "my-api",
  "type": "rest-api",
  "connection": {
    "url": "https://api.example.com"
  },
  "auth": {
    "type": "api_key",
    "key": "your-key",
    "header": "X-API-Key"
  }
}
EOF

# Run discovery
dstreams discover -c tap-config.json -o catalog.json

# View the catalog
dstreams catalog view catalog.json
```

### 2. Full Sync Workflow

Run a complete ETL pipeline:

```bash
# Create target configuration
cat > target-config.json << EOF
{
  "name": "my-database",
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
EOF

# Run sync
dstreams sync \
  --tap-config tap-config.json \
  --target-config target-config.json \
  --catalog catalog.json \
  --state state.json \
  --verbose
```

### 3. Tap-Only Workflow

Extract data to a file:

```bash
# Select specific streams
dstreams catalog select catalog.json users orders

# Run tap and save to file
dstreams tap \
  -c tap-config.json \
  --catalog catalog.json \
  --state state.json \
  -o data.jsonl \
  --verbose
```

### 4. Target-Only Workflow

Load data from a file:

```bash
# Load data into target
dstreams target \
  -c target-config.json \
  -i data.jsonl \
  --state state.json \
  --verbose
```

### 5. State Management

```bash
# View current state
dstreams state view state.json

# Set a bookmark manually
dstreams state set state.json users '{"updated_at":"2024-01-01T00:00:00Z"}'

# Clear all state
dstreams state clear state.json
```

### 6. Catalog Management

```bash
# View catalog
dstreams catalog view catalog.json

# Select specific streams
dstreams catalog select catalog.json users orders products

# Deselect streams
dstreams catalog deselect catalog.json products
```

## Logging

### Enable Verbose Logging
```bash
dstreams --verbose [COMMAND]
```

### Environment Variable Control
```bash
# Set log level via environment
RUST_LOG=debug dstreams [COMMAND]
```

## Piping Data

### Tap to Target via Pipe
```bash
dstreams tap -c tap.json | dstreams target -c target.json
```

### Save and Load
```bash
# Extract
dstreams tap -c tap.json -o data.jsonl

# Load later
dstreams target -c target.json -i data.jsonl
```

## Troubleshooting

### Check Configuration
```bash
# Validate by running with verbose flag
dstreams discover -c tap-config.json --verbose
```

### View State
```bash
# Check what's in the state file
dstreams state view state.json
```

### Clear State and Retry
```bash
# If sync is stuck, clear state
dstreams state clear state.json
dstreams sync --tap-config tap.json --target-config target.json
```

## Development Status

The CLI framework is complete, but tap and target connector implementations are pending. When you implement a connector:

1. Create a new module in `src/connectors/`
2. Implement the required traits from `src/core/traits.rs`
3. Register it in the CLI runner
4. Update configuration parsing to support your connector type

## Next Steps

- Implement specific tap connectors (REST API, databases, files)
- Implement specific target connectors (databases, data warehouses, files)
- Add transformation capabilities
- Add data validation
