use super::utils::*;
use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::music::parse_mini_notation;
use crate::value::{Value, PatternValue, EventType};

/// Create a pattern from mini-notation string
/// Usage: pattern "c3 d3 ~ e3"
pub fn eval_pattern(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let notation = extract_string_arg(&args, evaluator, "pattern")?;
    let events = parse_mini_notation(&notation)?;

    Ok(Value::Pattern(PatternValue::Simple {
        notation,
        events,
    }))
}

/// Debug output for patterns - shows detailed event breakdown
/// Usage: pattern-debug ~my-pattern
pub fn eval_pattern_debug(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let pattern_val = evaluator.eval_expression(args[0].clone())?;

    match pattern_val {
        Value::Pattern(pattern) => {
            let mut debug_output = String::new();
            debug_output.push_str(&format!("Pattern: \"{}\"\n", pattern.notation()));
            let events = pattern.events();
            debug_output.push_str(&format!("Events: {}\n", events.len()));

            for (i, event) in events.iter().enumerate() {
                debug_output.push_str(&format!("  {}: time={:.3} ", i, event.time));
                match &event.event_type {
                    EventType::Note(note) => debug_output.push_str(&format!("note={}\n", note)),
                    EventType::Rest => debug_output.push_str("rest\n"),
                }
            }

            Ok(Value::String(debug_output))
        }
        _ => Err("pattern-debug requires a pattern argument".to_string())
    }
}

/// ASCII timeline visualization for patterns
/// Usage: pattern-timeline ~my-pattern
pub fn eval_pattern_timeline(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let pattern_val = evaluator.eval_expression(args[0].clone())?;

    match pattern_val {
        Value::Pattern(pattern) => {
            let mut timeline = String::from("Time: |");
            let width = 40;
            let events = pattern.events();

            for i in 0..width {
                let time = i as f64 / width as f64;
                let mut found_event = false;

                for event in &events {
                    if (event.time - time).abs() < (1.0 / width as f64) {  // Within tolerance
                        match &event.event_type {
                            EventType::Note(_) => {
                                timeline.push('x');
                                found_event = true;
                                break;
                            }
                            EventType::Rest => {
                                timeline.push('~');
                                found_event = true;
                                break;
                            }
                        }
                    }
                }

                if !found_event {
                    timeline.push('-');
                }
            }
            timeline.push('|');
            timeline.push('\n');

            // Add note names below timeline
            timeline.push_str("Notes:");
            for i in 0..width {
                let time = i as f64 / width as f64;
                let mut found_note = false;

                for event in &events {
                    if (event.time - time).abs() < (1.0 / width as f64) {
                        match &event.event_type {
                            EventType::Note(note) => {
                                timeline.push(' ');
                                // Show first character of note if it fits
                                if let Some(first_char) = note.chars().next() {
                                    timeline.push(first_char);
                                }
                                found_note = true;
                                break;
                            }
                            EventType::Rest => {
                                timeline.push(' ');
                                timeline.push('~');
                                found_note = true;
                                break;
                            }
                        }
                    }
                }

                if !found_note {
                    timeline.push_str("  ");
                }
            }

            Ok(Value::String(timeline))
        }
        _ => Err("pattern-timeline requires a pattern argument".to_string())
    }
}

/// Get pattern notation as string
/// Usage: pattern-notation ~my-pattern
pub fn eval_pattern_notation(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let pattern_val = evaluator.eval_expression(args[0].clone())?;

    match pattern_val {
        Value::Pattern(pattern) => Ok(Value::String(pattern.notation())),
        _ => Err("pattern-notation requires a pattern argument".to_string())
    }
}

/// Get number of events in pattern
/// Usage: pattern-length ~my-pattern
pub fn eval_pattern_length(args: Vec<Expression>, evaluator: &mut Evaluator) -> Result<Value, String> {
    let pattern_val = evaluator.eval_expression(args[0].clone())?;

    match pattern_val {
        Value::Pattern(pattern) => Ok(Value::Number(pattern.events().len() as f64)),
        _ => Err("pattern-length requires a pattern argument".to_string())
    }
}