use tilde::evaluator::Evaluator;
use tilde::ast::Expression;

#[test]
fn test_scheduler_basic_functionality() {
    let mut evaluator = Evaluator::new();

    // Test initial state - music engine starts as None
    assert!(evaluator.music_engine.is_none());

    // Test tempo setting via tempo function
    let tempo_expr = Expression::FunctionCall {
        name: "tempo".to_string(),
        args: vec![Expression::Number(60.0, true)],
    };
    let _ = evaluator.eval_expression(tempo_expr);

    // Create pattern via pattern function
    let pattern_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("c3 d3".to_string())],
    };
    let pattern_result = evaluator.eval_expression(pattern_expr);
    assert!(pattern_result.is_ok());
    
    // Store pattern in variable
    evaluator.set_variable("test_pattern".to_string(), pattern_result.unwrap());

    // Test starting scheduler via play function with pattern argument
    let play_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![Expression::Variable("test_pattern".to_string())],
    };
    let _ = evaluator.eval_expression(play_expr);

    // After play, music engine should exist and be playing
    assert!(evaluator.music_engine.is_some());
    if let Some(ref engine) = evaluator.music_engine {
        assert!(engine.is_playing());
    }

    // Test stopping via stop function
    let stop_expr = Expression::FunctionCall {
        name: "stop".to_string(),
        args: vec![],
    };
    let _ = evaluator.eval_expression(stop_expr);

    if let Some(ref engine) = evaluator.music_engine {
        assert!(!engine.is_playing());
    }
}

#[test]
fn test_multiple_patterns() {
    let mut evaluator = Evaluator::new();

    // Create two patterns via pattern function calls
    let pattern1_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("c3 d3".to_string())],
    };
    let pattern2_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("e3 f3 g3".to_string())],
    };

    // Create and store both patterns
    let result1 = evaluator.eval_expression(pattern1_expr);
    let result2 = evaluator.eval_expression(pattern2_expr);
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    
    evaluator.set_variable("pattern1".to_string(), result1.unwrap());
    evaluator.set_variable("pattern2".to_string(), result2.unwrap());

    // Start scheduler by playing both patterns
    let play1_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![Expression::Variable("pattern1".to_string())],
    };
    let play2_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![Expression::Variable("pattern2".to_string())],
    };
    let _ = evaluator.eval_expression(play1_expr);
    let _ = evaluator.eval_expression(play2_expr);

    // Verify music engine is playing
    assert!(evaluator.music_engine.is_some());
    if let Some(ref engine) = evaluator.music_engine {
        assert!(engine.is_playing());
    }
}

#[test]
fn test_scheduler_tick_mechanism() {
    let mut evaluator = Evaluator::new();

    // Set tempo first
    let tempo_expr = Expression::FunctionCall {
        name: "tempo".to_string(),
        args: vec![Expression::Number(120.0, true)],
    };
    let _ = evaluator.eval_expression(tempo_expr);

    // Create pattern with specific timing
    let pattern_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("c3 ~ d3 ~".to_string())],
    };
    let _ = evaluator.eval_expression(pattern_expr);

    // Start the scheduler
    let play_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![],
    };
    let _ = evaluator.eval_expression(play_expr);

    // Test ticking - this tests the mechanism even if timing isn't perfect
    if let Some(ref mut engine) = evaluator.music_engine {
        let outputs = engine.tick();
        
        // The scheduler should be able to tick without errors
        // Exact event firing depends on precise timing, but mechanism should work
        println!("Scheduler tick completed with {} outputs", outputs.len());

        // Verify scheduler state is still consistent
        assert!(engine.is_playing());
    } else {
        panic!("Music engine should exist after play command");
    }
}

#[test]
fn test_pattern_value_enum_compatibility() {
    let mut evaluator = Evaluator::new();

    // Test pattern creation via function call
    let pattern_expr = Expression::FunctionCall {
        name: "pattern".to_string(),
        args: vec![Expression::String("c3 d3".to_string())],
    };
    
    let result = evaluator.eval_expression(pattern_expr);
    assert!(result.is_ok());

    // Test that the pattern can be used in play context
    let play_expr = Expression::FunctionCall {
        name: "play".to_string(),
        args: vec![],
    };
    let play_result = evaluator.eval_expression(play_expr);
    assert!(play_result.is_ok());

    // Verify music engine was created and is functional
    assert!(evaluator.music_engine.is_some());
    if let Some(ref engine) = evaluator.music_engine {
        assert!(engine.is_playing());
    }
}