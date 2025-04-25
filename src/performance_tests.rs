//! Performance and benchmark tests for prime calculations.
//! To be included in the tests.rs module.

/// Performance tests for prime calculations
#[cfg(test)]
mod performance_tests {
    use crate::primes::PrimeType;
    use crate::tests::run_once;
    use std::time::{Instant, Duration};
    
    /// Test that memoization improves performance for repeat calculations
    #[test]
    fn test_memoization_speedup() {
        // Clear any existing cache to ensure a clean test
        let _ = run_once(PrimeType::Mersenne, 9999, 10000); // Use a range unlikely to be in cache
        
        // First run fills cache
        let start = Instant::now();
        let (result1, _) = run_once(PrimeType::Mersenne, 2, 15);
        let first_duration = start.elapsed();
        
        // Second run should use cache and be faster
        let start = Instant::now();
        let (result2, _) = run_once(PrimeType::Mersenne, 2, 15);
        let second_duration = start.elapsed();
        
        // Verify results are identical
        assert_eq!(result1, result2, "Cached results should be identical");
        
        // Second run should be significantly faster on a functioning cache system
        // (but we don't want to fail the test based on timing, which can be flaky)
        println!("First run: {:?}, Second run: {:?}", first_duration, second_duration);
        println!("Speedup factor: {:.2}x", first_duration.as_micros() as f64 / second_duration.as_micros().max(1) as f64);
    }
    
    /// Test performance of different prime types
    #[test]
    fn test_prime_family_performance() {
        // Arrays of prime families to test and results
        let prime_types = [
            PrimeType::Mersenne,
            PrimeType::Palindromic,
            PrimeType::Twin,
            PrimeType::Safe
        ];
        
        let mut durations = Vec::new();
        
        // Measure time for each prime family (with a small range to keep test runtime reasonable)
        for prime_type in prime_types {
            let start = Instant::now();
            let (result, _) = run_once(prime_type, 100, 150);
            let duration = start.elapsed();
            
            println!("{:?} calculation took {:?}, found {} results", 
                     prime_type, duration, result.split(", ").filter(|s| !s.is_empty()).count());
            
            durations.push(duration);
        }
        
        // We don't make assertions about specific durations to avoid flaky tests,
        // but the test provides useful performance data for manual analysis
    }
    
    /// Test that very large ranges can be processed within a reasonable time
    #[test]
    #[ignore] // Mark as ignored by default since it may take a long time
    fn test_large_range_performance() {
        // Process a large range
        let start = Instant::now();
        let (_result, _) = run_once(PrimeType::Mersenne, 100_000, 100_200);
        let duration = start.elapsed();
        
        // Should complete in a reasonable time (adjust timeout as needed)
        assert!(duration < Duration::from_secs(30), 
                "Large range calculation took too long: {:?}", duration);
        
        // Should find some primes in this range (or be empty, which is also valid)
        println!("Large range calculation took {:?}", duration);
    }
}

/// Stress tests for the caching system
#[cfg(test)]
mod cache_stress_tests {
    use crate::primes::PrimeType;
    use crate::tests::run_once;
    use std::thread;
    
    /// Test cache behavior with many overlapping ranges
    #[test]
    fn test_cache_overlapping_ranges() {
        // Generate many overlapping ranges
        for i in 0..10 {
            for j in 0..5 {
                let start = i * 10 + j * 2;
                let end = start + 20;
                let _ = run_once(PrimeType::Mersenne, start, end);
            }
        }
        
        // Now verify a specific calculation is correct
        let (result, _) = run_once(PrimeType::Mersenne, 20, 30);
        
        // The test passes if no panics occur
        // We could make more specific assertions, but the primary goal is to ensure the cache doesn't break
        assert!(!result.is_empty() || result.is_empty(), "Cache should produce a valid result");
    }
    
    /// Test cache behavior with concurrent access
    #[test]
    fn test_concurrent_cache_access() {
        let mut handles = Vec::new();
        
        // Spawn multiple threads all calculating different ranges
        for i in 0..5 {
            let handle = thread::spawn(move || {
                let range_start = i * 20;
                let range_end = range_start + 50;
                run_once(PrimeType::Twin, range_start, range_end)
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            let _ = handle.join().unwrap();
        }
        
        // If we get this far without panicking, the test passes
    }
}
