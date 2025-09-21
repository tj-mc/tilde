use std::fs;
use std::path::Path;
use regex::Regex;

struct KebabCaseConverter {
    variable_pattern: Regex,
    function_call_pattern: Regex,
    function_def_pattern: Regex,
}

impl KebabCaseConverter {
    fn new() -> Self {
        Self {
            variable_pattern: Regex::new(r"~([a-z][a-z0-9]*(?:_[a-z0-9]+)+)").unwrap(),
            function_call_pattern: Regex::new(r"\*([a-z][a-z0-9]*(?:_[a-z0-9]+)+)").unwrap(),
            function_def_pattern: Regex::new(r"\bfunction\s+([a-z][a-z0-9]*(?:_[a-z0-9]+)+)").unwrap(),
        }
    }

    fn convert_content(&self, content: &str) -> String {
        let mut result = content.to_string();

        result = self.variable_pattern.replace_all(&result, |caps: &regex::Captures| {
            let var_name = &caps[1];
            let kebab_name = self.snake_to_kebab(var_name);
            format!("~{}", kebab_name)
        }).to_string();

        result = self.function_call_pattern.replace_all(&result, |caps: &regex::Captures| {
            let func_name = &caps[1];
            let kebab_name = self.snake_to_kebab(func_name);
            format!("*{}", kebab_name)
        }).to_string();

        result = self.function_def_pattern.replace_all(&result, |caps: &regex::Captures| {
            let func_name = &caps[1];
            let kebab_name = self.snake_to_kebab(func_name);
            format!("function {}", kebab_name)
        }).to_string();

        result
    }

    fn snake_to_kebab(&self, snake_case: &str) -> String {
        snake_case.replace('_', "-")
    }
}

fn main() {
    let converter = KebabCaseConverter::new();
    let file_path = Path::new("test_conversion_sample.tilde");

    println!("🔧 Testing conversion on sample file...");

    let content = fs::read_to_string(file_path).expect("Could not read test file");
    println!("\n--- BEFORE ---");
    println!("{}", content);

    let converted = converter.convert_content(&content);
    println!("\n--- AFTER ---");
    println!("{}", converted);

    println!("\n✅ Conversion test completed!");
}