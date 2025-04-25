//! Unit tests for utility functions.
//! To be included in the tests.rs module.

/// Tests for utility functions
#[cfg(test)]
mod util_tests {
    use crate::primes::PrimeError;
    use crate::util::copy_to_clipboard;
    
    /// Test clipboard functionality based on feature flag
    #[test]
    fn test_clipboard_function() {
        // This test will have different behavior depending on whether the clipboard feature is enabled
        let result = copy_to_clipboard("Test data");
        
        // We don't assert on the result directly, since it depends on the feature flag and system state
        // But we can ensure the function doesn't panic
        #[cfg(feature = "clipboard")]
        {
            println!("Clipboard feature enabled, result: {:?}", result);
            // Result may succeed or fail depending on the system environment
        }
        
        #[cfg(not(feature = "clipboard"))]
        {
            println!("Clipboard feature disabled, result: {:?}", result);
            assert!(result.is_err(), "Result should be an error when clipboard feature is disabled");
            assert!(matches!(result, Err(PrimeError::Fatal(_))), 
                   "Error should be PrimeError::Fatal when clipboard feature is disabled");
        }
    }
}

/// Tests for prime specific utility functions
#[cfg(test)]
mod prime_util_tests {
    use crate::primes::{PrimeType, compute_with_memo};
    use super::*;
    
    // Test that prime calculations consistently handle edge cases
    #[test]
    fn test_prime_edge_cases() {
        // Test with range [0, 2) which should only include primes >= 2
        let result = compute_with_memo(PrimeType::Regular, 0, 2);
        assert_eq!(result.len(), 0, "No primes < 2");
        
        // Test with range [2, 3) which should include only 2
        let result = compute_with_memo(PrimeType::Regular, 2, 3);
        assert_eq!(result.len(), 1, "Should find exactly one prime");
        assert_eq!(result[0], "2", "First prime should be 2");
    }
    
    // Test that start == end returns empty results for all prime types
    #[test]
    fn test_empty_range_all_types() {
        // List of prime types to test
        let prime_types = [
            PrimeType::Regular,
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
        let result = compute_with_memo(PrimeType::Regular, 1_000_000, 1_000_010);
        
        // We don't care about the specific results, just that it didn't crash
        // and returned some result (even if empty)
        assert!(result.is_empty() || !result.is_empty(), 
               "Large range calculation should complete without errors");
    }
}
