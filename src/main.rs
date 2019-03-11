use sha3::Digest;
use std::io;

mod sha3_utils;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let mut hasher = sha3_utils::sha_512();

            // write input message
            hasher.input(&input.as_bytes()[0..input.chars().count() - 1]);

            for byte in hasher.result() {
                print!("{:02x}", byte);
            }

            println!("");
        }
        Err(err) => println!("error: {}", err),
    }
}
