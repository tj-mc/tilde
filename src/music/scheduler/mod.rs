use crate::music::output::TimedEvent;
use crate::music::pattern::Pattern;
#[cfg(test)]
use crate::music::pattern::{Event, EventData};
use std::collections::HashMap;
use std::time::Instant;

/// Standalone scheduler for managing musical pattern timing and event generation
/// This is extracted from the original PatternScheduler in evaluator.rs to create
/// a clean, modular architecture that separates timing concerns from pattern representation.
#[derive(Debug)]
pub struct Scheduler {
    pub active_patterns: HashMap<String, ActivePattern>,
    pub current_time: f64,
    pub cpm: f64, // Cycles Per Minute
    pub is_playing: bool,
    start_time: Option<Instant>,
}

#[derive(Debug)]
pub struct ActivePattern {
    pub pattern: Pattern,
    pub next_event_time: f64,
    pub cycle_duration: f64,
    pub cycles_completed: u32,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            active_patterns: HashMap::new(),
            current_time: 0.0,
            cpm: 120.0, // Default 120 cycles per minute
            is_playing: false,
            start_time: None,
        }
    }
    
    /// Add or update a named pattern in the scheduler
    pub fn add_pattern(&mut self, name: String, pattern: Pattern) {
        let active = ActivePattern {
            pattern,
            next_event_time: self.current_time.floor(), // Start at next cycle boundary
            cycle_duration: 1.0, // Default 1 cycle
            cycles_completed: 0,
        };
        
        self.active_patterns.insert(name, active);
    }
    
    /// Remove a pattern from the scheduler
    pub fn remove_pattern(&mut self, name: &str) -> bool {
        self.active_patterns.remove(name).is_some()
    }
    
    /// Update an existing pattern (keeps timing state)
    pub fn update_pattern(&mut self, name: &str, pattern: Pattern) -> bool {
        if let Some(active) = self.active_patterns.get_mut(name) {
            active.pattern = pattern;
            true
        } else {
            false
        }
    }
    
    /// Start playback
    pub fn start(&mut self) {
        self.is_playing = true;
        self.start_time = Some(Instant::now());
        self.current_time = 0.0;
        
        // Reset all patterns to start at beginning
        for active in self.active_patterns.values_mut() {
            active.next_event_time = 0.0;
            active.cycles_completed = 0;
        }
    }
    
    /// Stop playback and clear all patterns
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.active_patterns.clear();
        self.start_time = None;
        self.current_time = 0.0;
    }
    
    /// Pause playback (keeps patterns but stops time progression)
    pub fn pause(&mut self) {
        self.is_playing = false;
    }
    
    /// Resume playback from current position
    pub fn resume(&mut self) {
        if self.start_time.is_some() {
            self.is_playing = true;
        }
    }
    
    /// Set the tempo in cycles per minute
    pub fn set_tempo(&mut self, cpm: f64) {
        if cpm > 0.0 {
            self.cpm = cpm;
        }
    }
    
    /// Get current tempo
    pub fn get_tempo(&self) -> f64 {
        self.cpm
    }
    
    /// Get current playback time
    pub fn get_current_time(&self) -> f64 {
        self.current_time
    }
    
    /// Get list of active pattern names
    pub fn get_pattern_names(&self) -> Vec<&String> {
        self.active_patterns.keys().collect()
    }
    
    /// Check if a pattern is active
    pub fn has_pattern(&self, name: &str) -> bool {
        self.active_patterns.contains_key(name)
    }
    
    /// Main scheduling tick - call this regularly to generate timed events
    /// Returns all events that should fire at the current time
    pub fn tick(&mut self) -> Vec<TimedEvent> {
        let mut events = Vec::new();
        
        if !self.is_playing {
            return events;
        }
        
        // Update current time based on elapsed real time
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs_f64();
            self.current_time = elapsed * (self.cpm / 60.0); // Convert CPM to cycles per second
        }
        
        // Process each active pattern
        let current_time = self.current_time;
        
        for (name, active) in &mut self.active_patterns {
            events.extend(Self::process_pattern_static(current_time, name, active));
        }
        
        events
    }
    
    /// Process a single pattern and generate any due events
    fn process_pattern_static(current_time: f64, name: &str, active: &mut ActivePattern) -> Vec<TimedEvent> {
        let mut events = Vec::new();
        
        // Check if we should fire events from this pattern
        while current_time >= active.next_event_time {
            let cycle_start_time = active.next_event_time;
            
            // Generate events for this cycle
            for event in &active.pattern.events {
                let absolute_time = cycle_start_time + (event.time * active.cycle_duration);
                let cycle_time = event.time; // Time within the cycle (0.0 to 1.0)
                
                // Only include events that should fire now (with small tolerance)
                if (absolute_time - current_time).abs() < 0.01 {
                    events.push(TimedEvent::new(
                        absolute_time,
                        cycle_time,
                        name.to_string(),
                        event.data.clone(),
                    ));
                }
            }
            
            // Schedule next cycle
            active.next_event_time += active.cycle_duration * active.pattern.duration;
            active.cycles_completed += 1;
        }
        
        events
    }
    
    /// Force tick at specific time (useful for testing)
    pub fn tick_at_time(&mut self, time: f64) -> Vec<TimedEvent> {
        self.current_time = time;
        self.tick()
    }
    
    /// Get scheduling statistics
    pub fn get_stats(&self) -> SchedulerStats {
        let mut stats = SchedulerStats {
            active_patterns: self.active_patterns.len(),
            current_time: self.current_time,
            cpm: self.cpm,
            is_playing: self.is_playing,
            total_cycles_completed: 0,
            pattern_stats: HashMap::new(),
        };
        
        for (name, active) in &self.active_patterns {
            stats.total_cycles_completed += active.cycles_completed;
            stats.pattern_stats.insert(name.clone(), PatternStats {
                cycles_completed: active.cycles_completed,
                next_event_time: active.next_event_time,
                event_count: active.pattern.event_count(),
            });
        }
        
        stats
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about scheduler state
#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub active_patterns: usize,
    pub current_time: f64,
    pub cpm: f64,
    pub is_playing: bool,
    pub total_cycles_completed: u32,
    pub pattern_stats: HashMap<String, PatternStats>,
}

