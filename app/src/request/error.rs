use std::{error::Error, fmt::Display};

pub(crate) type RequestResult<T> = Result<T, RequestError>;

#[derive(Debug)]
pub(crate) enum RequestError {
    RequestEncodingDismatchError(RequestEncodingDismatchError),
    NetRequestError(gloo::net::Error),
    LoadModelError(tobj::LoadError),
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            RequestError::RequestEncodingDismatchError(e) => e.to_string(),
            RequestError::NetRequestError(e) => e.to_string(),
            RequestError::LoadModelError(e) => e.to_string(),
        };

        write!(f, "{}", msg)
    }
}

impl Error for RequestError {}

impl From<RequestEncodingDismatchError> for RequestError {
    fn from(e: RequestEncodingDismatchError) -> Self {
        Self::RequestEncodingDismatchError(e)
    }
}

impl From<gloo::net::Error> for RequestError {
    fn from(e: gloo::net::Error) -> Self {
        Self::NetRequestError(e)
    }
}

impl From<tobj::LoadError> for RequestError {
    fn from(e: tobj::LoadError) -> Self {
        Self::LoadModelError(e)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RequestEncodingDismatchError {
    pub(super) url: String,
    pub(super) try_to_encode_as: &'static str,
}

impl Display for RequestEncodingDismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RequestEncodingDismatchError-> request from url [{}]-> try to encode as [{}]",
            self.url, self.try_to_encode_as
        )
    }
}

impl std::error::Error for RequestEncodingDismatchError {}
