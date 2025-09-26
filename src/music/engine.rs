use crate::music::pattern::Pattern;
use crate::value::PatternValue;
use crate::music::scheduler::Scheduler;
use crate::music::output::{OutputAdapter, OutputCollection, DebugOutput};

/// MusicEngine coordinates the Pattern Engine, Scheduler, and Output Adapters
/// This is the main entry point for the modular music system, providing a clean API
/// for the Evaluator and stdlib functions to interact with.
#[derive(Debug)]
pub struct MusicEngine {
    scheduler: Scheduler,
    outputs: OutputCollection,
    is_initialized: bool,
}

impl MusicEngine {
    /// Create a new MusicEngine with default configuration
    pub fn new() -> Self {
        MusicEngine {
            scheduler: Scheduler::new(),
            outputs: OutputCollection::new(),
            is_initialized: false,
        }
    }
    
    /// Create a MusicEngine with a debug output for testing
    pub fn with_debug_output() -> Self {
        let mut engine = Self::new();
        engine.add_output(Box::new(DebugOutput::default()));
        engine.is_initialized = true;
        engine
    }
    
    /// Initialize the engine (start outputs, etc.)
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.is_initialized {
            return Ok(());
        }
        
        // If no outputs are configured, add a debug output
        if self.outputs.is_empty() {
            self.outputs.add_adapter(Box::new(DebugOutput::default()));
        }
        
        // Start all outputs
        let errors = self.outputs.start_all();
        if !errors.is_empty() {
            return Err(format!("Failed to start outputs: {}", errors.join(", ")));
        }
        
