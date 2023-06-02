use sha2::{Digest, Sha256, Sha224, Sha384, Sha512};

use crate::{hash_types::HashType, errors::HashError, util::CLEAR_LINE};
use std::{fs::File, io::{BufReader, BufRead, Write}};

// Return alias
pub type Result<T> = std::result::Result<T, HashError>;

// Main struct
pub struct HashTable {
    pub wordlist_file: String,
    pub hash_input: String,
    pub hash_type: HashType,
    pub salt: Option<String>,   // TODO: Implement salt
}

impl HashTable {
    pub fn new(filename: String, input: String, salt: Option<String>) -> HashTable {
        let hash_type = match HashType::try_from(input.len() * 4) { // Bytes to bits -> x4
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
            HashType::MD5 => result = self.md5(&mut reader),
            HashType::SHA1 => result = self.sha1(&mut reader),
            HashType::SHA224 => result = self.sha224(&mut reader),
            HashType::SHA256 => result = self.sha256(&mut reader),
            HashType::SHA384 => result = self.sha384(&mut reader),
            HashType::SHA512 => result = self.sha512(&mut reader),
        }

        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(HashError::NoMatchFound(self.hash_input.clone()))
        }
    }

    // SHA2 versions
    fn sha224<R: BufRead>(&self, reader: &mut R) -> Result<String> {
        let mut tmp_hash: String;
        let mut counter: u64 = 0;

        for line in reader.lines() {
            let line = line.unwrap_or_default();
            tmp_hash = format!("{:x}", Sha224::digest(line.as_bytes()));

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

    fn sha256<R: BufRead>(&self, reader: &mut R) -> Result<String> {
        let mut tmp_hash: String;
        let mut counter: u64 = 0;

        for line in reader.lines() {
            let line = line.unwrap_or_default();
            tmp_hash = format!("{:x}", Sha256::digest(line.as_bytes()));

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
    
    fn sha384<R: BufRead>(&self, reader: &mut R) -> Result<String> {
        let mut tmp_hash: String;
        let mut counter: u64 = 0;

        for line in reader.lines() {
            let line = line.unwrap_or_default();
            tmp_hash = format!("{:x}", Sha384::digest(line.as_bytes()));

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
    
    fn sha512<R: BufRead>(&self, reader: &mut R) -> Result<String> {
        let mut tmp_hash: String;
        let mut counter: u64 = 0;

        for line in reader.lines() {
            let line = line.unwrap_or_default();
            tmp_hash = format!("{:x}", Sha512::digest(line.as_bytes()));

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


    // SHA1 version
    fn sha1<R: BufRead>(&self, _reader: &mut R) -> Result<String> {
        todo!();
    }

    // MD5 version
    fn md5<R: BufRead>(&self, _reader: &mut R) -> Result<String> {
        todo!();
    }

}