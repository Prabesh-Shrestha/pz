use crypto::{decrypt::decrypt, encrypt::encrypt};

pub mod crypto;

pub fn cli() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 3 {
        if args[1].contains("en") {
            encrypt(args[2].clone());
            println!("Encrypted {} successfully", args[2]);
        } else if args[1].contains("de") {
            decrypt(args[2].clone());
            println!("Decrypted {} successfully", args[2]);
        } else {
            println!(
                "Invalid Options, either use \n-en (--encrypt) <file>\n-de (--decrypt) <file>"
            );
        }
    } else {
        println!("Invalid Arguments");

        println!("Help: \nuse \n-en (--encrypt) <file>\n-de (--decrypt) <file>")
    }
}
