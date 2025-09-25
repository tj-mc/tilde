# Tilde Language Syntax Specification

### Variables
Variables are prefixed with `~` and can contain hyphens:
```
~counter is 0
~favorite-pet is "fergus"
~score is 0.345
~is-active is true
```

### Data Types
- **Numbers**: Integer (`0`, `100`) and floating-point (`0.345`)
- **Strings**: Double-quoted (`"fergus"`, `"you win"`) with backtick interpolation (`"hello `~var`"`)
- **Objects**: Key-value maps (`{"name": "Alice" "age": 30}`)
- **Lists**: Ordered collections (`[1, 2, 3]`, `["hello", 42, true]`)
- **Booleans**: `true` and `false`
- **Null**: Implicit for uninitialized variables

#### String Templates
Double-quoted strings support variable interpolation using backticks around variables:
```
~name is "Alice"
~age is 30
~message is "Hello `~name`, you are `~age` years old!"
say ~message  # outputs: Hello Alice, you are 30 years old!
```

### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`, `\` (integer division), `%` (modulo)
- **Comparison**: `<`, `>`, `<=`, `>=`, `==`, `!=`
- **Logical**: `and`, `or` (with short-circuit evaluation)
- **Assignment**: `is`
- **Increment/Decrement**: `up`, `down`

#### Increment and Decrement Operations
The `up` and `down` operators provide convenient increment and decrement operations for numeric variables:

```
~counter is 5
~counter up 3        # counter becomes 8
~counter down 2      # counter becomes 6
~counter up 1        # counter becomes 7

# Works with variables and expressions
~amount is 10
~counter up ~amount  # counter becomes 17
~counter down (~amount / 2)  # counter becomes 12
```

**Key Features:**
- Both variable and amount must be numbers
- Variable must exist before using `up` or `down`
- Supports any numeric expression as the amount
- Much cleaner than `~var is ~var + amount` syntax
- Works with both integers and floating-point numbers

**Practical Examples:**
```
# Loop counters
~i is 0
loop (
    if ~i >= 10 break-loop
    say "Count: " ~i
    ~i up 1
)

# Accumulating totals
~total is 0
for-each ~value in ~numbers (
    ~total up ~value
)

# Game scoring
~score is 0
~score up 100        # Player scored!
~lives is 3
~lives down 1        # Player lost a life
```

#### Integer Division and Modulo
The `\` operator performs integer division (floors the result), while `%` gives the remainder:
```
~result is 7 \ 3    # equals 2.0 (floored)
~remainder is 7 % 3 # equals 1.0
~even_check is ~n % 2 == 0  # true if n is even
```

#### Logical Operators
The `and` and `or` operators provide logical operations with short-circuit evaluation:

**`and` operator:**
- Returns the first falsy value, or the last value if all are truthy
- Short-circuits: if left operand is falsy, right operand is not evaluated

**`or` operator:**
- Returns the first truthy value, or the last value if all are falsy
- Short-circuits: if left operand is truthy, right operand is not evaluated

```
# Boolean logic
~result1 is true and false   # false
~result2 is true or false    # true

# With other types (truthy/falsy semantics)
~result3 is 1 and 2          # 2 (1 is truthy, return 2)
~result4 is 0 or 3           # 3 (0 is falsy, return 3)
~result5 is "hello" and ""   # "" ("hello" is truthy, return "")

# Practical usage
if ~age > 18 and ~hasLicense say "Can drive"
if ~weather == "rain" or ~weather == "snow" say "Bad weather"
```

**Operator Precedence (highest to lowest):**
1. Arithmetic: `*`, `/`, `\`, `%`
2. Arithmetic: `+`, `-`
3. Comparison: `<`, `>`, `<=`, `>=`, `==`, `!=`
4. Logical: `and`
5. Logical: `or`

**Truthy/Falsy Values:**
- **Falsy**: `false`, `0`, `""` (empty string), `[]` (empty list), `{}` (empty object), `null`
- **Truthy**: All other values

## Control Structures

### Conditional Statements
Single statement syntax:
```
if ~score <= 0.2 say "you lose"
else say "you win"
```

Block syntax for multiple statements:
```
if ~score >= 90 (
    ~grade is "A"
    say "Excellent work!"
) else if ~score >= 80 (
    ~grade is "B"  
    say "Good job!"
) else (
    ~grade is "C"
    say "Keep trying!"
)
```

### Loops
Loops use parentheses for block delimitation:
```
loop (
    ~counter up 1 
    say ~counter
    if (~counter >= 100) break-loop
)
```

### For-Each Loops
For-each loops provide elegant iteration over lists and objects:

#### Iterating Over Lists
```
~items is ["apple", "banana", "cherry"]

