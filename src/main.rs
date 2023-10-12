
pub mod utils;
pub mod functions;
pub mod Add_to_blockchain;
pub mod mempool;
pub mod accounts;
use accounts::data::{add_account, get_all_accounts};
use actix_web::{web, App, HttpServer};
use functions::{create_genesis_block, get_blocks};

use mempool::{transaction::{add_transaction, pick_transaction}, get_transactions::{get_all_transactions, get_all_pending_transactions}};
use utils::REMOTE_ADDRESS;



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
                .route("/take_transaction", web::post().to(pick_transaction))
                .route("getblocks",web::get().to(get_blocks))
                .route("/new_transaction",web::post().to(add_transaction))
                .route("/get_transactions",web::get().to(get_all_transactions))
                .route("/add_account",web::post().to(add_account))
                .route("/getall_accounts",web::get().to(get_all_accounts))
                .route("/get_all_pending_transactions",web::get().to(get_all_pending_transactions))

    )
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}