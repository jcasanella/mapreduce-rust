# mapreduce-rust

A distributed MapReduce framework built in Rust using gRPC for communication between nodes.

## What is MapReduce?

MapReduce is a programming model for processing large datasets in parallel across a distributed cluster. It works in two main phases:

1. **Map** - The input data is split into chunks and distributed to worker nodes. Each worker applies a map function to its chunk, producing a set of intermediate key-value pairs.
2. **Reduce** - The intermediate results are grouped by key and passed to a reduce function, which aggregates them into the final output.

A **coordinator** orchestrates the entire process: it assigns tasks to workers, tracks their progress, and handles failures.

## Purpose

This project implements a MapReduce framework from scratch in Rust, leveraging:

- **gRPC** (via `tonic`) for communication between the coordinator and workers
- **Protocol Buffers** for message serialization
- **Tokio** for async runtime
- Rust's type system and ownership model for safe concurrency

## Project Structure

```
mapreduce/
├── Cargo.toml                   # Workspace root
├── Makefile                     # Build, lint, format, and run targets
├── proto/                       # Shared protobuf definitions
│   ├── build.rs                 # Compiles .proto files
│   ├── registration.proto       # Worker registration service
│   ├── heartbeat.proto          # Worker heartbeat service
│   └── src/lib.rs               # Re-exports generated gRPC code
├── coordinator/                 # Coordinator server
│   └── src/
│       ├── main.rs              # gRPC server entrypoint (listens on [::1]:50051)
│       ├── coordinator_state.rs # Shared state: worker registry and heartbeat tracking
│       └── apis/
│           ├── mod.rs
│           ├── registration.rs  # Registration service implementation
│           └── heartbeat.rs     # Heartbeat service implementation
└── worker/                      # Worker client
    └── src/
        ├── main.rs              # Registers and sends periodic heartbeats
        └── config.rs            # Configuration loaded from .env
```

## What's Been Implemented

- **Proto crate** - Shared gRPC service definitions for worker registration and heartbeat, compiled at build time and re-exported for both coordinator and worker crates.
- **Coordinator** - A gRPC server that listens on `[::1]:50051` and handles two services:
  - **Registration** - Workers register with their `worker_id` and `hostname`. Registrations are stored in a thread-safe in-memory `HashMap` (`Arc<RwLock<HashMap>>`), keyed by `worker_id`.
  - **Heartbeat** - Workers send periodic heartbeats. The coordinator tracks each worker's last heartbeat timestamp and failed heartbeat count.
  - **Heartbeat monitoring** - A background task runs every 10 seconds to validate heartbeats against registered workers. If a worker has not sent a heartbeat within 10 seconds, it is considered dead and automatically removed from both the registered workers and heartbeats maps.
- **Coordinator state** - Shared state (`CoordinatorState`) uses `DashMap` for lock-free concurrent access to registered workers and heartbeat data. The coordinator runs two concurrent `tokio::spawn` tasks: one for the gRPC server and one for heartbeat monitoring, coordinated via `tokio::select!` with graceful shutdown on Ctrl+C.
- **Worker** - A gRPC client that:
  1. Registers with the coordinator on startup
  2. Sends heartbeats every 5 seconds via a background `tokio::spawn` task

## Worker Configuration

The worker reads its configuration from environment variables. You can set them via a `.env` file in the project root:

```env
WORKER_ID=worker-1
HOSTNAME=worker1.local
```

| Variable    | Required | Default        | Description                          |
|-------------|----------|----------------|--------------------------------------|
| `WORKER_ID` | Yes      | —              | Unique identifier for the worker     |
| `HOSTNAME`  | No       | `worker.local` | Hostname the worker registers with   |

## Running

Start the coordinator:

```bash
make run-coordinator
```

In another terminal, start a worker:

```bash
make run-worker
```

## Build and Development

| Command | Description |
|---|---|
| `make build` | Build all crates (proto, coordinator, worker) |
| `make check` | Fast type-check without producing binaries |
| `make clippy` | Run clippy lints on coordinator and worker |
| `make fmt` | Auto-format all code |
| `make fmt-check` | Check formatting (useful for CI) |
| `make test` | Run all tests across the workspace |
| `make all` | Full pipeline: format check + clippy + build + test |
| `make clean` | Remove build artifacts |
