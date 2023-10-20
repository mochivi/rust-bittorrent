pub mod bdecoder;
use std::env;

use crate::bdecoder::bdecoder::decode;


// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_string: &str = &args[2].clone();
        let dedoced_values = decode(&encoded_string);
        println!("{:?}", dedoced_values);
    } else {
        println!("unknown command: {}", args[1])
    }
}
