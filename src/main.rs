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
            unsafe {
                print!(
                    "{}",
                    std::str::from_utf8_unchecked(&decode(&input).expect("failed to decode"))
                )
            };
        }
        "-h" | "--help" | "help" => {
            println!("usage: b2048 encode|decode");
            println!("       pipe the input into stdin.");
        }
        _ => {
            println!("invalid args")
        }
    }
}
