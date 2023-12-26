//! This crate provides a wrapper of `anyhow::Error` and `eyre::Report` to be able to make use of
//! them in Rocket framework.

//! # Example
//! ```rust
//! // An example usage of `rocket_errors::anyhow::Result`.
//! use rocket::{get, routes};
//! use rocket_errors::anyhow;
//!
//! #[get("/")]
//! pub fn health_check() -> anyhow::Result<&'static str> {
//!     Ok("Hello, world!")
//! }
//!
//! #[rocket::main]
//! async fn main() -> anyhow::Result<()> {
//!     let _ = rocket::build()
//!         .mount("/hc", routes![health_check])
//!         .launch()
//!         .await?;
//!     Ok(())
//! }
//! ````

#[cfg(not(feature = "eyre"))]
pub mod anyhow;
#[cfg(feature = "eyre")]
pub mod eyre;
