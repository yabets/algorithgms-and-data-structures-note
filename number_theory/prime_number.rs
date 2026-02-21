pub fn is_prime(n: i32) -> bool {
    if n < 3 {
        return true;
    }
    for i in 2..n {
        if i*i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn generate_prime() -> Vec<bool> {
    let mut is_prime: [bool; 128] = [true; 128];
    // map primes
    is_prime[0] = false;
    is_prime[1] = false;
    for ix in 2..128_i32.isqrt() as usize {
        if is_prime[ix] {
                // println!("{ix} is prime");
            for iy in ((ix * ix)..128).step_by(ix) {
                    // print!(" {iy} ");
                is_prime[iy] = false;
            }
        }
            // println!("");
    }
}