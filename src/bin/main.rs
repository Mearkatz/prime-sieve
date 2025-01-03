use prime_sieve::prime_sieve_vec::{approx_primes_lt, PrimeSieveVec};

fn main() {
    const UPPERBOUND: usize = 1_000_000;
    let timer = std::time::Instant::now();
    let mut sieve = PrimeSieveVec::new();
    sieve.reserve_in_advance(approx_primes_lt(UPPERBOUND) * 10 / 9);
    println!(
        "Primes < {UPPERBOUND} = {}",
        sieve.count_primes_lt(UPPERBOUND)
    );
    println!("Finished in {:?}", timer.elapsed());
}
