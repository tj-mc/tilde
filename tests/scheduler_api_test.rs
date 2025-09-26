use tilde::evaluator::Evaluator;
use tilde::ast::*;
use tilde::value::Value;

#[test]
fn test_play_stop_tempo_api() {
    let mut evaluator = Evaluator::new();

    // Test tempo function
    let tempo_expr = Expression::FunctionCall {
        name: "tempo".to_string(),
        args: vec![Expression::Number(180.0, false)],
    };
    let result = evaluator.eval_expression(tempo_expr).unwrap();
    assert!(matches!(result, Value::String(_)));
    
    // Check that tempo was set by checking the music engine
    if let Some(ref engine) = evaluator.music_engine {
        assert_eq!(engine.get_tempo(), 180.0);
    }

    // Test pattern creation and play function
    let pattern_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("c3 d3 e3".to_string())],
    };
    let pattern_result = evaluator.eval_expression(pattern_expr).unwrap();
    
    // Store the pattern in a variable for play function
    evaluator.set_variable("test_pattern".to_string(), pattern_result);
    
    let play_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![Expression::Variable("test_pattern".to_string())],
    };
    let play_result = evaluator.eval_expression(play_expr).unwrap();
    assert!(matches!(play_result, Value::String(_)));
    
    // Check that music engine is playing and has patterns
    if let Some(ref engine) = evaluator.music_engine {
        assert!(engine.is_playing());
        assert!(!engine.get_pattern_names().is_empty());
    }

    // Test stop function
    let stop_expr = Expression::FunctionCall {
        name: "stop".to_string(),
        args: vec![],
    };
    let stop_result = evaluator.eval_expression(stop_expr).unwrap();
    assert!(matches!(stop_result, Value::String(_)));
    
    // Check that music engine stopped
    if let Some(ref engine) = evaluator.music_engine {
        assert!(!engine.is_playing());
    }
}

#[test]
fn test_multiple_pattern_play() {
    let mut evaluator = Evaluator::new();

    // Create and play first pattern
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

    // Create and play second pattern
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

    // Check that we have multiple patterns playing
    if let Some(ref engine) = evaluator.music_engine {
        assert!(engine.is_playing());
        assert!(engine.get_pattern_names().len() >= 2);
    }

    // Stop all patterns
    let stop_expr = Expression::FunctionCall {
        name: "stop".to_string(),
        args: vec![],
    };
    evaluator.eval_expression(stop_expr).unwrap();
    
    // Check that all patterns stopped
    if let Some(ref engine) = evaluator.music_engine {
        assert!(!engine.is_playing());
    }
}

#[test]
fn test_error_handling() {
    let mut evaluator = Evaluator::new();

    // Test tempo with invalid argument (should handle gracefully)
    let bad_tempo_expr = Expression::FunctionCall {
        name: "tempo".to_string(),
        args: vec![Expression::String("not-a-number".to_string())],
    };
    let result = evaluator.eval_expression(bad_tempo_expr);
    assert!(result.is_err());

    // Test tempo with zero (should handle gracefully)
    let zero_tempo_expr = Expression::FunctionCall {
        name: "tempo".to_string(),
        args: vec![Expression::Number(0.0, false)],
    };
    let result = evaluator.eval_expression(zero_tempo_expr);
    assert!(result.is_err());

    // Test play with invalid pattern (should handle gracefully)
    let bad_play_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![Expression::String("not-a-pattern".to_string())],
    };
    let result = evaluator.eval_expression(bad_play_expr);
    assert!(result.is_err());

    // Test stop with arguments (should handle gracefully)
    let bad_stop_expr = Expression::FunctionCall {
        name: "stop".to_string(),
        args: vec![Expression::Number(1.0, false)],
    };
    let result = evaluator.eval_expression(bad_stop_expr);
    assert!(result.is_err());
}

#[test]
fn test_scheduler_state_consistency() {
    let mut evaluator = Evaluator::new();

    // Create a pattern and play it
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

    // Verify initial state
    if let Some(ref engine) = evaluator.music_engine {
        assert!(engine.is_playing());
        assert_eq!(engine.get_pattern_names().len(), 1);
    }

    // Change tempo and verify playing continues
    let tempo_expr = Expression::FunctionCall {
        name: "tempo".to_string(),
        args: vec![Expression::Number(240.0, false)],
    };
    evaluator.eval_expression(tempo_expr).unwrap();
    
    if let Some(ref engine) = evaluator.music_engine {
        assert_eq!(engine.get_tempo(), 240.0);
        assert!(engine.is_playing()); // Should still be playing
    }

    // Stop and verify final state
    let stop_expr = Expression::FunctionCall {
        name: "stop".to_string(),
        args: vec![],
    };
    evaluator.eval_expression(stop_expr).unwrap();
    
    if let Some(ref engine) = evaluator.music_engine {
        assert!(!engine.is_playing());
    }
}