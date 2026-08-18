#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use axum::{Json, Router, routing::get};
use tower_service::Service;
use worker::*;

pub mod openrouter;

use base64::{Engine, engine::general_purpose::STANDARD};
use openrouter::{ImageGenerationResponse, ImageGenerationResponseDataItem};

static SAMPLE_JPG: &[u8] = include_bytes!("../assets/sample.jpg");

fn router() -> Router {
    Router::new().route("/images", get(images))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}

pub async fn images() -> Json<ImageGenerationResponse> {
    Json(ImageGenerationResponse {
        created: 1748372400,
        data: vec![ImageGenerationResponseDataItem {
            b64_json: STANDARD.encode(SAMPLE_JPG),
            media_type: Some("image/jpeg".to_string()),
        }],
        usage: None,
    })
}
