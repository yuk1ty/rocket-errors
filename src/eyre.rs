use rocket::response::{self, Responder};
use rocket::Request;

pub type Result<T, E = EyreReport> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct EyreReport(pub eyre::Report);

impl<E> From<E> for EyreReport
where
    E: Into<eyre::Report>,
{
    fn from(error: E) -> Self {
        EyreReport(error.into())
    }
}

impl<'r> Responder<'r, 'static> for EyreReport {
    fn respond_to(self, request: &Request<'_>) -> response::Result<'static> {
        response::Debug(self.0).respond_to(request)
    }
}
