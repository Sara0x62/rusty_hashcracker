use std::error::Error;

mod hash_types;
mod hash_table;

use crate::hash_table::*;


fn main() -> Result<(), Box<dyn Error>> {

    // TODO: Get rid of hardcoded here
    let wordlist = String::from("wordlist.txt");

    // Hardcoded hashes for testings
    //let input = String::from("3a6d64c24cf80b69ccda37650406467e8266667b50cfd0b984beb3651b129ed7"); // sara
    let input = String::from("01f92811cee6fd881f2f0eb71b40c07205327d2dfd9b93f3b8cf616ea2138706"); // not sara

    let table = HashTable::new(wordlist, input, None);

    println!("Detected hash is of type: {:?}", &table.get_hash());
    println!("Attempting to find matching hash...");

    let result = table.bruteforce().unwrap_or("Password Not found".to_string());

    println!("Found password: {:?}", &result);

    Ok(())
}