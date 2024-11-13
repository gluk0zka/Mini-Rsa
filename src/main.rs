use num_bigint::{ BigUint, RandBigInt, ToBigUint};
use base64::{engine::general_purpose, Engine as _};
use std::io::stdin;
mod math_algs;

fn prime_2048bits() -> BigUint{
    let mut rng = rand::thread_rng();
    let k = 64;

    loop {
        let n = rng.gen_biguint(2048);
        if !math_algs::small_primes_check(&n) {
            continue;
        }
        if math_algs::test_millera_rabina(&n, k){
            return n;
        }
    }
}

fn gen_values() -> Vec<BigUint>{
    loop {
        let p = prime_2048bits();
        let q = prime_2048bits();
        let n = &p * &q;
        let eiler_func = (&p-1.to_biguint().unwrap())*(&q-1.to_biguint().unwrap());
        let e = 65537.to_biguint().unwrap();
        if &eiler_func / &e == 0.to_biguint().unwrap(){
            continue;
        }
        let d = math_algs::extended_equlid_alg(&e.clone(), &eiler_func.clone()).to_biguint().unwrap();
        return  Vec::from([n,e,d]);
    }
}
fn encrypt_string_rsa(message: &str, n: &BigUint, e: &BigUint) -> Result<String, String> {
    let message_bytes = message.as_bytes();
    let max_block_size = (n.bits() / 8) + 1;
    let mut encrypted_data = Vec::new();

    let mut i = 0;
    while i < message_bytes.len() {
        let remaining = message_bytes.len() - i;
        let block_size = std::cmp::min(max_block_size, remaining as u64);
        let block = &message_bytes[i..i + (block_size as usize)];
        let block_int = BigUint::from_bytes_be(block);
        let encrypted_block = block_int.modpow(e, n);
        encrypted_data.extend_from_slice(&encrypted_block.to_bytes_be());
        i += block_size as usize;
    }
    Ok(general_purpose::STANDARD.encode(encrypted_data))
}

fn decrypt_string_rsa(encrypted_message: &str, n: &BigUint, d: &BigUint) -> Result<String, String> {
    let encrypted_data = general_purpose::STANDARD
        .decode(encrypted_message)
        .map_err(|e| format!("Base64 decode error: {}", e))?;
    let max_block_size = (n.bits() / 8) + 1;
    let mut decrypted_data = Vec::new();
    let mut i = 0;
    while i < encrypted_data.len() {
        let block_end = std::cmp::min(i + max_block_size as usize, encrypted_data.len());
        let block = &encrypted_data[i..block_end];
        let block_int = BigUint::from_bytes_be(block);
        let decrypted_block = block_int.modpow(d, n);
        decrypted_data.extend_from_slice(&decrypted_block.to_bytes_be()); 
        i = block_end;
    }

    String::from_utf8(decrypted_data).map_err(|e| format!("UTF-8 conversion error: {}", e))
}

fn main(){
    let init_vec = gen_values();
    let mut message =  String::new();
    println!("Введите сообщение: ");
    stdin().read_line(&mut message).expect("Ошибка ввода");
    let cryptotext = encrypt_string_rsa(&message, &init_vec[0], &init_vec[1]).unwrap();
    println!("----------------------------------------");
    println!("---- Begin ciphertext ----");
    println!("----------------------------------------");
    println!("{}",cryptotext);
    println!("----------------------------------------");
    println!("---- End ciphertext ----");
    println!("----------------------------------------");
    let my_message = decrypt_string_rsa(&cryptotext, &init_vec[0], &init_vec[2]).unwrap();
    println!("{}", my_message); 
}