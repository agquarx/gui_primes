<search/>
use futures::future::BoxFuture;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use eframe::egui;

use crate::{PrimeType, PrimeError};

// Move prime calculation functions from main.rs
pub fn is_prime_recursive(n: u64, divisor: u64, limit: u64) -> Result<bool, PrimeError> {
    if n < 2 {
        return Ok(false);
    }

    match divisor {
        d if d > limit => Ok(true),
        d if n % d == 0 => Ok(false),
        d => is_prime_recursive(n, d + 1, limit),
    }
}

pub fn is_prime_sync(n: u64) -> Result<bool, PrimeError> {
    // Prevent potential overflow in sqrt calculation
    if n > u64::MAX / 2 {
        return Err(PrimeError::Overflow("number too large for primality test"));
    }

    if n < 2 {
        return Ok(false);
    }

    if n == 2 {
        return Ok(true);
    }

    if n % 2 == 0 {
        return Ok(false);
    }

    let limit = (n as f64).sqrt() as u64;

    // Use iteration for small numbers
    if n < 1_000_000 {
        for d in (3..=limit).step_by(2) {
            if n % d == 0 {
                return Ok(false);
            }
        }
        return Ok(true);
    }

    // Use recursion for larger numbers
    is_prime_recursive(n, 3, limit)
}

pub async fn is_prime(n: u64) -> Result<bool, PrimeError> {
    is_prime_sync(n)
}

fn compose_rotation_check(digits: Vec<char>) -> impl Fn(usize) -> Result<bool, PrimeError> {
    move |i| {
        let len = digits.len();
        let rotation: String = digits.iter().cycle().skip(i).take(len).collect();
        rotation.parse::<u64>()
            .map_err(|_| PrimeError::ExecutionError("Failed to parse rotation".into()))
            .and_then(is_prime_sync)
    }
}

fn compose_overflow_check(n: u64, limit: u64, op: &'static str) -> Result<(), PrimeError> {
    if n > limit {
        Err(PrimeError::Overflow(op))
    } else {
        Ok(())
    }
}

