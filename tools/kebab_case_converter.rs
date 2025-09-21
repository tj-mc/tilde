use std::fs;
use std::io::Write;
use std::path::Path;
use regex::Regex;
use walkdir::WalkDir;

fn main() {
    println!("🔧 Starting kebab-case conversion for Tails codebase...");

    let converter = KebabCaseConverter::new();
    converter.run();

    println!("✅ Kebab-case conversion completed!");
}

struct KebabCaseConverter {
    // Regex patterns for different types of conversions
    variable_pattern: Regex,
    function_call_pattern: Regex,
    function_def_pattern: Regex,
    // Pattern to match snake_case identifiers
    snake_case_pattern: Regex,
}

impl KebabCaseConverter {
    fn new() -> Self {
        Self {
            // Match ~variable-name (variables with underscores)
            variable_pattern: Regex::new(r"~([a-z][a-z0-9]*(?:_[a-z0-9]+)+)").unwrap(),
            // Match *function-name (function calls with underscores)
            function_call_pattern: Regex::new(r"\*([a-z][a-z0-9]*(?:_[a-z0-9]+)+)").unwrap(),
            // Match function function-name (function definitions with underscores)
            function_def_pattern: Regex::new(r"\bfunction\s+([a-z][a-z0-9]*(?:_[a-z0-9]+)+)").unwrap(),
            // Helper pattern to identify snake_case
            snake_case_pattern: Regex::new(r"[a-z][a-z0-9]*(?:_[a-z0-9]+)+").unwrap(),
        }
    }

    fn run(&self) {
        let base_path = Path::new(".");

        // Process different types of files
        self.process_directory("docs", &["md"], base_path);
        self.process_directory("examples", &["tails"], base_path);
        self.process_directory("tests", &["rs"], base_path);
        self.process_directory("src", &["rs"], base_path);
        self.process_directory("web", &["js", "html"], base_path);
        self.process_directory("benchmark_comparison", &["tails"], base_path);
        self.process_directory("tools", &["rs"], base_path);

        // Process individual files
        self.process_file(&base_path.join("README.md"));
        self.process_file(&base_path.join("CHANGELOG.md"));
        self.process_file(&base_path.join("CONTRIBUTING.md"));

        // Process any .tails files in root
        for entry in WalkDir::new(base_path).max_depth(1) {
            if let Ok(entry) = entry {
                if entry.path().extension().map_or(false, |ext| ext == "tails") {
                    self.process_file(entry.path());
                }
            }
        }
    }

