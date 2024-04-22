#![allow(unused_imports)]

use std::env;
use std::{fs::File, sync::Arc};

use const_format::concatcp;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, EndpointExt, Route};
use poem_grants::GrantsMiddleware;
use poem_openapi::OpenApiService;
use sqlx::{migrate, PgPool};
use time::OffsetDateTime;

use crate::api::tag::TagService;
use crate::api::test::TestService;
use crate::auth::middleware::auth_extractor;
use crate::store::{Environment, Store};

const DOMAIN: &str = "localhost";
const PORT: u16 = 8080;
const LOCATION: &str = concatcp!("http://", DOMAIN, ":", PORT);

pub mod api;
pub mod auth;
pub mod store;
pub mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    color_eyre::install().unwrap();
    // Logging setup
    let tracing_sub = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(tracing_sub).unwrap();
    dotenvy::dotenv().ok();

    let env = Arc::new(Environment::init().unwrap());

    // Init database and migrations
    let store = {
        let pool = PgPool::connect(&env.pg_url).await?;

        // I have no idea why it doesn't acccept a string slice
        let client = redis::Client::open(env.redis_url.as_ref()).unwrap();
        let connection = client.get_multiplexed_tokio_connection().await.unwrap();

        // migrations should happen only when everything is connected ok
        migrate!("./migrations").run(&pool).await?;
        Store::new(pool, connection)
    };

    let test_service = OpenApiService::new(TestService, "Test", "0.1-alpha");
    let tag_service = OpenApiService::new(TagService::new(store), "Tag service", "0.1-alpha");

    let app = Route::new()
        .nest("/tag", tag_service)
        .nest("/", test_service);

    /*
        // Init services
        let main_service = OpenApiService::new(MainApi::new(pool.clone()), "Main", "1.0.0")
            .server(concatcp!("http://localhost:", PORT, "/api"));
        let auth_service = OpenApiService::new(AuthApi::new(pool.clone()), "Auth", "1.0.0")
            .server(concatcp!("http://localhost:", PORT, "/api/oauth"));
        let thread_service = OpenApiService::new(ThreadApi::new(pool.clone()), "Thread", "1.0.0")
            .server(concatcp!("http://localhost:", PORT, "/api/thread"));

        let app =
         Route::new()
            .nest(
                "/api",
                Route::new()
                    .nest("/docs", main_service.swagger_ui())
                    .nest("/", main_service)
                    .nest("/oauth/docs", auth_service.swagger_ui())
                    .nest("/oauth", auth_service)
                    .nest("/forum/docs", thread_service.swagger_ui())
                    .nest("/forum", thread_service)
                    .with(GrantsMiddleware::with_extractor(auth_extractor)),
            );
    */

    let listener = TcpListener::bind(("0.0.0.0", PORT));
    println!(
        "Starting server on port {} ({})",
        PORT, LOCATION
    );
    poem::Server::new(listener).run(app).await?;

    Ok(())
}
