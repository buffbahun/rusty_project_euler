use crate::helper;

pub fn soln_5(num_to: u64) -> u64 {
    let primes = helper::prime_gen(num_to);
    let mut product: u64 = 1;

    for prime in primes {
        let mut n = 2;
        let mut max_pow = prime;

        while prime.pow(n) <= num_to {
            max_pow = prime.pow(n);
            n += 1;
        }
        product *= max_pow;
    }

    product
}
