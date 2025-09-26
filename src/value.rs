use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
    Date(DateTime<Utc>),
    Error(ErrorValue),
    Pattern(PatternValue),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorValue {
    pub message: String,
    pub code: Option<String>,
    pub source: Option<String>,
    pub context: HashMap<String, Value>,
}

impl ErrorValue {
    pub fn new(message: impl Into<String>) -> Self {
        ErrorValue {
            message: message.into(),
            code: None,
            source: None,
            context: HashMap::new(),
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_context(mut self, key: impl Into<String>, value: Value) -> Self {
        self.context.insert(key.into(), value);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternValue {
    Simple {
        notation: String,
        events: Vec<PatternEvent>,
    },
    Stacked {
        patterns: Vec<PatternValue>,
    },
}

impl PatternValue {
    /// Get a display-friendly notation for this pattern
    pub fn notation(&self) -> String {
        match self {
            PatternValue::Simple { notation, .. } => notation.clone(),
            PatternValue::Stacked { patterns } => {
                let notations: Vec<String> = patterns.iter().map(|p| p.notation()).collect();
                format!("stack({})", notations.join(", "))
            }
        }
    }

    /// Get all events from this pattern (flattened for stacked patterns)
    pub fn events(&self) -> Vec<PatternEvent> {
        match self {
            PatternValue::Simple { events, .. } => events.clone(),
            PatternValue::Stacked { patterns } => {
                let mut all_events = Vec::new();
                for pattern in patterns {
                    all_events.extend(pattern.events());
                }
                all_events
            }
        }
    }

    /// Check if this pattern has any events
    pub fn is_empty(&self) -> bool {
        match self {
            PatternValue::Simple { events, .. } => events.is_empty(),
            PatternValue::Stacked { patterns } => patterns.is_empty() || patterns.iter().all(|p| p.is_empty()),
        }
    }
}

// Re-export the better event format from music module
pub use crate::music::{Event as PatternEvent, EventData as EventType};

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Object(map) => !map.is_empty(),
            Value::Date(_) => true,   // Dates are always truthy
            Value::Error(_) => false, // Errors are falsy
            Value::Pattern(p) => !p.is_empty(), // Patterns with events are truthy
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(items) => {
                let strings: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", strings.join(", "))
            }
            Value::Object(map) => {
                let pairs: Vec<String> = map.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", pairs.join(", "))
            }
            Value::Date(dt) => write!(f, "{}", dt.format("%Y-%m-%dT%H:%M:%SZ")),
            Value::Error(err) => write!(f, "Error: {}", err.message),
            Value::Pattern(pattern) => write!(f, "pattern(\"{}\")", pattern.notation()),
            Value::Null => write!(f, "null"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_display_integer() {
        let value = Value::Number(42.0);
        assert_eq!(value.to_string(), "42");
    }

    #[test]
    fn test_number_display_float() {
        let value = Value::Number(42.5);
        assert_eq!(value.to_string(), "42.5");
    }

    #[test]
    fn test_string_display() {
        let value = Value::String("hello".to_string());
        assert_eq!(value.to_string(), "hello");
    }

    #[test]
    fn test_boolean_display() {
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Boolean(false).to_string(), "false");
    }

    #[test]
    fn test_null_display() {
        let value = Value::Null;
        assert_eq!(value.to_string(), "null");
    }

    #[test]
    fn test_empty_list_display() {
        let value = Value::List(vec![]);
        assert_eq!(value.to_string(), "[]");
    }

    #[test]
    fn test_list_display() {
        let value = Value::List(vec![
            Value::Number(1.0),
            Value::String("hello".to_string()),
            Value::Boolean(true),
        ]);
        assert_eq!(value.to_string(), "[1, hello, true]");
    }

    #[test]
    fn test_empty_object_display() {
        let value = Value::Object(HashMap::new());
        assert_eq!(value.to_string(), "{}");
    }

    #[test]
    fn test_object_display() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::String("Alice".to_string()));
        map.insert("age".to_string(), Value::Number(30.0));
        let value = Value::Object(map);
        let display = value.to_string();
        // Order in HashMap is not guaranteed, so check both possibilities
        assert!(display == "{name: Alice, age: 30}" || display == "{age: 30, name: Alice}");
    }

    #[test]
    fn test_is_truthy_boolean() {
        assert!(Value::Boolean(true).is_truthy());
        assert!(!Value::Boolean(false).is_truthy());
    }

    #[test]
    fn test_is_truthy_null() {
        assert!(!Value::Null.is_truthy());
    }

    #[test]
    fn test_is_truthy_number() {
        assert!(Value::Number(42.0).is_truthy());
        assert!(Value::Number(-1.0).is_truthy());
        assert!(!Value::Number(0.0).is_truthy());
    }

    #[test]
    fn test_is_truthy_string() {
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(!Value::String("".to_string()).is_truthy());
    }

    #[test]
    fn test_is_truthy_list() {
        assert!(Value::List(vec![Value::Number(1.0)]).is_truthy());
        assert!(!Value::List(vec![]).is_truthy());
    }

    #[test]
    fn test_is_truthy_object() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        assert!(Value::Object(map).is_truthy());
        assert!(!Value::Object(HashMap::new()).is_truthy());
    }
}
