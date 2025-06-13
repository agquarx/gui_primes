// src/primes.rs
//! Pure, functional-style prime families engine with memoization.

use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

pub type CacheKey = (PrimeType, u64, u64);
static MEMO: Lazy<Mutex<HashMap<CacheKey, Vec<String>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrimeType {
    Mersenne,
    SophieGermain,
    Twin,
    Palindromic,
    Sexy,
    Cousin,
    Emirp,
    Safe,
    Chen,
    Circular,
    Fermat,
    Cuban,
    Ebl,
    Proth,
    Cullen,
    Woodall,
    Thabit,
    Euclid,
    Fibonacci,
    Perrin,
    Happy,
    Wilson,
    CenteredHex,
}

impl PrimeType {
    pub fn all() -> &'static [PrimeType] {
        use PrimeType::*;
        &[
            Mersenne, SophieGermain, Twin, Palindromic, Sexy, Cousin, Emirp, Safe,
            Chen, Circular, Fermat, Cuban, Ebl,
            Proth, Cullen, Woodall, Thabit, Euclid,
            Fibonacci, Perrin, Happy, Wilson, CenteredHex,
        ]
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        false
    } else {
        (2..=((n as f64).sqrt() as u64)).all(|i| n % i != 0)
    }
}

fn family_hit(ty: PrimeType, p: u64) -> Option<String> {
    use PrimeType::*;
    match ty {
        Mersenne if p <= 63 => {
            let m = (1u64 << p).wrapping_sub(1);
            is_prime(m).then(|| m.to_string())
        }
        SophieGermain => (is_prime(p) && is_prime(2 * p + 1)).then(|| p.to_string()),
        Twin => (is_prime(p) && is_prime(p + 2)).then(|| format!("({},{})", p, p + 2)),
        Palindromic => {
            let s = p.to_string();
            (is_prime(p) && s.chars().eq(s.chars().rev())).then_some(s)
        }
        Sexy => (is_prime(p) && is_prime(p + 6)).then(|| format!("({},{})", p, p + 6)),
        Cousin => (is_prime(p) && is_prime(p + 4)).then(|| format!("({},{})", p, p + 4)),
        Emirp => {
            let r = p
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<u64>()
                .unwrap_or(0);
            (r != p && is_prime(p) && is_prime(r)).then(|| p.to_string())
        }
        Safe => (is_prime(p) && is_prime((p - 1) / 2)).then(|| p.to_string()),
        Chen => {
            let q = p + 2;
            let semi = (2..q)
                .filter(|&i| q % i == 0)
                .any(|i| is_prime(i) && is_prime(q / i));
            (is_prime(p) && (is_prime(q) || semi)).then(|| p.to_string())
        }
        Circular => (is_prime(p)
            && (0..p.to_string().len())
                .map(|i| {
                    let digs: Vec<_> = p.to_string().chars().collect();
                    digs[i..]
                        .iter()
                        .chain(&digs[..i])
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap()
                })
                .all(is_prime))
        .then(|| p.to_string()),
        Fermat if p <= 6 => {
            let f = (1u128 << (1u32 << p)) + 1;
            (is_prime(f as u64)).then(|| format!("F{}", p))
        }
        Cuban => {
            let v = 3 * p * p + 3 * p + 1;
            is_prime(v).then(|| v.to_string())
        }
        Ebl => {
            let v = p * p + p + 41;
            is_prime(v).then(|| v.to_string())
        }
        Proth => (1..=63).find_map(|n| {
            let two_n = 1u64 << n;
            ((p > two_n && (p - 1) % two_n == 0) && is_prime(p)).then(|| p.to_string())
        }),
        Cullen => (1..=60)
            .map(|n| (n as u64).saturating_mul(1u64 << n).saturating_add(1))
            .find(|&v| v == p && is_prime(p))
            .map(|v| v.to_string()),
        Woodall => (1..=60)
            .map(|n| (n as u64).saturating_mul(1u64 << n).saturating_sub(1))
            .find(|&v| v == p && is_prime(p))
            .map(|v| v.to_string()),
        Thabit => (1..=60)
            .map(|n| (1u64 << n).saturating_mul(3).saturating_sub(1))
            .find(|&v| v == p && is_prime(p))
            .map(|v| v.to_string()),
        Euclid => {
            const PRIMES: [u64; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
            PRIMES
                .iter()
                .scan(1u64, |prod, &q| {
                    *prod = prod.saturating_mul(q);
                    Some(*prod + 1)
                })
                .find(|&v| v == p && is_prime(p))
                .map(|_| p.to_string())
        }
        Fibonacci => {
            let mut fibs = vec![1u64, 1];
            while *fibs.last().unwrap() < p {
                let n = fibs.len();
                fibs.push(fibs[n - 1] + fibs[n - 2]);
            }
            fibs.pop()
                .filter(|&v| v == p && is_prime(p))
                .map(|v| v.to_string())
        }
        Perrin => {
            let mut seq = vec![3u64, 0, 2];
            while *seq.last().unwrap() < p {
                let n = seq.len();
                seq.push(seq[n - 1] + seq[n - 2]);
            }
            seq.pop()
                .filter(|&v| v == p && is_prime(p))
                .map(|v| v.to_string())
        }
        Happy => {
            fn step(mut x: u64) -> u64 {
                let mut sum = 0;
                while x > 0 {
                    let d = x % 10;
                    sum += d * d;
                    x /= 10;
                }
                sum
            }

            // Check if it's a happy number by tracking the sequence
            let mut seen = std::collections::HashSet::new();
            let mut current = p;

            while current != 1 && !seen.contains(&current) {
                seen.insert(current);
                current = step(current);
            }

            // If current == 1, it's a happy number
            (current == 1 && is_prime(p)).then(|| p.to_string())
        }
        Wilson if p < 50 => {
            let fact = (2..p).fold(1u128, |acc, i| acc * i as u128 % p as u128);
            ((fact + 1) % p as u128 == 0 && is_prime(p)).then(|| p.to_string())
        }
        CenteredHex => {
            let n = (((12 * p - 3) as f64).sqrt() + 3.0) / 6.0;
            (n.fract() == 0.0 && 3 * (n as u64) * ((n as u64) - 1) + 1 == p && is_prime(p))
                .then(|| p.to_string())
        }
        _ => None,
    }
}

pub fn calculate_family(ty: PrimeType, start: u64, end: u64) -> Vec<String> {
    (start..end).filter_map(|p| family_hit(ty, p)).collect()
}

pub fn cache_stats() -> usize {
    MEMO.lock().unwrap().len()
}

pub fn compute_with_memo(ty: PrimeType, start: u64, end: u64) -> Vec<String> {
    // Ensure start <= end
    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };

    let key = (ty, start, end);
    let mut cache = MEMO.lock().unwrap();
    cache
        .entry(key)
        .or_insert_with(|| calculate_family(ty, start, end))
        .clone()
}
