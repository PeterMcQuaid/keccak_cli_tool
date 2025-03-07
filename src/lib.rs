use std::fs;
use primitive_types::H256;

pub mod hash;

/// Calculates and returns the Keccak256 hash of a vector of strings. If the string 
/// is a file, it reads the file and calculates the hash of the file (including newline chars).
/// 
/// # Examples
/// 
/// ```
/// use keccak_cli_tool::hash_logic;
/// use primitive_types::H256;
/// use hex_literal::hex;
/// 
/// let args = vec!["hello".to_string(), "test.txt".to_string()];   // test.txt contains "world"
/// let output = hash_logic(args);
/// assert_eq!(output.len(), 2);
/// assert_eq!(output[0], H256(hex!("1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8")));
/// assert_eq!(output[1], H256(hex!("1d63660020a5b5062fb35d9f82afa81581442281c43343763ab1d340e9861bae")));
/// ```
pub fn hash_logic(args: Vec<String>) -> Vec<H256> {
    let mut digests: Vec<Vec<u8>> = Vec::new();

    for arg in &args {
        match fs::read(arg) {
            Ok(file) => {
                digests.push(file);
            }
            _ => {
                digests.push(arg.bytes().collect());
            }
        }
    }

    let mut hashes: Vec<H256> = Vec::new();

    for digest in &mut digests {
        hashes.push(hash::return_hash(digest));
    }

    println!("{:?}", hashes);

    hashes
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    // Tests the simple keccak hash output for 3 strings
    #[test]
    fn test_hashed_output() {
        let args = vec![String::from("hello"), String::from("world"), String::from("test")];
        let output = hash_logic(args);
        assert_eq!(output.len(), 3);
        assert_eq!(output[0], H256(hex!("1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8")));
        assert_eq!(output[1], H256(hex!("8452c9b9140222b08593a26daa782707297be9f7b3e8281d7b4974769f19afd0")));
        assert_eq!(output[2], H256(hex!("9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb658")));
    }
}