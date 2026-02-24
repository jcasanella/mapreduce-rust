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
├── Cargo.toml                 # Workspace root
├── proto/                     # Shared protobuf definitions
│   ├── build.rs               # Compiles .proto files
│   ├── registration.proto     # Worker registration service definition
│   └── src/lib.rs             # Re-exports generated gRPC code
├── coordinator/               # Coordinator server
│   └── src/
│       ├── main.rs            # gRPC server entrypoint (listens on [::1]:50051)
│       └── apis/
│           ├── mod.rs
│           └── registration.rs  # Registration service implementation
└── worker/                    # Worker client
    └── src/
        └── main.rs            # Connects to coordinator and registers
```

## What's Been Implemented

- **Proto crate** - Shared gRPC service definition for worker registration (`Register` RPC), compiled at build time and re-exported for both coordinator and worker crates.
- **Coordinator** - A gRPC server that listens on `[::1]:50051` and handles worker registration. Registered workers are stored in a thread-safe in-memory `HashMap` (`Arc<RwLock<HashMap>>`), keyed by `worker_id`.
- **Worker** - A gRPC client that connects to the coordinator and sends a registration request with its `worker_id` and `hostname`.

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
cargo run -p coordinator
```

In another terminal, register a worker:

```bash
cargo run -p worker
```
