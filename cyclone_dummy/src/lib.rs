#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use std::sync::LazyLock;

use axum::{Json, Router, routing::get};
use tower_service::Service;
use worker::*;

pub mod openrouter;

use base64::{Engine, engine::general_purpose::STANDARD};
use openrouter::{ImageGenerationResponse, ImageGenerationResponseDataItem};

static SAMPLE_JPG: &[u8] = include_bytes!("../assets/sample.jpg");
static SAMPLE_JPG_B64: LazyLock<String> = LazyLock::new(|| STANDARD.encode(SAMPLE_JPG));

fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/images", get(images))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "api/v1/"
}

pub async fn images() -> Json<ImageGenerationResponse> {
    Json(ImageGenerationResponse {
        created: 1748372400,
        data: vec![ImageGenerationResponseDataItem {
            b64_json: SAMPLE_JPG_B64.clone(),
            media_type: Some("image/jpeg".to_string()),
        }],
        usage: None,
    })
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    async fn body_bytes(response: axum::http::Response<Body>) -> Vec<u8> {
        response
            .into_body()
            .collect()
            .await
            .expect("failed to read response body")
            .to_bytes()
            .to_vec()
    }

    #[tokio::test]
    async fn root_returns_ok() {
        let response = router()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = body_bytes(response).await;
        assert_eq!(body, b"api/v1/");
    }

    #[tokio::test]
    async fn images_returns_compliant_response() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/images")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = body_bytes(response).await;
        let parsed: ImageGenerationResponse =
            serde_json::from_slice(&body).expect("response is not a valid ImageGenerationResponse");

        assert_eq!(parsed.created, 1748372400);
        assert!(parsed.usage.is_none());
        assert_eq!(parsed.data.len(), 1);

        let image = &parsed.data[0];
        assert_eq!(image.media_type.as_deref(), Some("image/jpeg"));

        let decoded = STANDARD
            .decode(&image.b64_json)
            .expect("b64_json is not valid base64");
        assert_eq!(decoded, SAMPLE_JPG);
        // JPEG magic bytes
        assert_eq!(&decoded[0..2], &[0xFF, 0xD8]);
    }

    #[tokio::test]
    async fn unknown_route_returns_not_found() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/nope")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
