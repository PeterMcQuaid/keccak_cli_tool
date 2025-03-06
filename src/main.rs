use std::{env, fs};

pub mod hash;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut digests = Vec::new();

    for arg in &args[1..] {
        match fs::read(arg) {
            Ok(file) => {
                digests.push(file);
            }
            _ => {
                digests.push(arg.bytes().collect());
            }
        }
    }

    for digest in &mut digests {
        println!("{:?}", digest);
        let hash_result = hash::return_hash(digest);
        println!("{:?}", hash_result);
    }
}
