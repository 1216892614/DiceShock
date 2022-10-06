use std::str::FromStr;
use gloo::net::http::Request;

pub(crate) type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub(crate) enum AppError {
    BrowserIncompatible(&'static str),
    Request(crate::request::error::RequestError),
    Resources(crate::resources::error::ResourcesError),
}

impl AppError {
    pub(crate) fn submit(&self, msg: &str) {
        let msg = format!("{self}-> {msg}");

        wasm_bindgen_futures::spawn_local(async move {
            Request::post("/api/error/")
                .body(js_sys::JsString::from_str(&msg).unwrap())
                .send()
                .await
                .unwrap();
        })
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = js_sys::Date::new_0()
            .to_time_string()
            .as_string()
            .expect("Cannt convert time string to string when describe an error");

        let tag = match self {
            AppError::BrowserIncompatible(_) => "BROWSER_INCOMPATIBLE",
            AppError::Request(_) => "REQUEST",
            AppError::Resources(_) => "RESOURCES",
        };

        let msg = match self {
            AppError::BrowserIncompatible(s) => format!(
                "This page is not supported by your browser, check <{}> to get more information",
                s
            ),
            AppError::Request(e) => e.to_string(),
            AppError::Resources(e) => e.to_string(),
        };

        write!(f, "[{time} {tag}] {msg}")
    }
}

impl std::error::Error for AppError {}

impl From<crate::request::error::RequestError> for AppError{
    fn from(e: crate::request::error::RequestError) -> Self {
        Self::Request(e)
    }
}

impl From<crate::resources::error::ResourcesError> for AppError{
    fn from(e: crate::resources::error::ResourcesError) -> Self {
        Self::Resources(e)
    }
}