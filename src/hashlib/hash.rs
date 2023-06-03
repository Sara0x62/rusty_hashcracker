// Crypto crates
use md5;
use sha1::Sha1;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};

// Local crate
use crate::{errors::HashError, hash_types::HashType, util::*};

// std imports
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

// Return alias
pub type Result<T> = std::result::Result<T, HashError>;

// Main struct
pub struct HashTable {
    pub wordlist_file: String,
    pub hash_input: String,
    pub hash_type: HashType,
    pub salt: Option<String>,
}


impl HashTable {
    pub fn new(filename: String, input: String, salt: Option<String>) -> HashTable {
        // Get hash type, exit if given hash is not valid
        // Bytes to bits -> x4
        let hash_type = match HashType::try_from(input.len() * 4) {
            Ok(x) => x,
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        };

        HashTable {
            wordlist_file: filename,
            hash_input: input,
            hash_type: hash_type,
            salt: salt,
        }
    }

    pub fn bruteforce(&self) -> Result<String> {
        // Open file; exit if error occurs
        let file = match File::open(&self.wordlist_file) {
            Ok(file) => file,
            Err(err) => {
                println!("Error: {}", err);
                std::process::exit(1);
            }
        };

        let mut reader = BufReader::new(file);
        let result;

        match self.hash_type {
            HashType::MD5 =>    result = self.md5(&mut reader),
            HashType::SHA1 =>   result = self.sha::<_, Sha1>(&mut reader),
            HashType::SHA224 => result = self.sha::<_, Sha224>(&mut reader),
            HashType::SHA256 => result = self.sha::<_, Sha256>(&mut reader),
            HashType::SHA384 => result = self.sha::<_, Sha384>(&mut reader),
            HashType::SHA512 => result = self.sha::<_, Sha512>(&mut reader),
        }

        // Return the matching text for the hash or Err(NoMatchFound)
        result
    }

    // SHA1/2 versions
    fn sha<R: BufRead, Hasher: Digest>(&self, reader: &mut R) -> Result<String>
    where
        digest::Output<Hasher>: core::fmt::LowerHex,
    {
        let mut tmp_hash: String;
        let mut counter: u64 = 0;

        for line in reader.lines() {
            let mut line = line.unwrap_or_default();

            // Salt support right now is simply Hash(text + salt).
            line.push_str(self.salt.as_ref().unwrap_or(&EMPTY.to_string()).as_str());
            tmp_hash = format!("{:x}", Hasher::digest(line.as_bytes()));

            // Compare hashes
            if self.hash_input == tmp_hash {
                println!("\n[!] Found after {} attempts", counter);
                return Ok(line);
            }

            // Pretty print :3
            if counter % 50_000 == 0 {
                print!("{}\r[*] Attempts [{}] - {}", CLEAR_LINE, counter, line);
                _ = std::io::stdout().flush();
            }

            counter += 1;
        }

        Err(HashError::NoMatchFound(self.hash_input.clone()))
    }

    // TODO: reduce duplicate code?
    // MD5 version
    fn md5<R: BufRead>(&self, reader: &mut R) -> Result<String> {
        let mut tmp_hash: String;
        let mut counter: u64 = 0;

        for line in reader.lines() {
            let mut line = line.unwrap_or_default();

            // Salt support right now is simply Hash(text + salt).
            line.push_str(self.salt.as_ref().unwrap_or(&EMPTY.to_string()).as_str());
            tmp_hash = format!("{:x}", md5::compute(line.as_bytes()));

            if self.hash_input == tmp_hash {
                println!("\n[!] Found after {} attempts", counter);
                return Ok(line);
            }

            if counter % 50_000 == 0 {
                print!("{}\r[*] Attempts [{}] - {}", CLEAR_LINE, counter, line);
                _ = std::io::stdout().flush();
            }

            counter += 1;
        }

        Err(HashError::NoMatchFound(self.hash_input.clone()))
    }
}
