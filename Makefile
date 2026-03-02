.PHONY: all build build-proto build-coordinator build-worker \
       check clippy clippy-coordinator clippy-worker \
       fmt fmt-check test clean \
       run-coordinator run-worker

all: fmt-check clippy build test

build: build-proto build-coordinator build-worker

build-proto:
	cargo build -p proto

build-coordinator:
	cargo build -p coordinator

build-worker:
	cargo build -p worker

check:
	cargo check --workspace

clippy: clippy-coordinator clippy-worker

clippy-coordinator:
	cargo clippy -p coordinator -- -D warnings

clippy-worker:
	cargo clippy -p worker -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

test:
	cargo test --workspace

run-coordinator:
	cargo run -p coordinator

run-worker:
	cargo run -p worker

clean:
	cargo clean
