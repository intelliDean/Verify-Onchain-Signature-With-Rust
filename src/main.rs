mod server;
mod signature_verifier;
mod models;
mod utility;
mod signature;
mod verify_ownership;

use server::*;


#[tokio::main]
async fn main() {
    
    server().await.expect("Error!");

}
