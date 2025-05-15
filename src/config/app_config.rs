use std::env;
use std::sync::OnceLock;
use std::time::Duration;

use super::error::ServerError;

const DEFAULT_SERVER_REQUEST_TIMEOUT: u64 = 15;
const DEFAULT_SERVER_HOST: &str = "0.0.0.0";
const DEFAULT_SERVER_PORT: u16 = 8080;
const DEFAULT_SERVER_FILE_UPLOAD_MAX_SIZE: usize = 1024 * 1024 * 10; // 10 MB
const DEFAULT_SERVER_FILE_UPLOAD_MAX_SIZE_ENABLED: bool = true;
const DEFAULT_SERVER_ENVIRONMENT: &str = "development";

const DEFAULT_APP_NAME: &str = "axum-service-starter";

const DEFAULT_MAX_ACCESS_CONTROL_AGE: u64 = 600; // 10 minutes

pub fn app_config() -> &'static AppConfig {
    static INSTANCE: OnceLock<AppConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        AppConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("Unable to load application configuration: {ex:?}"))
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub application: ApplicationConfig,
    pub security: SecurityConfig,
    pub otel: OtelConfig,
    pub otel_provider: OtelProviderConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub file_upload_max_size: usize,
    pub file_upload_max_size_enabled: bool,
    pub environment: String,
    pub timeout: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityConfig {
    pub max_access_control_age: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationConfig {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtelProviderConfig {
    pub provider: Option<String>,
    pub organization: Option<String>,
    pub stream_name: Option<String>,
    pub auth_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtelConfig {
    pub sdk_enabled: bool,
    pub service_name: Option<String>,
    pub traces_endpoint: Option<String>,
    pub logs_endpoint: Option<String>,
    pub metrics_endpoint: Option<String>,
    pub metric_export_interval: Option<Duration>,
}

impl AppConfig {
    fn load_from_env() -> Result<AppConfig, ServerError> {
        Ok(AppConfig {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| DEFAULT_SERVER_HOST.to_string()),
                port: env::var("SERVER_PORT")
                    .ok()
                    .and_then(|s| s.parse::<u16>().ok())
                    .unwrap_or(DEFAULT_SERVER_PORT),
                file_upload_max_size: env::var("SERVER_FILE_UPLOAD_MAX_SIZE")
                    .ok()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(DEFAULT_SERVER_FILE_UPLOAD_MAX_SIZE),
                file_upload_max_size_enabled: env::var("SERVER_FILE_UPLOAD_MAX_SIZE_ENABLED")
                    .ok()
                    .and_then(|s| s.parse::<bool>().ok())
                    .unwrap_or(DEFAULT_SERVER_FILE_UPLOAD_MAX_SIZE_ENABLED),
                environment: env::var("SERVER_ENVIRONMENT")
                    .unwrap_or_else(|_| DEFAULT_SERVER_ENVIRONMENT.to_string()),
                timeout: Duration::from_secs(
                    env::var("SERVER_REQUEST_TIMEOUT")
                        .ok()
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(DEFAULT_SERVER_REQUEST_TIMEOUT),
                ),
            },
            application: ApplicationConfig {
                name: env::var("APP_NAME").unwrap_or_else(|_| DEFAULT_APP_NAME.to_string()),
            },
            security: SecurityConfig {
                max_access_control_age: Duration::from_secs(
                    env::var("SECURITY_MAX_ACCESS_CONTROL_AGE")
                        .ok()
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(DEFAULT_MAX_ACCESS_CONTROL_AGE),
                ),
            },
            otel: Self::load_otel_config(),
            otel_provider: OtelProviderConfig {
                provider: env::var("OTEL_PROVIDER").ok(),
                organization: env::var("OTEL_PROVIDER_ORGANIZATION").ok(),
                stream_name: env::var("OTEL_PROVIDER_STREAM_NAME").ok(),
                auth_token: env::var("OTEL_PROVIDER_AUTH_TOKEN").ok(),
            },
        })
    }

    /// Loads OpenTelemetry related configuration from environment variables.
    fn load_otel_config() -> OtelConfig {
        // Determine if the OTel SDK is explicitly disabled by the environment variable.
        // OTEL_SDK_DISABLED=true means the SDK is disabled.
        // If OTEL_SDK_DISABLED is not set or is "false", the SDK is considered enabled by default.
        let is_otel_sdk_explicitly_disabled: bool = env::var("OTEL_SDK_DISABLED")
            .ok()
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);

        if is_otel_sdk_explicitly_disabled {
            // SDK is disabled, populate OtelConfig with default "disabled" values.
            OtelConfig {
                sdk_enabled: false,
                service_name: None,
                traces_endpoint: None,
                logs_endpoint: None,
                metrics_endpoint: None,
                metric_export_interval: None,
            }
        } else {
            // SDK is enabled (or not explicitly disabled), parse related OTel environment variables.
            let common_otlp_endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok();

            OtelConfig {
                sdk_enabled: true,
                service_name: env::var("OTEL_SERVICE_NAME").ok(),
                traces_endpoint: env::var("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT")
                    .ok()
                    .or_else(|| common_otlp_endpoint.clone()),
                logs_endpoint: env::var("OTEL_EXPORTER_OTLP_LOGS_ENDPOINT")
                    .ok()
                    .or_else(|| common_otlp_endpoint.clone()),
                metrics_endpoint: env::var("OTEL_EXPORTER_OTLP_METRICS_ENDPOINT")
                    .ok()
                    .or(common_otlp_endpoint),
                metric_export_interval: env::var("OTEL_METRIC_EXPORT_INTERVAL")
                    .ok()
                    .and_then(|interval_str| interval_str.parse::<u64>().ok())
                    .map(Duration::from_millis),
            }
        }
    }
}
