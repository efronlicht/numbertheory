use std::collections;

///factors represents the prime factors of an integer. the key is the base, and the value is the exponent.
///we note that since (1<<64-1) < (2<<64) is the max u64, the maximum possible exponent we can store without going into 
///variable-precision arithmetic is 62, so a u8 is fine for the exponent.
pub type Factors = collections::HashMap<u64, u8>;

#[derive(Debug, Clone, PartialEq)]
///Primes represents a subset of the primes, starting with no gaps from the beginning of the positive integers, under a certain maximum.
///Valid Primes are [], [2, 3, 5], [2, 3, 5, 7, 11], but NOT [2, 5, 11].
pub struct Primes(Vec<u64>);

impl Primes {
    ///the maximum prime in the reperesentation
    pub fn max(&self) -> Option<u64> {
        if self.0.len() == 0 {
            None
        } else {
            Some(self.0[self.0.len()-1])
        }
    }

    ///create a Primes representing all primes under 'max' using the sieve of Eratosthenes.
    pub fn under(max: u64) -> Self {
        let mut primes = Primes(Vec::new());
        if max < 2 {
            return primes;
        }
        primes.0.push(2);
        let mut n = 3;

        while n < max {
            let mut isprime = true;
            for p in &primes.0 {
                if n % p == 0 {
                    isprime = false;
                    break;
                }
            }
            if isprime {
                primes.0.push(n);
            }

            n += 2
        }
        primes
    }
    
    ///convert a Vec<64> into Primes directly, without checking that they are actually prime.
    pub unsafe fn from_raw_vec(v: Vec<u64>) -> Self {Primes(v)}
}

impl Into<Vec<u64>> for Primes {fn into(self) -> Vec<u64> {self.0}}

pub fn union(a: &Factors, b: &Factors) -> Factors {
    let mut union = a.clone();
    for (k, v) in b {
        if !union.contains_key(k) {
            union.insert(*k, *v);
        } else {
            let m = union[k];
            union.insert(*k, m.max(*v));
        }
    }
    union
}

pub fn intersection(a: &Factors, b: &Factors) -> Factors {
    let mut intersection = Factors::new();
    for (k, v) in a {
        if b.contains_key(k) {
            let min = v.min(&b[k]);
            intersection.insert(*k, *min);
        }
    }
    intersection
}
///gcd is the greatest common divisor.  that is, the largest d such that ad == m bd == n for some positive integers a, b
pub fn gcd(m: u64, n: u64, primes: &Primes) -> Option<u64> {
    match (m, n) {
        (0, _) | (_, 0) => None,
        (1, _) | (_, 1) => Some(1),
        
        (m, n) => {
            let mut gcd = 1;
            for (p, exp) in intersection(&factors(m, primes)?, &factors(n, primes)?) {
                gcd *= p.pow(exp as u32);
            }
            Some(gcd)
        }
    }
}

//lcm is the least common multiple; that is, the smallest q such that a*m == q, b*n == q for some positive integers a, b
pub fn lcm(m: u64, n: u64, primes: &Primes) -> Option<u64> {
    match (m, n) {
        (0, 0) => Some(0), 
        (0, _) | (_, 0) => None, 
        (m, 1) => Some(m),
        (1, n) => Some(n),
        
        (m, n) => {
            let union = union(&factors(n, primes)?, &factors(m, primes)?);
            let mut lcm = 1;
            for (p, exp) in union {
                lcm *= p.pow(exp as u32);
            }
            Some(lcm)
        }
    }
}



///find the prime factors of a positive integer, if any. 0 is considered to have no prime factorization. 1 is considered to have an empty prime factorization.
pub fn factors(n: u64, primes: &Primes) -> Option<Factors> {
    let mut factors = Factors::new();
    match n {
        0 => return None,
        1 => return Some(factors),
        _ => {},
    }
    let mut n = n;
    for p in &primes.0 {
        let p = *p;
        while n % p == 0 {
            *factors.entry(p).or_insert(0) += 1;
            n /= p;
            }
        if p > n {
            return Some(factors); //we know the prime factorization for sure
            }   
    }
        None // n is larger than our largest known prime. it could be prime, but it could also be a sufficiently large composite number
}
