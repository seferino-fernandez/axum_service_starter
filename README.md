# Axum Service Starter

This repository contains Axum Service Starter built using [Axum](https://github.com/tokio-rs/axum).

The full list of crates used can be found in the [Cargo.toml](./Cargo.toml) file. However, here are some key ones:

-   [axum](https://github.com/tokio-rs/axum) - A user-friendly, modular web framework built with Tokio, Tower, and Hyper.
-   [Insta](https://insta.rs/) - A library for snapshot testing in Rust.
-   [utoipa](https://github.com/juhaku/utoipa) - A library for generating OpenAPI documentation in Rust.
-   [opentelemetry-rust](https://github.com/open-telemetry/opentelemetry-rust) - OpenTelemetry for Rust.

## Getting Started

### Rename the application

Rename all of these values to your application name:
- axum_service_starter
- axum-service-starter
- Axum Service Starter

### Configure the Application

#### Environment Variables

You can use the [.env.example](./.env.example) file or [src/config/app_config.rs](./src/config/app_config.rs) to view and configure the application.

### Starting the Application

With everything else set up, all you need to do now is:

```shell
cargo run
```

### Running Tests

Run tests:

```sh
cargo test
```

Run Snapshot tests:

```sh
cargo insta test
```

Run and review Snapshot tests:

```sh
cargo insta test --review
```

### Linting and Formatting

Run Clippy:

```sh
cargo clippy
```

Run Rustfmt:

```sh
cargo fmt
```

## Deployment

For building and running the docker image locally:

```sh
docker build -t axum-service-starter .
docker run -p 8080:8080 axum-service-starter
```

## API Documentation

The API documentation is available at [http://localhost:8080/system/api-docs](http://localhost:8080/system/api-docs) when running locally.

### curl Examples

#### Health Check

```bash
curl http://localhost:8080/system/health
```
