use aes_gcm::aead::{generic_array::GenericArray, Aead};
use aes_gcm::{Aes256Gcm, KeyInit};
use std::fs;
use std::io::{Read, Write};

pub fn decrypt(filename: String) {
    let mut file = fs::File::open(&filename).expect("Error while reading the file");
    let mut nonce: [u8; 12] = [0; 12];
    file.read_exact(&mut nonce)
        .expect("couldnt read nonce from the encrypted file");
    let mut key: [u8; 32] = [0; 32];

    file.read_exact(&mut key)
        .expect("couldnt read key from the encrypted file");

    let mut content = Vec::new();
    file.read_to_end(&mut content)
        .expect("couldnt read the ciphered text from the file");

    let cipher = Aes256Gcm::new(&key.into());
    // println!("nonce: {:#?}\nciphertext: {:#?}", nonce, content);
    let plaintext = cipher
        .decrypt(&GenericArray::from_slice(&nonce), content.as_ref())
        .expect("Cannot Decrypt");

    fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename.replace(".pz", ""))
        .expect("Couldnt Create A file")
        .write_all(plaintext.as_ref())
        .expect("Couldnt Write to file");

    // println!(
    //     "{:?}",
    //     String::from_utf8(plaintext).expect("Error in the encoding of decrypted file")
    // )
}



