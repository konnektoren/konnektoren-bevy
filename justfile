# Default command to show help
default:
    @just --list

# Start trunk server for web development (serves demo)
serve:
    cd demo && trunk serve --open

# Run the desktop demo
run:
    cargo run --package konnektoren-bevy-demo --bin demo

# Run with hot reloading for desktop development
dev:
    cargo run --features bevy/dynamic_linking --package konnektoren-bevy-demo --bin demo

# Build the web version (release)
build:
    cd demo && trunk build --release

# Run all tests in the workspace
test:
    cargo test --workspace

# Build and open documentation for the workspace
docs:
    cargo doc --workspace --open
