use rocksdb::{Options, DB};

use crate::utils::{Transaction, Account};

pub async fn validate_transaction(transaction:&Transaction)->bool
{
    let path = "./account_db";
    let mut opts = Options::default();
    let db = DB::open(&opts, path).expect("Failed to open database");

    opts.create_if_missing(true);
    let sender_id=transaction.from.to_string();
    let reciever_id=transaction.to.to_string();
    let mut sender_account=Account{
        id:"".to_string(),
        name:"".to_string(),
        balance:0,
        creation_timestamp:"".to_string()
    };
    let mut reciever_account=Account{
        id:"".to_string(),
        name:"".to_string(),
        balance:0,
        creation_timestamp:"".to_string()
    };

     // Use the `get` method to retrieve data for the specified ID
     match db.get(sender_id.as_bytes()) {
        Ok(Some(value)) => {
            // Data for the ID exists
            let data = String::from_utf8_lossy(&value);
            let val = serde_json::from_str::<Account>(&data);
            match val {
                Ok(account)=>
                {
                    sender_account=account;
                }
                Err(err)=>
                {
                    println!("{:?}",err);
                }
                
            }
        }
        Ok(None) => {
            // Data not found for the specified ID
            println!("Data not found for ID: {}", sender_id);
            return false;
        }
        Err(err) => {
            // Handle any potential errors
            eprintln!("Error while retrieving data: {}", err);
        }
    }
    match db.get(reciever_id.as_bytes()) {
        Ok(Some(value)) => {
            // Data for the ID exists
            let data = String::from_utf8_lossy(&value);
            let val = serde_json::from_str::<Account>(&data);
            match val {
                Ok(account)=>
                {
                    reciever_account=account;
                }
                Err(err)=>
                {
                    println!("{:?}",err);
                }
                
            }
        }
        Ok(None) => {
            // Data not found for the specified ID
            println!("Data not found for ID: {}", reciever_id);
            return false;
        }
        Err(err) => {
            // Handle any potential errors
            eprintln!("Error while retrieving data: {}", err);
        }
    }
    if sender_account.balance>=transaction.amount
    {
        true
    }
    else {
        false
    }



}