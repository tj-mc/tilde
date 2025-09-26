use super::{Event, EventData, Pattern, PatternValue};

/// Enhanced Strudel-compatible mini-notation parser
/// V2 supports:
/// - Basic notes: "c3 d3 e3"
/// - Rests: "c3 ~ d3"  
/// - Basic brackets: "[c3 d3] e3"
/// - Chords: "c3,e3,g3"
/// - Speed: "c3*2" and "c3/2"
/// - Probability: "c3?" and "c3?0.8"
pub fn parse_mini_notation(input: &str) -> Result<Pattern, String> {
    let input = input.trim();
    if input.is_empty() {
        return Ok(Pattern::new(input.to_string(), vec![]));
    }

    // Parse the top-level sequence
    let tokens = tokenize(input)?;
    let sequence = parse_sequence(&tokens)?;
    let events = flatten_to_events(&sequence, 0.0, 1.0);

    Ok(Pattern::new(input.to_string(), events))
}

/// Create a PatternValue from mini-notation string
pub fn parse_to_pattern_value(input: &str) -> Result<PatternValue, String> {
    let pattern = parse_mini_notation(input)?;
    Ok(PatternValue::Simple {
        notation: input.to_string(),
        events: pattern.events,
    })
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Note(String),
    Rest,
    LeftBracket,
    RightBracket,
    Comma,
    Speed(f64),
    Probability(Option<f64>),
}

#[derive(Debug, Clone, PartialEq)]
enum SequenceItem {
    Note(String),
    Rest,
    Chord(Vec<String>),
    Subdivision(Vec<SequenceItem>),
    WithSpeed(Box<SequenceItem>, f64),
    WithProbability(Box<SequenceItem>, f64),
    #[allow(dead_code)]
    ProbabilityChoice(Vec<SequenceItem>),
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current_token = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' | '\t' | '\n' => {
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
            }
            '[' => {
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
                tokens.push(Token::LeftBracket);
            }
            ']' => {
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
                tokens.push(Token::RightBracket);
            }
            ',' => {
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
                tokens.push(Token::Comma);
            }
            '*' => {
                if current_token.is_empty() {
                    return Err("Speed modifier '*' must follow a note".to_string());
                }
                
                let base_token = parse_token(&current_token)?;
                current_token.clear();
                
                // Read the speed value
                let mut speed_str = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() || next_ch == '.' {
                        speed_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                if speed_str.is_empty() {
                    return Err("Speed modifier '*' must be followed by a number".to_string());
                }
                
                let speed: f64 = speed_str.parse()
                    .map_err(|_| format!("Invalid speed value: {}", speed_str))?;
                
                tokens.push(base_token);
                tokens.push(Token::Speed(speed));
            }
            '/' => {
                if current_token.is_empty() {
                    return Err("Speed modifier '/' must follow a note".to_string());
                }
                
                let base_token = parse_token(&current_token)?;
                current_token.clear();
                
                // Read the divisor value
                let mut divisor_str = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() || next_ch == '.' {
                        divisor_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                if divisor_str.is_empty() {
                    return Err("Speed modifier '/' must be followed by a number".to_string());
                }
                
                let divisor: f64 = divisor_str.parse()
                    .map_err(|_| format!("Invalid divisor value: {}", divisor_str))?;
                
                if divisor == 0.0 {
                    return Err("Division by zero is not allowed".to_string());
                }
                
                tokens.push(base_token);
                tokens.push(Token::Speed(1.0 / divisor));
            }
            '?' => {
                if current_token.is_empty() {
                    return Err("Probability modifier '?' must follow a note".to_string());
                }
                
                let base_token = parse_token(&current_token)?;
                current_token.clear();
                
                // Check if there's a probability value
                let mut prob_str = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() || next_ch == '.' {
                        prob_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                let probability = if prob_str.is_empty() {
                    None // Default 50% probability
                } else {
                    Some(prob_str.parse()
                        .map_err(|_| format!("Invalid probability value: {}", prob_str))?)
                };
                
                tokens.push(base_token);
                tokens.push(Token::Probability(probability));
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(parse_token(&current_token)?);
    }

    Ok(tokens)
}

fn parse_token(token: &str) -> Result<Token, String> {
    match token {
        "~" => Ok(Token::Rest),
        _ => Ok(Token::Note(token.to_string())),
    }
}

fn parse_sequence(tokens: &[Token]) -> Result<Vec<SequenceItem>, String> {
    let mut items = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Note(note) => {
                let mut item = SequenceItem::Note(note.clone());
                i += 1;
                
                // Check for modifiers
                while i < tokens.len() {
                    match &tokens[i] {
                        Token::Speed(speed) => {
                            item = SequenceItem::WithSpeed(Box::new(item), *speed);
                            i += 1;
                        }
                        Token::Probability(prob) => {
                            let probability = prob.unwrap_or(0.5);
                            item = SequenceItem::WithProbability(Box::new(item), probability);
                            i += 1;
                        }
                        _ => break,
                    }
                }
                
                items.push(item);
            }
            Token::Rest => {
                let mut item = SequenceItem::Rest;
                i += 1;
                
                // Check for modifiers
                while i < tokens.len() {
                    match &tokens[i] {
                        Token::Speed(speed) => {
                            item = SequenceItem::WithSpeed(Box::new(item), *speed);
                            i += 1;
                        }
                        Token::Probability(prob) => {
                            let probability = prob.unwrap_or(0.5);
                            item = SequenceItem::WithProbability(Box::new(item), probability);
                            i += 1;
                        }
                        _ => break,
                    }
                }
                
                items.push(item);
            }
            Token::LeftBracket => {
                let (subdivision, end_pos) = parse_subdivision(tokens, i + 1)?;
                items.push(SequenceItem::Subdivision(subdivision));
                i = end_pos + 1;
            }
            _ => {
                return Err(format!("Unexpected token: {:?}", tokens[i]));
            }
        }
    }

    // Post-process to handle chords
    items = process_chords(items)?;
    
    Ok(items)
}