# Iterate over values only
for-each ~item in ~items (
    say "Processing: " ~item
)

# Iterate with index
for-each ~item ~index in ~items (
    say ~index ": " ~item
)
```

#### Iterating Over Objects
```
~person is {"name": "Alice", "age": 30, "city": "Boston"}

# Iterate over values only
for-each ~value in ~person (
    say "Value: " ~value
)

# Iterate over key-value pairs
for-each ~key ~value in ~person (
    say ~key " = " ~value
)
```

#### For-Each Features
- **Variable Scoping**: Loop variables are automatically scoped to the loop and don't affect outer variables
- **Break Support**: Use `break-loop` to exit early
- **Nested Loops**: For-each loops can be nested for processing complex data structures
- **Type Flexibility**: Works with both lists and objects seamlessly

#### Practical Examples
```
# Data processing
~grades is [85, 92, 78, 96]
~total is 0
~count is 0

for-each ~grade in ~grades (
    ~total is ~total + ~grade
    ~count is ~count + 1
)

~average is ~total / ~count
say "Average grade: " ~average

# Nested data processing
~departments is [
    {"name": "Engineering", "employees": ["Alice", "Bob"]},
    {"name": "Marketing", "employees": ["Carol", "David"]}
]

for-each ~dept in ~departments (
    say "Department: " ~dept.name
    for-each ~employee in ~dept.employees (
        say "  Employee: " ~employee
    )
)
```

## Functions 

Functions are reusable blocks of code that can be defined once and executed multiple times with different parameters. They provide a way to encapsulate logic in the language.

### Function Definition
Function are defined using the `function` keyword followed by the function name and optional parameters:

Basic function without parameters:
```
function greet (
    say "Hello, world!"
)
```

Function with parameters:
```
function greet-person ~name (
    say "Hello, " ~name "!"
)
```

Function with multiple parameters:
```
function calculate-area ~width ~height (
    ~area is ~width * ~height
    say "The area is " ~area
)
```

### Calling a Function 
Function are invoked using the `*` prefix followed by the function name:

Without arguments:
```
*greet
```

With arguments:
```
*greet-person "Alice"
*calculate-area 10 20
```

### Functions with Return Values
Functions use the `give` keyword to return values:
```
function add ~a ~b (
    ~result is ~a + ~b
    give ~result
)

~sum is *add 5 3
say ~sum  # outputs: 8
```

The last statement in the function body will also be returned, so you can write tiny functions like this:
```tilde
functions add ~a ~b ( ~a + ~b )

```

### Nested Functions
Functions can call other functions:
```
function double ~n (
    give ~n * 2
)

function quadruple ~n (
    ~doubled is *double ~n
    give *double ~doubled
)

~value is *quadruple 5
say ~value  # outputs: 20
```

### Fibonacci Example
A complete example demonstrating recursive functions:
```
function fibonacci ~n (
    if ~n <= 1 (
        give ~n
    ) else (
        ~n-minus-one is ~n - 1
        ~n-minus-two is ~n - 2
        ~fib-1 is *fibonacci ~n-minus-one
        ~fib-2 is *fibonacci ~n-minus-two
        give ~fib-1 + ~fib-2
    )
)

