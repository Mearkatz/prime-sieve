pub mod prime_sieve_vec;

mod tests {
    #[test]
    fn primes_less_than_n_works() {
        use crate::prime_sieve_vec::PrimeSieveVec;
        let mut sieve = PrimeSieveVec::default();
        let mut primes_lte = |n| unsafe { sieve.count_primes_less_or_equal_unchecked(n) };

        assert_eq!(primes_lte(10), 4);
        assert_eq!(primes_lte(100), 25);
        assert_eq!(primes_lte(1_000), 168);
        assert_eq!(primes_lte(10_000), 1_229);
        assert_eq!(primes_lte(100_000), 9_592);
        assert_eq!(primes_lte(1_000_000), 78_498);
    }
}
