//! Error handling example with `thiserror`.

use rocket::{get, http::Status, response::Responder, routes};
use rocket_errors::anyhow;

#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("Bad request was sent")]
    BadRequest,
    #[error("Unexpected error occurred")]
    InternalServerError,
}

impl Responder<'_, 'static> for MyError {
    fn respond_to(self, req: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            MyError::BadRequest => Status::BadRequest.respond_to(req),
            MyError::InternalServerError => Status::InternalServerError.respond_to(req),
        }
    }
}

#[get("/")]
fn health_check() -> anyhow::Result<&'static str> {
    Ok("Hello, world!")
}

#[get("/bad")]
fn bad_request() -> anyhow::Result<(), MyError> {
    // This endpoint always returns 400.
    Err(MyError::BadRequest)
}

#[get("/error")]
fn internal_server_error() -> anyhow::Result<(), MyError> {
    // This endpoint always returns 500.
    Err(MyError::InternalServerError)
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let _ = rocket::build()
        .mount("/hc", routes![health_check])
        .mount("/example", routes![bad_request, internal_server_error])
        .launch()
        .await?;
    Ok(())
}
