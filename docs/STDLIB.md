# Tails Standard Library

The Tails standard library provides powerful built-in functions for common programming tasks. All stdlib functions use the `*.` syntax to avoid conflicts with your own actions.

## Quick Reference

```tails
# List operations with stdlib helpers
~doubled is *.map ~numbers .double
~evens is *.filter ~numbers .is-even
~sum is *.reduce ~numbers .add 0
~sorted is *.sort ~numbers
~reversed is *.reverse ~numbers

# String operations
~parts is *.split "hello,world" ","
~text is *.join ~parts " "
~clean is *.trim "  hello  "
~upper is *.map ~words .uppercase
~lower is *.map ~words .lowercase

# Math operations and helpers
~positive is *.absolute -42
~root is *.square-root 16
~squares is *.map ~numbers .square
~halved is *.map ~numbers .half
```

## Helper Functions

The standard library includes common helper functions designed to work seamlessly with `map`, `filter`, and `reduce`. Helper functions are referenced by name with a `.` prefix (e.g., `.double`, `.is-even`).

### Predicate Functions (for filtering)

#### `*.is-even number` / `.is-even`
Returns `true` if the number is even, `false` otherwise.

```tails
~numbers is [1, 2, 3, 4, 5]
~evens is *.filter ~numbers .is-even
say ~evens  # [2, 4]

# Direct usage
~result is *.is-even 6
say ~result  # true
```

#### `*.is-odd number` / `.is-odd`
Returns `true` if the number is odd, `false` otherwise.

```tails
~numbers is [1, 2, 3, 4, 5]
~odds is *.filter ~numbers .is-odd
say ~odds  # [1, 3, 5]
```

#### `*.is-positive number` / `.is-positive`
Returns `true` if the number is greater than zero.

```tails
~numbers is [-2, -1, 0, 1, 2]
~positives is *.filter ~numbers .is-positive
say ~positives  # [1, 2]
```

#### `*.is-negative number` / `.is-negative`
Returns `true` if the number is less than zero.

```tails
~numbers is [-2, -1, 0, 1, 2]
~negatives is *.filter ~numbers .is-negative
say ~negatives  # [-2, -1]
```

#### `*.is-zero number` / `.is-zero`
Returns `true` if the number equals zero.

```tails
~numbers is [-1, 0, 1, 0, 2]
~zeros is *.filter ~numbers .is-zero
say ~zeros  # [0, 0]
```

### Transformation Functions (for mapping)

#### `*.double number` / `.double`
Multiplies a number by 2.

```tails
~numbers is [1, 2, 3, 4]
~doubled is *.map ~numbers .double
say ~doubled  # [2, 4, 6, 8]
```

#### `*.triple number` / `.triple`
Multiplies a number by 3.

```tails
~numbers is [1, 2, 3]
~tripled is *.map ~numbers .triple
say ~tripled  # [3, 6, 9]
```

#### `*.quadruple number` / `.quadruple`
Multiplies a number by 4.

```tails
~numbers is [1, 2, 3]
~quadrupled is *.map ~numbers .quadruple
say ~quadrupled  # [4, 8, 12]
```

#### `*.half number` / `.half`
Divides a number by 2.

```tails
~numbers is [2, 4, 6, 8]
~halved is *.map ~numbers .half
say ~halved  # [1, 2, 3, 4]
```

#### `*.square number` / `.square`
Multiplies a number by itself.

```tails
~numbers is [1, 2, 3, 4]
~squares is *.map ~numbers .square
say ~squares  # [1, 4, 9, 16]
```

#### `*.increment number` / `.increment`
Adds 1 to a number.

```tails
~numbers is [0, 1, 2]
~incremented is *.map ~numbers .increment
say ~incremented  # [1, 2, 3]
```

#### `*.decrement number` / `.decrement`
Subtracts 1 from a number.

```tails
~numbers is [3, 2, 1]
~decremented is *.map ~numbers .decrement
say ~decremented  # [2, 1, 0]
```

#### `*.uppercase string` / `.uppercase`
Converts a string to uppercase.

```tails
~words is ["hello", "world"]
~upper is *.map ~words .uppercase
say ~upper  # ["HELLO", "WORLD"]
```

