use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
    http::Method,
};
use config::Config;
use di::DiContainer;
use interface::handler;
use log::{error, info};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    env_logger::init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let config = match Config::init() {
        Ok(conf) => conf,
        Err(err) => {
            error!("server aborted: {}", err);
            return;
        }
    };

    let article_provider = Arc::new(match DiContainer::new(config).await {
        Ok(cont) => cont,
        Err(err) => {
            error!("server aborted: {}", err);
            return;
        }
    });

    let app = Router::new()
        .route("/", get(handler::check_health::check_health))
        .route("/login", get(handler::login_as_owner::login_as_owner))
        .route("/article", post(handler::post_article::post_article))
        .route("/article/:id", get(handler::read_article::read_article))
        .route("/article/:id", delete(handler::delete_article::delete_article))
        .route("/article/:id", put(handler::update_article::update_article))
        .route("/articles", get(handler::list_article::list_article))
        .layer(Extension(article_provider))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
