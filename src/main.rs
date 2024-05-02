use pz::pz;

fn main() {
    let key = pz::generate_key();
    pz::encrypt(key, "helloworld.lol".to_string());
    pz::decrypt("helloworld.lol.pz".to_string());
}
