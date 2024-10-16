use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use base64::{engine::general_purpose, Engine as _};
use num_traits::ToPrimitive;

fn test_millera_rabina(n: &BigUint, k: usize) -> bool{
    let zero = BigUint::from(0 as u16);
    let one = BigUint::from(1 as u16);
    let two = BigUint::from(2 as u16);
    if (n == &two ) || (n == &BigUint::from(3 as u16)){
        return true;
    }
    if (n < &two) || (n % &two == zero){
        return false;
    }
    let mut t: BigUint = n - &one;
    let mut s: i64 = 0;
    while &t % &two == zero{
        t /= &two;
        s += 1;
    }
    let mut rng = rand::thread_rng();
    for _ in 0..k{
        let a = rng.gen_biguint_range(&two, &(n - &two));
        let mut x = a.modpow(&t, n);
        if x == one || x == n - &one{
            continue;
        }
        for _ in 0..s-1{
            x = x.modpow(&two, n);
            if x == one{
                return false;
            }
            if x == n - &one{
                break;
            }
        }
        if x != n - &one{
            return false;
        }
    }
    return true;
}

fn small_primes_check(n: &BigUint) -> bool {
    for &prime in [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29].iter() {
        if n % prime == BigUint::from(0 as u16) {
            return false;
        }
    }
    true
}

fn prime_2048bits() -> BigUint{
    let mut rng = rand::thread_rng();
    let k = 64;

    loop {
        let n = rng.gen_biguint(2048);
        if !small_primes_check(&n) {
            continue;
        }
        if test_millera_rabina(&n, k){
            return n;
        }
    }
}

fn extended_equlid_alg(e: BigUint, eiler_func: BigUint) -> BigInt {
    let mut x1 = 1.to_bigint().unwrap();
    let mut y1 = 0.to_bigint().unwrap();
    let mut x2 = 0.to_bigint().unwrap();
    let mut y2 = 1.to_bigint().unwrap();
    let mut a = e.to_bigint().unwrap();
    let mut b = eiler_func.to_bigint().unwrap();
    while b != 0.to_bigint().unwrap(){
        let q = &a / &b;
        let r = &a % &b;
        let x = x1 - &q * &x2;
        let y = y1 - &q * &y2;
        a = b;
        b = r;
        x1 = x2;
        y1 = y2;
        x2 = x;
        y2 = y;
    }
    if x1 < 0.to_bigint().unwrap(){
        return x1 * -1.to_bigint().unwrap();
    }
    x1
}

fn gen_values() -> Vec<BigUint>{
    let p = prime_2048bits();
    let q = prime_2048bits();
    let n = &p * &q;
    let eiler_func = (&p-1.to_biguint().unwrap())*(&q-1.to_biguint().unwrap());
    let e = 65537.to_biguint().unwrap();
    let d = extended_equlid_alg(e.clone(), eiler_func.clone()).to_biguint().unwrap();
    Vec::from([e,n,d])
}

fn get_char_code(char: char, digit: BigUint) -> Option<BigUint> {
    let char_lower = char.to_lowercase().next()?;

    // Русские буквы
    if char_lower >= 'а' && char_lower <= 'я' {
        let base = 'а' as u32;
        return Some(BigUint::from(char_lower as u32 - base + 1));
    }

    // Пробел
    if char == ' ' {
        return Some(BigUint::from(34u32)); // Изменено на 34
    }

    // Цифры
    if char.is_digit(10) {
        return Some(BigUint::from(35u32) + digit); // Изменено на 35
    }

    None
}

fn get_string_codes(text: &str) -> Vec<BigUint> {
    let mut codes = Vec::new();
    let mut digit_counter = BigUint::from(0u32); // Счетчик для цифр - теперь BigUint

    for char in text.chars() {
        if let Some(code) = get_char_code(char, digit_counter.clone()) { // clone() для digit_counter
            codes.push(code);
            if char.is_digit(10) {
                digit_counter += BigUint::from(1u32); // Увеличиваем счетчик
            }
        }
    }

    codes
}

fn get_char_from_code(code: BigUint) -> Option<char> {
    // Русские буквы
    if code >= BigUint::from(1u32) && code <= BigUint::from(33u32) {
        let base = 'а' as u32;
        let char_code = (code - BigUint::from(1u32)).to_u32()?;
        return char::from_u32(base + char_code);
    }

    // Пробел
    if code == BigUint::from(34u32) {
        return Some(' ');
    }

    // Цифры
    if code >= BigUint::from(35u32) { // Изменено на >=
        let digit = (code - BigUint::from(35u32)).to_u32()?; // Изменено на 35
        return std::char::from_digit(digit, 10);
    }

    None
}


fn get_string_from_codes(codes: Vec<BigUint>) -> String {
    let mut result = String::new();

    for code in codes {
        if let Some(char) = get_char_from_code(code) {
            result.push(char);
        }
    }

    result
}


fn encrypt(digits: &Vec<BigUint>, e: &BigUint, n: &BigUint) -> Vec<BigUint>{
    let mut cryptochars =   Vec::new();
    for d in digits{
        let cryptochar = d.to_biguint().unwrap().modpow(e, n);
        cryptochars.push(cryptochar);
    }
    cryptochars

}

fn decrypt(cryptochars: &Vec<BigUint>, d: &BigUint, n: &BigUint) -> Vec<BigUint> {
    let mut digits = Vec::new();
    for c in cryptochars {
        let digit = c.modpow(d, n);
        digits.push(digit);
    }
    digits
}

fn encode_biguint_vector(cryptotext: &Vec<BigUint>) -> String {
    let strings: Vec<String> = cryptotext
        .iter()
        .map(|biguint| biguint.to_string())
        .collect();

    let combined_string = strings.join(","); // Объединяем строки с разделителем

    general_purpose::STANDARD.encode(combined_string.as_bytes()) // Кодируем объединенную строку
}

fn decode_biguint_vector(base64_string: &str) -> Vec<BigUint> {
    let decoded_bytes = match general_purpose::STANDARD.decode(base64_string) {
        Ok(bytes) => bytes,
        Err(_) => return Vec::new(),
    };

    let decoded_string = match String::from_utf8(decoded_bytes) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    decoded_string
        .split(',')
        .filter_map(|s| s.parse::<BigUint>().ok()) // filter_map для пропуска ошибок парсинга
        .collect()
}


fn main(){
    let init_vec = gen_values();
    let message = String::from("Я люблю котов");
    let cryptotext = encode_biguint_vector(&encrypt(&get_string_codes(&message),&init_vec[0],&init_vec[1]));
    println!("----------------------------------------");
    println!("---- Begin ciphertext ----");
    println!("----------------------------------------");
    println!("{}",cryptotext);
    println!("----------------------------------------");
    println!("---- End ciphertext ----");
    println!("----------------------------------------");
    let my_message = get_string_from_codes(decrypt(&decode_biguint_vector(&cryptotext),&init_vec[2],&init_vec[1]));
    println!("{}", my_message); 
}




