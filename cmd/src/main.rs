use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Extension, Router,
};
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

    let article_provider = Arc::new(match DiContainer::new().await {
        Ok(cont) => cont,
        Err(err) => {
            error!("server aborted: {}", err);
            return;
        }
    });

    let article_route = Router::new()
        .route("/", post(handler::post_article::post_article))
        .route("/:id", get(handler::read_article::read_article))
        .route("/:id", delete(handler::delete_article::delete_article))
        .route("/:id", put(handler::update_article::update_article));

    let app = Router::new()
        .route("/", get(handler::check_health::check_health))
        .route("/login", get(handler::login_as_owner::login_as_owner))
        .route("/articles", get(handler::list_article::list_article))
        .nest("/article", article_route)
        .layer(Extension(article_provider))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