pub async fn check_prime_family(ty: &PrimeType, p: u64) -> Result<Option<String>, PrimeError> {
    if p > 1_000_000 {
        return Err(PrimeError::Overflow("input value too large for prime family check"));
    }

    match ty {
        PrimeType::Mersenne => {
            if p > 63 {
                return Ok(None);
            }

            let m = (1u64 << p) - 1;
            match is_prime(m).await {
                Ok(true) => Ok(Some(m.to_string())),
                Ok(false) => Ok(None),
                Err(e) => Err(e),
            }
        }
        PrimeType::SophieGermain => {
            if p > u64::MAX / 2 - 1 {
                return Err(PrimeError::Overflow("sophie germain calculation would overflow"));
            }

            let is_p_prime = is_prime(p).await?;
            if !is_p_prime {
                return Ok(None);
            }

            match is_prime(2*p+1).await {
                Ok(true) => Ok(Some(p.to_string())),
                Ok(false) => Ok(None),
                Err(e) => Err(e),
            }
        }
        PrimeType::Twin => {
            compose_overflow_check(p, u64::MAX - 2, "twin prime calculation would overflow")?;

            let is_p_prime = is_prime(p).await?;
            if !is_p_prime {
                return Ok(None);
            }

            match is_prime(p + 2).await {
                Ok(true) => Ok(Some(format!("({},{})", p, p + 2))),
                Ok(false) => Ok(None),
                Err(e) => Err(e),
            }
        }
        PrimeType::Palindromic => {
            let s = p.to_string();
            let is_p_prime = is_prime(p).await?;
            let is_palindrome = s.chars().eq(s.chars().rev());

            if is_p_prime && is_palindrome {
                Ok(Some(s))
            } else {
                Ok(None)
            }
        }
        PrimeType::Sexy => {
            compose_overflow_check(p, u64::MAX - 6, "sexy prime calculation would overflow")?;

            let is_p_prime = is_prime(p).await?;
            if !is_p_prime {
                return Ok(None);
            }

            match is_prime(p + 6).await {
                Ok(true) => Ok(Some(format!("({},{})", p, p + 6))),
                Ok(false) => Ok(None),
                Err(e) => Err(e),
            }
        }
        PrimeType::Cousin => {
            compose_overflow_check(p, u64::MAX - 4, "cousin prime calculation would overflow")?;

            let is_p_prime = is_prime(p).await?;
            if !is_p_prime {
                return Ok(None);
            }

            match is_prime(p + 4).await {
                Ok(true) => Ok(Some(format!("({},{})", p, p + 4))),
                Ok(false) => Ok(None),
                Err(e) => Err(e),
            }
        }
        PrimeType::Emirp => {
            let rev_str = p.to_string().chars().rev().collect::<String>();
            let rev_result = rev_str.parse::<u64>();

            if let Ok(rev) = rev_result {
                if rev == p {
                    return Ok(None);  // Not an emirp if palindromic
                }

                let is_p_prime = is_prime(p).await?;
                if !is_p_prime {
                    return Ok(None);
                }

                match is_prime(rev).await {
                    Ok(true) => Ok(Some(p.to_string())),
                    Ok(false) => Ok(None),
                    Err(e) => Err(e),
                }
            } else {
                Err(PrimeError::ExecutionError("Failed to parse reversed number".into()))
            }
        }
        PrimeType::Safe => {
            if p < 3 || p % 2 == 0 {
                return Ok(None);
            }

            let safe_check = (p - 1) / 2;
            let is_p_prime = is_prime(p).await?;
            if !is_p_prime {
                return Ok(None);
            }

            match is_prime(safe_check).await {
                Ok(true) => Ok(Some(p.to_string())),
                Ok(false) => Ok(None),
                Err(e) => Err(e),
            }
        }
        PrimeType::Chen => {
            compose_overflow_check(p, u64::MAX - 2, "chen prime calculation would overflow")?;

            let is_p_prime = is_prime(p).await?;
            if !is_p_prime {
                return Ok(None);
            }

            let q = p + 2;
            let check_semi_prime = |n: u64| -> Result<bool, PrimeError> {
                let limit = (n as f64).sqrt() as u64;
                let mut is_semi_prime = false;
                for i in 2..=limit {
                    if n % i == 0 {
                        if let (Ok(is_i_prime), Ok(is_n_div_i_prime)) = (is_prime_sync(i), is_prime_sync(n/i)) {
                            if is_i_prime && is_n_div_i_prime {
                                is_semi_prime = true;
                                break;
                            }
                        }
                    }
                }
                Ok(is_semi_prime)
            };

            match is_prime(q).await {
                Ok(true) => Ok(Some(p.to_string())),
                Ok(false) => match check_semi_prime(q)? {
                    true => Ok(Some(p.to_string())),
                    false => Ok(None),
                },
                Err(e) => Err(e),
            }
        }
        PrimeType::Circular => {
            let is_p_prime = is_prime(p).await?;
            if !is_p_prime {
                return Ok(None);
            }

            let digits: Vec<_> = p.to_string().chars().collect();
            let len = digits.len();

            let rotation_checker = compose_rotation_check(digits);
            for i in 1..len {
                if !rotation_checker(i)? {
                    return Ok(None);
                }
            }

            Ok(Some(p.to_string()))
        }
        PrimeType::Fermat => {
            if p > 6 {
                return Ok(None);
            }

            let exp = 1u32 << p;
            if exp as u32 > 63 { // Check for overflow in calculation
                return Err(PrimeError::Overflow("fermat number would overflow"));
            }

            let f = (1u64 << exp) + 1;
            match is_prime_sync(f) {
                Ok(true) => Ok(Some(format!("F{}", p))),
                Ok(false) => Ok(None),
                Err(e) => Err(e),
            }
        }
        PrimeType::Cuban => {
            if p as u128 > (u64::MAX as u128) / 3 {
                return Err(PrimeError::Overflow("cuban prime calculation would overflow"));
            }

            let v = 3*p*p + 3*p + 1;
            if v > u64::MAX {
                return Err(PrimeError::Overflow("cuban prime result would overflow"));
            }

            match is_prime(v as u64).await {
                Ok(true) => Ok(Some((v as u64).to_string())),
                Ok(false) => Ok(None),
                Err(e) => Err(e),
            }
        }
        PrimeType::Ebl => {
            if p as u128 > (u64::MAX as u128) - 41 {
                return Err(PrimeError::Overflow("ebl prime calculation would overflow"));
            }

            let v = p*p + p + 41;
            match is_prime(v).await {
                Ok(true) => Ok(Some(v.to_string())),
                Ok(false) => Ok(None),
                Err(e) => Err(e),
            }
        }
    }
}

