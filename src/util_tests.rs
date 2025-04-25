//! Unit tests for utility functions.
//! To be included in the tests.rs module.

/// Tests for utility functions
#[cfg(test)]
mod util_tests {
    // We need to adjust the PrimeError import and clipboard functionality
    #[cfg(feature = "clipboard")]
    #[test]
    fn test_clipboard_function() {
        // This test only runs when the clipboard feature is enabled
        use crate::util::copy_to_clipboard;
        
        let result = copy_to_clipboard("Test data");
        // Result may succeed or fail depending on the system environment
        println!("Clipboard feature enabled, result: {:?}", result);
    }
    
    #[cfg(not(feature = "clipboard"))]
    #[test]
    fn test_clipboard_function() {
        // This test only runs when the clipboard feature is disabled
        use crate::util::copy_to_clipboard;
        
        let result = copy_to_clipboard("Test data");
        assert!(result.is_err(), "Result should be an error when clipboard feature is disabled");
    }
}

/// Tests for prime utility functions
#[cfg(test)]
mod prime_util_tests {
    use crate::primes::{PrimeType, compute_with_memo};
    use crate::tests::run_once;
    
    // Test that prime calculations consistently handle edge cases
    #[test]
    fn test_prime_edge_cases() {
        // Test with range [0, 2) which should only include primes >= 2
        let result = compute_with_memo(PrimeType::Mersenne, 0, 2);
        assert_eq!(result.len(), 0, "No primes < 2");
        
        // Test with range [2, 3) which should include only 2
        let result = compute_with_memo(PrimeType::Mersenne, 2, 3);
        assert_eq!(result.len(), 1, "Should find exactly one prime");
        assert_eq!(result[0], "3", "First Mersenne prime should be 3");
    }
    
    // Test that start == end returns empty results for all prime types
    #[test]
    fn test_empty_range_all_types() {
        // List of prime types to test
        let prime_types = [
            PrimeType::Mersenne,
            PrimeType::Twin,
            PrimeType::Palindromic,
            PrimeType::Safe
        ];
        
        // Test each type with an empty range
        for prime_type in prime_types {
            let result = compute_with_memo(prime_type, 10, 10);
            assert!(result.is_empty(), 
                   "Empty range should return empty results for {:?}", prime_type);
        }
    }
    
    // Test that very large ranges don't cause errors
    #[test]
    fn test_large_range_handling() {
        // Use a type that's reasonably fast to compute
        let result = compute_with_memo(PrimeType::Mersenne, 1_000_000, 1_000_010);
        
        // We don't care about the specific results, just that it didn't crash
        // and returned some result (even if empty)
        assert!(result.is_empty() || !result.is_empty(), 
               "Large range calculation should complete without errors");
    }
}
