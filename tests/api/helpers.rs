use axum::{Router, body::Body, http::Request, http::Response};
use tower::ServiceExt as _;

use axum_service_starter::{config::app_config::app_config, router};

pub struct TestApp {
    pub router: Router,
}

impl TestApp {
    pub fn new() -> Self {
        // Loads the .env file located in the environment's current directory or its parents in sequence.
        // .env used only for development, so we discard error in all other cases.
        dotenvy::dotenv().ok();

        // Parse configuration from the environment.
        // This will exit with a help message if something is wrong.
        let app_config = app_config().to_owned();

        let router = router(app_config);
        Self { router }
    }

    pub async fn request(&self, req: Request<Body>) -> Response<Body> {
        self.router.clone().oneshot(req).await.unwrap()
    }
}
