
pub mod utils;
pub mod take_transaction;
pub mod functions;
use actix_web::{web, App, HttpServer};
use functions::{create_genesis_block, get_blocks};
use utils::REMOTE_ADDRESS;
use take_transaction::take_transaction;


fn add_node(node : &'static str ) {
    unsafe {
        REMOTE_ADDRESS.push(node);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    add_node("172.16.14.97:8081");
    add_node("172.16.14.97:8080");
    add_node("172.16.14.97:8082");
    add_node("172.16.14.97:8083");
    let res=create_genesis_block();

    
    HttpServer::new(|| 
        App::new()
                .route("/take_transaction", web::post().to(take_transaction))
                .route("getblocks",web::get().to(get_blocks))
    )
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}