# Tails Language Syntax Specification

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
- **Assignment**: `is`

#### Integer Division and Modulo
The `\` operator performs integer division (floors the result), while `%` gives the remainder:
```
~result is 7 \ 3    # equals 2.0 (floored)
~remainder is 7 % 3 # equals 1.0
~even_check is ~n % 2 == 0  # true if n is even
```

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
    ~counter is (~counter + 1)
    say ~counter
    if (~counter >= 100) break-loop
)
```

## Actions

Actions are reusable blocks of code that can be defined once and executed multiple times with different parameters. They provide a way to encapsulate logic and create custom functions in the language.

### Action Definition
Actions are defined using the `action` keyword followed by the action name and optional parameters:

Basic action without parameters:
```
action greet (
    say "Hello, world!"
)
```

Action with parameters:
```
action greet-person ~name (
    say "Hello, " ~name "!"
)
```

Action with multiple parameters:
```
action calculate-area ~width ~height (
    ~area is ~width * ~height
    say "The area is " ~area
)
```

### Calling Actions
Actions are invoked using the `*` prefix followed by the action name:

Without arguments:
```
*greet
```

With arguments:
```
*greet-person "Alice"
*calculate-area 10 20
```

### Actions with Return Values
Actions use the `give` keyword to return values:
```
action add ~a ~b (
    ~result is ~a + ~b
    give ~result
)

~sum is *add 5 3
say ~sum  # outputs: 8
```

### Nested Actions
Actions can call other actions:
```
action double ~n (
    give ~n * 2
)

action quadruple ~n (
    ~doubled is *double ~n
    give *double ~doubled
)

~value is *quadruple 5
say ~value  # outputs: 20
```

### Actions with Control Flow
Actions can contain any valid language constructs including conditionals and loops:
```
action factorial ~n (
    if ~n <= 1 (
        give 1
    ) else (
        ~n-minus-one is ~n - 1
        ~recursive is *factorial ~n-minus-one
        give ~n * ~recursive
    )
)

~fact-5 is *factorial 5
say ~fact-5  # outputs: 120
```

### Fibonacci Example
A complete example demonstrating recursive actions:
```
action fibonacci ~n (
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

### Action Scope
- Parameters are local to the action and don't affect variables in the calling scope
- Variables defined inside an action are local to that action
- Actions can access global variables (variables defined outside any action)
- The `give` keyword is used to return a value from an action

### Best Practices
1. Use descriptive action names that indicate what the action does
2. Keep actions focused on a single task
3. Use parameters to make actions reusable
4. Use `give` to return values from actions
5. Consider breaking complex actions into smaller, composable actions

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
~keys is keys-of ~person          # Returns list of keys
~values is values-of ~person      # Returns list of values  
~exists is has-key "name" ~person # Returns true/false
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

## Parsing Rules

### Tokenization
1. Input is read line by line
2. Lines are split on whitespace (whitespace = token boundaries)
3. No multi-word instructions allowed

### Token Types
- **Variables**: Tokens starting with `~`
- **Keywords**: `is`, `if`, `else`, `loop`, `break-loop`, `say`, `ask`, `get`, `run`, `wait`, `action`, `give`, `length`, `append`
- **Literals**: Numbers and double-quoted strings
- **Operators**: Mathematical and comparison operators
- **Delimiters**: `(` and `)`

### AST Structure Example
For the loop example above:
```
Program {
    Assignment { variable: "counter", value: Number(0) }
    Loop {
        body: [
            Assignment { 
                variable: "counter", 
                value: BinaryOp { 
                    left: Variable("counter"), 
                    op: Add, 
                    right: Number(1) 
                }
            }
            Effect { function: "say", args: [Variable("counter")] }
            If {
                condition: BinaryOp { 
                    left: Variable("counter"), 
                    op: GreaterEqual, 
                    right: Number(100) 
                }
                then: BreakLoop
            }
        ]
    }
}
```

## Design Principles
- **Simplicity**: Minimal syntax, easy to parse
- **Whitespace-delimited**: No complex tokenization needed
- **Prefix notation for variables**: Clear distinction between variables and keywords
- **Parentheses for blocks**: Explicit block boundaries without indentation sensitivity