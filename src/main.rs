use prime_sieve_vec::PrimeSieveVec;

pub mod prime_sieve_vec;

/// Iterator for the sequence S where S[i] is the number of primes <= i
struct PrimesLessThan {
    n: usize,
    sieve: PrimeSieveVec,
    greatest_so_far: usize,
}

impl Iterator for PrimesLessThan {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current_greatest_so_far = self.greatest_so_far;

        while self.greatest_so_far <= current_greatest_so_far {
            let answer = unsafe { self.sieve.count_primes_less_or_equal(self.n) };
            self.greatest_so_far = answer;
            self.n = self.n.checked_add(1)?;
        }
        Some(self.greatest_so_far)
    }
}

// fn main() {
//     use std::time::Instant;

//     let timer = Instant::now();

//     let mut sieve = PrimeSieveVec::default();

//     let mut p = 0;
//     while let Some(n) = 2usize.checked_pow(p) {
//         let t = Instant::now();
//         p += 1;
//         let count = unsafe { sieve.count_primes_less_or_equal(n) };
//         println!("primes < 2^{p} = {count} ({:?} elapsed)", t.elapsed());
//         println!("CAPACITY {}", sieve.primes.capacity());
//     }

//     println!("{:?}", timer.elapsed());
// }

fn main() {
    use std::time::Instant;
    /// The sieve calculates all primes < TARGET, but may calculate more.
    const TARGET: usize = 10usize.pow(9);
    const EXTEND_AT_MOST_N_SEGMENTS_TARGET: usize = 1;

    // Print what power of two TARGET is.
    println!("Checking up to 10 ^ {}", TARGET.ilog10());

    let now = Instant::now();
    let mut sieve = PrimeSieveVec::new(EXTEND_AT_MOST_N_SEGMENTS_TARGET);
    sieve.reserve_in_advance(51_000_000);

    let primes_found = unsafe { sieve.count_primes_less_or_equal(TARGET) };
    let elapsed = now.elapsed();

    println!("Found all primes < {TARGET}");
    println!("Primes Found => {primes_found:?}");
    println!("CAPACITY OF PRIMES {}", sieve.primes.capacity());
    println!("Finished in {elapsed:?}");
}
