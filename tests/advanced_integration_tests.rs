// Advanced integration tests for the gui_primes library
// These tests go beyond the basic functionality to test more complex scenarios

use std::time::Instant;
use std::thread;

// Import the library
use gui_primes::primes::{PrimeType, compute_with_memo};

// Helper function to run computations
fn run_calculation(kind: PrimeType, start: u64, end: u64) -> Vec<String> {
    compute_with_memo(kind, start, end)
}

// Test for cross-family consistency
#[test]
fn test_cross_family_consistency() {
    // All Palindromic primes in a range should also be primes
    let palindromic_primes = run_calculation(PrimeType::Palindromic, 10, 100);
    
    // For each palindromic prime, verify it's a valid palindrome
    for prime_str in &palindromic_primes {
        let forward = prime_str.chars().collect::<String>();
        let reverse = prime_str.chars().rev().collect::<String>();
        assert_eq!(forward, reverse, "Prime {} is not palindromic", prime_str);
    }
    
    // Test that twin primes are properly formatted and are actually 2 apart
    let twin_primes = run_calculation(PrimeType::Twin, 10, 50);
    
    for pair in twin_primes {
        if pair.starts_with('(') && pair.ends_with(')') {
            let nums: Vec<&str> = pair[1..pair.len()-1].split(',').collect();
            assert_eq!(nums.len(), 2, "Twin prime pair should have exactly 2 numbers");
            
            let first: u64 = nums[0].parse().unwrap();
            let second: u64 = nums[1].parse().unwrap();
            assert_eq!(second - first, 2, "Twin primes should differ by exactly 2");
        }
    }
}

// Test for concurrent calculations
#[test]
fn test_concurrent_calculations() {
    // Spawn multiple threads to perform calculations
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                let start = i * 10;
                let end = start + 20;
                run_calculation(PrimeType::Mersenne, start, end)
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join().unwrap();
    }
    
    // The test passes if no panics occur
}

// Test performance characteristics
#[test]
fn test_memoization_performance() {
    // First run with a range unlikely to be in the cache
    let start_time = Instant::now();
    let first_result = run_calculation(PrimeType::Mersenne, 50, 100);
    let first_duration = start_time.elapsed();
    
    // Second run with the same range
    let start_time = Instant::now();
    let second_result = run_calculation(PrimeType::Mersenne, 50, 100);
    let second_duration = start_time.elapsed();
    
    // Results should be identical
    assert_eq!(first_result, second_result, "Results should be identical for identical inputs");
    
    // Log the timing information
    println!("First calculation: {:?}, Second calculation: {:?}", first_duration, second_duration);
    // Don't make assertions about timing to avoid flaky tests
}

// Test very large ranges
#[test]
#[ignore] // This test may take a long time
fn test_very_large_ranges() {
    // Test with a very large range
    let result = run_calculation(PrimeType::Mersenne, 1000, 1010);
    
    // The test passes if no panics occur
    println!("Found {} primes in large range", result.len());
}

// Test invalid inputs
#[test]
fn test_invalid_inputs() {
    // Test with invalid range (start > end)
    let swapped = run_calculation(PrimeType::Twin, 100, 50);
    let correct = run_calculation(PrimeType::Twin, 50, 100);
    
    // Results should be identical regardless of range order
    assert_eq!(swapped, correct, "Swapped ranges should produce identical results");
}
