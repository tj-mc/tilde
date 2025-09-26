pub mod pattern;
pub mod scheduler;
pub mod output;
pub mod engine;

// Re-export commonly used types
pub use pattern::{Pattern, Event, EventData, PatternValue, parse_mini_notation};
pub use scheduler::{Scheduler, ActivePattern, SchedulerStats, PatternStats};
pub use output::{OutputAdapter, TimedEvent, OutputCollection};
pub use engine::{MusicEngine, EngineStats};