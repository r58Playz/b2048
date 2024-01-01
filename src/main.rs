use base2048::{decode, encode};
use std::env::args;
use std::io::{stdin, Read};

fn main() {
    match args()
        .nth(1)
        .expect("need either encode or decode")
        .as_str()
    {
        "encode" => {
            let mut input = Vec::new();
            stdin()
                .read_to_end(&mut input)
                .expect("failed to read stdin");
            print!("{}", encode(&input));
        }
        "decode" => {
            let mut input = String::new();
            stdin()
                .read_to_string(&mut input)
                .expect("failed to read stdin");
            print!(
                "{}",
                std::str::from_utf8(&decode(&input).expect("failed to decode")).unwrap()
            );
        }
        _ => {
            println!("invalid args")
        }
    }
}