use rocket::{get, routes};
use rocket_errors::eyre;

#[get("/")]
pub fn health_check() -> eyre::Result<&'static str> {
    Ok("Hello, world!")
}

#[rocket::main]
async fn main() -> eyre::Result<()> {
    let _ = rocket::build()
        .mount("/hc", routes![health_check])
        .launch()
        .await?;
    Ok(())
}
