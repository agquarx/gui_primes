// Property-based tests for the gui_primes library
// These tests verify specific mathematical properties of different prime families

use gui_primes::primes::{PrimeType, compute_with_memo};

// Helper function to run computations
fn run_calculation(kind: PrimeType, start: u64, end: u64) -> Vec<String> {
    compute_with_memo(kind, start, end)
}

// Test for specific mathematical properties of Sophie Germain primes
#[test]
fn test_sophie_germain_properties() {
    // Sophie Germain primes p where 2p+1 is also prime
    let sophie_primes = run_calculation(PrimeType::SophieGermain, 2, 50);
    
    for prime_str in &sophie_primes {
        let p: u64 = prime_str.parse().unwrap();
        let q = 2 * p + 1;
        
        // Verify p is prime
        let is_p_prime = (2..=((p as f64).sqrt() as u64)).all(|i| p % i != 0);
        assert!(is_p_prime, "Sophie Germain prime {} is not actually prime", p);
        
        // Verify 2p+1 is prime
        let is_q_prime = (2..=((q as f64).sqrt() as u64)).all(|i| q % i != 0);
        assert!(is_q_prime, "For Sophie Germain prime {}, 2p+1={} is not prime", p, q);
    }
}

// Test for specific properties of Palindromic primes
#[test]
fn test_palindromic_properties() {
    let palindromic_primes = run_calculation(PrimeType::Palindromic, 10, 200);
    
    for prime_str in &palindromic_primes {
        // Verify it's a palindrome
        let forward = prime_str.chars().collect::<String>();
        let reverse = prime_str.chars().rev().collect::<String>();
        assert_eq!(forward, reverse, "Prime {} is not palindromic", prime_str);
        
        // Verify it's prime
        let p: u64 = prime_str.parse().unwrap();
        let is_prime = (2..=((p as f64).sqrt() as u64)).all(|i| p % i != 0);
        assert!(is_prime, "Palindromic number {} is not prime", p);
    }
}

// Test for specific properties of Emirp primes
#[test]
fn test_emirp_properties() {
    let emirp_primes = run_calculation(PrimeType::Emirp, 10, 100);
    
    for prime_str in &emirp_primes {
        let p: u64 = prime_str.parse().unwrap();
        
        // Verify p is prime
        let is_p_prime = (2..=((p as f64).sqrt() as u64)).all(|i| p % i != 0);
        assert!(is_p_prime, "Emirp {} is not prime", p);
        
        // Verify reverse is different and also prime
        let reverse_str = p.to_string().chars().rev().collect::<String>();
        let r: u64 = reverse_str.parse().unwrap();
        
        assert_ne!(p, r, "Emirp {} is the same when reversed", p);
        
        let is_r_prime = (2..=((r as f64).sqrt() as u64)).all(|i| r % i != 0);
        assert!(is_r_prime, "Reverse of emirp {} ({}) is not prime", p, r);
    }
}

// Test for happy primes
#[test]
fn test_happy_prime_properties() {
    let happy_primes = run_calculation(PrimeType::Happy, 1, 50);
    
    for prime_str in &happy_primes {
        let p: u64 = prime_str.parse().unwrap();
        
        // Verify p is prime
        let is_p_prime = p >= 2 && (2..=((p as f64).sqrt() as u64)).all(|i| p % i != 0);
        assert!(is_p_prime, "Happy prime {} is not prime", p);
        
        // Verify p is happy
        let mut current = p;
        let mut seen = std::collections::HashSet::new();
        
        while current != 1 && !seen.contains(&current) {
            seen.insert(current);
            current = sum_of_squares_of_digits(current);
        }
        
        assert_eq!(current, 1, "Prime {} is not happy", p);
    }
}

// Helper function for happy number calculation
fn sum_of_squares_of_digits(mut n: u64) -> u64 {
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    sum
}

// Test for special behaviors with all prime families in same range
#[test]
fn test_multiple_prime_families_consistency() {
    // Small range where we can verify results
    let range_start = 10;
    let range_end = 30;
    
    // Get results for several prime families
    let twin = run_calculation(PrimeType::Twin, range_start, range_end);
    let palindromic = run_calculation(PrimeType::Palindromic, range_start, range_end);
    let mersenne = run_calculation(PrimeType::Mersenne, range_start, range_end);
    
    // Verify twin prime pairs are correctly formed
    for pair in &twin {
        if pair.starts_with('(') && pair.ends_with(')') {
            let nums: Vec<&str> = pair[1..pair.len()-1].split(',').collect();
            let first: u64 = nums[0].parse().unwrap();
            let second: u64 = nums[1].parse().unwrap();
            
            assert_eq!(second - first, 2, "Twin primes should differ by exactly 2");
        }
    }
    
    // Verify palindromic primes are actually palindromes
    for prime in &palindromic {
        let forward = prime.chars().collect::<String>();
        let reverse = prime.chars().rev().collect::<String>();
        assert_eq!(forward, reverse, "Prime {} is not palindromic", prime);
    }
    
    // For Mersenne primes, verify they are of form 2^p - 1
    for prime in &mersenne {
        let m: u64 = prime.parse().unwrap();
        // Check if m+1 is a power of 2
        let is_power_of_two = (m + 1) & (m + 1 - 1) == 0;
        assert!(is_power_of_two, "Mersenne prime {} is not of form 2^p-1", m);
    }
}

