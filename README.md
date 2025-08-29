# Heartbeat
===============================================================================

This is a Prometheus client written in Rust that generates heartbeat messages.

## Features

- Prometheus metrics server on port 8000
- Heartbeat counter that increments every second
- Fuel gauge that decreases over time
- Graceful shutdown with Ctrl+C
- Docker containerization support

## Build
-------------------------------------------------------------------------------

### Native Build
```bash
cargo build --release
```

### Docker Build
```bash
docker build -f Dockerfile -t heartbeat .
```

## Run
-------------------------------------------------------------------------------

### Native Run
```bash
cargo run
```

### Docker Run
To view the CLI output, use the `-i` flag as below. To free up the terminal
(e.g. only the metrics at the endpoint are of interest), detach with the `-d`
flag.

```bash
docker run --rm --name heartbeat -tip 8000:8000 heartbeat
```

## Stop
-------------------------------------------------------------------------------

If running interactively, send a SIGINT with Ctrl+C.
If running detached, simply stop with: `docker stop heartbeat`

## Test
-------------------------------------------------------------------------------

```bash
curl http://localhost:8000/metrics
```

Also available from web browsers. On an additional command or a refresh, 
the heartbeat metric should increase and the fuel gauge should decrease.

## Dependencies
-------------------------------------------------------------------------------

- **prometheus**: Rust Prometheus client library
- **tokio**: Async runtime for Rust
- **hyper**: HTTP server implementation

## Architecture
-------------------------------------------------------------------------------

- Uses Rust's async/await for concurrent HTTP server and heartbeat loop
- Memory safe by design with automatic memory management
- Built-in error handling with Result types
- Uses Cargo for dependency management
- Graceful shutdown using tokio's signal handling

## Metrics

- `heartbeats`: Counter that increments every second
- `fuel_gauge`: Gauge that starts at 3600 and decreases by 2 every second
