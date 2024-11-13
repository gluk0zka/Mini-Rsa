use num_bigint::{ BigInt, BigUint, RandBigInt, ToBigInt};
use num_traits::identities::{Zero,One};
use num_traits::Signed;

fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (g, x, y) = extended_gcd(b, &(a % b));
        let new_y = x - ((a / b).clone() * y.clone());
        (g, y, new_y)
    }
}

pub fn extended_equlid_alg(e: &BigUint, eiler_func: &BigUint) -> BigUint {
    let (_g, x, _) = extended_gcd(&e.to_bigint().unwrap(), &eiler_func.to_bigint().unwrap());
    let positive_x = if x.is_negative() {
        (&x + &eiler_func.to_bigint().unwrap().clone()).to_biguint().unwrap()
    } else {
        x.to_biguint().unwrap()
    };
    positive_x
}


pub fn small_primes_check(n: &BigUint) -> bool {
    for &prime in [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29].iter() {
        if n % prime == BigUint::from(0 as u16) {
            return false;
        }
    }
    true
}

pub fn test_millera_rabina(n: &BigUint, k: usize) -> bool{
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