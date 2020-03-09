pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut num = 2;

    while primes.len() <= n as usize {
        if is_prime(num, &primes) {
            primes.push(num)
        }
        num += 1
    }

    primes.pop().unwrap()
}

fn is_prime(n: u32, primes: &[u32]) -> bool {
    for i in primes {
        if n % i == 0 {
            return false;
        }
    }
    true
}