/// Statistics about a specific pattern
#[derive(Debug, Clone)]
pub struct PatternStats {
    pub cycles_completed: u32,
    pub next_event_time: f64,
    pub event_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::music::pattern::Pattern;

    fn create_test_pattern() -> Pattern {
        Pattern::new(
            "c3 ~ d3".to_string(),
            vec![
                Event::new(0.0, EventData::Note { 
                    pitch: "c3".to_string(), 
                    velocity: 1.0, 
                    duration: 0.1 
                }),
                Event::new(0.5, EventData::Rest),
                Event::new(0.75, EventData::Note { 
                    pitch: "d3".to_string(), 
                    velocity: 1.0, 
                    duration: 0.1 
                }),
            ]
        )
    }

    #[test]
    fn test_scheduler_creation() {
        let scheduler = Scheduler::new();
        
        assert_eq!(scheduler.cpm, 120.0);
        assert!(!scheduler.is_playing);
        assert_eq!(scheduler.active_patterns.len(), 0);
        assert_eq!(scheduler.current_time, 0.0);
    }

    #[test]
    fn test_pattern_management() {
        let mut scheduler = Scheduler::new();
        let pattern = create_test_pattern();
        
        // Add pattern
        scheduler.add_pattern("test".to_string(), pattern.clone());
        assert!(scheduler.has_pattern("test"));
        assert_eq!(scheduler.get_pattern_names().len(), 1);
        
        // Update pattern
        let new_pattern = Pattern::new("e3".to_string(), vec![
            Event::new(0.0, EventData::Note {
                pitch: "e3".to_string(),
                velocity: 1.0,
                duration: 0.1,
            })
        ]);
        assert!(scheduler.update_pattern("test", new_pattern));
        
        // Remove pattern
        assert!(scheduler.remove_pattern("test"));
        assert!(!scheduler.has_pattern("test"));
        assert_eq!(scheduler.active_patterns.len(), 0);
    }

