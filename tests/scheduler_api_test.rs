use tilde::evaluator::Evaluator;
use tilde::ast::*;
use tilde::value::Value;

#[test]
fn test_play_stop_tempo_api() {
    let mut evaluator = Evaluator::new();

    // Test tempo function
    let tempo_args = vec![Expression::Literal(Value::Number(180.0))];
    let result = evaluator.eval_positional_function("tempo", tempo_args).unwrap();
    assert!(matches!(result, Value::String(_)));
    assert_eq!(evaluator.scheduler.cpm, 180.0);

    // Test play function with a pattern
    let pattern_args = vec![Expression::Literal(Value::String("c3 d3 e3".to_string()))];
    let pattern_result = evaluator.eval_positional_function("pattern", pattern_args).unwrap();

    let play_args = vec![Expression::Literal(pattern_result)];
    let play_result = evaluator.eval_positional_function("play", play_args).unwrap();
    assert!(matches!(play_result, Value::String(_)));
    assert!(evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 1);

    // Test stop function
    let stop_args = vec![];
    let stop_result = evaluator.eval_positional_function("stop", stop_args).unwrap();
    assert!(matches!(stop_result, Value::String(_)));
    assert!(!evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 0);
}

#[test]
fn test_multiple_pattern_play() {
    let mut evaluator = Evaluator::new();

    // Create and play first pattern
    let pattern1_args = vec![Expression::Literal(Value::String("c3 d3".to_string()))];
    let pattern1 = evaluator.eval_positional_function("pattern", pattern1_args).unwrap();
    let play1_args = vec![Expression::Literal(pattern1)];
    evaluator.eval_positional_function("play", play1_args).unwrap();

    // Create and play second pattern
    let pattern2_args = vec![Expression::Literal(Value::String("e3 f3 g3".to_string()))];
    let pattern2 = evaluator.eval_positional_function("pattern", pattern2_args).unwrap();
    let play2_args = vec![Expression::Literal(pattern2)];
    evaluator.eval_positional_function("play", play2_args).unwrap();

    // Should have both patterns loaded
    assert!(evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 2);

    // Stop should clear all patterns
    evaluator.eval_positional_function("stop", vec![]).unwrap();
    assert!(!evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 0);
}

#[test]
fn test_api_error_handling() {
    let mut evaluator = Evaluator::new();

    // Test tempo with invalid argument
    let bad_tempo_args = vec![Expression::Literal(Value::String("not-a-number".to_string()))];
    let result = evaluator.eval_positional_function("tempo", bad_tempo_args);
    assert!(result.is_err());

    // Test tempo with zero
    let zero_tempo_args = vec![Expression::Literal(Value::Number(0.0))];
    let result = evaluator.eval_positional_function("tempo", zero_tempo_args);
    assert!(result.is_err());

    // Test play with non-pattern
    let bad_play_args = vec![Expression::Literal(Value::String("not-a-pattern".to_string()))];
    let result = evaluator.eval_positional_function("play", bad_play_args);
    assert!(result.is_err());

    // Test stop with arguments (should take none)
    let bad_stop_args = vec![Expression::Literal(Value::Number(1.0))];
    let result = evaluator.eval_positional_function("stop", bad_stop_args);
    assert!(result.is_err());
}

#[test]
fn test_scheduler_public_tick() {
    let mut evaluator = Evaluator::new();

    // Set up a pattern
    let pattern_args = vec![Expression::Literal(Value::String("c3 d3".to_string()))];
    let pattern = evaluator.eval_positional_function("pattern", pattern_args).unwrap();
    let play_args = vec![Expression::Literal(pattern)];
    evaluator.eval_positional_function("play", play_args).unwrap();

    // Test that public tick_scheduler method works
    let outputs = evaluator.tick_scheduler();

    // Should not error and should be possible to call
    println!("Tick outputs: {:?}", outputs);

    // Scheduler should still be in consistent state
    assert!(evaluator.scheduler.is_playing);
    assert_eq!(evaluator.scheduler.patterns.len(), 1);
}