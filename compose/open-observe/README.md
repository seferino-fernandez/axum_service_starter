# Running Axum Service Starter with OpenObserve

See [openobserve.ai - Docs - Docker](https://openobserve.ai/docs/quickstart/#__tabbed_1_3) for more information.

## Prerequisites

-   Docker
-   Docker Compose

## Configuring OpenObserve and the Axum Service Starter

Add the following environment variables to the `openobserve` service in the `compose.yaml` file:

-   `ZO_ROOT_USER_EMAIL`: The email address of the root user.
-   `ZO_ROOT_USER_PASSWORD`: The password of the root user.

Add the following environment variables to the `axum_service_starter` service in the `compose.yaml` file:

-   `OTEL_PROVIDER_AUTH_TOKEN`: The authentication token for the OpenTelemetry provider. It's a base64 encoded string of the email and password separated by a colon.

## Building the Axum Service Starter

```bash
docker compose build
```

## Running the Axum Service Starter

```bash
docker compose up -d
```

## Stopping the Axum Service Starter

```bash
docker compose down
```

## OpenObserve UI

Open the OpenObserve UI at [http://localhost:5080](http://localhost:5080).
