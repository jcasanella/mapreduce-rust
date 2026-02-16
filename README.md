# mapreduce-rust

Project structure:

```
mapreduce/
├── Cargo.toml              # workspace with members: proto, coordinator, worker
├── proto/
│   ├── Cargo.toml          # shared proto crate
│   ├── build.rs            # compiles registration.proto
│   ├── registration.proto
│   └── src/
│       └── lib.rs          # re-exports generated code
├── coordinator/
│   ├── Cargo.toml          # depends on `proto`
│   └── src/
└── worker/
    └── ...                 # will also depend on `proto`

```