        self.is_initialized = true;
        Ok(())
    }
    
    /// Shutdown the engine
    pub fn shutdown(&mut self) -> Result<(), String> {
        self.scheduler.stop();
        
        let errors = self.outputs.stop_all();
        if !errors.is_empty() {
            return Err(format!("Failed to stop outputs: {}", errors.join(", ")));
        }
        
        self.is_initialized = false;
        Ok(())
    }
    
    /// Add an output adapter
    pub fn add_output(&mut self, adapter: Box<dyn OutputAdapter>) {
        self.outputs.add_adapter(adapter);
    }
    
    /// Remove an output adapter by name
    pub fn remove_output(&mut self, name: &str) -> bool {
        self.outputs.remove_adapter(name)
    }
    
    /// Get list of output adapter names
    pub fn get_output_names(&self) -> Vec<&str> {
        self.outputs.get_adapter_names()
    }
    
    /// Add a pattern from legacy PatternValue (for compatibility with existing code)
    pub fn add_pattern_value(&mut self, name: String, pattern_value: &PatternValue) -> Result<(), String> {
        let pattern = self.convert_legacy_pattern_value(pattern_value)?;
        self.add_pattern(name, pattern)
    }
    
    /// Convert legacy PatternValue from value.rs to new modular Pattern
    fn convert_legacy_pattern_value(&self, legacy: &PatternValue) -> Result<Pattern, String> {
        
        let notation = legacy.notation();
        let legacy_events = legacy.events();
        
        // Events are already in the correct format, no conversion needed
        let new_events = legacy_events;
        
        Ok(Pattern::new(notation, new_events))
    }
    
    /// Add a pattern to the scheduler
    pub fn add_pattern(&mut self, name: String, pattern: Pattern) -> Result<(), String> {
        if !self.is_initialized {
            self.initialize()?;
        }
        
        self.scheduler.add_pattern(name, pattern);
        Ok(())
    }
    
    /// Remove a pattern from the scheduler
    pub fn remove_pattern(&mut self, name: &str) -> bool {
        self.scheduler.remove_pattern(name)
    }
    
    /// Update an existing pattern
    pub fn update_pattern(&mut self, name: &str, pattern: Pattern) -> bool {
        self.scheduler.update_pattern(name, pattern)
    }
    
    /// Start playback
    pub fn start(&mut self) -> Result<(), String> {
        if !self.is_initialized {
            self.initialize()?;
        }
        
        self.scheduler.start();
        Ok(())
    }
    
    /// Stop playback and clear patterns
    pub fn stop(&mut self) {
        self.scheduler.stop();
    }
    
    /// Pause playback
    pub fn pause(&mut self) {
        self.scheduler.pause();
    }
    
    /// Resume playback
    pub fn resume(&mut self) {
        self.scheduler.resume();
    }
    
    /// Set tempo in cycles per minute
    pub fn set_tempo(&mut self, cpm: f64) {
        self.scheduler.set_tempo(cpm);
    }
    
    /// Get current tempo
    pub fn get_tempo(&self) -> f64 {
        self.scheduler.get_tempo()
    }
    
    /// Check if engine is playing
    pub fn is_playing(&self) -> bool {
        self.scheduler.is_playing
    }
    
    /// Get list of active pattern names
    pub fn get_pattern_names(&self) -> Vec<&String> {
        self.scheduler.get_pattern_names()
    }
    
    /// Check if a pattern is active
    pub fn has_pattern(&self, name: &str) -> bool {
        self.scheduler.has_pattern(name)
    }
    
    /// Main tick function - call this regularly to process events
    /// Returns debug strings for backward compatibility with Phase 2A
    pub fn tick(&mut self) -> Vec<String> {
        if !self.is_initialized {
            return Vec::new();
        }
        
        let events = self.scheduler.tick();
        let mut debug_strings = Vec::new();
        
        for event in &events {
            // Send to all outputs
            let errors = self.outputs.send_event_to_all(event);
            
            // Log any output errors (but don't fail the tick)
            if !errors.is_empty() {
                eprintln!("Output errors: {}", errors.join(", "));
            }
            
            // Generate debug string for backward compatibility
            debug_strings.push(format!("♪ {} {}", event.pattern_name, event.data));
        }
        
        debug_strings
    }
    
    /// Force tick at specific time (useful for testing)
    pub fn tick_at_time(&mut self, time: f64) -> Vec<String> {
        if !self.is_initialized {
            return Vec::new();
        }
        
        let events = self.scheduler.tick_at_time(time);
        let mut debug_strings = Vec::new();
        
        for event in &events {
            let errors = self.outputs.send_event_to_all(event);
            if !errors.is_empty() {
                eprintln!("Output errors: {}", errors.join(", "));
            }
            debug_strings.push(format!("♪ {} {}", event.pattern_name, event.data));
        }
        
        debug_strings
    }
    
    /// Get current playback time
    pub fn get_current_time(&self) -> f64 {
        self.scheduler.get_current_time()
    }
    
    /// Get comprehensive engine statistics
    pub fn get_stats(&self) -> EngineStats {
        let scheduler_stats = self.scheduler.get_stats();
        
        EngineStats {
            is_initialized: self.is_initialized,
            output_count: self.outputs.len(),
            output_names: self.outputs.get_adapter_names().into_iter().map(|s| s.to_string()).collect(),
            scheduler_stats,
        }
    }
}

impl Default for MusicEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the entire music engine
#[derive(Debug)]
pub struct EngineStats {
    pub is_initialized: bool,
    pub output_count: usize,
    pub output_names: Vec<String>, // Changed from Vec<&'static str> to Vec<String>
    pub scheduler_stats: crate::music::scheduler::SchedulerStats,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::{Event, EventData};

    fn create_test_pattern() -> Pattern {
        Pattern::new(
            "c3 d3".to_string(),
            vec![
                Event::new(0.0, EventData::Note {
                    pitch: "c3".to_string(),
                    velocity: 1.0,
                    duration: 0.1,
                }),
                Event::new(0.5, EventData::Note {
                    pitch: "d3".to_string(),
                    velocity: 1.0,
                    duration: 0.1,
                }),
            ]
        )
    }

    #[test]
    fn test_engine_creation() {
        let engine = MusicEngine::new();
        
        assert!(!engine.is_initialized);
        assert_eq!(engine.get_tempo(), 120.0);
        assert!(!engine.is_playing());
        assert_eq!(engine.get_pattern_names().len(), 0);
    }

