use std::collections::BTreeMap;
use std::collections;

#[derive(Debug, Clone, PartialEq, Eq)]
///factors represents the prime factors of an integer. the key is the base, and the value is the exponent.
///we note that since (1<<64-1) < (2<<64) is the max u64, the maximum possible exponent we can store without going into
///variable-precision arithmetic is 62, so a u8 is fine for the exponent.

///An assumed invariant is that all exponents are positive.
pub struct Factors(BTreeMap<u64, u8>); // we use a BTreeMap since prime factorizations are explicitly ordered.

impl Factors {
    ///find the prime factors of a positive integer, if any. 0 is considered to have no prime factorization. 1 is considered to have an empty prime factorization.
    pub fn of(n: u64, primes: &Primes) -> Option<Self> {
        let mut factors = BTreeMap::new();
        match n {
            0 => None,
            1 => Some(Factors(factors)),
            _ => {
                let mut n = n;

                for p in &primes.0 {
                    let p = *p;
                    while n % p == 0 {
                        *factors.entry(p).or_insert(0) += 1;
                        n /= p;
                    }
                    if p > n {
                        return Some(Factors(factors)); //we know the prime factorization for sure
                    }
                }
                None // n is larger than our largest known prime. it could be prime, but it could also be a sufficiently large composite number
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.0.is_empty();
    }
    pub fn mul(&self, rhs: &Self) -> Self {
        let mut product = self.0.clone();
        for (k, v) in &rhs.0 {
            *product.entry(*k).or_insert(0) += v;
        }
        Factors(product)
    }

    pub fn totient(&self) -> u64 {
        if self.is_empty() {
            return 0;
        }
        let mut totient = 1;
        for (p, exp) in &self.0 {
            totient *= p.pow(*exp as u32) - p.pow(*exp as u32 - 1)
        }
        totient
    }


    ///a f(a) is a subset of f(b) if b % a == 0
    pub fn is_subset(&self, other: &Self) -> bool {
        for (base, exp) in &self.0 {
            if other.get(*base) <= *exp {
                return false;
            }
        }
        true
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        return other.is_subset(self);
    }

    pub fn get(&self, k: u64) -> u8 {
        if let Some(exp) = self.0.get(&k) {
            *exp
        } else {
            0
        }
    }

    pub fn divisor_count(&self) -> u64 {
        let mut divisors = 1;
        for (_, exp) in &self.0 {
            divisors *= *exp as u64 + 1;
        }
        divisors
    }

    ///the union of two factorizations is the factorization of their least common multiple
    pub fn union(&self, b: &Factors) -> Factors {
        let mut union = self.0.clone();
        for (k, v) in &b.0 {
            if !union.contains_key(k) {
                union.insert(*k, *v);
            } else {
                let m = union[k];
                union.insert(*k, m.max(*v));
            }
        }
        Factors(union)
    }

    ///the intersection of two factorizations is the factorization of their greatest common divisor
    pub fn intersection(&self, b: &Factors) -> Factors {
        let b = &b.0;
        let mut intersection = BTreeMap::new();
        for (k, v) in &self.0 {
            if b.contains_key(&k) {
                let min = v.min(&b[&k]);
                intersection.insert(*k, *min);
            }
        }
        Factors(intersection)
    }


}

impl IntoIterator for Factors {
    type Item = (u64, u8);
    type IntoIter = collections::btree_map::IntoIter<u64, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Into<u64> for Factors {
    fn into(self) -> u64 {
        self.into_iter()
            .fold(1, |n, (base, exp)| n * (base.pow(exp as u32)))
    }
}

impl Into<BTreeMap<u64, u8>> for Factors {fn into(self) -> BTreeMap<u64, u8> {self.0}}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
///Primes represents a subset of the primes, starting with no gaps from the beginning of the positive integers, under a certain maximum.
///Valid Primes are [], [2, 3, 5], [2, 3, 5, 7, 11], but NOT [2, 5, 11].
pub struct Primes(Vec<u64>);

impl Primes {
    ///the maximum prime in the representation
    pub fn max(&self) -> Option<u64> {
        if self.0.len() == 0 {
            None
        } else {
            Some(self.0[self.0.len() - 1])
        }
    }

    ///create a Primes representing all primes under 'max'. this is not an efficient implementation.
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
    /// create the first n Primes. this is not an efficient implementation.
    pub fn first_n(size: usize) -> Self {
        let mut primes = Vec::new();
        if size == 0 {
            return Primes(primes);
        }
        primes.push(2);
        let mut m = 3;
        while primes.len() < size {
            let mut isprime = true;
            for p in &primes {
                if m % p == 0 {
                    isprime = false;
                    break;
                }
            }
            if isprime {
                primes.push(m);
            }

            m += 2
        }
        Primes(primes)
    }


    //lcm is the least common multiple; that is, the smallest q such that a*m == q, b*n == q for some positive integers a, b
    pub fn lcm(&self, m: u64, n: u64) -> Option<u64> {
        match (m, n) {
            (0, 0) => Some(0),
            (0, _) | (_, 0) => None,
            (m, 1) => Some(m),
            (1, n) => Some(n),

            (m, n) => {
                let m_factors = Factors::of(m, self)?;
                let n_factors = Factors::of(n, self)?;
                let mut lcm = 1;
                for (p, exp) in m_factors.union(&n_factors) {
                    lcm *= p.pow(exp as u32);
                }
                Some(lcm)
            }
        }
    }

    ///gcd is the greatest common divisor.  that is, the largest d such that ad == m bd == n for some positive integers a, b
pub fn gcd(&self, m: u64, n: u64) -> Option<u64> {
    match (m, n) {
        (0, _) | (_, 0) => None,
        (1, _) | (_, 1) => Some(1),

        (m, n) => {
            let mut gcd = 1;
            let (m_factors, n_factors) = (&Factors::of(m, self)?, &Factors::of(n, self)?);
            for (p, exp) in m_factors.intersection(n_factors){
                gcd *= p.pow(exp as u32);
            }
            Some(gcd)
        }
    }
}





    ///convert a Vec<64> into Primes directly, without checking that they are actually prime.
    pub unsafe fn from_raw_vec(v: Vec<u64>) -> Self {
        Primes(v)
    }
}

impl Into<Vec<u64>> for Primes {
    fn into(self) -> Vec<u64> {
        self.0
    }
}

