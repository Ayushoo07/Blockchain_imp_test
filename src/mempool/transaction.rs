use actix_web::{web, HttpResponse, Result};
use chrono::Local;
use permutohedron::LexicalPermutation;
use rocksdb::{IteratorMode, Options, DB};

use crate::{
    utils::{Status, Transaction, TransactionInfo},
    Add_to_blockchain::add_to_blockchain,
};

use super::validation::validate_transaction;

pub async fn add_transaction(transaction: web::Json<TransactionInfo>) -> Result<HttpResponse> {
    let path = "./mempool_db";
    let mut opts = Options::default();
    opts.create_if_missing(true);

    let mut new_id = "".to_string();

    let db = DB::open(&opts, path).expect("Failed to open database");
    let iter = db.iterator(IteratorMode::End);
    for item in iter {
        let (key, value) = item.unwrap();
        let string_data = String::from_utf8_lossy(&key);
        let string_val = String::from_utf8_lossy(&value);
        let val = serde_json::from_str::<Transaction>(&string_val);
        match val {
            Ok(last) => {
                // Calculate Id
                let mut chars: Vec<char> = last.id.chars().collect();
                chars.next_permutation();
                new_id = chars.iter().collect();
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        break;
    }

    if new_id.len() == 0 {
        new_id = "jasjfkasgfuwadqgcbjbaldeuohahdjasugejabs".to_string();
    }

    let transaction = Transaction {
        id: new_id,
        to: transaction.0.to,
        from: transaction.0.from,
        amount: transaction.0.amount,
        timestamp: Local::now().to_string(),
        gas_fees: transaction.0.gas_fees,
        status: Status::PENDING,
    };

    let transaction_serialized = serde_json::to_string(&transaction).expect("Failed to serialize");

    db.put(
        &transaction.id.to_string().as_bytes(),
        &transaction_serialized,
    )
    .expect("Failed to store genesis block");

    Ok(HttpResponse::Ok().json("New transaction added to mempool"))
}



pub async fn pick_transaction() -> Result<HttpResponse, actix_web::Error>  {
    let mut res: Result<(), reqwest::Error> = Ok(());
    let path = "./mempool_db";
    let mut opts = Options::default();
    let db = DB::open(&opts, path).expect("Failed to open database");
    let iter = db.iterator(IteratorMode::Start);
    let mut valid_transaction_vec=Vec::new();
    for item in iter {
        let (key, value) = item.unwrap();
        let string_data = String::from_utf8_lossy(&key);
        let string_val = String::from_utf8_lossy(&value);
        let val = serde_json::from_str::<Transaction>(&string_val);
        let mut cnt=0;
       
        match val {
            Ok(first) => {
                if (validate_transaction(&first).await) {
                    let mut val = first;

                    val.status = Status::PROCESSING;
                    let updated_transaction_id = val.id.clone();
                    let transaction_serialized =
                        serde_json::to_string(&val).expect("Failed to serialize");

                    db.put(&updated_transaction_id.as_bytes(), &transaction_serialized)
                        .expect("Failed to update rejected_transaction_status");
                    valid_transaction_vec.push(val);
                    cnt=cnt+1;
                } else {
                    let mut val = first;
                    val.status = Status::REJECTED;
                    let transaction_serialized =
                        serde_json::to_string(&val).expect("Failed to serialize");

                    db.put(&val.id.to_string().as_bytes(), &transaction_serialized)
                        .expect("Failed to update rejected_transaction_status");
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
            

        
    }
    res=add_to_blockchain(valid_transaction_vec).await;
    match res {
        Ok(()) => Ok(HttpResponse::Ok().json("Transaction Taken up by validator 1")),
        Err(err) => {
            let error_message = format!("Error: {:?}", err);
            let error_response = HttpResponse::InternalServerError().json(error_message);
            Ok(error_response)
        }
    }
}

