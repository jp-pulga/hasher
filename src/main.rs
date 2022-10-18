use clap::{value_t, App, Arg};
use digest::Digest;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
enum HashType {
	Sha2_224,
	Sha2_256,
	Sha2_384,
	Sha2_512,
	Sha3_224,
	Sha3_256,
	Sha3_384,
	Sha3_512,
	Whirlpool,
	Ripemd160,
	Ripemd320,
	Md2,
	Md4,
	Md5
}

impl FromStr for HashType {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"sha2_224" => Ok(HashType::Sha2_224),
			"sha2_256" => Ok(HashType::Sha2_256),
			"sha2_384" => Ok(HashType::Sha2_384),
			"sha2_512" => Ok(HashType::Sha2_512),
			"sha3_224" => Ok(HashType::Sha3_224),
			"sha3_256" => Ok(HashType::Sha3_256),
			"sha3_384" => Ok(HashType::Sha3_384),
			"sha3_512" => Ok(HashType::Sha3_512),
			"whirlpool" => Ok(HashType::Whirlpool),
			"ripemd160" => Ok(HashType::Ripemd160),
			"ripemd320" => Ok(HashType::Ripemd320),
			"md2" => Ok(HashType::Md2),
			"md4" => Ok(HashType::Md4),
			"md5" => Ok(HashType::Md5),
			_ => Err("No hash matched"),
		}
	}
}

fn hash_and_print<D: Digest>(to_hash: String) {
	for byte in D::digest(to_hash.as_bytes()) {
		print!("{:02x}", byte);
	}
}

fn match_hash_and_print(hash: &HashType, to_hash: String) {
	match hash {
		// Sha2
		HashType::Sha2_224 => hash_and_print::<sha2::Sha224>(to_hash),
		HashType::Sha2_256 => hash_and_print::<sha2::Sha256>(to_hash),
		HashType::Sha2_384 => hash_and_print::<sha2::Sha384>(to_hash),
		HashType::Sha2_512 => hash_and_print::<sha2::Sha512>(to_hash),

		// Sha3
		HashType::Sha3_224 => hash_and_print::<sha3::Sha3_224>(to_hash),
		HashType::Sha3_256 => hash_and_print::<sha3::Sha3_256>(to_hash),
		HashType::Sha3_384 => hash_and_print::<sha3::Sha3_384>(to_hash),
		HashType::Sha3_512 => hash_and_print::<sha3::Sha3_512>(to_hash),

		//whirlpool
		HashType::Whirlpool => hash_and_print::<whirlpool::Whirlpool>(to_hash),

		//ripemd
		HashType::Ripemd160 => hash_and_print::<ripemd::Ripemd160>(to_hash),
		HashType::Ripemd320 => hash_and_print::<ripemd::Ripemd320>(to_hash),

		//md
		HashType::Md2 => hash_and_print::<md2::Md2>(to_hash),
		HashType::Md4 => hash_and_print::<md4::Md4>(to_hash),
		HashType::Md5 => hash_and_print::<md5::Md5>(to_hash),
	}
}

fn main() {
    let matches = App::new("Hasher - Simple hash tool")
        .version("0.0.1")
        .author("João Paulo Pulga <pulgovisk@protonmail.com>")
        .about("Hash tool made with rust")
        .arg(
            Arg::with_name("algorithm")
                .short("a")
                .long("algorithm")
                .value_name("type")
				.takes_value(true)
				.default_value("md5")
                .possible_values(&[
                    "sha2_224",
                    "sha2_256",
                    "sha2_384",
                    "sha2_512",
                    "sha3_224",
                    "sha3_256",
                    "sha3_384",
                    "sha3_512",
                    "md2",
                    "md4",
                    "md5",
                    "ripemd160",
                    "ripemd320",
                    "whirlpool",
                ]),
        )
        .get_matches();

    let hash = value_t!(matches, "algorithm", HashType).unwrap_or(HashType::Md5);    

	use std::io::BufRead;

	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		match_hash_and_print(&hash, line.unwrap());
	}

    println!("");
}
