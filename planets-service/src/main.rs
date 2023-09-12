//modules
mod handlers;
mod graphql;
mod repository;
mod schemas;

//extern crate
use dotenv::dotenv;
use actix_web::{web::Data, middleware, App, HttpServer};
use actix_cors::Cors;

//intern crate
use crate::handlers::configure_service;
use crate::graphql::create_schema;

//#[actix_rt::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();
    //let pool = create_connection_pool();
    //run_migrations(&pool);

    //let schema = create_schema_with_context(pool);
    let schema = create_schema().await;
    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
        .configure(configure_service)
        .app_data(Data::new(schema.clone()))
        // the graphiql UI requires CORS to be enabled
        .wrap(Cors::permissive())
        .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8088))?
    .run()
    .await
}


// #[actix_web::main] // or #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(greet)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
