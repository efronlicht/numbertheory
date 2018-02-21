use super::primes::*;
#[test]
fn test_primes_under() {
    let got = Primes::under(12);
    let want = unsafe{Primes::from_raw_vec(vec![2, 3, 5, 7, 11])};
    assert_eq!(want, got);
}

#[test]
fn test_factors() {
    let got = factors(8 * 3, &Primes::under(12));
    let want = Some(hashmap!{2 => 3, 3 => 1});
    assert_eq!(want, got);
}

#[test]
fn test_gcd() {
    // tests intersection implicitly
    let p = Primes::under(12);
    let want =  Some(2 * 3 as u64);
    let got = gcd(2 * 3 * 5, 2 * 2 * 2 * 3, &p);
    assert_eq!(want, got);

    let got = gcd(23, 2, &p); // fail due to primes not being big enough
    assert_eq!(None, got)
}

#[test]
fn test_lcm() {
    let want = Some(2 * 3 * 5 as u64);
    let got = lcm(2 * 3, 5 * 2, &Primes::under(12));
    assert_eq!(want, got);
}
