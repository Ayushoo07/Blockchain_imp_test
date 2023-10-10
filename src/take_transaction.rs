
use actix_web::{web, HttpResponse, Result, HttpRequest};

use crate::{utils::REMOTE_TRANSACTIONS, Add_to_blockchain::add_to_blockchain};



pub async fn take_transaction()->Result<HttpResponse>
{
    let mut rm=REMOTE_TRANSACTIONS.lock();
    rm.push("Ayush");
    rm.push("Shivam");
    let data=rm[0];
    println!("{:?}",rm);
    drop(rm);
    let res=add_to_blockchain(data.to_string()).await;
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