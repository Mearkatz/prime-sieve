use std::{
    ops::Not,
    sync::atomic::{AtomicBool, Ordering},
};

use bisection::bisect_right;
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};

pub struct PrimeSieveVec {
    pub primes: Vec<usize>,
    pub end_segment: usize,
    pub extend_at_most_n_segments_target: usize,
}

impl Default for PrimeSieveVec {
    fn default() -> Self {
        Self {
            primes: vec![2, 3, 5, 7],
            end_segment: 1,
            extend_at_most_n_segments_target: 1,
        }
    }
}

#[allow(unused)]
impl PrimeSieveVec {
    /// Creates a new `PrimeSieveVec`
    #[must_use]
    pub fn new(extend_at_most_n_segments_target: usize) -> Self {
        Self {
            primes: {
                let mut sieve = Self::default();
                sieve.first_n_primes(extend_at_most_n_segments_target + 2);
                sieve.primes
            },
            extend_at_most_n_segments_target,
            ..Self::default()
        }
    }

    /// A reference to the last element of `self.primes`
    ///
    /// # Safety
    /// `self.primes` must be non-empty.
    #[must_use]
    pub unsafe fn most_recent_prime_unchecked(&self) -> &usize {
        unsafe { self.primes.get_unchecked(self.primes.len() - 1) }
    }

    #[must_use]
    pub fn most_recent_prime(&self) -> Option<&usize> {
        self.primes.last()
    }

    /// calls `.reverse(additional)` on `self.primes`
    pub fn reserve_in_advance(&mut self, additional: usize) {
        self.primes.reserve(additional);
    }

    #[allow(clippy::many_single_char_names)]
    pub fn extend_at_most_n_segments(&mut self, n: usize) {
        let k = self.end_segment;
        let p = self.primes[k];
        let q = self.primes[k + n];
        let segment = p * p..q * q;
        let segment_min = p * p;
        let segment_max = q * q;
        let segment_len = segment_max - segment_min + 1;

        let mut is_prime: Box<[bool]> = std::iter::repeat(true).take(segment_len).collect();

        for pk in self.primes[..k + n].iter().copied() {
            // Set all the multiples of pk to false (they aren't prime)
            let start = segment_min.next_multiple_of(pk) - segment_min;
            let stop = is_prime.len();
            for x in (start..stop).step_by(pk) {
                is_prime[x] = false;
            }
        }
        self.primes.extend(
            segment
                .zip(is_prime.iter())
                .filter_map(|(x, it_is_prime)| it_is_prime.then_some(x)),
        );

        self.end_segment += n;
    }

    pub fn extend_at_most_n_segments_threaded(&mut self, n: usize) {
        let k = self.end_segment;
        let p = self.primes[k];
        let q = self.primes[k + n];
        let segment_min = p * p;
        let segment_max = q * q - 1;
        let segment = segment_min..segment_max;
        let segment_len = segment_max - segment_min + 1;

        // let mut is_prime: Box<[bool]> = std::iter::repeat(true).take(segment_len).collect();
        let mut is_prime: Box<[AtomicBool]> = std::iter::repeat_with(|| AtomicBool::new(true))
            .take(segment_len)
            .collect();

        self.primes[..k + n].par_iter().for_each(|pk| {
            // Set all the multiples of pk to false (they aren't prime)
            let start = segment_min.next_multiple_of(*pk) - segment_min;
            let stop = is_prime.len();
            (start..stop)
                .step_by(*pk)
                .par_bridge()
                .into_par_iter()
                .for_each(|x| {
                    is_prime[x].store(false, Ordering::Relaxed);
                });
        });

        self.primes.extend(
            segment
                .zip(is_prime.iter())
                .filter_map(|(x, it_is_prime)| it_is_prime.load(Ordering::Relaxed).then_some(x)),
        );

        self.end_segment += n;
    }

    /// Shorthand for `self.extend_at_most_n_segments(self.extend_at_most_n_segments_target);`
    pub fn extend(&mut self) {
        self.extend_at_most_n_segments(self.extend_at_most_n_segments_target);
    }

    /// Returns the number of primes < `n`.
    ///
    /// # Safety
    /// `self.primes` must be non-empty when this is called.
    pub unsafe fn count_primes_less_or_equal_unchecked(&mut self, n: usize) -> usize {
        while self.most_recent_prime_unchecked() < &n {
            self.extend();
        }
        bisect_right(&self.primes, &n)
    }

    /// Returns the number of primes < `n` if self.primes is non-empty, otherwise `None`.
    pub fn count_primes_less_or_equal(&mut self, n: usize) -> Option<usize> {
        self.primes
            .is_empty()
            .not()
            .then_some(unsafe { self.count_primes_less_or_equal_unchecked(n) })
    }

    /// A slice of the first n primes calculated via an instance.
    pub fn first_n_primes(&mut self, n: usize) -> &[usize] {
        self.calculate_first_n_primes_exact(n)
    }

    /// Calculates primes without accidentally calculating more, returning a slice of them.
    pub fn calculate_first_n_primes_exact(&mut self, n: usize) -> &[usize] {
        while self.primes.len() < n {
            self.extend();
        }
        &self.primes[..n]
    }

    pub fn nth_prime(&mut self, n: usize) -> usize {
        loop {
            if let Some(x) = self.primes.get(n).copied() {
                return x;
            }
            self.extend();
        }
    }
}
