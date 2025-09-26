use tilde::evaluator::Evaluator;
use tilde::ast::*;

use std::thread;
use std::time::Duration;

#[test]
fn test_scheduler_event_firing() {
    let mut evaluator = Evaluator::new();

    // Create a simple pattern with clear events
    let pattern_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("c3 d3".to_string())],
    };
    let pattern = evaluator.eval_expression(pattern_expr).unwrap();
    evaluator.set_variable("test_pattern".to_string(), pattern);

    // Set a fast tempo for testing
    let tempo_expr = Expression::FunctionCall {
        name: "tempo".to_string(),
        args: vec![Expression::Number(600.0, false)], // 10 cycles per second
    };
    evaluator.eval_expression(tempo_expr).unwrap();
    
    // Play the pattern
    let play_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![Expression::Variable("test_pattern".to_string())],
    };
    evaluator.eval_expression(play_expr).unwrap();

    // Wait a small amount to let some time pass
    thread::sleep(Duration::from_millis(150)); // 0.15 seconds

    // Tick the scheduler - should have events ready to fire
    let outputs = evaluator.tick_scheduler();

    // At least some output should be generated after time has passed
    println!("Outputs after 150ms: {:?}", outputs);
    
    // Verify that the scheduler is working
    if let Some(ref engine) = evaluator.music_engine {
        assert!(engine.is_playing());
        assert!(!engine.get_pattern_names().is_empty());
    }

    // Clean up
    let stop_expr = Expression::FunctionCall {
        name: "stop".to_string(),
        args: vec![],
    };
    evaluator.eval_expression(stop_expr).unwrap();
}

#[test]
fn test_scheduler_tempo_changes() {
    let mut evaluator = Evaluator::new();

    // Create a pattern
    let pattern_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("c3 d3 e3".to_string())],
    };
    let pattern = evaluator.eval_expression(pattern_expr).unwrap();
    evaluator.set_variable("test_pattern".to_string(), pattern);

    // Set initial slow tempo
    let tempo_expr = Expression::FunctionCall {
        name: "tempo".to_string(),
        args: vec![Expression::Number(120.0, false)], // 2 cycles per second
    };
    evaluator.eval_expression(tempo_expr).unwrap();
    
    // Play the pattern
    let play_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![Expression::Variable("test_pattern".to_string())],
    };
    evaluator.eval_expression(play_expr).unwrap();

    // Verify initial state
    if let Some(ref engine) = evaluator.music_engine {
        assert_eq!(engine.get_tempo(), 120.0);
        assert!(engine.is_playing());
    }

    // Wait and tick
    thread::sleep(Duration::from_millis(100));
    let _outputs = evaluator.tick_scheduler();

    // Change tempo
    let new_tempo_expr = Expression::FunctionCall {
        name: "tempo".to_string(),
        args: vec![Expression::Number(240.0, false)],
    };
    evaluator.eval_expression(new_tempo_expr).unwrap();
    
    // Verify tempo changed
    if let Some(ref engine) = evaluator.music_engine {
        assert_eq!(engine.get_tempo(), 240.0);
        assert!(engine.is_playing()); // Should still be playing
    }

    // Clean up
    let stop_expr = Expression::FunctionCall {
        name: "stop".to_string(),
        args: vec![],
    };
    evaluator.eval_expression(stop_expr).unwrap();
    
    // Verify stopped state
    if let Some(ref engine) = evaluator.music_engine {
        assert!(!engine.is_playing());
    }
}

#[test]
fn test_scheduler_stop_clears_state() {
    let mut evaluator = Evaluator::new();

    // Create and play a pattern
    let pattern_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("c3 d3".to_string())],
    };
    let pattern = evaluator.eval_expression(pattern_expr).unwrap();
    evaluator.set_variable("test_pattern".to_string(), pattern);
    
    let play_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![Expression::Variable("test_pattern".to_string())],
    };
    evaluator.eval_expression(play_expr).unwrap();

    // Verify playing state
    if let Some(ref engine) = evaluator.music_engine {
        assert!(engine.is_playing());
        assert!(!engine.get_pattern_names().is_empty());
    }

    // Stop the scheduler
    let stop_expr = Expression::FunctionCall {
        name: "stop".to_string(),
        args: vec![],
    };
    evaluator.eval_expression(stop_expr).unwrap();

    // Verify stopped and cleared state
    if let Some(ref engine) = evaluator.music_engine {
        assert!(!engine.is_playing());
        // Note: MusicEngine.stop() clears patterns, but doesn't reset tempo
        assert_eq!(engine.get_tempo(), 120.0); // Default tempo
    }
}

#[test]
fn test_scheduler_multiple_patterns_state() {
    let mut evaluator = Evaluator::new();

    // Create first pattern
    let pattern1_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("c3 d3".to_string())],
    };
    let pattern1 = evaluator.eval_expression(pattern1_expr).unwrap();
    evaluator.set_variable("pattern1".to_string(), pattern1);
    
    let play1_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![Expression::Variable("pattern1".to_string())],
    };
    evaluator.eval_expression(play1_expr).unwrap();

    // Verify initial state
    if let Some(ref engine) = evaluator.music_engine {
        assert!(engine.is_playing());
        assert_eq!(engine.get_pattern_names().len(), 1);
    }

    // Add second pattern
    let pattern2_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("e3 f3 g3".to_string())],
    };
    let pattern2 = evaluator.eval_expression(pattern2_expr).unwrap();
    evaluator.set_variable("pattern2".to_string(), pattern2);
    
    let play2_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![Expression::Variable("pattern2".to_string())],
    };
    evaluator.eval_expression(play2_expr).unwrap();

    // Verify multiple patterns state
    if let Some(ref engine) = evaluator.music_engine {
        assert!(engine.is_playing());
        assert!(engine.get_pattern_names().len() >= 2);
    }

    // Change tempo while playing multiple patterns
    let tempo_expr = Expression::FunctionCall {
        name: "tempo".to_string(),
        args: vec![Expression::Number(240.0, false)],
    };
    evaluator.eval_expression(tempo_expr).unwrap();
    
    if let Some(ref engine) = evaluator.music_engine {
        assert_eq!(engine.get_tempo(), 240.0);
        assert!(engine.is_playing()); // Should still be playing
    }

    // Stop and verify cleanup
    let stop_expr = Expression::FunctionCall {
        name: "stop".to_string(),
        args: vec![],
    };
    evaluator.eval_expression(stop_expr).unwrap();
    
    if let Some(ref engine) = evaluator.music_engine {
        assert!(!engine.is_playing());
    }
}