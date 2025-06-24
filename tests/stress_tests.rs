// Stress and performance tests for the gui_primes library
// These tests evaluate how the library performs under more demanding conditions

use std::time::Instant;
use std::collections::HashSet;
use gui_primes::primes::{PrimeType, compute_with_memo, cache_stats};

// Helper function to run computations
fn run_calculation(kind: PrimeType, start: u64, end: u64) -> Vec<String> {
    compute_with_memo(kind, start, end)
}

// Test caching behavior with sequential calculations
#[test]
fn test_cache_sequential_requests() {
    // Record the initial cache size
    let initial_cache_size = cache_stats();
    
    // First run on a range that is likely not in cache
    let calculation_type = PrimeType::Palindromic;
    let range_start = 1000;
    let range_end = 1100;
    
    // First run should compute and cache the result
    let start_time = Instant::now();
    let first_result = run_calculation(calculation_type, range_start, range_end);
    let first_duration = start_time.elapsed();
    
    // Check that the cache size increased
    let cache_size_after_first = cache_stats();
    assert!(cache_size_after_first > initial_cache_size, 
            "Cache size should increase after first calculation");
    
    // Second run with the exact same parameters should use the cached result
    let start_time = Instant::now();
    let second_result = run_calculation(calculation_type, range_start, range_end);
    let second_duration = start_time.elapsed();
    
    // Results should be identical
    assert_eq!(first_result, second_result, 
              "Results should be identical for repeated calculations");
    
    // Cache size should not increase again
    let cache_size_after_second = cache_stats();
    assert_eq!(cache_size_after_first, cache_size_after_second, 
              "Cache size should not increase for repeated calculations");
    
    println!("First calculation: {:?}, Second calculation: {:?}", first_duration, second_duration);
}

// Test for cache behavior with many different types of primes
#[test]
fn test_cache_diverse_prime_families() {
    let initial_cache_size = cache_stats();
    
    // Define a set of prime families to test
    let prime_families = [
        PrimeType::Mersenne,
        PrimeType::SophieGermain,
        PrimeType::Twin,
        PrimeType::Palindromic,
        PrimeType::Sexy,
        PrimeType::Cousin,
        PrimeType::Emirp,
        PrimeType::Safe,
        PrimeType::Circular,
    ];
    
    // Use a range that we know will have results for all prime types
    // Starting at 10 to catch more palindromic primes
    let range_start = 10;
    let range_end = 100;
    
    // Run calculations for each prime family
    for &family in &prime_families {
        let result = run_calculation(family, range_start, range_end);
        assert!(!result.is_empty(), "Result for {:?} should not be empty", family);
    }
    
    // Cache should have entries for each prime family
    let cache_size_after = cache_stats();
    let expected_increase = prime_families.len();
    assert!(cache_size_after >= initial_cache_size + expected_increase, 
            "Cache should grow by at least {} entries", expected_increase);
}

// Test for cache behavior with overlapping ranges
#[test]
fn test_cache_overlapping_ranges() {
    let initial_cache_size = cache_stats();
    
    // Define a set of overlapping ranges
    let ranges = [
        (100, 200),
        (150, 250),
        (50, 150),
        (200, 300),
    ];
    
    let prime_type = PrimeType::Twin;
    
    // First, calculate all ranges independently
    for &(start, end) in &ranges {
        let result = run_calculation(prime_type, start, end);
        assert!(!result.is_empty() || start == end, 
                "Result for range {}-{} should not be empty unless start=end", start, end);
    }
    
    // Cache should have grown by the number of ranges
    let cache_size_after = cache_stats();
    let expected_increase = ranges.len();
    assert!(cache_size_after >= initial_cache_size + expected_increase, 
            "Cache should grow by at least {} entries", expected_increase);
}

// Test for consistency across multiple ranges
#[test]
fn test_consistency_across_ranges() {
    // Calculate primes in several overlapping ranges
    let prime_type = PrimeType::Palindromic;
    
    // Using smaller ranges with more guaranteed palindromic primes
    let full_range = run_calculation(prime_type, 10, 300);
    let first_half = run_calculation(prime_type, 10, 150);
    let second_half = run_calculation(prime_type, 150, 300);
    let middle_section = run_calculation(prime_type, 100, 200);
    
    // Convert to sets for easier comparison
    let full_set: HashSet<_> = full_range.into_iter().collect();
    let first_half_set: HashSet<_> = first_half.into_iter().collect();
    let second_half_set: HashSet<_> = second_half.into_iter().collect();
    let middle_section_set: HashSet<_> = middle_section.into_iter().collect();
    
    // First half and second half should be disjoint
    assert!(first_half_set.is_disjoint(&second_half_set), 
           "First half and second half should be disjoint");
    
    // Middle section should overlap with both first and second halves
    assert!(!middle_section_set.is_disjoint(&first_half_set), 
           "Middle section should overlap with first half");
    assert!(!middle_section_set.is_disjoint(&second_half_set), 
           "Middle section should overlap with second half");
    
    // Union of first and second halves should equal the full range
    let combined_set: HashSet<_> = first_half_set.union(&second_half_set).cloned().collect();
    assert_eq!(combined_set, full_set, 
              "Union of first and second halves should equal the full range");
}

// Test for large number of small ranges (stress test)
#[test]
#[ignore] // This test may be slow
fn test_many_small_ranges() {
    let initial_cache_size = cache_stats();
    
    // Generate many small non-overlapping ranges
    let prime_type = PrimeType::Twin;
    let num_ranges: usize = 20;
    let range_size = 10;
    
    for i in 0..num_ranges {
        let start = (i * range_size) as u64;
        let end = start + range_size as u64;
        let result = run_calculation(prime_type, start, end);
        
        // We don't assert on the content, just that it completes without error
        println!("Range {}-{}: Found {} primes", start, end, result.len());
    }
    
    // Cache should have grown by the number of ranges
    let cache_size_after = cache_stats();
    assert!(cache_size_after >= initial_cache_size + num_ranges, 
            "Cache should grow by at least {} entries", num_ranges);
}

// Test for cache effectiveness on repeated calculations
#[test]
fn test_cache_effectiveness() {
    let calculation_type = PrimeType::Cuban;
    let range_start = 100;
    let range_end = 200;
    
    // First run - should compute and cache
    let start_time = Instant::now();
    let first_result = run_calculation(calculation_type, range_start, range_end);
    let first_duration = start_time.elapsed();
    
    // Second run - should use cache
    let start_time = Instant::now();
    let second_result = run_calculation(calculation_type, range_start, range_end);
    let second_duration = start_time.elapsed();
    
    // Results should be identical
    assert_eq!(first_result, second_result, "Results should be identical");
    
    // Second run should generally be faster, though we can't guarantee this
    // in a test as it could be affected by system load
    println!("First run: {:?}, Second run: {:?}", first_duration, second_duration);
    
    // Verify the cache actually contains our query
    let cache_size = cache_stats();
    assert!(cache_size > 0, "Cache should not be empty after calculations");
}

// Test for very large ranges with heavy computation
#[test]
#[ignore] // This test is slow and resource-intensive
fn test_large_computation() {
    // This test might take a long time to run
    let prime_type = PrimeType::Circular;
    let range_start = 1000;
    let range_end = 2000;
    
    let start_time = Instant::now();
    let result = run_calculation(prime_type, range_start, range_end);
    let duration = start_time.elapsed();
    
    println!("Large computation took {:?} and found {} primes", duration, result.len());
}