    #[test]
    fn test_playback_control() {
        let mut scheduler = Scheduler::new();
        
        assert!(!scheduler.is_playing);
        
        scheduler.start();
        assert!(scheduler.is_playing);
        assert!(scheduler.start_time.is_some());
        
        scheduler.pause();
        assert!(!scheduler.is_playing);
        assert!(scheduler.start_time.is_some()); // Should preserve start time
        
        scheduler.resume();
        assert!(scheduler.is_playing);
        
        scheduler.stop();
        assert!(!scheduler.is_playing);
        assert!(scheduler.start_time.is_none());
    }

    #[test]
    fn test_tempo_control() {
        let mut scheduler = Scheduler::new();
        
        assert_eq!(scheduler.get_tempo(), 120.0);
        
        scheduler.set_tempo(140.0);
        assert_eq!(scheduler.get_tempo(), 140.0);
        
        // Should reject invalid tempos
        scheduler.set_tempo(0.0);
        assert_eq!(scheduler.get_tempo(), 140.0); // Unchanged
        
        scheduler.set_tempo(-10.0);
        assert_eq!(scheduler.get_tempo(), 140.0); // Unchanged
    }

    #[test]
    fn test_event_generation() {
        let mut scheduler = Scheduler::new();
        let pattern = create_test_pattern();
        
        scheduler.add_pattern("test".to_string(), pattern);
        scheduler.start();
        
        // Simulate time progression and check for events
        let events = scheduler.tick_at_time(0.0);
        
        // Should have events at the beginning of the pattern
        assert!(!events.is_empty());
        
        // Check that events have proper structure
        for event in events {
            assert_eq!(event.pattern_name, "test");
            assert!(event.timestamp >= 0.0);
            assert!(event.cycle_time >= 0.0 && event.cycle_time <= 1.0);
        }
    }

    #[test]
    fn test_scheduler_stats() {
        let mut scheduler = Scheduler::new();
        let pattern = create_test_pattern();
        
        scheduler.add_pattern("test".to_string(), pattern);
        scheduler.start();
        
        let stats = scheduler.get_stats();
        
        assert_eq!(stats.active_patterns, 1);
        assert_eq!(stats.cpm, 120.0);
        assert!(stats.is_playing);
        assert!(stats.pattern_stats.contains_key("test"));
        
        let pattern_stats = &stats.pattern_stats["test"];
        assert_eq!(pattern_stats.event_count, 3);
    }

    #[test]
    fn test_multiple_patterns() {
        let mut scheduler = Scheduler::new();
        
        let pattern1 = Pattern::new("c3".to_string(), vec![
            Event::new(0.0, EventData::Note {
                pitch: "c3".to_string(),
                velocity: 1.0,
                duration: 0.1,
            })
        ]);
        
        let pattern2 = Pattern::new("d3".to_string(), vec![
            Event::new(0.5, EventData::Note {
                pitch: "d3".to_string(),
                velocity: 1.0,
                duration: 0.1,
            })
        ]);
        
        scheduler.add_pattern("pattern1".to_string(), pattern1);
        scheduler.add_pattern("pattern2".to_string(), pattern2);
        
        assert_eq!(scheduler.get_pattern_names().len(), 2);
        assert!(scheduler.has_pattern("pattern1"));
        assert!(scheduler.has_pattern("pattern2"));
        
        scheduler.start();
        let events = scheduler.tick_at_time(0.0);
        
        // Should generate events from both patterns
        assert!(!events.is_empty());
    }
}