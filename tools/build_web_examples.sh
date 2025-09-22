#!/bin/bash

# Build Web REPL Examples
# =======================
#
# This script automatically generates the web REPL examples section in web/index.html
# from the curated .tde files in the examples/ directory. This ensures a single
# source of truth for all examples across the project.
#
# Usage:
#   ./tools/build_web_examples.sh
#
# What it does:
# - Reads selected .tde files from examples/ directory
# - Escapes content for HTML attributes (handles quotes, newlines, backslashes)
# - Injects the examples into web/index.html between the examples-list markers
# - Maintains metadata (titles and descriptions) for each example
#
# Manual execution only - not part of CI/CD pipeline
# Call this when you add/modify examples and want to update the web REPL

set -e

EXAMPLES_DIR="examples"
WEB_HTML="web/index.html"
TEMP_HTML="/tmp/web_examples_temp.html"

# Function to escape content for HTML attributes
escape_html_attr() {
    local content="$1"
    # Use perl for better cross-platform compatibility
    echo "$content" | perl -pe 's/\\/\\\\/g; s/\n/\\n/g; s/'\''/\\'\''/g; s/"/\\"/g'
}

# Function to get example metadata
get_example_metadata() {
    local filename="$1"
    case "$filename" in
        "hello_world.tde")
            echo "Hello World|Basic variable assignment and output"
            ;;
        "variables_math.tde")
            echo "Variables & Math|Simple arithmetic operations"
            ;;
        "conditionals.tde")
            echo "Conditionals|Nested if statements and else-if chains"
            ;;
        "objects.tde")
            echo "Objects|Working with object properties"
            ;;
        "functions.tde")
            echo "Functions |Define and call custom functions with parameters"
            ;;
        "fibonacci.tde")
            echo "Fibonacci Sequence|Functions with return values and loops"
            ;;
        "nested_function_calls.tde")
            echo "🔥 Nested Function Calls|Mathematical compositions with parentheses"
            ;;
        "data_structure_madness.tde")
            echo "🚀 Data Structure Madness|Functional programming patterns"
            ;;
        "http_get.tde")
            echo "Network Request|Fetch data from APIs and display results"
            ;;
        *)
            # Default metadata
            local title=$(echo "$filename" | sed 's/.tilde$//' | sed 's/_/ /g' | sed 's/\b\w/\U&/g')
            echo "$title|Example program"
            ;;
    esac
}

echo "🔧 Building web examples from $EXAMPLES_DIR..."

# Carefully curated examples for web REPL - no duplicates, progressive complexity
WEB_EXAMPLES=(
    "hello_world.tde"
    "variables_math.tde"
    "conditionals.tde"
    "objects.tde"
    "functions.tde"
    "fibonacci.tde"
    "nested_function_calls.tde"
    "data_structure_madness.tde"
    "http_get.tde"
)

# Generate examples HTML
examples_html=""
for filename in "${WEB_EXAMPLES[@]}"; do
    example_file="$EXAMPLES_DIR/$filename"
    if [ ! -f "$example_file" ]; then
        echo "Warning: $example_file not found, skipping"
        continue
    fi
    echo "Processing $filename..."

    # Read file content
    content=$(cat "$example_file")

    # Escape content for HTML
    escaped_content=$(escape_html_attr "$content")

    # Get metadata
    metadata=$(get_example_metadata "$filename")
    title=$(echo "$metadata" | cut -d'|' -f1)
    description=$(echo "$metadata" | cut -d'|' -f2)

    # Generate HTML for this example
    example_html="                <div class=\"example-item\" data-code='$escaped_content'>
                    <h4>$title</h4>
                    <p>$description</p>
                </div>"

    examples_html="$examples_html$example_html"$'\n'
done

# Remove trailing newline
examples_html=$(echo "$examples_html" | sed '$d')

echo "📝 Updating $WEB_HTML..."

# Write examples to temp file first
echo "$examples_html" > /tmp/examples_content.html

# Use sed to replace content between markers
sed -n '1,/<div class="examples-list">/p' "$WEB_HTML" > "$TEMP_HTML"
cat /tmp/examples_content.html >> "$TEMP_HTML"
sed -n '/^            <\/div>$/,$p' "$WEB_HTML" | tail -n +1 >> "$TEMP_HTML"

# Replace the original file
mv "$TEMP_HTML" "$WEB_HTML"
rm -f /tmp/examples_content.html

# Count examples
example_count=$(find "$EXAMPLES_DIR" -name "*.tde" | wc -l)

echo "✅ Updated $WEB_HTML with $example_count examples from $EXAMPLES_DIR/"
echo "🎉 Web examples build complete!"