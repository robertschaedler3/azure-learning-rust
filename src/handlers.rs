use std::convert::Infallible;

use warp::{http::StatusCode, reject::Reject, Rejection, Reply};

use crate::{error::Error, module::Payload, Platform};

impl Reject for crate::error::Error {}

pub async fn info(platform: Platform) -> Result<impl Reply, Rejection> {
    let _platform = platform.lock().unwrap();

    // Future Exercise: get all the info from the modules
    Ok(warp::reply::json(&123))
}

pub async fn reported(
    platform: Platform,
    component: &str,
    object: &str,
) -> Result<impl Reply, Rejection> {
    let platform = platform.lock().unwrap();
    let value = platform.get(component, object)?;
    Ok(warp::reply::json(&value))
}

pub async fn desired(
    component: &str,
    object: &str,
    platform: Platform,
    payload: Payload,
) -> Result<impl Reply, Rejection> {
    let platform = platform.lock().unwrap();
    platform.set(component, object, &payload)?;
    Ok(warp::reply::json(&()))
}

/// Handles a rejection by logging the error and returning a JSON response with the error message.
/// If the error is a `crate::error::Error`, the error message will be returned. Otherwise, a
/// generic "Internal Server Error" message will be returned.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = match err.find::<crate::error::Error>() {
        Some(err) => {
            log::error!("{}", err);
            match err {
                // --- LAB 4 ---
                // - Uncomment to add the Error::ComponentNotFound variant to the match arm.
                // Error::ComponentNotFound(component) => {
                //     (StatusCode::NOT_FOUND, format!("Component not found: {}", component))
                // }
                Error::Json(err) => (StatusCode::BAD_REQUEST, err.to_string()),
                Error::Null(err) => (StatusCode::BAD_REQUEST, err.to_string()),
                Error::Io(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
                Error::Library(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
                Error::Errno(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.0.to_string()),
                // _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            }
        }
        None => {
            log::error!("Unhandled rejection: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            )
        }
    };

    let json = warp::reply::json(&serde_json::json!( {
        "error": message,
    }));

    Ok(warp::reply::with_status(json, code))
}
