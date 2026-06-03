FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin axum_service_starter

# Distroless CC (glibc support) on Debian 13, running as a secure non-root user
FROM gcr.io/distroless/cc-debian13:nonroot AS runtime
WORKDIR /app

# Copy the binary from the builder, ensuring the nonroot user owns it
COPY --chown=nonroot:nonroot --from=builder /app/target/release/axum_service_starter /app/axum_service_starter

ENTRYPOINT ["/app/axum_service_starter"]
