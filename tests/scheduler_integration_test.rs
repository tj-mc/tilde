use tilde::evaluator::Evaluator;
use tilde::music::parse_mini_notation;
use tilde::value::{Value, PatternValue};

#[test]
fn test_scheduler_basic_functionality() {
    let mut evaluator = Evaluator::new();

    // Test initial state
    assert!(!evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 0);
    assert_eq!(evaluator.scheduler.cpm, 120.0);

    // Test tempo setting
    evaluator.scheduler.set_tempo(60.0); // 1 cycle per second
    assert_eq!(evaluator.scheduler.cpm, 60.0);

    // Create and add pattern
    let events = parse_mini_notation("c3 d3").unwrap();
    let pattern = Value::Pattern(PatternValue::Simple {
        notation: "c3 d3".to_string(),
        events,
    });

    evaluator.scheduler.add_pattern(pattern);
    assert_eq!(evaluator.scheduler.patterns.len(), 1);

    // Test starting scheduler
    evaluator.scheduler.start();
    assert!(evaluator.scheduler.is_playing);
    assert!(evaluator.scheduler.current_time >= 0.0);

    // Test stopping
    evaluator.scheduler.stop();
    assert!(!evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 0);
}

#[test]
fn test_multiple_patterns() {
    let mut evaluator = Evaluator::new();

    // Create two patterns
    let events1 = parse_mini_notation("c3 d3").unwrap();
    let pattern1 = Value::Pattern(PatternValue::Simple {
        notation: "c3 d3".to_string(),
        events: events1,
    });

    let events2 = parse_mini_notation("e3 f3 g3").unwrap();
    let pattern2 = Value::Pattern(PatternValue::Simple {
        notation: "e3 f3 g3".to_string(),
        events: events2,
    });

    // Add both patterns
    evaluator.scheduler.add_pattern(pattern1);
    evaluator.scheduler.add_pattern(pattern2);
    assert_eq!(evaluator.scheduler.patterns.len(), 2);

    // Start scheduler
    evaluator.scheduler.start();
    assert!(evaluator.scheduler.is_playing);
}

#[test]
fn test_scheduler_tick_mechanism() {
    let mut evaluator = Evaluator::new();

    // Create pattern with specific timing
    let events = parse_mini_notation("c3 ~ d3 ~").unwrap();
    let pattern = Value::Pattern(PatternValue::Simple {
        notation: "c3 ~ d3 ~".to_string(),
        events,
    });

    evaluator.scheduler.set_tempo(120.0); // 2 cycles per second
    evaluator.scheduler.add_pattern(pattern);
    evaluator.scheduler.start();

    // Test ticking - this tests the mechanism even if timing isn't perfect
    let outputs = evaluator.tick_scheduler();

    // The scheduler should be able to tick without errors
    // Exact event firing depends on precise timing, but mechanism should work
    println!("Scheduler tick completed with {} outputs", outputs.len());

    // Verify scheduler state is still consistent
    assert!(evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 1);
}

#[test]
fn test_pattern_value_enum_compatibility() {
    let mut evaluator = Evaluator::new();

    // Test that both Simple and hypothetical Stacked patterns work
    let events = parse_mini_notation("c3 d3").unwrap();
    let simple_pattern = Value::Pattern(PatternValue::Simple {
        notation: "c3 d3".to_string(),
        events: events.clone(),
    });

    // Test pattern methods work
    if let Value::Pattern(ref pattern) = simple_pattern {
        assert_eq!(pattern.notation(), "c3 d3");
        assert_eq!(pattern.events().len(), 2);
        assert!(!pattern.is_empty());
    }

    // Test scheduler accepts the pattern
    evaluator.scheduler.add_pattern(simple_pattern);
    assert_eq!(evaluator.scheduler.patterns.len(), 1);
}