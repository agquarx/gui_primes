// Integration tests for the gui_primes library
// These tests verify the library works correctly from an external perspective

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use eframe::egui;
use std::thread;
use std::time::Duration;

// We need to import the actual crate, not using 'gui_primes::'
use gui_primes::primes::{PrimeType, compute_with_memo};

// Helper function to run computations synchronously (similar to the one in src/tests.rs)
fn run_calculation(kind: PrimeType, start: u64, end: u64) -> Vec<String> {
    compute_with_memo(kind, start, end)
}

// Test basic prime calculation across different families
#[test]
fn test_prime_calculation() {
    // Test Mersenne primes in different ranges
    let primes = run_calculation(PrimeType::Mersenne, 2, 6);
    assert!(primes.contains(&"3".to_string()));
    assert!(primes.contains(&"7".to_string()));
    assert!(primes.contains(&"31".to_string()));
    
    // Test twin primes
    let twin_primes = run_calculation(PrimeType::Twin, 10, 20);
    assert!(twin_primes.contains(&"(11,13)".to_string()));
    assert!(twin_primes.contains(&"(17,19)".to_string()));
}

// Test range swapping functionality
#[test]
fn test_range_swapping() {
    let forward_range = run_calculation(PrimeType::Mersenne, 2, 6);
    let backward_range = run_calculation(PrimeType::Mersenne, 6, 2);
    
    assert_eq!(forward_range, backward_range, "Swapped ranges should produce identical results");
}

// Test memoization across multiple calls
#[test]
fn test_memoization() {
    // First call should compute and cache
    let first_call = run_calculation(PrimeType::Palindromic, 10, 100);
    
    // Second call with same parameters should use cached results
    let second_call = run_calculation(PrimeType::Palindromic, 10, 100);
    
    assert_eq!(first_call, second_call, "Memoized results should be identical");
    
    // Call with different parameters should return different results
    let different_range = run_calculation(PrimeType::Palindromic, 100, 200);
    assert_ne!(first_call, different_range, "Different ranges should produce different results");
}

// Test large ranges
#[test]
fn test_large_ranges() {
    // This test verifies the library can handle reasonably large ranges
    let large_range = run_calculation(PrimeType::Mersenne, 20, 30);
    
    // There should be primes in this range
    assert!(!large_range.is_empty(), "Large range should contain primes");
}

// Test handling of edge cases
#[test]
fn test_edge_cases() {
    // Empty range (start == end)
    let empty_range = run_calculation(PrimeType::Mersenne, 10, 10);
    assert!(empty_range.is_empty(), "Empty range should produce no results");
    
    // Range including 0 and 1 should skip them
    let range_with_non_primes = run_calculation(PrimeType::Mersenne, 0, 4);
    assert!(!range_with_non_primes.contains(&"0".to_string()), "0 should not be considered prime");
    assert!(!range_with_non_primes.contains(&"1".to_string()), "1 should not be considered prime");
    assert!(range_with_non_primes.contains(&"3".to_string()), "3 should be identified as a Mersenne prime");
}
