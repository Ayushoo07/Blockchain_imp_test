use chrono::Local;
use hex::encode;
use permutohedron::LexicalPermutation;
use ring::digest::{Context, SHA256};
use rocksdb::{Options, IteratorMode, DB};

use crate::utils::Block;


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
                // Calculate Id
                let mut chars: Vec<char> = last.id.chars().collect();
                chars.next_permutation();
                let latest:String =chars.iter().collect();

                // calculate Hash
                let mut curr_hash = Context::new(&SHA256);

                let curr_timestamp = Local::now().to_string();
            
                // Calculate the SHA-256 hash
                curr_hash.update((curr_timestamp.to_string() + "gensis_block").as_bytes());
                block = Block {
                    id: latest,
                    prev_hash: last.curr_hash.to_string(),
                    curr_hash:encode(curr_hash.finish()),
                    data,
                    timestamp: curr_timestamp
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
