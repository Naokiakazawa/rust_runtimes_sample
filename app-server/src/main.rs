use actix_web::{web, App, HttpResponse, HttpServer};
use async_graphql::{
    http::graphiql_source, Context, EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use log::info;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, ctx: &Context<'_>) -> String {
        info!("Received hello request");
        "Hello, world!".to_string()
    }
}

type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn index(
    // Schema now accessible here
    schema: web::Data<MySchema>,
    request: GraphQLRequest,
) -> web::Json<GraphQLResponse> {
    web::Json(schema.execute(request.into_inner()).await.into())
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8000/", None);
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
            .data(schema.clone())
            .service(web::resource("/").guard(actix_web::guard::Post()).to(index))
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
