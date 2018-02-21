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
    assert_eq!(got, want);

    assert_eq!(gcd(23, 2, &p), None); // 23 is a prime larger than 12

    assert_eq!(gcd(1, 90000, &p), Some(1));
    assert_eq!(gcd(90000, 1, &p), Some(1));
 
    assert_eq!(gcd(0, 8, &p), None);
       
}

#[test]
fn test_lcm() {
    let p = Primes::under(12);
    let want = Some(2 * 3 * 5 as u64);
    let got = lcm(2 * 3, 5 * 2, &p);
    assert_eq!(want, got);

    assert_eq!(lcm(1, 5, &p), Some(5));
    assert_eq!(lcm(5, 5, &p), Some(5));

    assert_eq!(lcm(23, 5, &p), None); // 23 is larger than our largest prime 
}
