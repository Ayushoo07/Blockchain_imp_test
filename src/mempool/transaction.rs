use actix_web::{web, Result,HttpResponse, };
use chrono::Local;
use permutohedron::LexicalPermutation;
use rocksdb::{Options, DB, IteratorMode};

use crate::{utils::{Transaction, Status,TransactionInfo}, Add_to_blockchain::add_to_blockchain};

pub async fn add_transaction(transaction:web::Json<TransactionInfo>)->Result<HttpResponse>
{


    let path = "./mempool_db";
    let mut opts = Options::default();
    opts.create_if_missing(true);
   
    let mut new_id="".to_string();




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
                new_id =chars.iter().collect();
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        break;
    }

    
    if new_id.len()==0{

    new_id="jasjfkasgfuwadqgcbjbaldeuohahdjasugejabs".to_string();

    }


    let transaction=Transaction{
        id:new_id,
        to:transaction.0.to,
        from:transaction.0.from,
        amount:transaction.0.amount,
        timestamp:Local::now().to_string(),
        gas_fees:transaction.0.gas_fees,
        status:Status::PENDING

    };

    let transaction_serialized =
        serde_json::to_string(&transaction).expect("Failed to serialize");
   
    db.put(
        &transaction.id.to_string().as_bytes(),
        &transaction_serialized,
    )
    .expect("Failed to store genesis block");

    

    Ok(HttpResponse::Ok().json("New transaction added to mempool"))
} 

pub async fn get_all_transactions()->Result<HttpResponse, actix_web::Error>
{
    let path = "./mempool_db";
    let mut opts = Options::default();
    let db = DB::open(&opts, path).expect("Failed to open database");
    let mut transactions:Vec<Transaction>=Vec::new();
    let iter = db.iterator(IteratorMode::End);
    for item in iter {
        let (key, value) = item.unwrap();
        let string_data = String::from_utf8_lossy(&key);
        let string_val = String::from_utf8_lossy(&value);
        let val = serde_json::from_str::<Transaction>(&string_val);
        match val {
            Ok(value)=>
            {
                transactions.push(value);
            }
            Err(err)=>
            {

            }
            
        };
    }
    let response_json = serde_json::to_string(&transactions).expect("Failed to serialize response");

    // Return the response as JSON
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response_json))

}




pub async fn pick_transaction()->Result<HttpResponse>
{
    let mut res:Result<(),reqwest::Error>= Ok(());
    let path = "./mempool_db";
    let mut opts = Options::default();
    let db = DB::open(&opts, path).expect("Failed to open database");
    let iter = db.iterator(IteratorMode::Start);
    for item in iter {
        let (key, value) = item.unwrap();
        let string_data = String::from_utf8_lossy(&key);
        let string_val = String::from_utf8_lossy(&value);
        let val = serde_json::from_str::<Transaction>(&string_val);
        match val {
            Ok(first) => {
                // Calculate Id
                let mut val=first;
                val.status=Status::PROCESSING;
                res=add_to_blockchain(val).await;
                
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        break;
    }
    match res {
        Ok(()) => {
            Ok(HttpResponse::Ok().json("Transaction Taken up by validator 1"))
        },
        Err(err) => {
            let error_message = format!("Error: {:?}", err);
            let error_response = HttpResponse::InternalServerError().json(error_message);
            Ok(error_response)
        }
    }

   
   
    
}