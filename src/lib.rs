use axum::extract::DefaultBodyLimit;
use opentelemetry::global;
use utoipa::OpenApi;
pub mod config;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod utils;

use config::app_config::AppConfig;
use middleware::{security, server};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable as _};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AppState {
    pub app_config: AppConfig,
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Axum Service Starter",
        description = "API documentation for Axum Service Starter",
    ),
    tags(
        (name = "system", description = "API's to manage this system"),
    )
)]
struct ApiDoc;

pub fn router(app_config: AppConfig) -> axum::Router {
    let app_state = AppState { app_config };

    // Create the router with the routes and the OpenAPI documentation.
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/system", routes::HealthApi::router())
        .split_for_parts();

    let otel_metrics_layer = if !app_state.app_config.otel.sdk_disabled {
        // Use `leak()` because the meter provider wants a static string (&str) but the service name is from an env variable.
        let global_meter =
            global::meter_provider().meter(app_state.app_config.application.name.clone().leak());

        Some(
            tower_otel_http_metrics::HTTPMetricsLayerBuilder::builder()
                .with_meter(global_meter)
                .build()
                .unwrap(),
        )
    } else {
        None
    };

    let body_limit_layer = if app_state.app_config.server.file_upload_max_size_enabled {
        Some(server::body_limit_layer(&app_state.app_config.server))
    } else {
        None
    };

    // Combine all the routes and apply the middleware layers.
    // The order of the layers is important. The first layer is the outermost layer.
    let mut router = router
        .merge(Scalar::with_url("/system/api-docs", api))
        .layer(security::cors_layer(&app_state.app_config.security))
        .layer(DefaultBodyLimit::disable());

    if let Some(limit_layer) = body_limit_layer {
        router = router.layer(limit_layer);
    }
    router = router
        .layer(server::normalize_path_layer())
        .layer(server::timeout_layer(&app_state.app_config.server));

    if let Some(otel_metrics_layer) = otel_metrics_layer {
        router = router.layer(otel_metrics_layer);
    }
    router.with_state(app_state)
}
