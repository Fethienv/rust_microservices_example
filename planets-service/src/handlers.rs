
//extern crate
use actix_web::{guard, web, HttpRequest, HttpResponse, Result};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};


use crate::graphql::AppSchema;

// Webserver handlers configurations
pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::resource("/")
        .route(web::post().to(index))
        .route(web::get().guard(guard::Header("upgrade", "websocket")).to(index_ws))
        .route(web::get().to(index_playground))
        
    );
}

//processes GraphQL queries and mutations
async fn index(
    schema: web::Data<AppSchema>,
    //http_req: HttpRequest,
    req: GraphQLRequest,
) -> GraphQLResponse {
     let query = req.into_inner();
    // let getting_role_result = common_utils::get_role(http_req);
    // query = query.data(getting_role_result);
    schema.execute(query).await.into()
}

//processes GraphQL subscriptions
async fn index_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

//provides Playground GraphQL IDE
async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/")))
}


