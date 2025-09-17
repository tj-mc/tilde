#!/bin/bash

# Build web REPL examples from the examples/ directory
# This ensures a single source of truth for all examples

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
        "hello_world.tails")
            echo "Hello World|Basic variable assignment and output"
            ;;
        "variables_math.tails")
            echo "Variables & Math|Simple arithmetic operations"
            ;;
        "conditionals.tails")
            echo "Conditionals|Nested if statements and else-if chains"
            ;;
        "objects.tails")
            echo "Objects|Working with object properties"
            ;;
        "actions.tails")
            echo "Actions (Functions)|Define and call custom actions with parameters"
            ;;
        "fibonacci.tails")
            echo "Fibonacci Sequence|Functions with return values and loops"
            ;;
        "nested_function_calls.tails")
            echo "🔥 Nested Function Calls|Mathematical compositions with parentheses"
            ;;
        "data_structure_madness.tails")
            echo "🚀 Data Structure Madness|Functional programming patterns"
            ;;
        "http_get.tails")
            echo "Network Request|Fetch data from APIs and display results"
            ;;
        *)
            # Default metadata
            local title=$(echo "$filename" | sed 's/.tails$//' | sed 's/_/ /g' | sed 's/\b\w/\U&/g')
            echo "$title|Example program"
            ;;
    esac
}

echo "🔧 Building web examples from $EXAMPLES_DIR..."

# Carefully curated examples for web REPL - no duplicates, progressive complexity
WEB_EXAMPLES=(
    "hello_world.tails"
    "variables_math.tails"
    "conditionals.tails"
    "objects.tails"
    "actions.tails"
    "fibonacci.tails"
    "nested_function_calls.tails"
    "data_structure_madness.tails"
    "http_get.tails"
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
example_count=$(find "$EXAMPLES_DIR" -name "*.tails" | wc -l)

echo "✅ Updated $WEB_HTML with $example_count examples from $EXAMPLES_DIR/"
echo "🎉 Web examples build complete!"