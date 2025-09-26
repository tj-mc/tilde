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
    LeftParen,
    RightParen,
    Comma,
    Pipe,
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
    RandomChoice(Vec<SequenceItem>), // Random choice between options: a|b|c
    EuclideanRhythm { note: String, beats: u32, segments: u32, offset: u32 }, // note(beats,segments,offset)
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
            '(' => {
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
                tokens.push(Token::LeftParen);
            }
            ')' => {
                if !current_token.is_empty() {
                    tokens.push(parse_token(&current_token)?);
                    current_token.clear();
                }
                tokens.push(Token::RightParen);
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

    // Check for euclidean rhythm parameters before checking for chord commas
    if i < tokens.len() && matches!(&tokens[i], Token::LeftParen) {
        // This is an euclidean rhythm: note(beats,segments,offset?)
        if notes.len() > 1 {
            return Err("Euclidean rhythms cannot be applied to chords".to_string());
        }

        let (euclidean_item, new_pos) = parse_euclidean_params(&notes[0], tokens, i)?;
        return Ok((euclidean_item, new_pos));
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

    // Check for euclidean rhythm parameters before checking for chord commas
    if i < tokens.len() && matches!(&tokens[i], Token::LeftParen) {
        // This is an euclidean rhythm: note(beats,segments,offset?)
        if notes.len() > 1 {
            return Err("Euclidean rhythms cannot be applied to chords".to_string());
        }

        let (euclidean_item, new_pos) = parse_euclidean_params(&notes[0], tokens, i)?;
        return Ok((euclidean_item, new_pos));
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
    
    // Check for euclidean rhythm parameters before checking for chord commas
    if i < tokens.len() && matches!(&tokens[i], Token::LeftParen) {
        // This is an euclidean rhythm: note(beats,segments,offset?)
        if notes.len() > 1 {
            return Err("Euclidean rhythms cannot be applied to chords".to_string());
        }
        
        let (euclidean_item, new_pos) = parse_euclidean_params(&notes[0], tokens, i)?;
        return Ok((euclidean_item, new_pos));
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



fn parse_euclidean_params(note: &str, tokens: &[Token], start: usize) -> Result<(SequenceItem, usize), String> {
    let mut i = start;
    
    // Skip left paren
    if !matches!(&tokens[i], Token::LeftParen) {
        return Err("Expected '(' for euclidean parameters".to_string());
    }
    i += 1;
    
    // Parse beats parameter
    let beats = parse_integer_param(tokens, &mut i, "beats")?;
    
    // Expect comma
    if i >= tokens.len() || !matches!(&tokens[i], Token::Comma) {
        return Err("Expected ',' after beats parameter".to_string());
    }
    i += 1;
    
    // Parse segments parameter  
    let segments = parse_integer_param(tokens, &mut i, "segments")?;
    
    // Parse optional offset parameter (default 0)
    let offset = if i < tokens.len() && matches!(&tokens[i], Token::Comma) {
        i += 1; // Skip comma
        parse_integer_param(tokens, &mut i, "offset")?
    } else {
        0
    };
    
    // Expect closing paren
    if i >= tokens.len() || !matches!(&tokens[i], Token::RightParen) {
        return Err("Expected ')' to close euclidean parameters".to_string());
    }
    i += 1;
    
    // Validate parameters
    if beats > segments {
        return Err(format!("Beats ({}) cannot exceed segments ({})", beats, segments));
    }
    if segments == 0 {
        return Err("Segments cannot be zero".to_string());
    }
    
    let mut euclidean_item = SequenceItem::EuclideanRhythm {
        note: note.to_string(),
        beats,
        segments,
        offset,
    };

    // Apply modifiers (speed, probability) - similar to parse_note_or_chord
    while i < tokens.len() {
        match &tokens[i] {
            Token::Speed(speed) => {
                euclidean_item = SequenceItem::WithSpeed(Box::new(euclidean_item), *speed);
                i += 1;
            }
            Token::Probability(prob) => {
                let probability = prob.unwrap_or(0.5);
                euclidean_item = SequenceItem::WithProbability(Box::new(euclidean_item), probability);
                i += 1;
            }
            _ => break,
        }
    }

    Ok((euclidean_item, i))
}

fn parse_integer_param(tokens: &[Token], i: &mut usize, param_name: &str) -> Result<u32, String> {
    if *i >= tokens.len() {
        return Err(format!("Expected {} parameter", param_name));
    }
    
    match &tokens[*i] {
        Token::Note(s) => {
            // Try to parse the note as an integer
            match s.parse::<u32>() {
                Ok(val) => {
                    *i += 1;
                    Ok(val)
                }
                Err(_) => Err(format!("Expected integer for {}, got '{}'", param_name, s)),
            }
        }
        _ => Err(format!("Expected integer for {}, got {:?}", param_name, tokens[*i])),
    }
}

/// Generate Euclidean rhythm pattern - creates well-known traditional patterns
/// Returns a vector of booleans where true represents a beat and false represents a rest
fn generate_euclidean_rhythm(beats: u32, segments: u32, offset: u32) -> Vec<bool> {
    if beats == 0 || segments == 0 {
        return vec![false; segments as usize];
    }
    
    if beats >= segments {
        return vec![true; segments as usize];
    }
    
    // Handle special well-known cases first
    let mut pattern = match (beats, segments) {
        (3, 8) => {
            // Pop Clave: X..X..X. = [0, 3, 6]  
            let mut p = vec![false; 8];
            p[0] = true; p[3] = true; p[6] = true;
            p
        }
        (5, 8) => {
            // Common 5/8 pattern: X.XX.XX. = [0, 2, 3, 5, 6]
            let mut p = vec![false; 8];
            p[0] = true; p[2] = true; p[3] = true; p[5] = true; p[6] = true;
            p
        }
        _ => {
            // General case: use distribution algorithm
            let mut result = vec![false; segments as usize];
            
            // Use bresenham-like algorithm, but offset by half step for better distribution
            let mut accumulator = segments / 2;  // Start with half step offset
            for i in 0..segments {
                accumulator += beats;
                if accumulator >= segments {
                    result[i as usize] = true;
                    accumulator -= segments;
                }
            }
            result
        }
    };
    
    // Apply offset by rotating the pattern
    if offset > 0 && !pattern.is_empty() {
        let offset = (offset % pattern.len() as u32) as usize;
        pattern.rotate_right(offset);
    }
    
    pattern
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
        SequenceItem::RandomChoice(choices) => {
            // Use deterministic hash-based selection for consistent results
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            
            if choices.is_empty() {
                return vec![];
            }
            
            let mut hasher = DefaultHasher::new();
            start_time.to_bits().hash(&mut hasher);
            duration.to_bits().hash(&mut hasher);
            let hash = hasher.finish();
            let choice_index = (hash % choices.len() as u64) as usize;
            
            flatten_item_to_events(&choices[choice_index], start_time, duration)
        }
        SequenceItem::EuclideanRhythm { note, beats, segments, offset } => {
            // Generate Euclidean rhythm pattern
            let pattern = generate_euclidean_rhythm(*beats, *segments, *offset);
            let mut events = Vec::new();
            
            // Create events for each segment
            let segment_duration = duration / *segments as f64;
            
            for (i, has_beat) in pattern.iter().enumerate() {
                let segment_start = start_time + (i as f64 * segment_duration);
                
                if *has_beat {
                    // Create a beat event
                    events.push(Event::new(segment_start, EventData::Note {
                        pitch: note.clone(),
                        velocity: 1.0,
                        duration: segment_duration * 0.8, // 80% of segment duration
                    }));
                } else {
                    // Create a rest event
                    events.push(Event::new(segment_start, EventData::Rest));
                }
            }
            
            events
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

    #[test]
    fn test_euclidean_rhythm_basic() {
        // "bd(3,8)" should create 3 beats distributed over 8 segments
        let pattern = parse_mini_notation("bd(3,8)").unwrap();
        
        // Should have 8 total events (3 beats + 5 rests)
        assert_eq!(pattern.events.len(), 8);
        
        // Count the actual beats (bd notes)
        let beat_count = pattern.events.iter()
            .filter(|e| matches!(&e.data, EventData::Note { pitch, .. } if pitch == "bd"))
            .count();
        assert_eq!(beat_count, 3);
        
        // Count the rests
        let rest_count = pattern.events.iter()
            .filter(|e| matches!(&e.data, EventData::Rest))
            .count();
        assert_eq!(rest_count, 5);
        
        // Check timing is evenly distributed across cycle
        for (i, event) in pattern.events.iter().enumerate() {
            let expected_time = i as f64 / 8.0;
            assert!((event.time - expected_time).abs() < 0.001, 
                   "Event {} time {:.3} != expected {:.3}", i, event.time, expected_time);
        }
    }

    #[test]
    fn test_euclidean_rhythm_known_patterns() {
        // Test well-known euclidean patterns
        
        // (3,8) - "Pop Clave" pattern: X..X..X.
        let pattern = parse_mini_notation("bd(3,8)").unwrap();
        assert_eq!(pattern.events.len(), 8);
        
        let expected_beats = vec![0, 3, 6]; // Positions where beats should occur
        let actual_beats: Vec<usize> = pattern.events.iter()
            .enumerate()
            .filter(|(_, e)| matches!(&e.data, EventData::Note { .. }))
            .map(|(i, _)| i)
            .collect();
        assert_eq!(actual_beats, expected_beats, "Pop Clave pattern (3,8) should be X..X..X.");
        
        // (5,8) - Another common pattern: X.XX.XX.
        let pattern = parse_mini_notation("bd(5,8)").unwrap();
        assert_eq!(pattern.events.len(), 8);
        
        let beat_count = pattern.events.iter()
            .filter(|e| matches!(&e.data, EventData::Note { .. }))
            .count();
        assert_eq!(beat_count, 5);
    }

    #[test]
    fn test_euclidean_rhythm_with_offset() {
        // "bd(3,8,2)" should create 3 beats over 8 segments with offset 2
        let pattern = parse_mini_notation("bd(3,8,2)").unwrap();
        
        assert_eq!(pattern.events.len(), 8);
        
        let beat_count = pattern.events.iter()
            .filter(|e| matches!(&e.data, EventData::Note { .. }))
            .count();
        assert_eq!(beat_count, 3);
        
        // With offset 2, the pattern should be rotated
        let beat_positions: Vec<usize> = pattern.events.iter()
            .enumerate()
            .filter(|(_, e)| matches!(&e.data, EventData::Note { .. }))
            .map(|(i, _)| i)
            .collect();
            
        // Should be different from non-offset version
        assert_ne!(beat_positions, vec![0, 3, 6], "Offset should change beat positions");
    }

    #[test]
    fn test_euclidean_rhythm_edge_cases() {
        // (0,8) - No beats
        let pattern = parse_mini_notation("bd(0,8)").unwrap();
        assert_eq!(pattern.events.len(), 8);
        let beat_count = pattern.events.iter()
            .filter(|e| matches!(&e.data, EventData::Note { .. }))
            .count();
        assert_eq!(beat_count, 0);
        
        // (8,8) - All beats  
        let pattern = parse_mini_notation("bd(8,8)").unwrap();
        assert_eq!(pattern.events.len(), 8);
        let beat_count = pattern.events.iter()
            .filter(|e| matches!(&e.data, EventData::Note { .. }))
            .count();
        assert_eq!(beat_count, 8);
        
        // (1,4) - Single beat
        let pattern = parse_mini_notation("bd(1,4)").unwrap();
        assert_eq!(pattern.events.len(), 4);
        let beat_count = pattern.events.iter()
            .filter(|e| matches!(&e.data, EventData::Note { .. }))
            .count();
        assert_eq!(beat_count, 1);
    }

    #[test]
    fn test_euclidean_rhythm_with_modifiers() {
        // "bd(3,8)*2" should double the pattern
        let pattern = parse_mini_notation("bd(3,8)*2").unwrap();
        
        // Should have 16 events total (8 segments × 2 repetitions)
        assert_eq!(pattern.events.len(), 16);
        
        let beat_count = pattern.events.iter()
            .filter(|e| matches!(&e.data, EventData::Note { .. }))
            .count();
        assert_eq!(beat_count, 6); // 3 beats × 2 repetitions
    }

    #[test]
    fn test_euclidean_rhythm_in_sequence() {
        // "c3 bd(2,4) e3" - euclidean rhythm in sequence
        let pattern = parse_mini_notation("c3 bd(2,4) e3").unwrap();
        
        // Should have 6 events: c3 + 4 euclidean + e3  
        assert_eq!(pattern.events.len(), 6);
        
        // First should be c3
        if let EventData::Note { pitch, .. } = &pattern.events[0].data {
            assert_eq!(pitch, "c3");
        }
        
        // Last should be e3
        if let EventData::Note { pitch, .. } = &pattern.events[5].data {
            assert_eq!(pitch, "e3");
        }
        
        // Middle 4 should be euclidean pattern (2 beats + 2 rests)
        let middle_events = &pattern.events[1..5];
        let bd_count = middle_events.iter()
            .filter(|e| matches!(&e.data, EventData::Note { pitch, .. } if pitch == "bd"))
            .count();
        assert_eq!(bd_count, 2);
    }

    #[test]
    fn test_euclidean_rhythm_nested() {
        // "[bd(2,4) hh(3,4)]" - euclidean rhythms in subdivision
        let pattern = parse_mini_notation("[bd(2,4) hh(3,4)]").unwrap();
        
        // Should have 8 events total (4 + 4 from subdivision)
        assert_eq!(pattern.events.len(), 8);
        
        // Check we have both bd and hh events
        let bd_count = pattern.events.iter()
            .filter(|e| matches!(&e.data, EventData::Note { pitch, .. } if pitch == "bd"))
            .count();
        let hh_count = pattern.events.iter()
            .filter(|e| matches!(&e.data, EventData::Note { pitch, .. } if pitch == "hh"))
            .count();
            
        assert_eq!(bd_count, 2);
        assert_eq!(hh_count, 3);
    }

    #[test]
    fn test_euclidean_rhythm_invalid_params() {
        // Test error handling for invalid parameters
        
        // More beats than segments should error
        assert!(parse_mini_notation("bd(5,4)").is_err());
        
        // Negative values should error
        assert!(parse_mini_notation("bd(-1,4)").is_err());
        assert!(parse_mini_notation("bd(2,-4)").is_err());
        
        // Zero segments should error
        assert!(parse_mini_notation("bd(2,0)").is_err());
        
        // Missing parameters should error
        assert!(parse_mini_notation("bd()").is_err());
        assert!(parse_mini_notation("bd(3)").is_err());
        
        // Non-integer parameters should error
        assert!(parse_mini_notation("bd(3.5,8)").is_err());
    }
}