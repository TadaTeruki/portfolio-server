use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Extension, Router,
};
use config::Config;
use di::DiContainer;
use interface::handler;
use log::error;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::init().expect("server aborted: config file must be set");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin([config.get_allow_origin().parse().expect(
            "server aborted: an error occured while parsing Access-Control-Allow-Origin value",
        )]);

    let article_provider = Arc::new(match DiContainer::new(config.clone()).await {
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
        .route("/login", post(handler::login_as_owner::login_as_owner))
        .route(
            "/auth",
            get(handler::check_authorization::check_authorization),
        )
        .route("/articles", get(handler::list_article::list_article))
        .nest("/article", article_route)
        .layer(Extension(article_provider))
        .layer(cors);

    let addr = SocketAddr::from((
        match config.is_release_mode() {
            true => [0, 0, 0, 0],
            false => [127, 0, 0, 1],
        },
        config.get_port(),
    ));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
