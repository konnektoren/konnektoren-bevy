# Default command to show help
default:
    @just --list

# Run the demo application
run:
    cargo run --package konnektoren-bevy-demo --bin demo

# Run with hot reloading for desktop development
dev:
    cargo run --features bevy/dynamic_linking --package konnektoren-bevy-demo --bin demo

# Run all tests in the workspace
test:
    cargo test --workspace

# Build and open documentation for the workspace
docs:
    cargo doc --workspace --open
