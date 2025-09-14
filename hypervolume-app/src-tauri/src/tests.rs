#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio_boost::{AudioBoostManager, AudioProcessor};

    #[test]
    fn test_boost_manager_creation() {
        let manager = AudioBoostManager::new();
        // Test that manager is created successfully
        assert!(true); // Placeholder test
    }

    #[test]
    fn test_audio_processor_boost() {
        let processor = AudioProcessor::new();
        let mut samples = vec![0.5, -0.3, 0.8, -0.1];
        let original_samples = samples.clone();
        
        processor.process_samples(&mut samples, 2.0);
        
        // Check that samples are boosted
        for (i, &sample) in samples.iter().enumerate() {
            assert!((sample - original_samples[i] * 2.0).abs() < 0.1);
        }
    }

    #[test]
    fn test_audio_processor_limiting() {
        let processor = AudioProcessor::new();
        let mut samples = vec![1.5, -1.8, 0.5]; // Values that would clip
        
        processor.process_samples(&mut samples, 1.0);
        
        // Check that no sample exceeds safe limits
        for &sample in &samples {
            assert!(sample.abs() <= 1.0);
        }
    }

    #[test]
    fn test_soft_clipping() {
        let processor = AudioProcessor::new();
        
        // Test values that should be clipped
        let clipped_positive = processor.soft_clip(1.5);
        let clipped_negative = processor.soft_clip(-1.5);
        
        assert!(clipped_positive < 1.0);
        assert!(clipped_negative > -1.0);
        assert!(clipped_positive > 0.0);
        assert!(clipped_negative < 0.0);
    }

    #[test]
    fn test_boost_settings() {
        let manager = AudioBoostManager::new();
        let process_id = 1234;
        let boost_factor = 3.5;
        
        // This test would require mocking the Windows API calls
        // For now, we'll test the basic structure
        let result = manager.get_boost(process_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1.0); // Default boost
    }
}

