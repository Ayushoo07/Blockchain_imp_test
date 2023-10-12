use actix_web::{HttpResponse, get, web};
use rocksdb::{Options, DB, IteratorMode};

use crate::utils::{Transaction, Status};

pub async fn get_all_transactions() -> Result<HttpResponse, actix_web::Error> {
    let path = "./mempool_db";
    let mut opts = Options::default();
    let db = DB::open(&opts, path).expect("Failed to open database");
    let mut transactions: Vec<Transaction> = Vec::new();
    let iter = db.iterator(IteratorMode::End);
    for item in iter {
        let (key, value) = item.unwrap();
        let string_data = String::from_utf8_lossy(&key);
        let string_val = String::from_utf8_lossy(&value);
        let val = serde_json::from_str::<Transaction>(&string_val);
        match val {
            Ok(value) => {
                transactions.push(value);
            }
            Err(err) => {}
        };
    }
    let response_json = serde_json::to_string(&transactions).expect("Failed to serialize response");

    // Return the response as JSON
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response_json))
}

pub async fn get_all_pending_transactions() -> Result<HttpResponse, actix_web::Error> {
    let path = "./mempool_db";
    let mut opts = Options::default();
    let db = DB::open(&opts, path).expect("Failed to open database");
    let mut transactions: Vec<Transaction> = Vec::new();
    let iter = db.iterator(IteratorMode::End);
    for item in iter {
        let (key, value) = item.unwrap();
        let string_data = String::from_utf8_lossy(&key);
        let string_val = String::from_utf8_lossy(&value);
        let val = serde_json::from_str::<Transaction>(&string_val);
        match val {
            Ok(value) => {
                
                match value.status{
                    Status::PENDING=>
                    {
                        transactions.push(value);
                    }
                    _=>
                    {

                    }
                }
                    
                
                
            }
            Err(err) => {}
        };
    }
    let response_json = serde_json::to_string(&transactions).expect("Failed to serialize response");

    // Return the response as JSON
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response_json))
}

#[get("/get_transaction_by_id/{transaction_id}")] // <- define path parameters
pub async fn get_transaction_by_id(url: web::Path< String>) -> Result<HttpResponse, actix_web::Error> {
    

    let path = "./mempool_db";
    let mut opts = Options::default();
    let db = DB::open(&opts, path).expect("Failed to open database");
    
    let mut response_json="".to_string();
    match db.get(url.as_bytes()) {
        Ok(Some(value)) => {
            // Data for the ID exists
            let data = String::from_utf8_lossy(&value);
            let val = serde_json::from_str::<Transaction>(&data);
            match val {
                Ok(transaction)=>
                {
                    response_json = serde_json::to_string(&transaction).expect("Failed to serialize response");

                    // Return the response as JSON
                    
                }
                Err(err)=>
                {
                    println!("{:?}",err);
                }
                
            }
        }
        Ok(None) => {
            // Data not found for the specified ID
            println!("Data not found for ID: {}", url);
            
        }
        Err(err) => {
            // Handle any potential errors
            eprintln!("Error while retrieving data: {}", err);
        }
    }
    
    

    // Return the response as JSON
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response_json))
}
