
IMAGE_NAME := "axum-service-starter"

default:
    @just --list

# Run all tests
test:
	cargo nextest run --all-targets --all-features
	cargo nextest run
	cargo test --doc

# Lint the code with Clippy
lint:
    cargo clippy

# Format the code using rustfmt
format:
    cargo fmt --all
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode for production
release:
    cargo build --release

# Clean the project directory
clean:
    cargo clean

# Build Docker image
docker-build:
    docker build -t {{IMAGE_NAME}} .

# Run Docker image
docker-run:
    docker run -e OTEL_SDK_DISABLED=true {{IMAGE_NAME}}

# Run project locally
run:
    OTEL_SDK_DISABLED=true cargo run

# Generate and show documentation
docs:
    cargo doc --open
