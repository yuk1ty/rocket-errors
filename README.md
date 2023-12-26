# rocket-errors

A crate that can handle `anyhow` and `eyre` on Rocket v0.5+.

## Usage example

Please see actual examples in the `/examples` directory.

### `anyhow`

```rust
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
```

### `eyre`

```rust
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
```

## Install

### `anyhow`

`anyhow` is turned on by default. You just need to add a dependency to this crate:

```
rocket-errors = { version = "0.1" }
```

### `eyre`

Using `eyre` is optional. You would like to use it, you need to add a dependency to this crate with a feature flag.

```
rocket-errors = { version = "0.1", features = ["eyre"] }
```

## License

This project is licensed under the [MIT license](https://github.com/yuk1ty/rocket-errors/blob/main/LICENSE).
