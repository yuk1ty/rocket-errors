use rocket::response::{self, Responder};
use rocket::Request;

/// A type alias with [`AnyhowError`] to use `anyhow::Result` in Rocket framework.
pub type Result<T, E = AnyhowError> = std::result::Result<T, E>;

/// A wrapper of `anyhow::Error` to be able to make use of `anyhow` in Rocket framework.
/// [`rocket::response::Responder`] is implemented to this type.
#[derive(Debug)]
pub struct AnyhowError(pub anyhow::Error);

impl<E> From<E> for AnyhowError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        AnyhowError(error.into())
    }
}

impl<'r> Responder<'r, 'static> for AnyhowError {
    fn respond_to(self, request: &Request<'_>) -> response::Result<'static> {
        response::Debug(self.0).respond_to(request)
    }
}
