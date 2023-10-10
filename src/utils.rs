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
   pub data:String,
   pub timestamp:String
}