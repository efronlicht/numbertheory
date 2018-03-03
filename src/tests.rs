use super::primes::*;
use std::collections::BTreeMap;



#[test]
fn test_primes_under() {
    let got = Primes::under(12);
    let want = unsafe { Primes::from_raw_vec(vec![2, 3, 5, 7, 11]) };
    assert_eq!(want, got);
}


#[test]
fn test_primes_first_n() {
    let got = Primes::first_n(5);
    let want = Primes::under(12);
    assert_eq!(want, got);
}

#[test]
fn test_factors() {
    let p = Primes::under(20);
    let got: BTreeMap<u64, u8> = Factors::of(8 * 3, &p).unwrap().into();
    let want = btreemap!{2 => 3, 3 => 1};
    assert_eq!(got, want);
}

#[test]
fn test_mul() {
    let p = Primes::under(30);
    let f_12 = &Factors::of(2 * 2 * 3, &p).unwrap();
    let f_30 = &Factors::of(2 * 3 * 5, &p).unwrap();
    let got = f_12.mul(f_30);
    assert_eq!(got, Factors::of(2 * 2 * 2 * 3 * 3 * 5, &p).unwrap());
}

#[test]
fn test_totient() {
    let p = Primes::under(30);
    let factors = Factors::of(60, &p).unwrap();
    assert_eq!(16, factors.totient())
}

#[test]
fn test_count_divisors() {
    //the 12 divisors of 60 are 1,2,3,4,5,6,10,12,15,20,30,60
    let p = Primes::under(30);
    let factors = Factors::of(60, &p).unwrap();
    assert_eq!(12, factors.divisor_count());
}

#[test]
fn test_gcd() {
    // tests intersection implicitly
    let p = Primes::under(12);
    let want = Some(2 * 3 as u64);
    let got = p.gcd(2 * 3 * 5, 2 * 2 * 2 * 3);
    assert_eq!(got, want);

    assert_eq!(p.gcd(23, 2), None); // 23 is a prime larger than 12

    assert_eq!(p.gcd(1, 90000), Some(1));
    assert_eq!(p.gcd(90000, 1), Some(1));

    assert_eq!(p.gcd(0, 8), None);

}

#[test]
fn test_factors_into_u64() {
    
}

#[test]
fn test_lcm() {
        let p = Primes::under(12);

    // tests union implicitly
    let want = Some(2 * 3 * 5 as u64);
    let got = p.lcm(2 * 3, 5 * 2);
    assert_eq!(want, got);

    assert_eq!(p.lcm(1, 5), Some(5));
    assert_eq!(p.lcm(5, 5), Some(5));

    assert_eq!(p.lcm(23, 5), None); // 23 is larger than our largest prime
}
