use std::time::Duration;

use blueprint_sdk::logging;
use blueprint_sdk::tokio;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use txrelayer_blueprint as blueprint;

#[blueprint_sdk::main(env, skip_logger)]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "gadget=debug,{}=debug,tower_http=debug",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    color_eyre::install()?;

    let app_config_name = std::env::var("APP_CONFIG_NAME").unwrap_or_else(|_| "config".to_string());
    let context = blueprint::ServiceContext::new(
        env,
        blueprint::call_permit::CALL_PERMIT_ADDRESS,
        &app_config_name,
    )
    .await?;
    let port = context.app_config().port;
    // build our application with some routes
    let app = axum::Router::new()
        .nest("/api/v1", blueprint::http::routes())
        .fallback(handler_404)
        .with_state(context)
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(10)),
        ));

    // run it with hyper
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    logging::debug!(%addr, "HTTP service started");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    logging::info!("Exiting...");
    Ok(())
}

async fn handler_404() -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "nothing to see here")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
