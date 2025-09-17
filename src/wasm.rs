use wasm_bindgen::prelude::*;
use crate::{evaluator::Evaluator, parser::Parser, value::Value};

// Import JavaScript functions we might need
#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // JavaScript function to handle user input (will be implemented in JS)
    #[wasm_bindgen(js_namespace = window, js_name = tailsPrompt)]
    pub fn prompt_user(message: &str) -> String;

    // JavaScript function to handle HTTP fetch (will be implemented in JS)
    #[wasm_bindgen(js_namespace = window, js_name = tailsFetch)]
    pub fn fetch_url(url: &str) -> String;
}

// Define a macro for console logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct WasmTailsRepl {
    evaluator: Evaluator,
}

#[wasm_bindgen]
impl WasmTailsRepl {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmTailsRepl {
        // Set up panic hook for better error reporting in the browser
        console_error_panic_hook::set_once();

        WasmTailsRepl {
            evaluator: Evaluator::new(),
        }
    }

    /// Execute Tails code and return the result as a JSON string
    #[wasm_bindgen]
    pub fn execute(&mut self, code: &str) -> String {
        // Clear output buffer for this execution
        self.evaluator.output_buffer.clear();

        let mut parser = Parser::new(code);
        match parser.parse() {
            Ok(program) => {
                match self.evaluator.eval_program(program) {
                    Ok(value) => {
                        let result = ExecutionResult {
                            success: true,
                            value: if self.evaluator.output_buffer.is_empty() && value != Value::Null {
                                Some(value_to_js_value(&value))
                            } else {
                                None
                            },
                            error: None,
                            output: self.evaluator.output_buffer.clone(),
                        };
                        serde_json::to_string(&result).unwrap_or_else(|_|
                            r#"{"success":false,"error":"Failed to serialize result"}"#.to_string()
                        )
                    }
                    Err(e) => {
                        let result = ExecutionResult {
                            success: false,
                            value: None,
                            error: Some(format!("Runtime error: {}", e)),
                            output: self.evaluator.output_buffer.clone(),
                        };
                        serde_json::to_string(&result).unwrap_or_else(|_|
                            r#"{"success":false,"error":"Failed to serialize error"}"#.to_string()
                        )
                    }
                }
            }
            Err(e) => {
                let result = ExecutionResult {
                    success: false,
                    value: None,
                    error: Some(format!("Parse error: {}", e)),
                    output: Vec::new(),
                };
                serde_json::to_string(&result).unwrap_or_else(|_|
                    r#"{"success":false,"error":"Failed to serialize parse error"}"#.to_string()
                )
            }
        }
    }

    /// Get the current output buffer (for debugging)
    #[wasm_bindgen]
    pub fn get_output(&self) -> String {
        self.evaluator.output_buffer.join("\n")
    }

    /// Clear the output buffer
    #[wasm_bindgen]
    pub fn clear_output(&mut self) {
        self.evaluator.output_buffer.clear();
    }

    /// Reset the REPL state (clear variables and actions)
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.evaluator = Evaluator::new();
    }

    /// Add output to the buffer (called internally by Tails operations)
    pub fn add_output(&mut self, output: String) {
        self.evaluator.output_buffer.push(output);
    }
}

#[derive(serde::Serialize)]
struct ExecutionResult {
    success: bool,
    value: Option<serde_json::Value>,
    error: Option<String>,
    output: Vec<String>,
}

/// Convert Tails Value to JSON Value for JavaScript interop
fn value_to_js_value(value: &Value) -> serde_json::Value {
    match value {
        Value::Number(n) => serde_json::Value::Number(
            serde_json::Number::from_f64(*n).unwrap_or_else(|| serde_json::Number::from(0))
        ),
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Boolean(b) => serde_json::Value::Bool(*b),
        Value::List(items) => {
            let json_items: Vec<serde_json::Value> = items.iter().map(value_to_js_value).collect();
            serde_json::Value::Array(json_items)
        }
        Value::Object(map) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in map {
                json_map.insert(k.clone(), value_to_js_value(v));
            }
            serde_json::Value::Object(json_map)
        }
        Value::Null => serde_json::Value::Null,
    }
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    console_log!("Tails WASM module initialized");
}

// Custom implementations for web environment
impl WasmTailsRepl {
    /// Handle say operations by adding to output buffer
    pub fn handle_say(&mut self, message: &str) {
        self.add_output(message.to_string());
        console_log!("Tails output: {}", message);
    }

    /// Handle ask operations by prompting the user
    pub fn handle_ask(&mut self, prompt: &str) -> String {
        console_log!("Tails prompt: {}", prompt);
        prompt_user(prompt)
    }
}