#### `*.lowercase string` / `.lowercase`
Converts a string to lowercase.

```tails
~words is ["HELLO", "WORLD"]
~lower is *.map ~words .lowercase
say ~lower  # ["hello", "world"]
```

### Reduction Functions (for reducing)

#### `*.add number number` / `.add`
Adds two numbers together.

```tails
~numbers is [1, 2, 3, 4, 5]
~sum is *.reduce ~numbers .add 0
say ~sum  # 15
```

#### `*.multiply number number` / `.multiply`
Multiplies two numbers together.

```tails
~numbers is [2, 3, 4]
~product is *.reduce ~numbers .multiply 1
say ~product  # 24
```

#### `*.max number number` / `.max`
Returns the larger of two numbers.

```tails
~numbers is [5, 2, 8, 1, 9]
~maximum is *.reduce ~numbers .max 0
say ~maximum  # 9
```

#### `*.min number number` / `.min`
Returns the smaller of two numbers.

```tails
~numbers is [5, 2, 8, 1, 9]
~minimum is *.reduce ~numbers .min 999
say ~minimum  # 1
```

## List Operations

### `*.map list function_name`

Transforms each element in a list using the provided function.

**Example:**
```tails
# Using stdlib helper functions
~numbers is [1, 2, 3]
~doubled is *.map ~numbers .double
say ~doubled  # [2, 4, 6]

# Using custom actions (when you need complex logic)
action greet ~name (give "Hello " + ~name)
~names is ["Alice", "Bob"]
~greetings is *.map ~names greet
say ~greetings  # ["Hello Alice", "Hello Bob"]
```

**Requirements:**
- First argument must be a list
- Second argument must be either:
  - The name of a stdlib helper function with `.` prefix (e.g., `.double`, `.uppercase`)
  - The name of a custom action that takes exactly one parameter
- The function/action must return a value

### `*.filter list function_name`

Keeps only elements that match the given predicate function.

**Example:**
```tails
# Using stdlib helper functions
~numbers is [1, 2, 3, 4, 5, 6]
~evens is *.filter ~numbers .is-even
say ~evens  # [2, 4, 6]

~mixed is [-2, -1, 0, 1, 2]
~positives is *.filter ~mixed .is-positive
say ~positives  # [1, 2]

# Using custom actions (when you need complex logic)
action is_long ~word (give (length ~word) > 4)
~words is ["cat", "elephant", "dog", "hippopotamus"]
~long_words is *.filter ~words is_long
say ~long_words  # ["elephant", "hippopotamus"]
```

**Requirements:**
- First argument must be a list
- Second argument must be either:
  - The name of a stdlib predicate function with `.` prefix (e.g., `.is-even`, `.is-positive`)
  - The name of a custom action that takes exactly one parameter
- The function/action must return a boolean value

### `*.reduce list function_name initial_value`

Combines all elements in a list into a single value using the provided function.

**Example:**
```tails
# Using stdlib helper functions
~numbers is [1, 2, 3, 4]
~sum is *.reduce ~numbers .add 0
say ~sum  # 10

~product is *.reduce ~numbers .multiply 1
say ~product  # 24

~maximum is *.reduce ~numbers .max 0
say ~maximum  # 4

# Using custom actions (when you need complex logic)
action concatenate ~a ~b (give ~a + " " + ~b)
~words is ["hello", "beautiful", "world"]
~sentence is *.reduce ~words concatenate ""
say ~sentence  # " hello beautiful world"
```

**Requirements:**
- First argument must be a list
- Second argument must be either:
  - The name of a stdlib reduction function with `.` prefix (e.g., `.add`, `.multiply`, `.max`)
  - The name of a custom action that takes exactly two parameters
- Third argument is the initial/starting value

### `*.sort list`

Sorts a list in ascending order. Works with numbers, strings, and booleans.

**Example:**
```tails
~numbers is [3, 1, 4, 1, 5]
~sorted is *.sort ~numbers
say ~sorted  # [1, 1, 3, 4, 5]

~words is ["zebra", "apple", "banana"]
~sorted_words is *.sort ~words
say ~sorted_words  # ["apple", "banana", "zebra"]
```

