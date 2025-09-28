pub mod debug;
pub mod audio;
pub mod direct_audio;

use crate::music::pattern::EventData;
use std::fmt;

// Re-export output adapters
pub use debug::{DebugOutput, ConsoleOutput};
pub use audio::AudioOutput;
pub use direct_audio::DirectAudioOutput;

/// Output system for the modular music architecture
/// This provides a clean interface for sending timed musical events to various destinations

#[derive(Debug, Clone)]
pub struct TimedEvent {
    pub timestamp: f64,         // Absolute time in seconds since playback start
    pub cycle_time: f64,        // Time within the current cycle (0.0 to 1.0)
    pub pattern_name: String,   // Name/ID of the pattern that generated this event
    pub data: EventData,        // The actual musical event data
}

impl TimedEvent {
    pub fn new(timestamp: f64, cycle_time: f64, pattern_name: String, data: EventData) -> Self {
        TimedEvent {
            timestamp,
            cycle_time,
            pattern_name,
            data,
        }
    }
    
    pub fn note(timestamp: f64, cycle_time: f64, pattern_name: String, pitch: String, velocity: f64, duration: f64) -> Self {
        TimedEvent::new(
            timestamp,
            cycle_time,
            pattern_name,
            EventData::Note { pitch, velocity, duration },
        )
    }
    
    pub fn rest(timestamp: f64, cycle_time: f64, pattern_name: String) -> Self {
        TimedEvent::new(timestamp, cycle_time, pattern_name, EventData::Rest)
    }
    
    pub fn control(timestamp: f64, cycle_time: f64, pattern_name: String, param: String, value: f64) -> Self {
        TimedEvent::new(
            timestamp,
            cycle_time,
            pattern_name,
            EventData::Control { param, value },
        )
    }
}

impl fmt::Display for TimedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] @{:.3}s (cycle:{:.3}): {}",
            self.pattern_name, self.timestamp, self.cycle_time, self.data
        )
    }
}

/// Trait for output adapters that can receive and process timed musical events
/// Implementations can send events to MIDI devices, audio engines, OSC servers, etc.
pub trait OutputAdapter {
    /// Send a timed event to this output destination
    fn send_event(&mut self, event: &TimedEvent) -> Result<(), String>;
    
    /// Start the output (initialize connections, allocate resources, etc.)
    fn start(&mut self) -> Result<(), String> {
        Ok(()) // Default implementation does nothing
    }
    
    /// Stop the output (cleanup resources, close connections, etc.)
    fn stop(&mut self) -> Result<(), String> {
        Ok(()) // Default implementation does nothing
    }
    
    /// Get a human-readable name for this output adapter
    fn name(&self) -> &str;
    
    /// Check if this output is currently active/connected
    fn is_active(&self) -> bool {
        true // Default implementation assumes always active
    }
}

/// Collection of output adapters with helper methods
pub struct OutputCollection {
    adapters: Vec<Box<dyn OutputAdapter>>,
}

impl OutputCollection {
    pub fn new() -> Self {
        OutputCollection {
            adapters: Vec::new(),
        }
    }
    
    pub fn add_adapter(&mut self, adapter: Box<dyn OutputAdapter>) {
        self.adapters.push(adapter);
    }
    
    pub fn remove_adapter(&mut self, name: &str) -> bool {
        if let Some(pos) = self.adapters.iter().position(|a| a.name() == name) {
            self.adapters.remove(pos);
            true
        } else {
            false
        }
    }
    
    pub fn send_event_to_all(&mut self, event: &TimedEvent) -> Vec<String> {
        let mut errors = Vec::new();
        
        for adapter in &mut self.adapters {
            if let Err(e) = adapter.send_event(event) {
                errors.push(format!("{}: {}", adapter.name(), e));
            }
        }
        
        errors
    }
    
    pub fn start_all(&mut self) -> Vec<String> {
        let mut errors = Vec::new();
        
        for adapter in &mut self.adapters {
            if let Err(e) = adapter.start() {
                errors.push(format!("{}: {}", adapter.name(), e));
            }
        }
        
        errors
    }
    
    pub fn stop_all(&mut self) -> Vec<String> {
        let mut errors = Vec::new();
        
        for adapter in &mut self.adapters {
            if let Err(e) = adapter.stop() {
                errors.push(format!("{}: {}", adapter.name(), e));
            }
        }
        
        errors
    }
    
    pub fn get_adapter_names(&self) -> Vec<&str> {
        self.adapters.iter().map(|a| a.name()).collect()
    }
    
    pub fn len(&self) -> usize {
        self.adapters.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }
}

impl Default for OutputCollection {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for OutputCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OutputCollection")
            .field("adapter_count", &self.adapters.len())
            .field("adapter_names", &self.get_adapter_names())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::pattern::EventData;

    // Mock output adapter for testing
    struct MockOutput {
        name: String,
        events: Vec<TimedEvent>,
        should_error: bool,
    }
    
    impl MockOutput {
        fn new(name: String) -> Self {
            MockOutput {
                name,
                events: Vec::new(),
                should_error: false,
            }
        }
        
        fn with_error(mut self) -> Self {
            self.should_error = true;
            self
        }
    }
    
    impl OutputAdapter for MockOutput {
        fn send_event(&mut self, event: &TimedEvent) -> Result<(), String> {
            if self.should_error {
                Err("Mock error".to_string())
            } else {
                self.events.push(event.clone());
                Ok(())
            }
        }
        
        fn name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_timed_event_creation() {
        let event = TimedEvent::note(
            1.5, 
            0.25, 
            "test_pattern".to_string(), 
            "c3".to_string(), 
            0.8, 
            0.1
        );
        
        assert_eq!(event.timestamp, 1.5);
        assert_eq!(event.cycle_time, 0.25);
        assert_eq!(event.pattern_name, "test_pattern");
        
        if let EventData::Note { pitch, velocity, duration } = &event.data {
            assert_eq!(pitch, "c3");
            assert_eq!(*velocity, 0.8);
            assert_eq!(*duration, 0.1);
        } else {
            panic!("Expected note event");
        }
    }

    #[test]
    fn test_output_collection() {
        let mut collection = OutputCollection::new();
        
        let output1 = Box::new(MockOutput::new("output1".to_string()));
        let output2 = Box::new(MockOutput::new("output2".to_string()));
        
        collection.add_adapter(output1);
        collection.add_adapter(output2);
        
        assert_eq!(collection.len(), 2);
        assert_eq!(collection.get_adapter_names(), vec!["output1", "output2"]);
        
        // Test event sending
        let event = TimedEvent::rest(0.0, 0.0, "test".to_string());
        let errors = collection.send_event_to_all(&event);
        assert!(errors.is_empty());
        
        // Test adapter removal
        assert!(collection.remove_adapter("output1"));
        assert_eq!(collection.len(), 1);
        assert!(!collection.remove_adapter("nonexistent"));
    }

    #[test]
    fn test_output_collection_errors() {
        let mut collection = OutputCollection::new();
        
        let good_output = Box::new(MockOutput::new("good".to_string()));
        let bad_output = Box::new(MockOutput::new("bad".to_string()).with_error());
        
        collection.add_adapter(good_output);
        collection.add_adapter(bad_output);
        
        let event = TimedEvent::rest(0.0, 0.0, "test".to_string());
        let errors = collection.send_event_to_all(&event);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("bad: Mock error"));
    }
}