use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
    Null,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Object(map) => !map.is_empty(),
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
