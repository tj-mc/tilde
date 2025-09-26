use crate::value::{PatternEvent, EventType};

/// Parse Strudel-compatible mini-notation
/// V1 supports:
/// - Basic notes: "c3 d3 e3"
/// - Rests: "c3 ~ d3"
/// - Basic brackets: "[c3 d3] e3"
/// - Chords: "c3,e3,g3"
/// - Speed: "c3*2" and "c3/2"
/// - Probability: "c3?" and "c3?0.8"
pub fn parse_mini_notation(input: &str) -> Result<Vec<PatternEvent>, String> {
    let input = input.trim();
    if input.is_empty() {
        return Ok(vec![]);
    }

    // Parse the top-level sequence
    let tokens = tokenize(input)?;
    let sequence = parse_sequence(&tokens)?;
    let events = flatten_to_events(&sequence, 0.0, 1.0);

    Ok(events)
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Note(String),
    Rest,
    LeftBracket,
    RightBracket,
    LeftAngle,
    RightAngle,
    LeftParen,
    RightParen,
    Comma,
    Multiply(f64),
    Divide(f64),
    Probability(Option<f64>), // None = 0.5, Some(x) = x
    Pipe,
    Exclamation(i32),
    At(f64),
}

#[derive(Debug, Clone)]
enum SequenceItem {
    Note(String),
    Rest,
    Chord(Vec<String>),
    Subdivision(Vec<SequenceItem>),
    ProbabilityChoice(Vec<SequenceItem>),
    WithSpeed(Box<SequenceItem>, f64),
    WithProbability(Box<SequenceItem>, f64),
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next(); // Skip whitespace
            }
            '[' => {
                chars.next();
                tokens.push(Token::LeftBracket);
            }
            ']' => {
                chars.next();
                tokens.push(Token::RightBracket);
            }
            '<' => {
                chars.next();
                tokens.push(Token::LeftAngle);
            }
            '>' => {
                chars.next();
                tokens.push(Token::RightAngle);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            '|' => {
                chars.next();
                tokens.push(Token::Pipe);
            }
            '~' => {
                chars.next();
                tokens.push(Token::Rest);
            }
            '*' => {
                chars.next();
                let number = read_number(&mut chars)?;
                tokens.push(Token::Multiply(number));
            }
            '/' => {
                chars.next();
                let number = read_number(&mut chars)?;
                tokens.push(Token::Divide(number));
            }
            '?' => {
                chars.next();
                // Check if there's a probability value
                if chars.peek().map_or(false, |c| c.is_ascii_digit()) {
                    let prob = read_number(&mut chars)?;
                    if prob < 0.0 || prob > 1.0 {
                        return Err("Probability must be between 0.0 and 1.0".to_string());
                    }
                    tokens.push(Token::Probability(Some(prob)));
                } else {
                    tokens.push(Token::Probability(None)); // Default 50%
                }
            }
            '!' => {
                chars.next();
                let repeats = read_integer(&mut chars)? as i32;
                tokens.push(Token::Exclamation(repeats));
            }
            '@' => {
                chars.next();
                let weight = read_number(&mut chars)?;
                tokens.push(Token::At(weight));
            }
            _ => {
                // Read a note name
                let note = read_note(&mut chars);
                if note.is_empty() {
                    return Err(format!("Unexpected character: {}", ch));
                }
                tokens.push(Token::Note(note));
            }
        }
    }

    Ok(tokens)
}

