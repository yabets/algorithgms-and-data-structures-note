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