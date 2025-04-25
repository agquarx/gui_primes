// Integration tests for the gui_primes library
// These tests verify the library works correctly from an external perspective

use gui_primes::primes::{PrimeType, compute_with_memo};

// Test basic prime calculation across different families
#[test]
fn test_prime_calculation() {
    // Test regular primes in different ranges
    let primes = compute_with_memo(PrimeType::Regular, 10, 20);
    assert!(primes.contains(&"11".to_string()));
    assert!(primes.contains(&"13".to_string()));
    assert!(primes.contains(&"17".to_string()));
    assert!(primes.contains(&"19".to_string()));
    
    // Test twin primes
    let twin_primes = compute_with_memo(PrimeType::Twin, 10, 20);
    assert!(twin_primes.contains(&"(11,13)".to_string()));
    assert!(twin_primes.contains(&"(17,19)".to_string()));
}

// Test range swapping functionality
#[test]
fn test_range_swapping() {
    let forward_range = compute_with_memo(PrimeType::Regular, 5, 15);
    let backward_range = compute_with_memo(PrimeType::Regular, 15, 5);
    
    assert_eq!(forward_range, backward_range, "Swapped ranges should produce identical results");
}

// Test memoization across multiple calls
#[test]
fn test_memoization() {
    // First call should compute and cache
    let first_call = compute_with_memo(PrimeType::Palindromic, 10, 100);
    
    // Second call with same parameters should use cached results
    let second_call = compute_with_memo(PrimeType::Palindromic, 10, 100);
    
    assert_eq!(first_call, second_call, "Memoized results should be identical");
    
    // Call with different parameters should return different results
    let different_range = compute_with_memo(PrimeType::Palindromic, 100, 200);
    assert_ne!(first_call, different_range, "Different ranges should produce different results");
}

// Test large ranges
#[test]
fn test_large_ranges() {
    // This test verifies the library can handle reasonably large ranges
    let large_range = compute_with_memo(PrimeType::Regular, 1000, 1100);
    
    // There should be several primes in this range
    assert!(!large_range.is_empty(), "Large range should contain primes");
}

// Test handling of edge cases
#[test]
fn test_edge_cases() {
    // Empty range (start == end)
    let empty_range = compute_with_memo(PrimeType::Regular, 10, 10);
    assert!(empty_range.is_empty(), "Empty range should produce no results");
    
    // Range with only composite numbers
    let composite_only = compute_with_memo(PrimeType::Regular, 24, 28);
    assert!(composite_only.is_empty(), "Range with only composite numbers should return empty result");
}
