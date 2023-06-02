use std::{fs::File, io::BufReader, io::BufRead, error::Error, io::Write};
use crate::hash_types::HashType;

// Crypto imports
use sha2::Digest;
use sha2::{Sha224, Sha256, Sha384, Sha512};

pub struct HashTable {
    wordlist: String,
    hash: String,
    hash_type: HashType,
    salt: Option<String>,

}
impl HashTable {
    pub fn new(wordlist: String, hash: String , salt: Option<String>) -> HashTable {
        HashTable {
            wordlist: wordlist,
            hash_type: HashType::from(hash.len() * 4), // u8 bytes to bits
            hash: hash,
            salt: salt
        }
    }

    pub fn get_hash(&self) -> &HashType {
        &self.hash_type
    }

    pub fn bruteforce(&self) -> Result<String, Box<dyn Error>> {
        let file = File::open(&self.wordlist)?;
        let mut reader = BufReader::new(file);
        let mut result: String = String::from("No match found");

        match self.hash_type {
            HashType::MD5 => self.md5(&mut reader),
            HashType::SHA1 => self.sha1(&mut reader),
            HashType::SHA224 => self.sha224(&mut reader),
            HashType::SHA256 => result = self.sha256(&mut reader).unwrap(),
            HashType::SHA384 => self.sha384(&mut reader),
            HashType::SHA512 => self.sha512(&mut reader),
        }

        Ok(result)
    }

    fn md5<R: BufRead>(&self, reader: &mut R) {
        todo!("Add MD5 support")
    }
    fn sha1<R: BufRead>(&self, reader: &mut R)  {
        todo!("Add SHA1 support")
    }
    fn sha224<R: BufRead>(&self, reader: &mut R) {
    }
    fn sha256<R: BufRead>(&self, reader: &mut R) -> Result<String, Box<dyn Error>> {
        let mut tmp_pass: &[u8];
        let mut tmp_hash: String;
        let mut counter: u64 = 0;
        let mut found: String = String::new();

        for line in reader.lines() {
            let line = line.unwrap_or_default();
            tmp_pass = line.as_bytes();
            tmp_hash = format!("{:x}", Sha256::digest(&tmp_pass));

            if counter % 50_000 == 0 {
                print!("\rAttempts: [{}] - {}" , counter, line);
                std::io::stdout().flush()?;
            }

            if self.hash == tmp_hash {
                found = line;
                println!("\nFound!");
                break;
            }

            counter += 1;
        }

        Ok(found)
    }
    fn sha384<R: BufRead>(&self, reader: &mut R) {
    }
    fn sha512<R: BufRead>(&self, reader: &mut R) {
    }


}