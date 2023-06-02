use hashlib::hash::HashTable;
use hashlib::util::*;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg()]
    hash: String,

    #[arg(short, long, default_value = "wordlist.txt" )]
    wordlist: String,

    #[arg(short, long, default_value = None)]
    salt: Option<String>,
}

fn main() {
    let args = Args::parse();

    let table = HashTable::new(args.wordlist, args.hash, args.salt);

    println!("[*] Detected hash type: {}{:?}{}", BOLD_ON, table.hash_type, ATTRIBUES_OFF);

    println!("[*] Attempting to bruteforce...");
    println!("[*]  - Provided wordlist: {:?}", table.wordlist_file);

    let result = table.bruteforce();

    match result {
        Ok(result) => println!("\n[*] Result: {:?}", result),
        Err(err) => println!("\n\n{:?}", err.to_string()),
    }
}