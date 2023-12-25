use rocket::response::{self, Responder};
use rocket::Request;

pub type Result<T, E = AnyhowError> = std::result::Result<T, E>;

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
