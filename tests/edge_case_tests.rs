// Edge case tests for the gui_primes library
// These tests focus on boundary conditions and unexpected inputs

use gui_primes::primes::{PrimeType, compute_with_memo};

// Helper function to run computations
fn run_calculation(kind: PrimeType, start: u64, end: u64) -> Vec<String> {
    compute_with_memo(kind, start, end)
}

// Test for zero and one handling
#[test]
fn test_zero_and_one() {
    // Test with a range including 0 and 1
    let range_with_non_primes = run_calculation(PrimeType::Mersenne, 0, 5);
    
    // 0 and 1 should not be in the results
    assert!(!range_with_non_primes.contains(&"0".to_string()), "0 should not be considered prime");
    assert!(!range_with_non_primes.contains(&"1".to_string()), "1 should not be considered prime");
    
    // But the range should contain valid primes
    assert!(range_with_non_primes.contains(&"3".to_string()), "3 should be identified as a Mersenne prime");
}

// Test for empty range handling
#[test]
fn test_empty_ranges() {
    // Empty range (start == end)
    let empty_range = run_calculation(PrimeType::Twin, 10, 10);
    assert!(empty_range.is_empty(), "Empty range should produce no results");
    
    // Another empty range
    let empty_range = run_calculation(PrimeType::Safe, 100, 100);
    assert!(empty_range.is_empty(), "Empty range should produce no results");
}

// Test for inverted range handling
#[test]
fn test_inverted_ranges() {
    // Test with start > end
    let normal_order = run_calculation(PrimeType::Palindromic, 10, 50);
    let inverted_order = run_calculation(PrimeType::Palindromic, 50, 10);
    
    // Results should be identical regardless of range order
    assert_eq!(normal_order, inverted_order, "Results should be identical regardless of range order");
    
    // Test with a larger range
    let normal_large = run_calculation(PrimeType::Twin, 100, 200);
    let inverted_large = run_calculation(PrimeType::Twin, 200, 100);
    
    assert_eq!(normal_large, inverted_large, "Results should be identical for large inverted ranges");
}

// Test for single element range
#[test]
fn test_single_element_ranges() {
    // Range with just one number that is prime
    let single_prime = run_calculation(PrimeType::Palindromic, 11, 12);
    assert!(!single_prime.is_empty(), "Single element range should find prime 11");
    assert!(single_prime.contains(&"11".to_string()), "Single element range should find prime 11");
    
    // Range with just one number that is not prime
    let single_non_prime = run_calculation(PrimeType::Palindromic, 12, 13);
    assert!(single_non_prime.is_empty(), "Single element range with non-prime should be empty");
}

// Test for very large values
#[test]
fn test_large_values() {
    // Test with values near u64::MAX for safe prime types
    let high_range = run_calculation(PrimeType::Palindromic, u64::MAX - 100, u64::MAX);
    
    // We're not asserting on the content here, just that it completes without overflow
    println!("Found {} palindromic primes near u64::MAX", high_range.len());
    
    // Test with large but reasonable values
    let large_range = run_calculation(PrimeType::Twin, 1_000_000, 1_000_010);
    println!("Found {} twin primes in large range", large_range.len());
}

// Test for primes with special constraints
#[test]
fn test_constrained_prime_families() {
    // Wilson primes are very rare and have a limit in the implementation
    let wilson_primes = run_calculation(PrimeType::Wilson, 2, 50);
    assert!(!wilson_primes.is_empty(), "Should find at least one Wilson prime");
    
    // Mersenne primes have an upper limit to prevent overflow
    let mersenne_primes_safe = run_calculation(PrimeType::Mersenne, 2, 63);
    assert!(!mersenne_primes_safe.is_empty(), "Should find Mersenne primes in safe range");
    
    // Beyond the limit, should return empty
    let mersenne_primes_unsafe = run_calculation(PrimeType::Mersenne, 64, 70);
    assert!(mersenne_primes_unsafe.is_empty(), "Should not find Mersenne primes beyond safe range");
}

// Test for specific prime values across families
#[test]
fn test_specific_prime_values() {
    // Test small prime 5 across different families
    let twin_primes = run_calculation(PrimeType::Twin, 3, 6);
    let palindromic_primes = run_calculation(PrimeType::Palindromic, 3, 6);
    
    // 5 should be part of a twin prime pair (3,5)
    assert!(twin_primes.iter().any(|p| p.contains("5")), "5 should be part of twin prime pair");
    
    // 5 should be a palindromic prime
    assert!(palindromic_primes.contains(&"5".to_string()), "5 should be a palindromic prime");
}

// Test for non-prime numbers in various families
#[test]
fn test_non_prime_handling() {
    // Test with range containing non-primes
    let range = run_calculation(PrimeType::Palindromic, 10, 15);
    
    // 10, 12, 14 are not prime
    assert!(!range.contains(&"10".to_string()), "10 should not be in results");
    assert!(!range.contains(&"12".to_string()), "12 should not be in results");
    assert!(!range.contains(&"14".to_string()), "14 should not be in results");
    
    // 11 is prime and palindromic
    assert!(range.contains(&"11".to_string()), "11 should be in results");
}

// Test for extreme cases with potentially problematic inputs
#[test]
#[ignore] // This test is resource-intensive
fn test_extreme_inputs() {
    // Test with potentially problematic inputs
    
    // Very large range
    let large_range_size = run_calculation(PrimeType::Twin, 1_000_000, 1_000_100);
    println!("Found {} twin primes in large range", large_range_size.len());
    
    // Range where both start and end are already prime
    let both_prime = run_calculation(PrimeType::Palindromic, 11, 101);
    assert!(!both_prime.is_empty(), "Should find primes when both range ends are prime");
    assert!(both_prime.contains(&"11".to_string()), "11 should be in results");
    assert!(both_prime.contains(&"101".to_string()), "101 should be in results");
}

