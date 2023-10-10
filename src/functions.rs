use actix_web::{HttpResponse, Result};
use chrono::Local;
use rocksdb::{IteratorMode, Options, DB};

use crate::utils::Block;

pub fn create_genesis_block() {
    let genesis_block = Block {
        id: 0.to_string(),
        prev_hash: "".to_string(),
        curr_hash: "xxxxx".to_string(),
        data: "genesis".to_string(),
        timestamp: Local::now().to_string(),
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

pub async fn add_to_blockchain(data: String) -> Result<(), reqwest::Error> {
    let mut block = Block {
        id: "".to_string(),
        prev_hash: "".to_string(),
        curr_hash: "".to_string(),
        data: "".to_string(),
        timestamp: "".to_string(),
    };
    let path = "./my_db";
    let mut opts = Options::default();
    //opts.create_if_missing(true);
    let db = DB::open(&opts, path).expect("Failed to open database");
    let iter = db.iterator(IteratorMode::End);
    for item in iter {
        let (key, value) = item.unwrap();
        let string_data = String::from_utf8_lossy(&key);
        let string_val = String::from_utf8_lossy(&value);
        let val = serde_json::from_str::<Block>(&string_val);
        println!("hiiiiii");
        println!("{:?}",val);
        let cnt = 1;
        match val {
            Ok(last) => {
                block = Block {
                    id: last.id + "1",
                    prev_hash: last.curr_hash.to_string(),
                    curr_hash: "newblock".to_string(),
                    data,
                    timestamp: Local::now().to_string(),
                };
                println!("{:?}",block);
                let block_serialized = serde_json::to_string(&block).expect("Failed to serialize");
                db.put(&block.id.to_string().as_bytes(), &block_serialized)
                    .expect("Failed to store genesis block");
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        break;
    }

    

    Ok(())
}