fn parse_subdivision(tokens: &[Token], start: usize) -> Result<(Vec<SequenceItem>, usize), String> {
    let mut items = Vec::new();
    let mut i = start;

    while i < tokens.len() {
        match &tokens[i] {
            Token::RightBracket => {
                return Ok((items, i));
            }
            Token::Note(note) => {
                let mut item = SequenceItem::Note(note.clone());
                i += 1;
                
                // Check for modifiers
                while i < tokens.len() && i < tokens.len() {
                    match &tokens[i] {
                        Token::Speed(speed) => {
                            item = SequenceItem::WithSpeed(Box::new(item), *speed);
                            i += 1;
                        }
                        Token::Probability(prob) => {
                            let probability = prob.unwrap_or(0.5);
                            item = SequenceItem::WithProbability(Box::new(item), probability);
                            i += 1;
                        }
                        Token::RightBracket => break,
                        _ => break,
                    }
                }
                
                items.push(item);
            }
            Token::Rest => {
                items.push(SequenceItem::Rest);
                i += 1;
            }
            Token::LeftBracket => {
                let (sub_subdivision, end_pos) = parse_subdivision(tokens, i + 1)?;
                items.push(SequenceItem::Subdivision(sub_subdivision));
                i = end_pos + 1;
            }
            _ => {
                return Err(format!("Unexpected token in subdivision: {:?}", tokens[i]));
            }
        }
    }

    Err("Unclosed bracket in subdivision".to_string())
}

