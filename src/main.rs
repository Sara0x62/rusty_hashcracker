use hashlib::hash::HashTable;
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

    println!("[*] Detected hash type: {:?}", table.hash_type);

    println!("[*] Attempting to bruteforce...");
    println!("[*]  - Provided wordlist: {:?}", table.wordlist_file);

    let result = table.bruteforce();

    println!("\n[*] Result: {:?}", result.unwrap());
}

/*
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
*/