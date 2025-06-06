mod server;
mod signature_verifier;
mod models;
mod utility;
mod signature;
mod verify_ownership;
mod certificate;
mod qr_code;
mod swagger_config;
mod app_router;
mod app_state;

use server::*;


#[tokio::main]
async fn main() {
    
    server().await.expect("Error!");

}
