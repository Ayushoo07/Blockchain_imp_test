use actix_web::{HttpResponse, Result};
use chrono::Local;
use hex::encode;
use permutohedron::LexicalPermutation;
use ring::digest::{Context, SHA256};
use rocksdb::{IteratorMode, Options, DB};

use crate::utils::{Block, Transaction, Status};

pub fn create_genesis_block() {
    let mut curr_hash = Context::new(&SHA256);

    let curr_timestamp = Local::now().to_string();

    // Calculate the SHA-256 hash
    curr_hash.update((curr_timestamp.to_string() + "gensis_block").as_bytes());
    let gensis_transaction=Transaction
    {
        id:"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
        from:"genesis".to_string(),
        to:"genesis".to_string(),
        status:Status::ACCEPTED,
        amount:0,
        gas_fees:0,
        timestamp:Local::now().to_string()

    };

    let genesis_block = Block {
        id: "abcdefghijklmnopqrstuvwxyz".to_string(),
        prev_hash: "".to_string(),
        curr_hash: encode(curr_hash.finish()),
        data: gensis_transaction,
        timestamp: curr_timestamp,
    };
    let path = "./my_db";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db = DB::open(&opts, path).expect("Failed to open database");
    let genesis_block_serialized =
        serde_json::to_string(&genesis_block).expect("Failed to serialize");
    db.put(
        &genesis_block.id.to_string().as_bytes(),
        &genesis_block_serialized,
    )
    .expect("Failed to store genesis block");
}
pub async fn get_blocks() -> Result<HttpResponse> {
    let path = "./my_db";
    let mut opts = Options::default();
    // opts.create_if_missing(true);
    let db = DB::open(&opts, path).expect("Failed to open database");
    let iter = db.iterator(IteratorMode::From(&[0u8], rocksdb::Direction::Forward));
    for item in iter {
        let (key, value) = item.unwrap();
        let string_data = String::from_utf8_lossy(&key);
        let string_val = String::from_utf8_lossy(&value);
        let val = serde_json::from_str::<Block>(&string_val);

        println!("Saw {:?} {:?}", string_data, val);
        println!(" ");
    }
    Ok(HttpResponse::Ok().json("All blocks get Successfully"))
}