### `*.reverse list`

Reverses the order of elements in a list.

**Example:**
```tails
~numbers is [1, 2, 3, 4, 5]
~backwards is *.reverse ~numbers
say ~backwards  # [5, 4, 3, 2, 1]
```

## String Operations

### `*.split string delimiter`

Splits a string into a list using the given delimiter.

**Example:**
```tails
~csv is "apple,banana,cherry"
~fruits is *.split ~csv ","
say ~fruits  # ["apple", "banana", "cherry"]

~sentence is "hello world from tails"
~words is *.split ~sentence " "
say ~words  # ["hello", "world", "from", "tails"]
```

### `*.join list delimiter`

Joins a list of values into a single string using the given delimiter.

**Example:**
```tails
~words is ["hello", "world", "from", "tails"]
~sentence is *.join ~words " "
say ~sentence  # "hello world from tails"

~numbers is [1, 2, 3, 4]
~csv is *.join ~numbers ","
say ~csv  # "1,2,3,4"
```

### `*.trim string`

Removes whitespace from the beginning and end of a string.

**Example:**
```tails
~messy is "  hello world  "
~clean is *.trim ~messy
say "'" ~clean "'"  # 'hello world'
```

## Math Operations

### `*.absolute number`

Returns the absolute value of a number.

**Example:**
```tails
~positive is *.absolute -42
say ~positive  # 42

~already_positive is *.absolute 15
say ~already_positive  # 15
```

### `*.square-root number`

Returns the square root of a number. The number must be non-negative.

**Example:**
```tails
~root is *.square-root 16
say ~root  # 4

~decimal is *.square-root 2
say ~decimal  # 1.4142135623730951
```

**Error:** Throws an error if the number is negative.

## Functional Programming Patterns

### Chaining Operations

You can chain stdlib functions together for powerful data processing:

```tails
~numbers is [1, 2, 3, 4, 5, 6, 7, 8]

# Get even numbers, double them, then sort
~evens is *.filter ~numbers .is-even        # [2, 4, 6, 8]
~doubled_evens is *.map ~evens .double       # [4, 8, 12, 16]
~final is *.sort ~doubled_evens             # [4, 8, 12, 16]

say ~final
```

### Data Processing Pipeline

```tails
~data is [1, -2, 3, -4, 5, 0]

# Process: filter positive → square each → sum all
~positives is *.filter ~data .is-positive    # [1, 3, 5]
~squares is *.map ~positives .square         # [1, 9, 25]
~sum is *.reduce ~squares .add 0             # 35

say "Sum of squares of positive numbers: " ~sum
```

### Working with Strings and Lists

```tails
~text is "apple,banana,cherry,date"
~fruits is *.split ~text ","               # ["apple", "banana", "cherry", "date"]
~sorted_fruits is *.sort ~fruits           # ["apple", "banana", "cherry", "date"]
~result is *.join ~sorted_fruits " | "     # "apple | banana | cherry | date"

say ~result
```

## Error Handling

The stdlib functions provide clear error messages:

```tails
# Wrong number of arguments
~result is *.map [1, 2, 3]  # Error: map requires exactly 2 arguments

# Function doesn't exist
~result is *.map [1, 2, 3] nonexistent  # Error: Action 'nonexistent' not found
~result is *.map [1, 2, 3] .nonexistent  # Error: Unknown stdlib function: .nonexistent

# Wrong function signature for custom actions
action bad-func ~a ~b (give ~a + ~b)
~result is *.map [1, 2, 3] bad_func  # Error: Action 'bad-func' must take exactly one parameter for map

# Type errors
~result is *.sort "not a list"  # Error: sort first argument must be a list
~result is *.is-even "not a number"  # Error: is-even argument must be a number
~result is *.uppercase 123  # Error: uppercase argument must be a string

# Math errors
~result is *.square-root -4  # Error: square-root argument must be non-negative
```

## See Also

- [SYNTAX.md](SYNTAX.md) - Complete Tails language reference
- [examples/stdlib_demo.tails](../examples/stdlib_demo.tails) - Working examples
- [examples/foreach_advanced.tails](../examples/foreach_advanced.tails) - Alternative iteration patterns