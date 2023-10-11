use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::{Serialize, Deserialize};


pub static THIS_IP : &'static str= "172.16.14.97:8080";
pub static mut REMOTE_ADDRESS : Vec<&str>= Vec::new();



lazy_static! {
      pub static ref REMOTE_TRANSACTIONS: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block
{
   pub id:String,
   pub prev_hash:String,
   pub curr_hash:String,
   pub data:Transaction,
   pub timestamp:String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    PENDING,
    PROCESSING,
    ACCEPTED,
    REJECTED
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction
{
   pub id:String,
   pub gas_fees:i64,
   pub to:String,
   pub from:String,
   pub amount:i64,
   pub timestamp:String,
   pub status:Status
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo
{
   pub gas_fees:i64,
   pub to:String,
   pub from:String,
   pub amount:i64,
}