use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, KeyInit};
use std::fs;
use std::io::Write;

fn generate_key() -> [u8; 32] {
    Aes256Gcm::generate_key(OsRng).into()
}

pub fn encrypt(filename: String) {
    let key = generate_key();
    let mut content = fs::read_to_string(&filename).expect("Error while reading the file");
    // println!("content: {:#?}", content);
    let cipher = Aes256Gcm::new(&key.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, content.as_ref())
        .expect("Couldnt Encrypt the File");
    content.clear();

    // println!("nonce: {:#?}", nonce);
    // println!("ciphertext: {:#?}", ciphertext);

    let mut outputfile = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename + ".pz")
        .expect("Couldnt Create A file");

    outputfile
        .write_all(&nonce)
        .expect("cannot write nonce to the provided file");

    outputfile
        .write_all(&key)
        .expect("Couldnt write key to the output file ");

    outputfile
        .write_all(&ciphertext)
        .expect("Couldnt Create A file");
}