    fn process_directory(&self, dir_name: &str, extensions: &[&str], base_path: &Path) {
        let dir_path = base_path.join(dir_name);
        if !dir_path.exists() {
            println!("⚠️  Directory {} doesn't exist, skipping", dir_name);
            return;
        }

        println!("📁 Processing directory: {}", dir_name);

        for entry in WalkDir::new(&dir_path) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extensions.iter().any(|&ext| extension == ext) {
                            self.process_file(path);
                        }
                    }
                }
            }
        }
    }

    fn process_file(&self, file_path: &Path) {
        // Skip git files and binary files
        if file_path.to_string_lossy().contains(".git") {
            return;
        }

        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(_) => {
                println!("⚠️  Could not read file: {}", file_path.display());
                return;
            }
        };

        let original_content = content.clone();
        let converted_content = self.convert_content(&content, file_path);

        if converted_content != original_content {
            match fs::write(file_path, converted_content) {
                Ok(_) => println!("✏️  Converted: {}", file_path.display()),
                Err(e) => println!("❌ Failed to write {}: {}", file_path.display(), e),
            }
        }
    }

    fn convert_content(&self, content: &str, file_path: &Path) -> String {
        let mut result = content.to_string();

        // Convert variables (~variable-name → ~variable-name)
        result = self.variable_pattern.replace_all(&result, |caps: &regex::Captures| {
            let var_name = &caps[1];
            let kebab_name = self.snake_to_kebab(var_name);
            format!("~{}", kebab_name)
        }).to_string();

        // Convert function calls (*function-name → *function-name)
        result = self.function_call_pattern.replace_all(&result, |caps: &regex::Captures| {
            let func_name = &caps[1];
            let kebab_name = self.snake_to_kebab(func_name);
            format!("*{}", kebab_name)
        }).to_string();

        // Convert function definitions (function function-name → function function-name)
        result = self.function_def_pattern.replace_all(&result, |caps: &regex::Captures| {
            let func_name = &caps[1];
            let kebab_name = self.snake_to_kebab(func_name);
            format!("function {}", kebab_name)
        }).to_string();

        // Handle special cases based on file type
        if let Some(extension) = file_path.extension() {
            match extension.to_str() {
                Some("rs") => {
                    // For Rust files, be more careful and only convert string literals
                    result = self.convert_rust_string_literals(&result);
                }
                Some("md") => {
                    // For markdown, also handle code blocks and inline code
                    result = self.convert_markdown_code(&result);
                }
                _ => {}
            }
        }

        result
    }

    fn convert_rust_string_literals(&self, content: &str) -> String {
        // Pattern to match string literals that might contain Tails code
        let string_literal_pattern = Regex::new(r#""([^"\\]*(\\.[^"\\]*)*)""#).unwrap();

        string_literal_pattern.replace_all(content, |caps: &regex::Captures| {
            let inner = &caps[1];
            let converted_inner = self.convert_tails_code_in_string(inner);
            format!("\"{}\"", converted_inner)
        }).to_string()
    }

    fn convert_markdown_code(&self, content: &str) -> String {
        // Handle inline code `...` and code blocks ```...```
        let inline_code_pattern = Regex::new(r"`([^`]+)`").unwrap();
        let code_block_pattern = Regex::new(r"```([^`]+)```").unwrap();

        let mut result = content.to_string();

        // Convert inline code
        result = inline_code_pattern.replace_all(&result, |caps: &regex::Captures| {
            let code = &caps[1];
            let converted = self.convert_tails_code_in_string(code);
            format!("`{}`", converted)
        }).to_string();

        // Convert code blocks
        result = code_block_pattern.replace_all(&result, |caps: &regex::Captures| {
            let code = &caps[1];
            let converted = self.convert_tails_code_in_string(code);
            format!("```{}```", converted)
        }).to_string();

        result
    }

    fn convert_tails_code_in_string(&self, content: &str) -> String {
        let mut result = content.to_string();

        // Apply the same conversions as for regular content
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_conversion() {
        let converter = KebabCaseConverter::new();
        let input = "~user-result is 42\n~long-words-list is empty";
        let expected = "~user-result is 42\n~long-words-list is empty";
        assert_eq!(converter.convert_content(input, Path::new("test.tails")), expected);
    }

    #[test]
    fn test_function_call_conversion() {
        let converter = KebabCaseConverter::new();
        let input = "*sum-list ~numbers\n*filter-even ~data";
        let expected = "*sum-list ~numbers\n*filter-even ~data";
        assert_eq!(converter.convert_content(input, Path::new("test.tails")), expected);
    }

    #[test]
    fn test_function_definition_conversion() {
        let converter = KebabCaseConverter::new();
        let input = "function is-not-zero ~x (\n    give ~x > 0\n)";
        let expected = "function is-not-zero ~x (\n    give ~x > 0\n)";
        assert_eq!(converter.convert_content(input, Path::new("test.tails")), expected);
    }

    #[test]
    fn test_preserve_kebab_case() {
        let converter = KebabCaseConverter::new();
        let input = "~already-kebab is good\n*also-kebab ~data";
        let expected = "~already-kebab is good\n*also-kebab ~data";
        assert_eq!(converter.convert_content(input, Path::new("test.tails")), expected);
    }

    #[test]
    fn test_preserve_single_words() {
        let converter = KebabCaseConverter::new();
        let input = "~counter is 0\n*double ~value";
        let expected = "~counter is 0\n*double ~value";
        assert_eq!(converter.convert_content(input, Path::new("test.tails")), expected);
    }
}