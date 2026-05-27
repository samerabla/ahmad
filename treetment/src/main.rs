#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::compression::CompressionLayer;
    use tower_http::cors::{Any, CorsLayer};
    use treetment::api::contact::contact_handler;
    use treetment::app::App;
    use treetment::db;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "treetment=debug,tower_http=debug".into()),
        )
        .init();

    dotenvy::dotenv().ok();

    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    db::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/contact", axum::routing::post(contact_handler))
        .route("/health", axum::routing::get(health_check))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(leptos_axum::file_and_error_handler(leptos_options.clone()))
        .layer(cors)
        .layer(CompressionLayer::new())
        .with_state(leptos_options);

    // Provide pool via extension
    let app = app.layer(axum::Extension(pool));

    tracing::info!("Listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
async fn health_check() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!({ "status": "ok" }))
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
