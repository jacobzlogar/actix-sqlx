use crate::service::users::{queryAll};
use actix_web::{get, post, web, Error, HttpResponse};

#[get("/users")]
pub async fn index(pool: web::Data<PgPoolOptions>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = pool.get()?;
        user_repository::get_users(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}
