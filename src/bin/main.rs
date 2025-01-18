use prime_sieve::prime_sieve_vec::{approx_primes_lt, PrimeSieveVec};

fn main() {
    const UPPERBOUND: usize = 1_000_000_000;
    let timer = std::time::Instant::now();
    let mut sieve = PrimeSieveVec::new();

    let approx = approx_primes_lt(UPPERBOUND);
    println!("Reserving {approx} elements in sieve");
    sieve.reserve_in_advance(approx);
    println!(
        "Primes < {UPPERBOUND} = {}",
        sieve.count_primes_lt(UPPERBOUND)
    );
    println!("Finished in {:?}", timer.elapsed());
}
