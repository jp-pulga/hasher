use clap::{App, Arg};
use digest::Digest;
use ripemd160;
use ripemd320;
use sha3;
use std::io::*;

fn hash_and_print<D: Digest>(to_hash: String) {
    let mut hasher = D::new();
    hasher.input(to_hash.as_bytes());

    for byte in hasher.result() {
        print!("{:02x}", byte);
    }
}

fn match_hash(hash : &str, to_hash: String) {
    match hash {
        // Sha3
        "sha3_224" => hash_and_print::<sha3::Sha3_224>(to_hash),
        "sha3_256" => hash_and_print::<sha3::Sha3_256>(to_hash),
        "sha3_384" => hash_and_print::<sha3::Sha3_384>(to_hash),

        //ripemd
        "ripemd160" => hash_and_print::<ripemd160::Ripemd160>(to_hash),
        "ripemd320" => hash_and_print::<ripemd320::Ripemd320>(to_hash),
        _ => hash_and_print::<sha3::Sha3_512>(to_hash),
    }
}

fn main() {
    let matches = App::new("Hasher - Simple hash tool")
        .version("0.0.1")
        .author("João Paulo Pulga <pulgovisk@protonmail.com>")
        .about("Hash tool made with rust")
        .arg(
            Arg::with_name("hash")
                .short("h")
                .long("hash")
                .value_name("Hash type")
                .long_help(
                    "The supported list of hashes is
Sha3:
    sha3_224
    sha3_256
    sha3_384
    sha3_512
    
ripemd:
    ripemd160
    ripemd320",
                )
                .takes_value(true),
        )
        .arg(Arg::with_name("hide").long("hide"))
        .get_matches();

    let hash = matches.value_of("hash").unwrap_or("sha3_512");

    if matches.is_present("hide") {
        use rpassword;
        
        match_hash(hash, rpassword::read_password().unwrap());
    } else {
        let stdin = stdin();
        for line in stdin.lock().lines() {
            match_hash(hash, line.unwrap());
        }
    }

    println!("");
}
