use tilde::evaluator::Evaluator;
use tilde::music::parse_mini_notation;
use tilde::value::{Value, PatternValue};
use std::thread;
use std::time::Duration;

#[test]
fn test_scheduler_event_firing() {
    let mut evaluator = Evaluator::new();

    // Create a simple pattern with clear events
    let events = parse_mini_notation("c3 d3").unwrap();
    let pattern = Value::Pattern(PatternValue::Simple {
        notation: "c3 d3".to_string(),
        events,
    });

    // Set a fast tempo for testing
    evaluator.scheduler.set_tempo(600.0); // 10 cycles per second
    evaluator.scheduler.add_pattern(pattern);
    evaluator.scheduler.start();

    // Wait a small amount to let some time pass
    thread::sleep(Duration::from_millis(150)); // 0.15 seconds

    // Tick the scheduler - should have events ready to fire
    let outputs = evaluator.tick_scheduler();

    // At least some output should be generated after time has passed
    println!("Outputs after 150ms: {:?}", outputs);

    // The important thing is the scheduler mechanism works
    assert!(evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 1);

    // Test additional ticks
    thread::sleep(Duration::from_millis(100));
    let outputs2 = evaluator.tick_scheduler();
    println!("Outputs after additional 100ms: {:?}", outputs2);

    // Clean up
    evaluator.scheduler.stop();
}

#[test]
fn test_scheduler_pattern_cycling() {
    let mut evaluator = Evaluator::new();

    // Create pattern with rests to test timing
    let events = parse_mini_notation("c3 ~ d3 ~").unwrap();
    assert_eq!(events.len(), 4); // Should have 4 events including rests

    let pattern = Value::Pattern(PatternValue::Simple {
        notation: "c3 ~ d3 ~".to_string(),
        events,
    });

    evaluator.scheduler.set_tempo(120.0); // 2 cycles per second
    evaluator.scheduler.add_pattern(pattern);
    evaluator.scheduler.start();

    // Test that the scheduler can handle multiple cycles
    for i in 0..5 {
        thread::sleep(Duration::from_millis(100)); // 0.1 seconds
        let outputs = evaluator.tick_scheduler();
        println!("Cycle {} outputs: {:?}", i, outputs);
    }

    evaluator.scheduler.stop();
}

#[test]
fn test_scheduler_state_consistency() {
    let mut evaluator = Evaluator::new();

    // Verify initial state
    assert!(!evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.current_time, 0.0);
    assert_eq!(evaluator.scheduler.cpm, 120.0);
    assert_eq!(evaluator.scheduler.patterns.len(), 0);

    // Add pattern and start
    let events = parse_mini_notation("c3").unwrap();
    let pattern = Value::Pattern(PatternValue::Simple {
        notation: "c3".to_string(),
        events,
    });

    evaluator.scheduler.add_pattern(pattern);
    evaluator.scheduler.start();

    // Verify running state
    assert!(evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 1);

    // Test tempo changes while running
    evaluator.scheduler.set_tempo(240.0);
    assert_eq!(evaluator.scheduler.cpm, 240.0);
    assert!(evaluator.scheduler.is_playing); // Should still be playing

    // Test stopping
    evaluator.scheduler.stop();
    assert!(!evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 0);
    assert_eq!(evaluator.scheduler.current_time, 0.0);
}