    #[test]
    fn test_engine_with_debug_output() {
        let engine = MusicEngine::with_debug_output();
        
        assert!(engine.is_initialized);
        assert_eq!(engine.outputs.len(), 1);
        assert_eq!(engine.get_output_names(), vec!["debug"]);
    }

    #[test]
    fn test_pattern_management() {
        let mut engine = MusicEngine::with_debug_output();
        let pattern = create_test_pattern();
        
        // Add pattern
        assert!(engine.add_pattern("test".to_string(), pattern).is_ok());
        assert!(engine.has_pattern("test"));
        assert_eq!(engine.get_pattern_names().len(), 1);
        
        // Remove pattern
        assert!(engine.remove_pattern("test"));
        assert!(!engine.has_pattern("test"));
        assert_eq!(engine.get_pattern_names().len(), 0);
    }

    #[test]
    fn test_playback_control() {
        let mut engine = MusicEngine::with_debug_output();
        let pattern = create_test_pattern();
        
        engine.add_pattern("test".to_string(), pattern).unwrap();
        
        assert!(!engine.is_playing());
        
        engine.start().unwrap();
        assert!(engine.is_playing());
        
        engine.pause();
        assert!(!engine.is_playing());
        
        engine.resume();
        assert!(engine.is_playing());
        
        engine.stop();
        assert!(!engine.is_playing());
    }

    #[test]
    fn test_tempo_control() {
        let mut engine = MusicEngine::with_debug_output();
        
        assert_eq!(engine.get_tempo(), 120.0);
        
        engine.set_tempo(140.0);
        assert_eq!(engine.get_tempo(), 140.0);
    }

    #[test]
    fn test_tick_functionality() {
        let mut engine = MusicEngine::with_debug_output();
        let pattern = create_test_pattern();
        
        engine.add_pattern("test".to_string(), pattern).unwrap();
        engine.start().unwrap();
        
        // Test tick at time 0
        let events = engine.tick_at_time(0.0);
        
        // Should generate some debug strings
        assert!(!events.is_empty());
        
        // Check format of debug strings
        for event_str in events {
            assert!(event_str.starts_with("♪ test"));
        }
    }

    #[test]
    fn test_pattern_value_compatibility() {
        use crate::value::{PatternEvent, EventType};
        
        let mut engine = MusicEngine::with_debug_output();
        
        let pattern_value = PatternValue::Simple {
            notation: "c3 d3".to_string(),
            events: vec![
                Event::new(0.0, EventData::Note { 
                    pitch: "c3".to_string(), 
                    velocity: 1.0, 
                    duration: 0.1 
                }),
            ],
        };
        
        assert!(engine.add_pattern_value("test".to_string(), &pattern_value).is_ok());
        assert!(engine.has_pattern("test"));
    }

    #[test]
    fn test_engine_stats() {
        let mut engine = MusicEngine::with_debug_output();
        let pattern = create_test_pattern();
        
        engine.add_pattern("test".to_string(), pattern).unwrap();
        
        let stats = engine.get_stats();
        
        assert!(stats.is_initialized);
        assert_eq!(stats.output_count, 1);
        assert_eq!(stats.output_names, vec!["debug".to_string()]);
        assert_eq!(stats.scheduler_stats.active_patterns, 1);
    }

    #[test]
    fn test_auto_initialization() {
        let mut engine = MusicEngine::new();
        let pattern = create_test_pattern();
        
        assert!(!engine.is_initialized);
        
        // Adding a pattern should auto-initialize
        assert!(engine.add_pattern("test".to_string(), pattern).is_ok());
        assert!(engine.is_initialized);
        
        // Should have added a default debug output
        assert!(!engine.outputs.is_empty());
    }

    #[test]
    fn test_engine_shutdown() {
        let mut engine = MusicEngine::with_debug_output();
        let pattern = create_test_pattern();
        
        engine.add_pattern("test".to_string(), pattern).unwrap();
        engine.start().unwrap();
        
        assert!(engine.is_initialized);
        assert!(engine.is_playing());
        
        assert!(engine.shutdown().is_ok());
        
        assert!(!engine.is_initialized);
        assert!(!engine.is_playing());
    }
}