//! Unit tests for error handling and edge cases.
//! To be included in the tests.rs module.

/// Tests for edge case handling in prime calculations
#[cfg(test)]
mod error_tests {
    use crate::primes::{PrimeType};
    use crate::tests::{run_once};

    /// Test that extremely large ranges don't cause crashes
    #[test]
    fn test_extremely_large_range() {
        // A range that's too large to process efficiently but shouldn't crash
        let result = run_once(PrimeType::Mersenne, 1, u64::MAX / 2);
        // We don't care about the actual results, just that it didn't crash
        // Progress should complete
        assert_eq!(result.1, 1.0, "Progress should complete");
    }

    /// Test handling of invalid ranges
    #[test]
    fn test_invalid_ranges() {
        // Test with zero as start
        let (zero_start, _) = run_once(PrimeType::Mersenne, 0, 10);
        assert!(!zero_start.contains("0"), "Zero should not be considered prime");
        
        // Test with one as start
        let (one_start, _) = run_once(PrimeType::Mersenne, 1, 10);
        assert!(!one_start.contains("1"), "One should not be considered prime");
    }

    /// Test concurrent access to cache
    #[test]
    fn test_concurrent_cache_access() {
        use std::thread;
        
        // Spawn multiple threads to access the cache concurrently
        let mut handles = vec![];
        
        for i in 0..5 {
            let handle = thread::spawn(move || {
                let start = i * 10;
                let end = start + 20;
                run_once(PrimeType::Mersenne, start, end)
            });
            handles.push(handle);
        }
        
        // Join all threads and ensure they all completed successfully
        for handle in handles {
            let _ = handle.join().unwrap();
        }
        
        // If we got here without panicking, the test passes
    }
}

/// Tests for interoperability between prime families
#[cfg(test)]
mod interop_tests {
    use crate::primes::{PrimeType};
    use crate::tests::{run_once};

    /// Test that prime numbers in one family are consistent with properties of that family
    #[test]
    fn test_prime_family_consistency() {
        // Test that every palindromic prime is actually palindromic
        let (palindromic_primes, _) = run_once(PrimeType::Palindromic, 10, 200);
        
        for prime in palindromic_primes.split(", ") {
            if !prime.is_empty() {
                let forward = prime.chars().collect::<String>();
                let reverse = prime.chars().rev().collect::<String>();
                assert_eq!(forward, reverse, "Prime {prime} is not palindromic");
            }
        }
        
        // Test that twin primes are actually pairs that differ by 2
        let (twin_primes, _) = run_once(PrimeType::Twin, 10, 50);
        
        for pair in twin_primes.split(", ") {
            if !pair.is_empty() && pair.starts_with('(') && pair.ends_with(')') {
                let nums: Vec<&str> = pair[1..pair.len()-1].split(',').collect();
                assert_eq!(nums.len(), 2, "Twin prime pair should have exactly 2 numbers");
                
                let first: u64 = nums[0].parse().unwrap();
                let second: u64 = nums[1].parse().unwrap();
                assert_eq!(second - first, 2, "Twin primes should differ by exactly 2");
            }
        }
    }

    /// Test that every prime family includes only actual prime numbers
    #[test]
    fn test_all_families_contain_primes() {
        // Get a reference list of palindromic primes (which are all primes)
        let (ref_primes_str, _) = run_once(PrimeType::Palindromic, 10, 50);
        let ref_primes: Vec<u64> = ref_primes_str
            .split(", ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        
        // Helper function to check if a number is in the list of reference primes
        let is_prime = |n: u64| ref_primes.contains(&n);
        
        // Test various prime families to ensure all numbers they contain are prime
        let family_types = [
            PrimeType::Palindromic,
            PrimeType::Safe,
            PrimeType::Sexy,
            PrimeType::Cousin
        ];
        
        for family in family_types {
            let (family_primes, _) = run_once(family, 10, 50);
            
            for prime_str in family_primes.split(", ") {
                if prime_str.is_empty() {
                    continue;
                }
                
                // Handle pair notations like (11,13) for twin primes
                if prime_str.starts_with('(') && prime_str.ends_with(')') {
                    let nums: Vec<&str> = prime_str[1..prime_str.len()-1].split(',').collect();
                    for num in nums {
                        let n: u64 = num.parse().unwrap();
                        assert!(is_prime(n), "Number {n} in {family:?} family is not prime");
                    }
                } else {
                    // Handle single number notation
                    let n: u64 = prime_str.parse().unwrap();
                    assert!(is_prime(n), "Number {n} in {family:?} family is not prime");
                }
            }
        }
    }
}

/// Tests for stop signal handling
#[cfg(test)]
mod stop_signal_tests {
    use crate::primes::{PrimeType};
    use crate::tests::{spawn_worker};
    use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
    use std::thread;
    use std::time::Duration;
    use eframe::egui;
    
    /// Test that the stop signal is properly handled
    #[test]
    fn test_stop_signal() {
        let out = Arc::new(Mutex::new(String::new()));
        let prog = Arc::new(Mutex::new(0.0));
        let stop = Arc::new(AtomicBool::new(false));
        let egui_ctx = egui::Context::default();
        
        // Start a calculation with a large range that will take some time
        spawn_worker(
            PrimeType::Mersenne, 1000, 10000,
            stop.clone(), out.clone(), prog.clone(), egui_ctx);
        
        // Let it run for a short time
        thread::sleep(Duration::from_millis(100));
        
        // Set the stop signal
        stop.store(true, Ordering::SeqCst);
        
        // Wait for it to respond to the stop signal
        for _ in 0..50 {
            if out.lock().unwrap().contains("stopped") {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        
        // Verify that the operation was stopped
        assert!(out.lock().unwrap().contains("stopped"), "Worker did not respond to stop signal");
    }
}
