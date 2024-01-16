use actix_web::{web, App, HttpResponse, HttpServer};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use log::info;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> String {
        info!("Received hello request");
        "Hello, world!".to_string()
    }
}

type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> HttpResponse {
    let html = playground_source(GraphQLPlaygroundConfig::new("/graph"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Starting server");

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graph")
                    .guard(actix_web::guard::Post())
                    .to(index),
            )
            .service(
                web::resource("/graphiql")
                    .guard(actix_web::guard::Get())
                    .to(graphiql),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
