pub mod mini_notation;

use std::fmt;

// Re-export mini-notation parser
pub use mini_notation::{parse_mini_notation, parse_to_pattern_value};

/// Core pattern data structures for the modular music system
/// This module provides clean separation between pattern representation and evaluation

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub notation: String,
    pub events: Vec<Event>,
    pub duration: f64, // Duration in cycles
}

impl Pattern {
    pub fn new(notation: String, events: Vec<Event>) -> Self {
        Pattern {
            notation,
            events,
            duration: 1.0, // Default to 1 cycle
        }
    }
    
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }
    
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub time: f64,           // 0.0 to 1.0 within pattern cycle
    pub data: EventData,
}

impl Event {
    pub fn new(time: f64, data: EventData) -> Self {
        Event { time, data }
    }
    
    pub fn note(time: f64, pitch: String) -> Self {
        Event::new(time, EventData::Note { 
            pitch, 
            velocity: 1.0, 
            duration: 0.1 
        })
    }
    
    pub fn rest(time: f64) -> Self {
        Event::new(time, EventData::Rest)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventData {
    Note { 
        pitch: String, 
        velocity: f64, 
        duration: f64 
    },
    Rest,
    Control { 
        param: String, 
        value: f64 
    },
}

impl fmt::Display for EventData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventData::Note { pitch, velocity, duration } => {
                write!(f, "{}(v:{:.2},d:{:.2})", pitch, velocity, duration)
            },
            EventData::Rest => write!(f, "~"),
            EventData::Control { param, value } => {
                write!(f, "{}:{:.2}", param, value)
            },
        }
    }
}

/// Enhanced pattern value enum for different pattern types
/// This replaces the PatternValue enum from value.rs
#[derive(Debug, Clone, PartialEq)]
pub enum PatternValue {
    Simple { 
        notation: String, 
        events: Vec<Event> 
    },
    Stacked { 
        patterns: Vec<PatternValue> 
    },
    // Future pattern types can be added here:
    // Euclidean { steps: u32, pulses: u32, rotation: u32 },
    // Probability { pattern: Box<PatternValue>, chance: f64 },
    // Transform { pattern: Box<PatternValue>, transform: PatternTransform },
}

impl PatternValue {
    pub fn notation(&self) -> String {
        match self {
            PatternValue::Simple { notation, .. } => notation.clone(),
            PatternValue::Stacked { patterns } => {
                let notations: Vec<String> = patterns.iter().map(|p| p.notation()).collect();
                format!("stack[{}]", notations.join(", "))
            }
        }
    }

    pub fn events(&self) -> Vec<Event> {
        match self {
            PatternValue::Simple { events, .. } => events.clone(),
            PatternValue::Stacked { patterns } => {
                // Combine all events from stacked patterns
                let mut combined_events = Vec::new();
                for pattern in patterns {
                    combined_events.extend(pattern.events());
                }
                // Sort by time to ensure proper ordering
                combined_events.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
                combined_events
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            PatternValue::Simple { events, .. } => events.is_empty(),
            PatternValue::Stacked { patterns } => {
                patterns.is_empty() || patterns.iter().all(|p| p.is_empty())
            }
        }
    }
    
    pub fn to_pattern(&self) -> Pattern {
        Pattern::new(self.notation(), self.events())
    }
}

impl fmt::Display for PatternValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternValue::Simple { notation, events } => {
                write!(f, "Pattern[{}] ({} events)", notation, events.len())
            },
            PatternValue::Stacked { patterns } => {
                write!(f, "Stack[{} patterns]", patterns.len())
            }
        }
    }
}

// Legacy conversion removed - handled in MusicEngine instead

/// Pattern transformation trait for future extensibility
pub trait PatternTransform: Send + Sync {
    fn apply(&self, pattern: &Pattern) -> Pattern;
    fn name(&self) -> &str;
}

/// Basic pattern transformations
#[derive(Debug, Clone)]
pub struct SpeedTransform(pub f64);

impl PatternTransform for SpeedTransform {
    fn apply(&self, pattern: &Pattern) -> Pattern {
        let mut transformed = pattern.clone();
        transformed.duration = pattern.duration / self.0;
        // Speed up events by scaling time
        for event in &mut transformed.events {
            event.time = event.time / self.0;
        }
        transformed.notation = format!("speed({}, {})", self.0, pattern.notation);
        transformed
    }
    
    fn name(&self) -> &str {
        "speed"
    }
}

#[derive(Debug, Clone)]
pub struct TransposeTransform(pub f64);

impl PatternTransform for TransposeTransform {
    fn apply(&self, pattern: &Pattern) -> Pattern {
        let mut transformed = pattern.clone();
        // Transpose note events (simplified - real implementation would parse note names)
        for event in &mut transformed.events {
            if let EventData::Note { pitch, .. } = &mut event.data {
                // For now, just append the transpose amount to the notation
                *pitch = format!("{}+{}", pitch, self.0);
            }
        }
        transformed.notation = format!("transpose({}, {})", self.0, pattern.notation);
        transformed
    }
    
    fn name(&self) -> &str {
        "transpose"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_creation() {
        let events = vec![
            Event::note(0.0, "c3".to_string()),
            Event::rest(0.25),
            Event::note(0.5, "d3".to_string()),
        ];
        
        let pattern = Pattern::new("c3 ~ d3".to_string(), events);
        
        assert_eq!(pattern.notation, "c3 ~ d3");
        assert_eq!(pattern.event_count(), 3);
        assert!(!pattern.is_empty());
    }

    #[test]
    fn test_pattern_value_simple() {
        let events = vec![Event::note(0.0, "c3".to_string())];
        let pattern_value = PatternValue::Simple {
            notation: "c3".to_string(),
            events: events.clone(),
        };
        
        assert_eq!(pattern_value.notation(), "c3");
        assert_eq!(pattern_value.events(), events);
        assert!(!pattern_value.is_empty());
    }

    #[test]
    fn test_pattern_value_stacked() {
        let pattern1 = PatternValue::Simple {
            notation: "c3".to_string(),
            events: vec![Event::note(0.0, "c3".to_string())],
        };
        let pattern2 = PatternValue::Simple {
            notation: "e3".to_string(),
            events: vec![Event::note(0.5, "e3".to_string())],
        };
        
        let stacked = PatternValue::Stacked {
            patterns: vec![pattern1, pattern2],
        };
        
        assert_eq!(stacked.notation(), "stack[c3, e3]");
        assert_eq!(stacked.events().len(), 2);
        assert!(!stacked.is_empty());
    }

    #[test]
    fn test_speed_transform() {
        let pattern = Pattern::new(
            "c3 d3".to_string(),
            vec![
                Event::note(0.0, "c3".to_string()),
                Event::note(0.5, "d3".to_string()),
            ]
        );
        
        let transform = SpeedTransform(2.0);
        let transformed = transform.apply(&pattern);
        
        assert_eq!(transformed.duration, 0.5); // Half duration = double speed
        assert_eq!(transformed.events[0].time, 0.0);
        assert_eq!(transformed.events[1].time, 0.25); // Halved time
    }
}