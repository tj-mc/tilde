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
    LeftAngle,
    RightAngle,
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
    AngleSequence(Vec<SequenceItem>), // Auto-length sequence <items>
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
            '<' => {
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
                tokens.push(Token::LeftAngle);
            }
            '>' => {
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
                tokens.push(Token::RightAngle);
            }
            '*' => {
                // Push any current token first
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
                
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
                
                tokens.push(Token::Speed(speed));
            }
            '/' => {
                // Push any current token first
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
                
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
                
                tokens.push(Token::Speed(1.0 / divisor));
            }
            '?' => {
                // Push any current token first
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
                
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
            Token::Note(_) => {
                // Check if this starts a chord (look ahead for commas)
                let (item, new_pos) = parse_note_or_chord(tokens, i)?;
                items.push(item);
                i = new_pos;
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
                let mut item = SequenceItem::Subdivision(subdivision);
                i = end_pos + 1;
                
                // Check for modifiers after the subdivision
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
            Token::LeftAngle => {
                let (angle_seq, end_pos) = parse_angle_sequence(tokens, i + 1)?;
                let mut item = SequenceItem::AngleSequence(angle_seq);
                i = end_pos + 1;
                
                // Check for modifiers after the angle sequence
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
            _ => {
                return Err(format!("Unexpected token: {:?}", tokens[i]));
            }
        }
    }

    // Chords are now handled directly during parsing
    
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
            Token::Note(_) => {
                let (item, new_pos) = parse_note_or_chord_in_subdivision(tokens, i)?;
                items.push(item);
                i = new_pos;
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

fn parse_angle_sequence(tokens: &[Token], start: usize) -> Result<(Vec<SequenceItem>, usize), String> {
    let mut items = Vec::new();
    let mut i = start;

    while i < tokens.len() {
        match &tokens[i] {
            Token::RightAngle => {
                return Ok((items, i));
            }
            Token::Note(_) => {
                let (item, new_pos) = parse_note_or_chord_in_angle_sequence(tokens, i)?;
                items.push(item);
                i = new_pos;
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
                return Err(format!("Unexpected token in angle sequence: {:?}", tokens[i]));
            }
        }
    }

    Err("Unclosed angle bracket in sequence".to_string())
}

fn parse_note_or_chord_in_angle_sequence(tokens: &[Token], start: usize) -> Result<(SequenceItem, usize), String> {
    let mut notes = Vec::new();
    let mut i = start;
    
    // Parse the first note
    if let Token::Note(note) = &tokens[i] {
        notes.push(note.clone());
        i += 1;
    } else {
        return Err("Expected note".to_string());
    }
    
    // Look for comma-separated additional notes (but stop at right angle bracket)
    while i < tokens.len() && matches!(&tokens[i], Token::Comma) {
        i += 1; // Skip comma
        
        if i >= tokens.len() || matches!(&tokens[i], Token::RightAngle) {
            return Err("Expected note after comma".to_string());
        }
        
        if let Token::Note(note) = &tokens[i] {
            notes.push(note.clone());
            i += 1;
        } else {
            return Err("Expected note after comma".to_string());
        }
    }
    
    // Create the appropriate item
    let mut item = if notes.len() == 1 {
        SequenceItem::Note(notes[0].clone())
    } else {
        SequenceItem::Chord(notes)
    };
    
    // Apply modifiers (but stop at right angle bracket)
    while i < tokens.len() && !matches!(&tokens[i], Token::RightAngle) {
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
    
    Ok((item, i))
}

fn parse_note_or_chord_in_subdivision(tokens: &[Token], start: usize) -> Result<(SequenceItem, usize), String> {
    let mut notes = Vec::new();
    let mut i = start;
    
    // Parse the first note
    if let Token::Note(note) = &tokens[i] {
        notes.push(note.clone());
        i += 1;
    } else {
        return Err("Expected note".to_string());
    }
    
    // Look for comma-separated additional notes (but stop at right bracket)
    while i < tokens.len() && matches!(&tokens[i], Token::Comma) {
        i += 1; // Skip comma
        
        if i >= tokens.len() || matches!(&tokens[i], Token::RightBracket) {
            return Err("Expected note after comma".to_string());
        }
        
        if let Token::Note(note) = &tokens[i] {
            notes.push(note.clone());
            i += 1;
        } else {
            return Err("Expected note after comma".to_string());
        }
    }
    
    // Create the appropriate item
    let mut item = if notes.len() == 1 {
        SequenceItem::Note(notes[0].clone())
    } else {
        SequenceItem::Chord(notes)
    };
    
    // Apply modifiers (but stop at right bracket)
    while i < tokens.len() && !matches!(&tokens[i], Token::RightBracket) {
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
    
    Ok((item, i))
}

fn parse_note_or_chord(tokens: &[Token], start: usize) -> Result<(SequenceItem, usize), String> {
    let mut notes = Vec::new();
    let mut i = start;
    
    // Parse the first note
    if let Token::Note(note) = &tokens[i] {
        notes.push(note.clone());
        i += 1;
    } else {
        return Err("Expected note".to_string());
    }
    
    // Look for comma-separated additional notes
    while i < tokens.len() && matches!(&tokens[i], Token::Comma) {
        i += 1; // Skip comma
        
        if i >= tokens.len() {
            return Err("Expected note after comma".to_string());
        }
        
        if let Token::Note(note) = &tokens[i] {
            notes.push(note.clone());
            i += 1;
        } else {
            return Err("Expected note after comma".to_string());
        }
    }
    
    // Create the appropriate item
    let mut item = if notes.len() == 1 {
        SequenceItem::Note(notes[0].clone())
    } else {
        SequenceItem::Chord(notes)
    };
    
    // Apply modifiers (speed, probability)
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
    
    Ok((item, i))
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
        SequenceItem::AngleSequence(sub_items) => {
            // Angle sequences spread their items across the full duration
            // This is equivalent to [items]/N where N is the number of items
            flatten_to_events(sub_items, start_time, duration)
        }
        SequenceItem::WithSpeed(inner, speed) => {
            if *speed > 1.0 {
                // Multiplication: repeat the pattern *speed times within the duration
                let mut events = Vec::new();
                let repetitions = *speed as usize;
                let repetition_duration = duration / *speed;
                
                for rep in 0..repetitions {
                    let rep_start_time = start_time + (rep as f64 * repetition_duration);
                    events.extend(flatten_item_to_events(inner, rep_start_time, repetition_duration));
                }
                events
            } else {
                // Division: stretch the pattern over longer duration
                let stretched_duration = duration / speed;
                flatten_item_to_events(inner, start_time, stretched_duration)
            }
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

    #[test]
    fn test_toplevel_chord_syntax() {
        // Test single top-level chord - Strudel style
        let pattern = parse_mini_notation("c3,e3,g3").unwrap();
        
        assert_eq!(pattern.notation, "c3,e3,g3");
        assert_eq!(pattern.events.len(), 3);
        
        // All notes should start at time 0.0 (simultaneous)
        for event in &pattern.events {
            assert_eq!(event.time, 0.0);
        }
        
        // Check the pitches
        let pitches: Vec<String> = pattern.events.iter()
            .filter_map(|e| {
                if let EventData::Note { pitch, .. } = &e.data {
                    Some(pitch.clone())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(pitches, vec!["c3", "e3", "g3"]);
    }

    #[test]
    fn test_toplevel_chord_sequence() {
        // Test multiple top-level chords in sequence
        let pattern = parse_mini_notation("c3,e3,g3 f3,a3,c4").unwrap();
        
        assert_eq!(pattern.events.len(), 6);
        
        // First chord at time 0.0
        let first_chord: Vec<_> = pattern.events.iter()
            .filter(|e| e.time == 0.0)
            .collect();
        assert_eq!(first_chord.len(), 3);
        
        // Second chord at time 0.5
        let second_chord: Vec<_> = pattern.events.iter()
            .filter(|e| e.time == 0.5)
            .collect();
        assert_eq!(second_chord.len(), 3);
    }

    #[test]
    fn test_mixed_notes_and_chords() {
        // Test mixing single notes and chords
        let pattern = parse_mini_notation("c3 e3,g3 b3").unwrap();
        
        assert_eq!(pattern.events.len(), 4);
        
        // Single note at 0.0
        assert_eq!(pattern.events[0].time, 0.0);
        
        // Chord at 1/3 (0.333...)
        let chord_events: Vec<_> = pattern.events.iter()
            .filter(|e| (e.time - 1.0/3.0).abs() < 0.001)
            .collect();
        assert_eq!(chord_events.len(), 2);
        
        // Single note at 2/3 (0.666...)
        let single_events: Vec<_> = pattern.events.iter()
            .filter(|e| (e.time - 2.0/3.0).abs() < 0.001)
            .collect();
        assert_eq!(single_events.len(), 1);
    }

    #[test]
    fn test_chord_compatibility_both_syntaxes() {
        // Both syntaxes should produce identical results
        let bracketed = parse_mini_notation("[c3,e3,g3]").unwrap();
        let toplevel = parse_mini_notation("c3,e3,g3").unwrap();
        
        assert_eq!(bracketed.events.len(), toplevel.events.len());
        
        // Both should have all events at time 0.0
        for event in &bracketed.events {
            assert_eq!(event.time, 0.0);
        }
        for event in &toplevel.events {
            assert_eq!(event.time, 0.0);
        }
    }

    #[test]
    fn test_angle_brackets_basic() {
        // <c3 d3 e3> should spread across full cycle automatically
        let angle = parse_mini_notation("<c3 d3 e3>").unwrap();
        
        assert_eq!(angle.events.len(), 3);
        
        // Should spread across full cycle: 0, 1/3, 2/3
        assert_eq!(angle.events[0].time, 0.0);
        assert!((angle.events[1].time - 1.0/3.0).abs() < 0.001);
        assert!((angle.events[2].time - 2.0/3.0).abs() < 0.001);
        
        // Compare to regular sequence which spreads differently
        let regular = parse_mini_notation("c3 d3 e3").unwrap();
        assert_eq!(regular.events.len(), 3);
        
        // Regular sequence also spreads across cycle (1/3 each)
        // The key difference is semantic - angle brackets are explicit
        assert_eq!(regular.events[0].time, 0.0);
        assert!((regular.events[1].time - 1.0/3.0).abs() < 0.001);
        assert!((regular.events[2].time - 2.0/3.0).abs() < 0.001);
    }

    #[test]
    fn test_angle_brackets_different_lengths() {
        // Different number of events should automatically adjust timing
        let three_notes = parse_mini_notation("<c3 d3 e3>").unwrap();
        let five_notes = parse_mini_notation("<c3 d3 e3 f3 g3>").unwrap();
        
        // Three notes: 0, 1/3, 2/3
        assert_eq!(three_notes.events.len(), 3);
        assert!((three_notes.events[1].time - 1.0/3.0).abs() < 0.001);
        
        // Five notes: 0, 1/5, 2/5, 3/5, 4/5
        assert_eq!(five_notes.events.len(), 5);
        assert!((five_notes.events[1].time - 1.0/5.0).abs() < 0.001);
        assert!((five_notes.events[4].time - 4.0/5.0).abs() < 0.001);
    }

    #[test]
    fn test_angle_brackets_with_multiplication() {
        // <c3 d3 e3>*2 should play pattern twice per cycle
        let doubled = parse_mini_notation("<c3 d3 e3>*2").unwrap();
        
        // Should have 6 events total (3 notes × 2 repetitions)
        assert_eq!(doubled.events.len(), 6);
        
        // First repetition
        assert_eq!(doubled.events[0].time, 0.0);
        assert!((doubled.events[1].time - 1.0/6.0).abs() < 0.001);
        assert!((doubled.events[2].time - 2.0/6.0).abs() < 0.001);
        
        // Second repetition  
        assert!((doubled.events[3].time - 3.0/6.0).abs() < 0.001);
        assert!((doubled.events[4].time - 4.0/6.0).abs() < 0.001);
        assert!((doubled.events[5].time - 5.0/6.0).abs() < 0.001);
    }

    #[test]
    fn test_angle_brackets_with_chords() {
        // <c3,e3 d3 f3,a3> - chords within angle brackets
        let pattern = parse_mini_notation("<c3,e3 d3 f3,a3>").unwrap();
        
        // Should have 5 events: chord(2) + note(1) + chord(2)
        assert_eq!(pattern.events.len(), 5);
        
        // First chord at time 0
        let first_chord: Vec<_> = pattern.events.iter()
            .filter(|e| e.time == 0.0)
            .collect();
        assert_eq!(first_chord.len(), 2);
        
        // Single note at time 1/3
        let middle_notes: Vec<_> = pattern.events.iter()
            .filter(|e| (e.time - 1.0/3.0).abs() < 0.001)
            .collect();
        assert_eq!(middle_notes.len(), 1);
        
        // Second chord at time 2/3
        let last_chord: Vec<_> = pattern.events.iter()
            .filter(|e| (e.time - 2.0/3.0).abs() < 0.001)
            .collect();
        assert_eq!(last_chord.len(), 2);
    }

    #[test]
    fn test_angle_brackets_with_rests() {
        // <c3 ~ d3> with rests
        let pattern = parse_mini_notation("<c3 ~ d3>").unwrap();
        
        assert_eq!(pattern.events.len(), 3);
        
        // Check event types
        if let EventData::Note { pitch, .. } = &pattern.events[0].data {
            assert_eq!(pitch, "c3");
        }
        assert!(matches!(pattern.events[1].data, EventData::Rest));
        if let EventData::Note { pitch, .. } = &pattern.events[2].data {
            assert_eq!(pitch, "d3");
        }
        
        // Check timing
        assert_eq!(pattern.events[0].time, 0.0);
        assert!((pattern.events[1].time - 1.0/3.0).abs() < 0.001);
        assert!((pattern.events[2].time - 2.0/3.0).abs() < 0.001);
    }
}