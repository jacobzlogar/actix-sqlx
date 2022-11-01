use derive_more::{Display};
use actix_web::{middleware::Logger, web, web::Data, App, HttpServer, Responder, error};
use sqlx::{postgres::PgPoolOptions, PgPool, query_file_as};

struct User {
    id: u32,
    email: String,
}
#[derive(Debug, Display)]
struct QueryError {
    name: &'static str,
}

impl error::ResponseError for QueryError {}


async fn index() -> impl Responder {
    "Users"
}

async fn store() -> impl Responder {
    "store"
}

async fn find(id: web::Path<String>, conn: web::Data<State>) -> Result<Option<User>, QueryError> {
    let result = query_file_as!(User, "src/sql/users/find_user.sql", id)
        .fetch_all(&conn.pool)
        .await?;

    return result;
}

async fn update(id: web::Path<String>) -> impl Responder {
    format!("{}", id)
}

fn user_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(index))
            .route(web::post().to(store)),
    );

    cfg.service(
        web::resource("/users/{id}")
            .route(web::get().to(find))
            .route(web::put().to(update)),
    );
}

struct State {
    pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:secret@localhost/actix-jwt")
        .await
        .unwrap_or_else(|error| {
            panic!("{}", error);
        });

    HttpServer::new(move || {
        let state = State { pool: pool.clone() };
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(state))
            .service(web::scope("/api").configure(user_api_routes))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
