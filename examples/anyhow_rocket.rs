use rocket::{get, routes};
use rocket_errors::anyhow;

#[get("/")]
pub fn health_check() -> anyhow::Result<&'static str> {
    Ok("Hello, world!")
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let _ = rocket::build()
        .mount("/hc", routes![health_check])
        .launch()
        .await?;
    Ok(())
}