// Test for consistency between related prime families
#[test]
fn test_related_prime_families() {
    // Twin, Cousin, and Sexy primes are related by the gap between prime pairs
    // Twin: (p, p+2), Cousin: (p, p+4), Sexy: (p, p+6)
    
    let range_start = 10;
    let range_end = 50;
    
    let twin = run_calculation(PrimeType::Twin, range_start, range_end);
    let cousin = run_calculation(PrimeType::Cousin, range_start, range_end);
    let sexy = run_calculation(PrimeType::Sexy, range_start, range_end);
    
    // Extract first numbers from twin pairs
    let twin_firsts: Vec<u64> = twin.iter()
        .filter_map(|pair| {
            if pair.starts_with('(') && pair.contains(',') {
                let first = &pair[1..pair.find(',').unwrap()];
                first.parse().ok()
            } else {
                None
            }
        })
        .collect();
    
    // Extract first numbers from cousin pairs
    let cousin_firsts: Vec<u64> = cousin.iter()
        .filter_map(|pair| {
            if pair.starts_with('(') && pair.contains(',') {
                let first = &pair[1..pair.find(',').unwrap()];
                first.parse().ok()
            } else {
                None
            }
        })
        .collect();
    
    // Extract first numbers from sexy pairs
    let sexy_firsts: Vec<u64> = sexy.iter()
        .filter_map(|pair| {
            if pair.starts_with('(') && pair.contains(',') {
                let first = &pair[1..pair.find(',').unwrap()];
                first.parse().ok()
            } else {
                None
            }
        })
        .collect();
    
    // Verify no overlap between these sets (a prime can't start both a twin and cousin pair)
    for &p in &twin_firsts {
        assert!(!cousin_firsts.contains(&p), "Prime {} starts both twin and cousin pairs", p);
    }
    
    // Verify the sets are different
    assert_ne!(twin_firsts, cousin_firsts, "Twin and Cousin prime sets should be different");
    assert_ne!(twin_firsts, sexy_firsts, "Twin and Sexy prime sets should be different");
    assert_ne!(cousin_firsts, sexy_firsts, "Cousin and Sexy prime sets should be different");
}

// Test cache behavior with complex sequences
#[test]
fn test_cache_with_overlapping_ranges() {
    // A sequence of overlapping range calculations
    let ranges = [
        (10, 50),
        (30, 70),
        (20, 60),
        (10, 30),
        (50, 70)
    ];
    
    let prime_type = PrimeType::Palindromic;
    
    // Calculate reference results for the full range
    let full_range = run_calculation(prime_type, 10, 70);
    
    // Calculate results for each subrange
    for (start, end) in ranges.iter() {
        let subrange = run_calculation(prime_type, *start, *end);
        
        // Verify subrange results are consistent with full range
        for prime in &subrange {
            assert!(full_range.contains(prime), 
                  "Prime {} in range {}-{} not found in full range", 
                  prime, start, end);
        }
    }
    
    // Calculate the union of all subranges
    let mut all_results = Vec::new();
    for (start, end) in ranges.iter() {
        let subrange = run_calculation(prime_type, *start, *end);
        for prime in subrange {
            if !all_results.contains(&prime) {
                all_results.push(prime);
            }
        }
    }
    
    // Sort for comparison
    all_results.sort_by(|a, b| {
        let a_val: u64 = a.parse().unwrap_or(0);
        let b_val: u64 = b.parse().unwrap_or(0);
        a_val.cmp(&b_val)
    });
    
    let mut sorted_full_range = full_range.clone();
    sorted_full_range.sort_by(|a, b| {
        let a_val: u64 = a.parse().unwrap_or(0);
        let b_val: u64 = b.parse().unwrap_or(0);
        a_val.cmp(&b_val)
    });
    
    // Verify the union of all subranges equals the full range
    assert_eq!(all_results, sorted_full_range, 
              "Union of all subranges should equal the full range");
}

// Test extreme ranges that might cause overflow or other issues
#[test]
fn test_extreme_ranges() {
    // Very large start value
    let _large_start = run_calculation(PrimeType::Palindromic, 10_000_000, 10_000_010);
    
    // Very small range
    let _small_range = run_calculation(PrimeType::Twin, 2, 3);
    
    // Range including 0 and 1 (which are not prime)
    let with_non_primes = run_calculation(PrimeType::Mersenne, 0, 5);
    
    // Empty range
    let empty_range = run_calculation(PrimeType::Safe, 100, 100);
    
    // Zero should not be considered prime
    assert!(!with_non_primes.contains(&"0".to_string()), "0 should not be considered prime");
    
    // One should not be considered prime
    assert!(!with_non_primes.contains(&"1".to_string()), "1 should not be considered prime");
    
    // Empty range should produce empty results
    assert!(empty_range.is_empty(), "Empty range should produce no results");
}

