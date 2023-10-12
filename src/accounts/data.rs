use actix_web::{HttpResponse,Result, web};
use chrono::Local;
use permutohedron::LexicalPermutation;
use rocksdb::{Options, IteratorMode, DB};

use crate::utils::{AccountInfo, Account};


pub async fn add_account(accountinfo:web::Json<AccountInfo>)->Result<HttpResponse>
{
    let mut account_id="".to_string();

    let path = "./account_db";
    let mut opts = Options::default();
    opts.create_if_missing(true);

    let db = DB::open(&opts, path).expect("Failed to open database");
    let iter = db.iterator(IteratorMode::End);
    for item in iter {
        let (key, value) = item.unwrap();
        let string_data = String::from_utf8_lossy(&key);
        let string_val = String::from_utf8_lossy(&value);
        let val = serde_json::from_str::<Account>(&string_val);
        match val {
            Ok(last) => {
                // Calculate Id
                let mut chars: Vec<char> = last.id.chars().collect();
                chars.next_permutation();
                account_id=chars.iter().collect();
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        break;
    }

    
    if account_id.len()==0{

    account_id="asjfkasxbgfuwugejavxbvnxzvnbxzbzqowyebs".to_string();

    }


    let account=Account{
        id:account_id,
        name:accountinfo.0.name,
        balance:accountinfo.0.balance,
        creation_timestamp:Local::now().to_string(),

    };

    let account_serialized =
        serde_json::to_string(&account).expect("Failed to serialize");
   
    db.put(
        &account.id.to_string().as_bytes(),
        &account_serialized
    )
    .expect("Failed to add Account");

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(account_serialized))
}

pub async fn get_all_accounts()->Result<HttpResponse>
{
    let mut accounts_list:Vec<Account>=Vec::new();
    let path = "./account_db";
    let mut opts = Options::default();
    opts.create_if_missing(true);

    let db = DB::open(&opts, path).expect("Failed to open database");
    let iter = db.iterator(IteratorMode::Start);
    for item in iter {
        let (key, value) = item.unwrap();
        let string_data = String::from_utf8_lossy(&key);
        let string_val = String::from_utf8_lossy(&value);
        let val = serde_json::from_str::<Account>(&string_val);
        match val
        {
            Ok(value)=>
            {
                accounts_list.push(value);
            }
            Err(err)=>
            {
                println!("{:?}",err);
            }
        }
        
    }
    let response_json = serde_json::to_string(&accounts_list).expect("Failed to serialize response");

    // Return the response as JSON
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response_json))
}