~fib-10 is *fibonacci 10
say "The 10th Fibonacci number is " ~fib-10
```

### Anonymous Functions

Anonymous functions provide a concise way to define functions inline without naming them. They use the pipe syntax: `|~param1 ~param2 (body)|`

**Single parameter:**
```tilde
~numbers is [1, 2, 3, 4, 5]
~doubled is map ~numbers |~x (~x * 2)|
say ~doubled  # [2, 4, 6, 8, 10]
```

**Multiple parameters:**
```tilde
~numbers is [1, 2, 3, 4, 5]
~sum is reduce ~numbers |~a ~b (~a + ~b)| 0
say ~sum  # 15
```

**Property access in anonymous functions:**
```tilde
~orders is [
    {"id": 101, "amount": 50},
    {"id": 102, "amount": 75},
    {"id": 103, "amount": 30}
]
~order-ids is map ~orders |~order (~order.id)|
~sorted is sort ~order-ids
say ~sorted  # [101, 102, 103]
```

**Filtering with anonymous functions:**
```tilde
~numbers is [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
~evens is filter ~numbers |~n (~n % 2 == 0)|
say ~evens  # [2, 4, 6, 8, 10]
```

**Complex anonymous functions:**
```tilde
~words is ["cat", "elephant", "dog", "bird"]
~long-words is filter ~words |~word ((length ~word) > 3)|
say ~long-words  # ["elephant", "bird"]
```

**Nested anonymous functions:**
```tilde
~matrix is [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
~doubled-matrix is map ~matrix |~row (
    map ~row |~x (~x * 2)|
)|
say ~doubled-matrix  # [[2, 4, 6], [8, 10, 12], [14, 16, 18]]
```

#### Anonymous Function Features

- **Scope isolation:** Variables defined inside anonymous functions don't affect outer scope
- **Closure support:** Anonymous functions can read variables from outer scopes
- **Parameter shadowing:** Parameters hide outer variables with the same name
- **Single expression body:** Anonymous functions contain one expression that's automatically returned

#### When to Use Anonymous Functions vs Named Functions

**Use anonymous functions for:**
- Simple transformations with `map`, `filter`, `reduce`
- One-time use functions
- Short, obvious operations

**Use named functions for:**
- Complex logic requiring multiple statements
- Functions used multiple times
- Functions that improve code readability when named

### Function Scope
- Parameters are local to the function and don't affect variables in the calling scope
- Variables defined inside an function are local to that function 
- Functions can access global variables (variables defined outside any function)
- The `give` keyword is used to return a value from a function 

### Best Practices
1. Use descriptive function names that indicate what the function does
2. Keep functions focused on a single task
3. Use parameters to make functions reusable
4. Use `give` to return values from functions
5. Consider breaking complex functions into smaller, composable functions

## Built-in Functions

### Output
```
say "Hello, world!"
say ~counter
say "The count is " ~counter " out of " ~total
```
Multiple arguments to `say` are concatenated without spaces. To include spaces, add them explicitly in string literals.

### Input
```
~name is ask "What's your name? "
~age is ask "Enter your age: "
~guess is ask
```
The `ask` statement prompts for user input. Optional prompt arguments are displayed before reading input. Returns numbers if the input can be parsed as a number, otherwise returns a string.


### HTTP GET Requests
The `get` statement performs HTTP GET requests and returns the JSON response:
```
~response is get "https://api.github.com/users/octocat"
say ~response.login  # outputs: octocat
say ~response.name   # outputs: The Octocat
```

### Shell Commands
Execute shell commands using `run`:
```
~result is run "echo Hello World"
say ~result.output  # outputs: Hello World
```

### Wait/Delay
Pause execution for a specified number of seconds:
```
wait 2.5  # Wait 2.5 seconds
~delay is 1
wait ~delay  # Wait 1 second
```

### Random Numbers
Generate random numbers within a specified range:
```
~dice is random 1 6          # Random integer 1-6 (inclusive)
~probability is random 0.0 1.0  # Random float 0.0-1.0 (inclusive)
~mixed is random 5 10.5      # Random float 5-10.5 (inclusive)

# Can be used in expressions
~total is (random 1 3) + (random 4 6)

# Can be used with variables
~min is 10
~max is 20
~value is random ~min ~max

# As a standalone statement (discards result)
random 1 100
```

The `random` function takes exactly two numeric arguments:
- If both arguments are integers (no decimal part), returns a random integer in the range
- If either argument is a float, returns a random float in the range
- Range is inclusive on both ends
- Minimum value cannot be greater than maximum value

### File Reading
Read files from the filesystem:
```
~file is read "config.txt"
say ~file.content           # File contents as string
say ~file.size              # File size in bytes
say ~file.exists            # true if file exists

# Handle errors
if ~file.error (
    say "Error reading file: " ~file.error
) else (
    say "File content: " ~file.content
)

# Can be used with variables
~filename is "data.json"
~data is read ~filename

# As a standalone statement (discards result)
read "log.txt"
```

The `read` function takes exactly one string argument (the file path) and returns an object with:
- `content`: File contents as a string (empty string if file doesn't exist or error occurs)
- `size`: File size in bytes (0 if file doesn't exist or error occurs)
- `exists`: Boolean indicating if the file exists
- `error`: Error message string if an error occurred, otherwise null

### File Writing
Write content to files on the filesystem:
```
~result is write "output.txt" "Hello, World!"
say ~result.success         # true if write succeeded
say ~result.bytes_written   # Number of bytes written
say ~result.path            # File path that was written to

# Handle errors
if ~result.error (
    say "Error writing file: " ~result.error
) else (
    say "Successfully wrote " ~result.bytes_written " bytes"
)

# Can write different data types
~result1 is write "number.txt" 42.5      # Writes "42.5"
~result2 is write "bool.txt" true        # Writes "true"
~result3 is write "object.txt" {"a": 1}  # Writes object string representation

# Can be used with variables
~filename is "data.txt"
~content is "File content"
~result is write ~filename ~content

# As a standalone statement (discards result)
write "log.txt" "Log entry"
```

The `write` function takes exactly two arguments (file path and content) and returns an object with:
- `success`: Boolean indicating if the write operation succeeded
- `path`: The file path that was written to
- `bytes_written`: Number of bytes written to the file (0 if write failed)
- `error`: Error message string if an error occurred, otherwise null

The content argument can be any data type:
- Strings are written as-is
- Numbers are converted to their string representation
- Booleans are written as "true" or "false"
- Objects and lists are written as their string representation
- Null is written as "null"

### Terminal Control
Clear the terminal screen for graphics and animation:
```
clear
```

The `clear` command:
- Takes no arguments
- Clears the entire terminal screen
- Positions cursor at top-left (0,0)
- Returns null
- Works on all platforms (ANSI escape sequences)

**Terminal Graphics Examples:**
```
# Interactive dashboard
loop (
    clear
    say "┌─── TILDE DASHBOARD ───┐"
    say "│ Status: Running       │"
    say "│ Memory: 42MB          │"
    say "└───────────────────────┘"
    wait 1.0
)
```


## Objects and Properties

### Object Creation
Objects are created with curly brace syntax:
```
~person is {"name": "Alice" "age": 30}
~config is {"theme": "dark" "debug": true}
~data is {"x": 10 "y": 20 "nested": {"inner": 42}}
```

### Property Access
Dot notation for accessing object properties:
```
~name is ~person.name
~age is ~person.age
~inner-value is ~data.nested.inner
```

### Property Assignment
Set or update object properties:
```
~person.age is 31
~person.city is "New York"
~config.theme is "light"
```

### Object Functions
Built-in functions for working with objects:
```
~keys is keys ~person          # Returns list of keys
~values is values ~person      # Returns list of values
~exists is has "name" ~person # Returns true/false
```

### Objects in Conditionals
Objects can be used in conditional expressions:
```
if ~config.debug (
    say "Debug mode is enabled"
)

if ~person.age > 25 (
    ~category is "adult"
) else (
    ~category is "young"
)
```

## Lists

### List Creation
Lists are created with square bracket syntax:
```
~numbers is [1, 2, 3, 4, 5]
~mixed is [1, "hello", true, [1, 2]]
~empty is []
~words is ["apple", "banana", "cherry"]
```

### List Indexing
Access list elements using dot notation with numeric indices:
```
~first is ~numbers.0    # Gets 1
~second is ~numbers.1   # Gets 2
~last is ~words.2       # Gets "cherry"
~missing is ~numbers.10 # Gets null (out of bounds)
```

### List Assignment
Modify or extend lists using indexed assignment:
```
~numbers.1 is 99        # Changes second element to 99
~numbers.10 is 100      # Extends list with nulls, then sets index 10
~newlist.0 is "first"   # Creates new list if variable doesn't exist
```

### List Functions
Built-in functions for working with lists:
```
~count is length ~numbers        # Returns number of items
~new-list is append ~words "date" # Returns new list with item added
~str-len is length "hello"       # Length also works on strings
```

### Lists in Conditionals and Interpolation
Lists work seamlessly with other language features:
```
~items is ["apple", "banana"]
if length ~items > 0 (
    say "First item: `~items.0`"
)

~message is "List has `length ~items` items: `~items.0` and `~items.1`"
```

### Nested Lists
Lists can contain other lists:
```
~matrix is [[1, 2], [3, 4]]
~row is ~matrix.0           # Gets [1, 2]
~element is ~row.0          # Gets 1 (two-step access required)
```

## Example Program
```
~counter is 0

loop (
    ~counter is (~counter + 1)
    say ~counter
    if (~counter >= 100) break-loop
)
```
## Standard Library

Tilde includes a comprehensive standard library for functional programming and common operations. Stdlib functions are called directly by name and take precedence over user-defined functions when resolving function names.

### List Operations
```tilde
~numbers is [1, 2, 3, 4, 5]
~doubled is map ~numbers double        # [2, 4, 6, 8, 10]
~evens is filter ~numbers is-even      # [2, 4]
~sum is reduce ~numbers add 0          # 15
~sorted is sort [3, 1, 4, 1, 5]        # [1, 1, 3, 4, 5]
~reversed is reverse ~numbers          # [5, 4, 3, 2, 1]
```

### String Operations
```tilde
~words is split "hello,world,tilde" ","    # ["hello", "world", "tilde"]
~sentence is join ~words " "               # "hello world tilde"
~clean is trim "  hello world  "           # "hello world"
```

### Math Operations
```tilde
~positive is absolute -42                  # 42
~root is square-root 16                    # 4
```

### Functional Programming Patterns
```tilde
# Chain operations for data processing
~data is [1, -2, 3, -4, 5]
~positives is filter ~data is_positive
~squares is map ~positives square
~result is reduce ~squares add 0
```

**📖 See [STDLIB.md](STDLIB.md) for complete standard library documentation with examples and best practices.**

## Error Handling

Tilde provides structured error handling through the `attempt`/`rescue` pattern, allowing you to gracefully handle errors without stopping program execution.

### Basic Error Handling

The `attempt`/`rescue` syntax catches errors and provides an alternative execution path:

```tilde
attempt (
    ~result is 10 / 0  # This will cause a division by zero error
) rescue (
    ~result is "Error occurred"
)
say ~result  # outputs: Error occurred
```

### Capturing Error Information

You can capture the error detilde by providing a variable name after `rescue`:

```tilde
attempt (
    ~value is ~undefined_variable + 1
) rescue ~error (
    say "Caught error: " ~error
    say "Error type: " ~error.message
)
```

### Error Object Structure

When an error is caught, it becomes an `Error` value with the following properties:
- `message`: A string describing what went wrong
- `code`: Optional error code (currently unused, reserved for future use)
- `source`: Optional source information (currently unused)
- `context`: Optional additional context (currently unused)

### Nested Error Handling

Error handling blocks can be nested for complex error recovery scenarios:

```tilde
attempt (
    attempt (
        ~risky_operation is some_dangerous_function
    ) rescue (
        say "Inner error caught, trying fallback"
        ~risky_operation is fallback_function
    )
    ~final_result is ~risky_operation * 2
) rescue (
    say "Both attempts failed"
    ~final_result is 0
)
```

### Error Propagation

Errors in rescue blocks are still propagated. Only the attempt block is protected:

```tilde
attempt (
    ~value is ~undefined_var
) rescue (
    ~another_undefined is ~also_missing  # This error will still halt execution
)
```

### Common Error Types

Tilde automatically creates errors for common failure scenarios:

- **Division by zero**: `Division by zero`
- **Undefined variables**: `Undefined variable: variable_name`
- **Unknown functions**: `Unknown function: function_name`
- **Type mismatches**: `Invalid operation: cannot add string and number`
- **Property access errors**: `Cannot access property 'prop' on number`
- **Date parsing errors**: `date-add first argument must be a date`

### Best Practices

1. **Use for Expected Failures**: Use `attempt`/`rescue` for operations that might legitimately fail
2. **Don't Hide Real Bugs**: Avoid using error handling to mask programming errors
3. **Provide Meaningful Fallbacks**: Ensure rescue blocks provide sensible default behavior

```tilde
# Good: Handle expected file operations
attempt (
    ~config is read "config.json"
) rescue ~err (
    say "Config file not found, using defaults"
    ~config is {"theme": "default", "debug": false}
)

# Good: Handle user input validation
attempt (
    ~number is parse_number ~user_input
) rescue (
    say "Invalid number entered, please try again"
    ~number is 0
)
```