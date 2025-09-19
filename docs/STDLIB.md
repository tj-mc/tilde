# Tails Standard Library

The Tails standard library provides powerful built-in functions for common programming tasks. All stdlib functions use the `*.` syntax to avoid conflicts with your own actions.

## Quick Reference

```tails
# List operations
~doubled is *.map ~numbers double
~evens is *.filter ~numbers is_even
~sum is *.reduce ~numbers add 0
~sorted is *.sort ~numbers
~reversed is *.reverse ~numbers

# String operations
~parts is *.split "hello,world" ","
~text is *.join ~parts " "
~clean is *.trim "  hello  "

# Math operations
~positive is *.absolute -42
~root is *.square-root 16
```

## List Operations

### `*.map list function_name`

Transforms each element in a list using the provided function.

**Example:**
```tails
action double ~x (give ~x * 2)
action greet ~name (give "Hello " + ~name)

~numbers is [1, 2, 3]
~doubled is *.map ~numbers double
say ~doubled  # [2, 4, 6]

~names is ["Alice", "Bob"]
~greetings is *.map ~names greet
say ~greetings  # ["Hello Alice", "Hello Bob"]
```

**Requirements:**
- First argument must be a list
- Second argument must be the name of an action that takes exactly one parameter
- The action must return a value

### `*.filter list function_name`

Keeps only elements that match the given predicate function.

**Example:**
```tails
action is_even ~x (give ~x % 2 == 0)
action is_long ~word (give (length ~word) > 4)

~numbers is [1, 2, 3, 4, 5, 6]
~evens is *.filter ~numbers is_even
say ~evens  # [2, 4, 6]

~words is ["cat", "elephant", "dog", "hippopotamus"]
~long_words is *.filter ~words is_long
say ~long_words  # ["elephant", "hippopotamus"]
```

**Requirements:**
- First argument must be a list
- Second argument must be the name of an action that takes exactly one parameter
- The action must return a boolean value

### `*.reduce list function_name initial_value`

Combines all elements in a list into a single value using the provided function.

**Example:**
```tails
action add ~a ~b (give ~a + ~b)
action multiply ~a ~b (give ~a * ~b)
action max ~a ~b (
    if ~a > ~b (give ~a) else (give ~b)
)

~numbers is [1, 2, 3, 4]
~sum is *.reduce ~numbers add 0
say ~sum  # 10

~product is *.reduce ~numbers multiply 1
say ~product  # 24

~maximum is *.reduce ~numbers max 0
say ~maximum  # 4
```

**Requirements:**
- First argument must be a list
- Second argument must be the name of an action that takes exactly two parameters
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
action double ~x (give ~x * 2)
action is_even ~x (give ~x % 2 == 0)

~numbers is [1, 2, 3, 4, 5, 6, 7, 8]

# Get even numbers, double them, then sort
~evens is *.filter ~numbers is_even        # [2, 4, 6, 8]
~doubled_evens is *.map ~evens double       # [4, 8, 12, 16]
~final is *.sort ~doubled_evens             # [4, 8, 12, 16]

say ~final
```

### Data Processing Pipeline

```tails
action is_positive ~x (give ~x > 0)
action square ~x (give ~x * ~x)
action add ~a ~b (give ~a + ~b)

~data is [1, -2, 3, -4, 5, 0]

# Process: filter positive → square each → sum all
~positives is *.filter ~data is_positive    # [1, 3, 5]
~squares is *.map ~positives square         # [1, 9, 25]
~sum is *.reduce ~squares add 0             # 35

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

# Wrong function signature
action bad-func ~a ~b (give ~a + ~b)
~result is *.map [1, 2, 3] bad_func  # Error: Action 'bad-func' must take exactly one parameter for map

# Type errors
~result is *.sort "not a list"  # Error: sort argument must be a list
```

## See Also

- [SYNTAX.md](SYNTAX.md) - Complete Tails language reference
- [examples/stdlib_demo.tails](../examples/stdlib_demo.tails) - Working examples
- [examples/foreach_advanced.tails](../examples/foreach_advanced.tails) - Alternative iteration patterns