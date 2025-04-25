// Advanced integration tests for the gui_primes library
// These tests go beyond the basic functionality to test more complex scenarios

use gui_primes::primes::{PrimeType, compute_with_memo};

// Test for cross-family consistency
#[test]
fn test_regular_vs_specialized_primes() {
    // All regular primes in a range
    let regular_primes = compute_with_memo(PrimeType::Regular, 10, 20);
    
    // Individual prime checks from other families should be consistent with regular primes
    // For example, a palindromic prime must also be a regular prime
    let palindromic_primes = compute_with_memo(PrimeType::Palindromic, 10, 20);
    
    // Check that each palindromic prime also exists in the regular primes
    for prime_str in palindromic_primes {
        assert!(regular_primes.contains(&prime_str), 
                "Prime {} is palindromic but not found in regular primes", prime_str);
    }
}

// Test for extremely large prime calculations
#[test]
fn test_large_primes() {
    // Calculate some large primes (this tests handling of large numbers)
    let large_primes = compute_with_memo(PrimeType::Regular, 9000, 9050);
    
    // There should be primes in this range
    assert!(!large_primes.is_empty(), "No primes found in range 9000-9050");
    
    // The first prime after 9000 should be 9001
    assert!(large_primes.contains(&"9001".to_string()), "Prime 9001 not found");
    
    // 9007, 9011, 9013, 9029, 9041, 9043, 9049 are known primes in this range
    assert!(large_primes.contains(&"9007".to_string()), "Prime 9007 not found");
    assert!(large_primes.contains(&"9011".to_string()), "Prime 9011 not found");
}

// Test resilience of the caching system under repeated operations
#[test]
fn test_cache_stress() {
    // Make a series of overlapping range requests to stress the caching system
    for i in 0..5 {
        let start = i * 10;
        let end = start + 20;
        let _ = compute_with_memo(PrimeType::Regular, start, end);
    }
    
    // Now make a request that should leverage parts of previous cached results
    let combined = compute_with_memo(PrimeType::Regular, 0, 50);
    
    // Verify we get at least the expected primes
    assert!(combined.contains(&"2".to_string()), "Prime 2 not found");
    assert!(combined.contains(&"3".to_string()), "Prime 3 not found");
    assert!(combined.contains(&"5".to_string()), "Prime 5 not found");
    assert!(combined.contains(&"7".to_string()), "Prime 7 not found");
    assert!(combined.contains(&"11".to_string()), "Prime 11 not found");
    assert!(combined.contains(&"13".to_string()), "Prime 13 not found");
    assert!(combined.contains(&"17".to_string()), "Prime 17 not found");
    assert!(combined.contains(&"19".to_string()), "Prime 19 not found");
    assert!(combined.contains(&"23".to_string()), "Prime 23 not found");
    assert!(combined.contains(&"29".to_string()), "Prime 29 not found");
    assert!(combined.contains(&"31".to_string()), "Prime 31 not found");
    assert!(combined.contains(&"37".to_string()), "Prime 37 not found");
    assert!(combined.contains(&"41".to_string()), "Prime 41 not found");
    assert!(combined.contains(&"43".to_string()), "Prime 43 not found");
    assert!(combined.contains(&"47".to_string()), "Prime 47 not found");
}