fn read_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<f64, String> {
    let mut number_str = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() || ch == '.' {
            number_str.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    if number_str.is_empty() {
        // Default to 2 for * and /
        Ok(2.0)
    } else {
        number_str.parse().map_err(|_| format!("Invalid number: {}", number_str))
    }
}

fn read_integer(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<f64, String> {
    let mut number_str = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            number_str.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    if number_str.is_empty() {
        Ok(2.0) // Default repeat
    } else {
        number_str.parse().map_err(|_| format!("Invalid integer: {}", number_str))
    }
}

fn read_note(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut note = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '#' || ch == 'b' {
            note.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    note
}

fn parse_sequence(tokens: &[Token]) -> Result<Vec<SequenceItem>, String> {
    let mut items = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        let (item, consumed) = parse_item(tokens, i)?;
        items.push(item);
        i += consumed;
    }

    Ok(items)
}

fn parse_item(tokens: &[Token], start: usize) -> Result<(SequenceItem, usize), String> {
    if start >= tokens.len() {
        return Err("Unexpected end of input".to_string());
    }

    let mut item = match &tokens[start] {
        Token::Note(note) => SequenceItem::Note(note.clone()),
        Token::Rest => SequenceItem::Rest,
        Token::LeftBracket => {
            let (sub_items, _end) = parse_bracketed_sequence(tokens, start)?;
            SequenceItem::Subdivision(sub_items)
        }
        _ => return Err("Expected note, rest, or bracket".to_string()),
    };

    let mut consumed = if matches!(&tokens[start], Token::LeftBracket) {
        // parse_bracketed_sequence already consumed the entire bracket
        let (_, end) = parse_bracketed_sequence(tokens, start)?;
        end - start + 1
    } else {
        1
    };

    // Handle modifiers (speed, probability, etc.)
    while start + consumed < tokens.len() {
        match &tokens[start + consumed] {
            Token::Multiply(speed) => {
                item = SequenceItem::WithSpeed(Box::new(item), *speed);
                consumed += 1;
            }
            Token::Divide(speed) => {
                item = SequenceItem::WithSpeed(Box::new(item), 1.0 / speed);
                consumed += 1;
            }
            Token::Probability(prob) => {
                let prob_val = prob.unwrap_or(0.5);
                item = SequenceItem::WithProbability(Box::new(item), prob_val);
                consumed += 1;
            }
            _ => break,
        }
    }

    Ok((item, consumed))
}

fn parse_bracketed_sequence(tokens: &[Token], start: usize) -> Result<(Vec<SequenceItem>, usize), String> {
    if start >= tokens.len() || !matches!(tokens[start], Token::LeftBracket) {
        return Err("Expected left bracket".to_string());
    }

    let mut items = Vec::new();
    let mut i = start + 1;
    let mut chord_notes = Vec::new();

    while i < tokens.len() {
        match &tokens[i] {
            Token::RightBracket => {
                // If we have accumulated chord notes, create a chord
                if !chord_notes.is_empty() {
                    items.push(SequenceItem::Chord(chord_notes));
                }
                return Ok((items, i));
            }
            Token::Comma => {
                // This indicates we're building a chord
                i += 1;
                continue;
            }
            Token::Note(note) => {
                // Check if next token is comma (chord) or not
                if i + 1 < tokens.len() && matches!(tokens[i + 1], Token::Comma) {
                    chord_notes.push(note.clone());
                } else if !chord_notes.is_empty() {
                    // Last note in chord
                    chord_notes.push(note.clone());
                    items.push(SequenceItem::Chord(chord_notes.clone()));
                    chord_notes.clear();
                } else {
                    // Regular note
                    items.push(SequenceItem::Note(note.clone()));
                }
                i += 1;
            }
            Token::Rest => {
                items.push(SequenceItem::Rest);
                i += 1;
            }
            Token::LeftBracket => {
                let (sub_items, end) = parse_bracketed_sequence(tokens, i)?;
                items.push(SequenceItem::Subdivision(sub_items));
                i = end + 1;
            }
            _ => {
                return Err(format!("Unexpected token in bracket: {:?}", tokens[i]));
            }
        }
    }

    Err("Unclosed bracket".to_string())
}

fn flatten_to_events(items: &[SequenceItem], start_time: f64, duration: f64) -> Vec<PatternEvent> {
    let mut events = Vec::new();
    let item_duration = duration / items.len() as f64;

    for (i, item) in items.iter().enumerate() {
        let item_start = start_time + i as f64 * item_duration;
        let mut item_events = flatten_item_to_events(item, item_start, item_duration);
        events.append(&mut item_events);
    }

    events
}

fn flatten_item_to_events(item: &SequenceItem, start_time: f64, duration: f64) -> Vec<PatternEvent> {
    match item {
        SequenceItem::Note(note) => {
            vec![PatternEvent {
                time: start_time,
                event_type: EventType::Note(note.clone()),
            }]
        }
        SequenceItem::Rest => {
            vec![PatternEvent {
                time: start_time,
                event_type: EventType::Rest,
            }]
        }
        SequenceItem::Chord(notes) => {
            // All chord notes start at the same time
            notes.iter().map(|note| PatternEvent {
                time: start_time,
                event_type: EventType::Note(note.clone()),
            }).collect()
        }
        SequenceItem::Subdivision(sub_items) => {
            flatten_to_events(sub_items, start_time, duration)
        }
        SequenceItem::WithSpeed(inner, speed) => {
            let events = flatten_item_to_events(inner, start_time, duration / speed);
            // For speed > 1, we need to repeat the pattern
            if *speed > 1.0 {
                let repeats = speed.floor() as usize;
                let mut all_events = Vec::new();
                let repeat_duration = duration / repeats as f64;

                for r in 0..repeats {
                    let repeat_start = start_time + r as f64 * repeat_duration;
                    for event in &events {
                        all_events.push(PatternEvent {
                            time: repeat_start + (event.time - start_time),
                            event_type: event.event_type.clone(),
                        });
                    }
                }
                all_events
            } else {
                events
            }
        }
        SequenceItem::WithProbability(inner, _prob) => {
            // For V1, we'll always include probabilistic events
            // Real implementation would use random number generation
            flatten_item_to_events(inner, start_time, duration)
        }
        SequenceItem::ProbabilityChoice(_choices) => {
            // For V1, we'll pick the first choice
            // Real implementation would randomly select
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_notes() {
        let events = parse_mini_notation("c3 d3 e3").unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].time, 0.0);
        assert_eq!(events[1].time, 1.0/3.0);
        assert_eq!(events[2].time, 2.0/3.0);

        match &events[0].event_type {
            EventType::Note(note) => assert_eq!(note, "c3"),
            _ => panic!("Expected note"),
        }
    }

    #[test]
    fn test_rests() {
        let events = parse_mini_notation("c3 ~ e3").unwrap();
        assert_eq!(events.len(), 3);

        match &events[1].event_type {
            EventType::Rest => {},
            _ => panic!("Expected rest"),
        }
    }

    #[test]
    fn test_chords() {
        let events = parse_mini_notation("[c3,e3,g3]").unwrap();
        assert_eq!(events.len(), 3);

        // All chord notes should start at the same time
        assert_eq!(events[0].time, 0.0);
        assert_eq!(events[1].time, 0.0);
        assert_eq!(events[2].time, 0.0);
    }

    #[test]
    fn test_subdivisions() {
        let events = parse_mini_notation("[c3 d3] e3").unwrap();
        assert_eq!(events.len(), 3);

        // First two events should be in the first half
        assert_eq!(events[0].time, 0.0);
        assert_eq!(events[1].time, 0.25);
        assert_eq!(events[2].time, 0.5);
    }
}