use crate::certificate::{__path_create_item, __path_get_item, __path_get_owner};
use crate::models::cert_model::Item;
use crate::models::sig_model::AssetDto;
use crate::signature::__path_signature;
use crate::signature_verifier::{__path_check_status, __path_verify_signature};
use utoipa::OpenApi;

// Swagger/OpenAPI configuration
#[derive(OpenApi)]
#[openapi(
    paths(verify_signature, check_status, signature, create_item, get_item, get_owner),
    components(
        schemas(AssetDto, Item),
        // responses(Item)
    ),
    tags(
        (name = "ERI", description = "Signature Verifying APIs")
    ),
    // security(
    //     (),
    //     ("my_auth" = ["read:items", "edit:items"]),
    //     ("token_jwt" = [])
    // ),
    info(
        title = "ERI APIs",
        description = "Signature Verifying Project on the Blockchain",
        contact(name = "DEAN"),
    
    ),
    // servers(
    //     (url = "http://localhost:8989", description = "Local server"),
    //     (url = "http://api.{username}:{port}", description = "Remote API",
    //         variables(
    //             ("username" = (default = "demo", description = "Default username for API")),
    //             ("port" = (default = "8080", enum_values("8080", "5000", "3030"), description = "Supported ports for API"))
    //         )
    //     )
    // )
)]
pub struct ApiDoc;