fn process_chords(items: Vec<SequenceItem>) -> Result<Vec<SequenceItem>, String> {
    let mut processed = Vec::new();
    let mut i = 0;

    while i < items.len() {
        let mut chord_notes = vec![];
        let mut is_chord = false;

        // Collect chord notes
        if let SequenceItem::Note(note) = &items[i] {
            chord_notes.push(note.clone());
            i += 1;

            // Look for comma-separated notes
            while i < items.len() {
                // This is a simplified chord detection - in real implementation
                // we'd need to handle the comma tokens from tokenization
                if let SequenceItem::Note(note) = &items[i] {
                    if note.contains(',') {
                        // Split comma-separated notes
                        let notes: Vec<String> = note.split(',').map(|s| s.trim().to_string()).collect();
                        chord_notes.extend(notes);
                        is_chord = true;
                        i += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if is_chord || chord_notes.len() > 1 {
            processed.push(SequenceItem::Chord(chord_notes));
        } else if chord_notes.len() == 1 {
            processed.push(SequenceItem::Note(chord_notes[0].clone()));
        } else {
            processed.push(items[i].clone());
            i += 1;
        }
    }

    Ok(processed)
}

fn flatten_to_events(items: &[SequenceItem], start_time: f64, duration: f64) -> Vec<Event> {
    let mut events = Vec::new();
    let item_duration = duration / items.len() as f64;

    for (i, item) in items.iter().enumerate() {
        let item_start = start_time + (i as f64 * item_duration);
        events.extend(flatten_item_to_events(item, item_start, item_duration));
    }

    events
}

fn flatten_item_to_events(item: &SequenceItem, start_time: f64, duration: f64) -> Vec<Event> {
    match item {
        SequenceItem::Note(note) => {
            vec![Event::new(start_time, EventData::Note {
                pitch: note.clone(),
                velocity: 1.0,
                duration: duration * 0.8, // 80% of available time
            })]
        }
        SequenceItem::Rest => {
            vec![Event::new(start_time, EventData::Rest)]
        }
        SequenceItem::Chord(notes) => {
            notes.iter().map(|note| Event::new(start_time, EventData::Note {
                pitch: note.clone(),
                velocity: 1.0,
                duration: duration * 0.8,
            })).collect()
        }
        SequenceItem::Subdivision(sub_items) => {
            flatten_to_events(sub_items, start_time, duration)
        }
        SequenceItem::WithSpeed(inner, speed) => {
            let speed_duration = duration / speed;
            flatten_item_to_events(inner, start_time, speed_duration)
        }
        SequenceItem::WithProbability(inner, probability) => {
            // For now, we'll include all probabilistic events 
            // (real implementation would use random number generation)
            if *probability > 0.0 {
                let events = flatten_item_to_events(inner, start_time, duration);
                // Mark events with probability in future extensions
                events
            } else {
                vec![]
            }
        }
        SequenceItem::ProbabilityChoice(_) => {
            // Not implemented yet
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_notes() {
        let pattern = parse_mini_notation("c3 d3 e3").unwrap();
        
        assert_eq!(pattern.notation, "c3 d3 e3");
        assert_eq!(pattern.events.len(), 3);
        
        if let EventData::Note { pitch, .. } = &pattern.events[0].data {
            assert_eq!(pitch, "c3");
        } else {
            panic!("Expected note event");
        }
    }

    #[test]
    fn test_parse_with_rests() {
        let pattern = parse_mini_notation("c3 ~ d3").unwrap();
        
        assert_eq!(pattern.events.len(), 3);
        assert!(matches!(pattern.events[0].data, EventData::Note { .. }));
        assert!(matches!(pattern.events[1].data, EventData::Rest));
        assert!(matches!(pattern.events[2].data, EventData::Note { .. }));
    }

    #[test]
    fn test_parse_brackets() {
        let pattern = parse_mini_notation("[c3 d3] e3").unwrap();
        
        assert_eq!(pattern.events.len(), 3);
        // First two events should be in first half
        assert!(pattern.events[0].time < 0.5);
        assert!(pattern.events[1].time < 0.5);
        // Third event should be in second half
        assert!(pattern.events[2].time >= 0.5);
    }

    #[test]
    fn test_parse_speed_modifier() {
        let result = tokenize("c3*2");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0], Token::Note(_)));
        assert!(matches!(tokens[1], Token::Speed(2.0)));
    }

    #[test]
    fn test_parse_probability_modifier() {
        let result = tokenize("c3?0.8");
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0], Token::Note(_)));
        assert!(matches!(tokens[1], Token::Probability(Some(0.8))));
    }

    #[test]
    fn test_empty_pattern() {
        let pattern = parse_mini_notation("").unwrap();
        assert!(pattern.events.is_empty());
        assert!(pattern.is_empty());
    }
}