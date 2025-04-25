//! Unit tests for GUI functionality.
//! To be included in the tests.rs module.

/// Tests for UI state management
#[cfg(test)]
mod gui_tests {
    use crate::primes::PrimeType;
    use super::*;
    use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
    use std::thread;
    use std::time::Duration;
    use eframe::egui;
    
    /// Test progress bar updates
    #[test]
    fn test_progress_updates() {
        let out = Arc::new(Mutex::new(String::new()));
        let prog = Arc::new(Mutex::new(0.0));
        let stop = Arc::new(AtomicBool::new(false));
        let egui_ctx = egui::Context::default();
        
        // Start a calculation
        spawn_worker(
            PrimeType::Regular, 10, 50,
            stop.clone(), out.clone(), prog.clone(), egui_ctx);
        
        // Check that progress updates over time
        let initial_progress = *prog.lock().unwrap();
        
        // Wait a bit for progress to change
        thread::sleep(Duration::from_millis(100));
        
        // Progress should have increased by now
        let later_progress = *prog.lock().unwrap();
        assert!(later_progress > initial_progress || later_progress == 1.0, 
            "Progress should increase over time or be complete");
        
        // Wait for completion
        for _ in 0..50 {
            if *prog.lock().unwrap() >= 1.0 {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        
        // Verify that the progress reached 100%
        assert_eq!(*prog.lock().unwrap(), 1.0, "Progress should reach 100%");
    }
    
    /// Test output text formatting
    #[test]
    fn test_output_formatting() {
        // Run a calculation that returns multiple prime numbers
        let (result, _) = run_once(PrimeType::Regular, 10, 20);
        
        // Check that the output is formatted correctly with commas between numbers
        let prime_count = result.split(", ").filter(|s| !s.is_empty()).count();
        assert!(prime_count > 0, "Should find some primes");
        
        // For multiple primes, there should be commas
        if prime_count > 1 {
            assert!(result.contains(", "), "Multiple primes should be separated by commas");
        }
        
        // No leading or trailing commas
        assert!(!result.starts_with(", "), "Result should not start with a comma");
        assert!(!result.ends_with(", "), "Result should not end with a comma");
        
        // No double commas
        assert!(!result.contains(", , "), "Result should not contain double commas");
    }
    
    /// Test that repaint requests are made
    #[test]
    fn test_repaint_requests() {
        // This test is a bit tricky since we can't directly observe repaint requests
        // We'll count how many times the progress is updated, which should correspond to repaint requests
        
        let out = Arc::new(Mutex::new(String::new()));
        let prog = Arc::new(Mutex::new(0.0));
        let stop = Arc::new(AtomicBool::new(false));
        let egui_ctx = egui::Context::default();
        
        // We'll use this to track progress updates
        let last_progress = Arc::new(Mutex::new(0.0));
        let update_count = Arc::new(Mutex::new(0));
        
        // Start a calculation
        spawn_worker(
            PrimeType::Regular, 10, 30,
            stop.clone(), out.clone(), prog.clone(), egui_ctx);
        
        // Monitor progress for a short period, counting changes
        for _ in 0..20 {
            let current_progress = *prog.lock().unwrap();
            let mut last = last_progress.lock().unwrap();
            
            if current_progress != *last {
                *last = current_progress;
                *update_count.lock().unwrap() += 1;
            }
            
            thread::sleep(Duration::from_millis(10));
        }
        
        // There should be multiple progress updates (and thus repaint requests)
        let updates = *update_count.lock().unwrap();
        assert!(updates > 0, "There should be at least one progress update");
    }
}
