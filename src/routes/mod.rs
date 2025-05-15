use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

pub mod health;

use crate::{AppState, models::health::HealthResponse};

#[derive(OpenApi)]
#[openapi(components(schemas(HealthResponse)))]
pub struct HealthApi;

impl HealthApi {
    pub fn router() -> OpenApiRouter<AppState> {
        OpenApiRouter::with_openapi(HealthApi::openapi()).routes(routes!(health::health))
    }
}
