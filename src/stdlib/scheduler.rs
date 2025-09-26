use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Play a pattern using the internal scheduler
pub fn eval_play(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("play requires exactly 1 argument: pattern".to_string());
    }

    let pattern = evaluator.eval_expression(args[0].clone())?;
    match pattern {
        Value::Pattern(ref pattern_value) => {
            let engine = evaluator.ensure_music_engine();
            let pattern_name = format!("pattern_{}", engine.get_pattern_names().len());
            
            engine.add_pattern_value(pattern_name, pattern_value)?;
            if !engine.is_playing() {
                engine.start()?;
            }
            Ok(Value::String("Pattern added to scheduler".to_string()))
        }
        _ => Err("play argument must be a pattern".to_string()),
    }
}

/// Stop the pattern scheduler
pub fn eval_stop(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("stop takes no arguments".to_string());
    }

    if let Some(ref mut engine) = evaluator.music_engine {
        engine.stop();
    }
    Ok(Value::String("Scheduler stopped".to_string()))
}

/// Set the scheduler tempo in cycles per minute (CPM)
pub fn eval_tempo(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("tempo requires exactly 1 argument: cpm".to_string());
    }

    let tempo = evaluator.eval_expression(args[0].clone())?;
    match tempo {
        Value::Number(cpm) => {
            if cpm <= 0.0 {
                return Err("tempo must be positive".to_string());
            }
            let engine = evaluator.ensure_music_engine();
            engine.set_tempo(cpm);
            Ok(Value::String(format!("Tempo set to {} CPM", cpm)))
        }
        _ => Err("tempo argument must be a number".to_string()),
    }
}

/// Debug function to inspect scheduler state
pub fn eval_scheduler_debug(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("__scheduler-debug takes no arguments".to_string());
    }

    if let Some(ref engine) = evaluator.music_engine {
        let stats = engine.get_stats();
        let debug_info = format!(
            "Scheduler Debug:\n\
            - Playing: {}\n\
            - Tempo: {} CPM\n\
            - Current time: {:.3}\n\
            - Active patterns: {}\n\
            - Outputs: {} ({})\n\
            - Output buffer: {} items",
            stats.scheduler_stats.is_playing,
            stats.scheduler_stats.cpm,
            stats.scheduler_stats.current_time,
            stats.scheduler_stats.active_patterns,
            stats.output_count,
            stats.output_names.join(", "),
            evaluator.output_buffer.len()
        );
        Ok(Value::String(debug_info))
    } else {
        Ok(Value::String("Music engine not initialized".to_string()))
    }
}

/// Debug function to manually tick the scheduler
pub fn eval_scheduler_tick(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("__scheduler-tick takes no arguments".to_string());
    }

    let outputs = evaluator.tick_scheduler();
    let result = if outputs.is_empty() {
        "No events fired".to_string()
    } else {
        format!("Events fired: {}", outputs.join(", "))
    };

    Ok(Value::String(result))
}