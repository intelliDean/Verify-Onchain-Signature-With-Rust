use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::certificate::{create_item, get_item, get_owner};
use crate::models::cert_model::RouterPath;
use crate::signature::signature;
use crate::signature_verifier::{check_status, verify_signature};
use crate::swagger_config::ApiDoc;
use crate::utility::AppState;

pub fn paths(state: AppState, path: RouterPath) -> Router {
    let app = Router::new()
        .route(&path.verify, post(verify_signature))
        .route(&path.verify_status, get(check_status))
        .route(&path.signature, post(signature))
        .route(&path.create_item, post(create_item))
        .route(&path.get_item, get(get_item))
        .route(&path.get_owner, get(get_owner))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
        .layer(CorsLayer::permissive()); // Optional: Enable CORS

    app
}