# Tilde Standard Library

The Tilde standard library provides powerful built-in functions for common programming tasks. Stdlib functions are called directly by name and take precedence over user-defined functions when resolving function names.

## Quick Reference

```tilde
# List operations with stdlib helpers
~doubled is map ~numbers double
~evens is filter ~numbers is-even
~sum is reduce ~numbers add 0
~sorted is sort ~numbers
~reversed is reverse ~numbers
~extended is append ~numbers 10
~count is length ~numbers

# Advanced list operations with predicates and grouping
~parts is partition ~numbers is-even
~by_length is group-by ~words length
~sorted_by_length is sort-by ~words length

# Set operations for list manipulation
~combined is union ~list1 ~list2
~only_in_first is difference ~list1 ~list2
~common is intersection ~list1 ~list2

# Advanced list operations with predicates
action is-big ~n (give ~n > 10)
~first-big is find ~numbers is-big
~all-positive is every ~numbers is-positive
~has-negative is some ~numbers is-negative
~non-zeros is remove-if ~numbers is-zero

# String operations
~parts is split "hello,world" ","
~text is join ~parts " "
~clean is trim "  hello  "
~upper is map ~words uppercase
~lower is map ~words lowercas

# Date and time operations
~current is now
~birthday is date "1990-12-25"
~meeting is date "2024-03-15T14:30:00Z"
~event is date "2024-03-15T16:30:00+02:00"

# Math operations and helpers
~positive is absolute -42
~root is square-root 16
~squares is map ~numbers square
~halved is map ~numbers half

# System operations
~port is env "PORT" or 8080
~db_url is env "DATABASE_URL" or "localhost"
~debug is env "DEBUG" or false
```

## Helper Functions

The standard library includes common helper functions designed to work seamlessly with `map`, `filter`, and `reduce`. Helper functions are called directly by name (e.g., `double`, `is-even`).

### Predicate Functions (for filtering)

#### `is-even number`
Returns `true` if the number is even, `false` otherwise.

```tilde
~numbers is [1, 2, 3, 4, 5]
~evens is filter ~numbers is-even
say ~evens  # [2, 4]

# Direct usage
~result is is-even 6
say ~result  # true
```

#### `is-odd number` / `is-odd`
Returns `true` if the number is odd, `false` otherwise.

```tilde
~numbers is [1, 2, 3, 4, 5]
~odds is filter ~numbers is-odd
say ~odds  # [1, 3, 5]
```

#### `is-positive number` / `is-positive`
Returns `true` if the number is greater than zero.

```tilde
~numbers is [-2, -1, 0, 1, 2]
~positives is filter ~numbers is-positive
say ~positives  # [1, 2]
```

#### `is-negative number` / `is-negative`
Returns `true` if the number is less than zero.

```tilde
~numbers is [-2, -1, 0, 1, 2]
~negatives is filter ~numbers is-negative
say ~negatives  # [-2, -1]
```

#### `is-zero number` / `is-zero`
Returns `true` if the number equals zero.

```tilde
~numbers is [-1, 0, 1, 0, 2]
~zeros is filter ~numbers is-zero
say ~zeros  # [0, 0]
```

### Transformation Functions (for mapping)

#### `double number` / `double`
Multiplies a number by 2.

```tilde
~numbers is [1, 2, 3, 4]
~doubled is map ~numbers double
say ~doubled  # [2, 4, 6, 8]
```

#### `triple number` / `triple`
Multiplies a number by 3.

```tilde
~numbers is [1, 2, 3]
~tripled is map ~numbers triple
say ~tripled  # [3, 6, 9]
```

#### `quadruple number` / `quadruple`
Multiplies a number by 4.

```tilde
~numbers is [1, 2, 3]
~quadrupled is map ~numbers quadruple
say ~quadrupled  # [4, 8, 12]
```

#### `half number` / `half`
Divides a number by 2.

```tilde
~numbers is [2, 4, 6, 8]
~halved is map ~numbers half
say ~halved  # [1, 2, 3, 4]
```

#### `square number` / `square`
Multiplies a number by itself.

```tilde
~numbers is [1, 2, 3, 4]
~squares is map ~numbers square
say ~squares  # [1, 4, 9, 16]
```

#### `increment number` / `increment`
Adds 1 to a number.

```tilde
~numbers is [0, 1, 2]
~incremented is map ~numbers increment
say ~incremented  # [1, 2, 3]
```

#### `decrement number` / `decrement`
Subtracts 1 from a number.

```tilde
~numbers is [3, 2, 1]
~decremented is map ~numbers decrement
say ~decremented  # [2, 1, 0]
```

#### `uppercase string` / `uppercase`
Converts a string to uppercase.

```tilde
~words is ["hello", "world"]
~upper is map ~words uppercase
say ~upper  # ["HELLO", "WORLD"]
```

#### `lowercase string` / `lowercase`
Converts a string to lowercase.

```tilde
~words is ["HELLO", "WORLD"]
~lower is map ~words lowercase
say ~lower  # ["hello", "world"]
```

### Reduction Functions (for reducing)

#### `add number number` / `add`
Adds two numbers together.

```tilde
~numbers is [1, 2, 3, 4, 5]
~sum is reduce ~numbers add 0
say ~sum  # 15
```

#### `multiply number number` / `multiply`
Multiplies two numbers together.

```tilde
~numbers is [2, 3, 4]
~product is reduce ~numbers multiply 1
say ~product  # 24
```

#### `max number number` / `max`
Returns the larger of two numbers.

```tilde
~numbers is [5, 2, 8, 1, 9]
~maximum is reduce ~numbers max 0
say ~maximum  # 9
```

#### `min number number` / `min`
Returns the smaller of two numbers.

```tilde
~numbers is [5, 2, 8, 1, 9]
~minimum is reduce ~numbers min 999
say ~minimum  # 1
```

## List Operations

### List Creation

#### `list length`

Creates a list of consecutive numbers from 1 to the specified length.

```tilde
~numbers is list 10
say ~numbers  # [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

~range is list 100
~evens is filter ~range is-even
say (length ~evens)  # 50

~empty is list 0
say (length ~empty)  # 0
```

**Properties:**
- **Range**: Numbers from 1 to length (inclusive)
- **Performance**: O(n) time and space, pre-allocated for efficiency
- **Limit**: Maximum 1,000,000 elements for safety
- **Errors**: Negative length, non-number argument, or length > 1,000,000

### `map list function_name`

Transforms each element in a list using the provided function.

**Example:**
```tilde
# Using stdlib helper functions
~numbers is [1, 2, 3]
~doubled is map ~numbers double
say ~doubled  # [2, 4, 6]

# Using custom actions (when you need complex logic)
action greet ~name (give "Hello " + ~name)
~names is ["Alice", "Bob"]
~greetings is map ~names greet
say ~greetings  # ["Hello Alice", "Hello Bob"]
```

**Requirements:**
- First argument must be a list
- Second argument must be either:
  - The name of a stdlib helper function (e.g., `double`, `uppercase`)
  - The name of a custom action that takes exactly one parameter
- The function/action must return a value

### `filter list function_name`

Keeps only elements that match the given predicate function.

**Example:**
```tilde
# Using stdlib helper functions
~numbers is [1, 2, 3, 4, 5, 6]
~evens is filter ~numbers is-even
say ~evens  # [2, 4, 6]

~mixed is [-2, -1, 0, 1, 2]
~positives is filter ~mixed is-positive
say ~positives  # [1, 2]

# Using custom actions (when you need complex logic)
action is_long ~word (give (length ~word) > 4)
~words is ["cat", "elephant", "dog", "hippopotamus"]
~long_words is filter ~words is_long
say ~long_words  # ["elephant", "hippopotamus"]
```

**Requirements:**
- First argument must be a list
- Second argument must be either:
  - The name of a stdlib predicate function (e.g., `is-even`, `is-positive`)
  - The name of a custom action that takes exactly one parameter
- The function/action must return a boolean value

### `reduce list function_name initial_value`

Combines all elements in a list into a single value using the provided function.

**Example:**
```tilde
# Using stdlib helper functions
~numbers is [1, 2, 3, 4]
~sum is reduce ~numbers add 0
say ~sum  # 10

~product is reduce ~numbers multiply 1
say ~product  # 24

~maximum is reduce ~numbers max 0
say ~maximum  # 4

# Using custom actions (when you need complex logic)
action concatenate ~a ~b (give ~a + " " + ~b)
~words is ["hello", "beautiful", "world"]
~sentence is reduce ~words concatenate ""
say ~sentence  # " hello beautiful world"
```

**Requirements:**
- First argument must be a list
- Second argument must be either:
  - The name of a stdlib reduction function (e.g., `add`, `multiply`, `max`)
  - The name of a custom action that takes exactly two parameters
- Third argument is the initial/starting value

### `sort list`

Sorts a list in ascending order. Works with numbers, strings, and booleans.

**Example:**
```tilde
~numbers is [3, 1, 4, 1, 5]
~sorted is sort ~numbers
say ~sorted  # [1, 1, 3, 4, 5]

~words is ["zebra", "apple", "banana"]
~sorted_words is sort ~words
say ~sorted_words  # ["apple", "banana", "zebra"]
```

### `reverse list`

Reverses the order of elements in a list.

**Example:**
```tilde
~numbers is [1, 2, 3, 4, 5]
~backwards is reverse ~numbers
say ~backwards  # [5, 4, 3, 2, 1]
```

### `append list value`

Adds a new element to the end of a list, returning a new list with the element appended. This operation is functional - it does not modify the original list.

**Example:**
```tilde
~numbers is [1, 2, 3]
~extended is append ~numbers 4
say ~extended  # [1, 2, 3, 4]
say ~numbers   # [1, 2, 3] (original unchanged)

# Chain multiple appends
~step1 is append ~numbers 4
~more is append ~step1 5
say ~more  # [1, 2, 3, 4, 5]
```

### Advanced List Operations with Predicates

The following functions work with custom predicate actions to provide powerful list processing capabilities.

#### `find list function_name`

Finds the first item in a list that matches a predicate function. Returns the item or null if no match found.

**Example:**
```tilde
action is-big ~n (give ~n > 5)
~numbers is [1, 3, 7, 2, 9]
~first-big is find ~numbers is-big
say ~first-big  # 7
```

#### `find-index list function_name`

Finds the index of the first item that matches a predicate function. Returns the index as a number or null if no match found.

**Example:**
```tilde
action is-negative ~n (give ~n < 0)
~numbers is [5, -2, 3, -1]
~neg-index is find-index ~numbers is-negative
say ~neg-index  # 1
```

#### `find-last list function_name`

Finds the last item in a list that matches a predicate function. Returns the item or null if no match found.

**Example:**
```tilde
action is-even ~n (give ~n % 2 == 0)
~numbers is [1, 2, 3, 4, 5, 6]
~last-even is find-last ~numbers is-even
say ~last-even  # 6
```

#### `every list function_name`

Tests if all items in a list match a predicate function. Returns true if all match, false otherwise.

**Example:**
```tilde
action is-positive ~n (give ~n > 0)
~numbers is [1, 2, 3, 4]
~all-positive is every ~numbers is-positive
say ~all-positive  # true

~mixed is [1, -2, 3]
~all-pos-mixed is every ~mixed is-positive
say ~all-pos-mixed  # false
```

#### `some list function_name`

Tests if any item in a list matches a predicate function. Returns true if at least one matches, false otherwise.

**Example:**
```tilde
action is-zero ~n (give ~n == 0)
~numbers is [1, 2, 0, 4]
~has-zero is some ~numbers is-zero
say ~has-zero  # true

~no-zeros is [1, 2, 3]
~has-zero-2 is some ~no-zeros is-zero
say ~has-zero-2  # false
```

#### `remove-if list function_name`

Removes items from a list that match a predicate function. Returns a new list with matching items removed.

**Example:**
```tilde
action is-negative ~n (give ~n < 0)
~numbers is [1, -2, 3, -4, 5]
~positives is remove-if ~numbers is-negative
say ~positives  # [1, 3, 5]
```

#### `count-if list function_name`

Counts items in a list that match a predicate function. Returns the count as a number.

**Example:**
```tilde
action is-long ~word (give (length ~word) > 4)
~words is ["cat", "elephant", "dog", "hippopotamus"]
~long-count is count-if ~words is-long
say ~long-count  # 2
```

#### `take-while list function_name`

Takes items from the beginning of a list while a predicate is true. Stops at the first item that doesn't match.

**Example:**
```tilde
action is-small ~n (give ~n < 5)
~numbers is [1, 2, 3, 7, 4, 1]
~small-prefix is take-while ~numbers is-small
say ~small-prefix  # [1, 2, 3]
```

#### `drop-while list function_name`

Drops items from the beginning of a list while a predicate is true. Returns the remaining items after the first non-match.

**Example:**
```tilde
action is-small ~n (give ~n < 5)
~numbers is [1, 2, 3, 7, 4, 1]
~after-small is drop-while ~numbers is-small
say ~after-small  # [7, 4, 1]
```

**Requirements for all predicate functions:**
- First argument must be a list
- Second argument must be the name of a custom action that takes exactly one parameter
- The predicate action must return a value that can be evaluated for truthiness

#### `partition list function_name`

Splits a list into two lists based on a predicate function. Returns an object with `matched` and `unmatched` properties containing the elements that did and didn't match the predicate, respectively.

**Example:**
```tilde
~numbers is [1, 2, 3, 4, 5, 6]
~parts is partition ~numbers is-even
say ~parts.matched    # [2, 4, 6]
say ~parts.unmatched  # [1, 3, 5]

# Useful for separating valid from invalid data
action is-valid-email ~email (give (contains ~email "@"))
~emails is ["user@domain.com", "invalid", "test@site.org"]
~sorted-emails is partition ~emails is-valid-email
say ~sorted-emails.matched    # ["user@domain.com", "test@site.org"]
say ~sorted-emails.unmatched  # ["invalid"]
```

#### `group-by list function_name`

Groups list elements by the result of applying a function to each element. Returns an object where keys are the function results (converted to strings) and values are lists of matching elements.

**Example:**
```tilde
# Group words by length
~words is ["cat", "dog", "bird", "cow", "elephant"]
~by-length is group-by ~words length
say ~by-length.3  # ["cat", "dog", "cow"] (all 3-letter words)
say ~by-length.4  # ["bird"] (all 4-letter words)
say ~by-length.8  # ["elephant"] (all 8-letter words)

# Group numbers by even/odd
~numbers is [1, 2, 3, 4, 5, 6]
~by-parity is group-by ~numbers is-even
say ~by-parity.true   # [2, 4, 6] (even numbers)
say ~by-parity.false  # [1, 3, 5] (odd numbers)
```

#### `sort-by list function_name`

Sorts a list by applying a function to each element and sorting by the results. The original elements are returned in the new order.

**Example:**
```tilde
# Sort words by length (shortest first)
~words is ["elephant", "cat", "dog", "bird"]
~by-length is sort-by ~words length
say ~by-length  # ["cat", "dog", "bird", "elephant"]

# Sort numbers by absolute value
~numbers is [-3, 1, -1, 4, -2]
~by-abs is sort-by ~numbers absolute
say ~by-abs  # [1, -1, -2, -3, 4]

# Sort strings alphabetically (case-insensitive)
~names is ["Bob", "alice", "Charlie"]
~alphabetical is sort-by ~names lowercase
say ~alphabetical  # ["alice", "Bob", "Charlie"]
```

## Set Operations

These functions treat lists as sets, performing mathematical set operations like union, intersection, and difference. All operations preserve the order from the first list and automatically remove duplicates.

#### `union list1 list2`

Returns a new list containing all unique elements from both input lists. Elements from the first list appear first, followed by unique elements from the second list.

**Example:**
```tilde
~list1 is [1, 2, 3, 4]
~list2 is [3, 4, 5, 6]
~combined is union ~list1 ~list2
say ~combined  # [1, 2, 3, 4, 5, 6]

# Works with mixed types
~mixed1 is [1, "hello", true]
~mixed2 is [2, "hello", false]
~all is union ~mixed1 ~mixed2
say ~all  # [1, "hello", true, 2, false]

# Automatically removes duplicates
~dupes1 is [1, 2, 2, 3]
~dupes2 is [3, 4, 4, 5]
~clean is union ~dupes1 ~dupes2
say ~clean  # [1, 2, 3, 4, 5]
```

#### `difference list1 list2`

Returns a new list containing elements that are in the first list but not in the second list.

**Example:**
```tilde
~list1 is [1, 2, 3, 4, 5]
~list2 is [2, 4, 6]
~only-in-first is difference ~list1 ~list2
say ~only-in-first  # [1, 3, 5]

# Useful for filtering out unwanted items
~all-users is ["admin", "user1", "user2", "guest"]
~banned-users is ["user2", "guest"]
~allowed-users is difference ~all-users ~banned-users
say ~allowed-users  # ["admin", "user1"]
```

#### `intersection list1 list2`

Returns a new list containing elements that appear in both input lists. Duplicates are automatically removed.

**Example:**
```tilde
~list1 is [1, 2, 3, 4, 5]
~list2 is [2, 4, 6, 8]
~common is intersection ~list1 ~list2
say ~common  # [2, 4]

# Find common interests
~alice-hobbies is ["reading", "hiking", "cooking", "gaming"]
~bob-hobbies is ["cooking", "gaming", "sports", "music"]
~shared is intersection ~alice-hobbies ~bob-hobbies
say ~shared  # ["cooking", "gaming"]

# Handles duplicates correctly
~dupes1 is [1, 2, 2, 3]
~dupes2 is [2, 2, 3, 4]
~common-clean is intersection ~dupes1 ~dupes2
say ~common-clean  # [2, 3] (no duplicates)
```

## List Mutation Operations

The following functions provide powerful ways to modify lists while maintaining immutability - they return new lists with the changes applied.

#### `remove list value`

Removes the first occurrence of a value from a list.

**Example:**
```tilde
~numbers is [1, 2, 3, 2, 4]
~result is remove ~numbers 2
say ~result  # [1, 3, 2, 4]

~words is ["apple", "banana", "apple"]
~result is remove ~words "apple"
say ~result  # ["banana", "apple"]
```

#### `remove-at list index`

Removes the item at the specified index from a list.

**Example:**
```tilde
~words is ["first", "second", "third"]
~result is remove-at ~words 1
say ~result  # ["first", "third"]

# Remove first item
~result is remove-at ~words 0
say ~result  # ["second", "third"]
```

#### `insert list index value`

Inserts a value at the specified index in a list.

**Example:**
```tilde
~numbers is [1, 3, 4]
~result is insert ~numbers 1 2
say ~result  # [1, 2, 3, 4]

# Insert at beginning
~result is insert ~numbers 0 0
say ~result  # [0, 1, 3, 4]

# Insert at end
~result is insert ~numbers 3 5
say ~result  # [1, 3, 4, 5]
```

#### `set-at list index value`

Replaces the value at the specified index in a list.

**Example:**
```tilde
~words is ["hello", "world", "test"]
~result is set-at ~words 1 "universe"
say ~result  # ["hello", "universe", "test"]

~numbers is [1, 2, 3]
~result is set-at ~numbers 0 10
say ~result  # [10, 2, 3]
```

#### `pop list`

Removes and returns the last element from a list. Returns an object with `value` and `list` properties.

**Example:**
```tilde
~numbers is [1, 2, 3, 4]
~result is pop ~numbers
say ~result.value  # 4
say ~result.list   # [1, 2, 3]

# Chain operations
~popped is pop ~result.list
say ~popped.value  # 3
say ~popped.list   # [1, 2]
```

#### `shift list`

Removes and returns the first elemet from a list. Returns an object with `value` and `list` properties.

**Example:**
```tilde
~numbers is [1, 2, 3, 4]
~result is shift ~numbers
say ~result.value  # 1
say ~result.list   # [2, 3, 4]
```

#### `unshift list value`

Adds an element to the beginning of a list.

**Example:**
```tilde
~numbers is [2, 3, 4]
~result is unshift ~numbers 1
say ~result  # [1, 2, 3, 4]

~words is ["world"]
~result is unshift ~words "hello"
say ~result  # ["hello", "world"]
```

## List Query Operations

These functions help you search and extract information from lists.

#### `length list`

Returns the number of elements in a list as a number.

**Example:**
```tilde
~numbers is [1, 2, 3, 4, 5]
~count is length ~numbers
say ~count  # 5

~empty is []
say (length ~empty)  # 0

# Useful for conditionals
~items is [1, 2, 3]
if (length ~items) > 0 then (
  say "List has items"
) else (
  say "List is empty"
)
```

#### `index-of list_or_string value`

Finds the index of the first occurrence of a value in a list, or the position of a substring in a string. Returns the index as a number or null if not found.

**Examples:**
```tilde
# With lists
~fruits is ["apple", "banana", "cherry", "banana"]
~index is index-of ~fruits "banana"
say ~index  # 1

~index is index-of ~fruits "grape"
say ~index  # null

# With strings
~email is "user@example.com"
~at_pos is index-of ~email "@"
say ~at_pos  # 4

~missing is index-of ~email "xyz"
say ~missing  # null
```

#### `contains list_or_string value`

Checks if a list contains a specific value, or if a string contains a substring. Returns true or false.

**Examples:**
```tilde
# With lists
~numbers is [1, 2, 3, 4, 5]
~has-three is contains ~numbers 3
say ~has-three  # true

~has-ten is contains ~numbers 10
say ~has-ten  # false

# With strings
~email is "user@example.com"
~has-at is contains ~email "@"
say ~has-at  # true

~text is "hello world"
~has-xyz is contains ~text "xyz"
say ~has-xyz  # false
```

#### `slice list start [end]`

Extracts a portion of a list from start index to end index (exclusive). If end is not provided, slices to the end.

**Example:**
```tilde
~numbers is [0, 1, 2, 3, 4, 5]
~middle is slice ~numbers 2 4
say ~middle  # [2, 3]

# Slice to end
~tail is slice ~numbers 3
say ~tail  # [3, 4, 5]

# Slice entire list
~copy is slice ~numbers 0
say ~copy  # [0, 1, 2, 3, 4, 5]
```

#### `concat list1 list2 [list3 ...]`

Concatenates multiple lists into a single list.

**Example:**
```tilde
~first is [1, 2]
~second is [3, 4]
~third is [5, 6]
~combined is concat ~first ~second ~third
say ~combined  # [1, 2, 3, 4, 5, 6]

# Concatenate with empty lists
~result is concat [] ~first []
say ~result  # [1, 2]
```

#### `take list count`

Takes the first n elements from a list.

**Example:**
```tilde
~numbers is [1, 2, 3, 4, 5]
~first-three is take ~numbers 3
say ~first-three  # [1, 2, 3]

# Take more than available
~all is take ~numbers 10
say ~all  # [1, 2, 3, 4, 5]
```

#### `drop list count`

Drops the first n elements and returns the rest.

**Example:**
```tilde
~numbers is [1, 2, 3, 4, 5]
~last-three is drop ~numbers 2
say ~last-three  # [3, 4, 5]

# Drop more than available
~empty is drop ~numbers 10
say ~empty  # []
```

## Advanced List Operations

These functions provide sophisticated list processing capabilities.

#### `flatten list [depth]`

Flattens nested lists into a single list. Optional depth parameter controls how many levels to flatten.

**Example:**
```tilde
~nested is [1, [2, 3], [4, [5, 6]], 7]
~flat is flatten ~nested
say ~flat  # [1, 2, 3, 4, 5, 6, 7]

# Flatten with depth limit
~partial is flatten ~nested 1
say ~partial  # [1, 2, 3, 4, [5, 6], 7]
```

#### `unique list`

Removes duplicate values from a list, preserving order.

**Example:**
```tilde
~numbers is [1, 2, 2, 3, 1, 4, 3]
~unique-nums is unique ~numbers
say ~unique-nums  # [1, 2, 3, 4]

~mixed is [1, "hello", 1, true, "hello", false]
~unique-mixed is unique ~mixed
say ~unique-mixed  # [1, "hello", true, false]
```

#### `zip list1 list2`

Combines two lists element-wise into pairs. Length is determined by the shorter list.

**Example:**
```tilde
~numbers is [1, 2, 3]
~letters is ["a", "b", "c", "d"]
~pairs is zip ~numbers ~letters
say ~pairs  # [[1, "a"], [2, "b"], [3, "c"]]
```

#### `chunk list size`

Partitions a list into chunks of the specified size.

**Example:**
```tilde
~numbers is [1, 2, 3, 4, 5, 6, 7]
~chunks is chunk ~numbers 3
say ~chunks  # [[1, 2, 3], [4, 5, 6], [7]]

~pairs is chunk ~numbers 2
say ~pairs  # [[1, 2], [3, 4], [5, 6], [7]]
```

#### `transpose matrix`

Transposes a matrix (list of lists) where rows become columns.

**Example:**
```tilde
~matrix is [[1, 2, 3], [4, 5, 6]]
~transposed is transpose ~matrix
say ~transposed  # [[1, 4], [2, 5], [3, 6]]

# Works with jagged arrays
~jagged is [[1, 2], [3, 4, 5], [6]]
~result is transpose ~jagged
say ~result  # [[1, 3, 6], [2, 4, null], [null, 5, null]]
```

## String Operations

### `split string delimiter`

Splits a string into a list using the given delimiter.

**Example:**
```tilde
~csv is "apple,banana,cherry"
~fruits is split ~csv ","
say ~fruits  # ["apple", "banana", "cherry"]

~sentence is "hello world from tilde"
~words is split ~sentence " "
say ~words  # ["hello", "world", "from", "tilde"]
```

### `join list delimiter`

Joins a list of values into a single string using the given delimiter.

**Example:**
```tilde
~words is ["hello", "world", "from", "tilde"]
~sentence is join ~words " "
say ~sentence  # "hello world from tilde"

~numbers is [1, 2, 3, 4]
~csv is join ~numbers ","
say ~csv  # "1,2,3,4"
```

### `trim string`

Removes whitespace from the beginning and end of a string.

**Example:**
```tilde
~messy is "  hello world  "
~clean is trim ~messy
say "'" ~clean "'"  # 'hello world'
```

## Math Operations

### `absolute number`

Returns the absolute value of a number.

**Example:**
```tilde
~positive is absolute -42
say ~positive  # 42

~already_positive is absolute 15
say ~already_positive  # 15
```

### `square-root number`

Returns the square root of a number. The number must be non-negative.

**Example:**
```tilde
~root is square-root 16
say ~root  # 4

~decimal is square-root 2
say ~decimal  # 1.4142135623730951
```

**Error:** Throws an error if the number is negative.

## Date and Time Operations

Tilde provides comprehensive date and time functionality for working with temporal data. All dates are stored internally as UTC timestamps and formatted using the ISO 8601 standard for maximum compatibility and consistency.

### Date Creation

#### `now`

Returns the current date and time as a UTC timestamp with millisecond precision.

**Example:**
```tilde
~current is now
say "Current time: " ~current  # "2024-03-15T14:30:00.123Z"

# Use in conditionals - dates are always truthy
if ~current (
    say "Time exists!"
)

# Store for later use for precise timing
~start-time is now
# ... do some work ...
~end-time is now
~duration is date-diff ~start-time ~end-time
say "Operation took: " ~duration.milliseconds "ms"
```

**Properties:**
- Takes no arguments
- Always returns the current moment in UTC with millisecond precision
- Each call returns a fresh timestamp
- Output format: ISO 8601 with milliseconds (e.g., `"2024-03-15T14:30:00.123Z"`)
- Useful for timestamping events and measuring precise time differences

#### `date string`

Creates a date from a string representation. Supports both date-only and full datetime formats with automatic timezone conversion to UTC.

**Supported Formats:**
- **Date only:** `YYYY-MM-DD` (e.g., `"2024-03-15"`)
  - Time is set to `00:00:00` UTC
- **Full datetime:** ISO 8601 format (e.g., `"2024-03-15T14:30:00Z"`)
  - Supports timezone offsets (e.g., `"2024-03-15T16:30:00+02:00"`)
  - All times are converted to UTC internally

**Example:**
```tilde
# Date only - time defaults to midnight UTC
~birthday is date "1990-12-25"
say ~birthday  # "1990-12-25T00:00:00Z"

# Full datetime in UTC
~meeting is date "2024-03-15T14:30:00Z"
say ~meeting  # "2024-03-15T14:30:00Z"

# Datetime with timezone - converted to UTC
~event is date "2024-03-15T16:30:00+02:00"
say ~event  # "2024-03-15T14:30:00Z" (same as above!)

# Edge cases
~new-years is date "2000-01-01"
~leap-day is date "2024-02-29"
~unix-epoch is date "1970-01-01T00:00:00Z"
```

**Error Handling:**
The `date` function validates input and provides clear error messages for invalid formats:

```tilde
# These will all throw "Invalid date format" errors:
~bad1 is date "March 15, 2024"     # Wrong format
~bad2 is date "15/03/2024"         # Wrong format
~bad3 is date "2024-13-01"         # Invalid month
~bad4 is date "2024-02-30"         # Invalid day
~bad5 is date "2023-02-29"         # Invalid leap year
~bad6 is date "not-a-date"         # Not a date
```

#### String Interpolation

Dates automatically format to ISO 8601 strings when used in string contexts:

```tilde
~meeting-time is date "2024-03-15T14:30:00Z"
~message is "The meeting is scheduled for `~meeting-time`"
say ~message  # "The meeting is scheduled for 2024-03-15T14:30:00Z"

# Multiple dates in strings
~start is date "2024-03-15T09:00:00Z"
~end is date "2024-03-15T17:00:00Z"
~schedule is "Event from `~start` to `~end`"
say ~schedule  # "Event from 2024-03-15T09:00:00Z to 2024-03-15T17:00:00Z"
```

### Date Comparison and Equality

Dates can be compared for equality, and the same moment in time represented in different formats will be equal:

```tilde
# Same moment in time with different timezone representations
~utc-time is date "2024-03-15T14:30:00Z"
~offset-time is date "2024-03-15T16:30:00+02:00"

# These are equal because they represent the same UTC moment
if ~utc-time == ~offset-time (
    say "Times are equal!"  # This will execute
)

# Date-only vs datetime comparison
~date-only is date "2024-03-15"
~datetime is date "2024-03-15T00:00:00Z"
# These are equal because date-only defaults to midnight UTC
if ~date-only == ~datetime (
    say "Midnight matches!"  # This will execute
)
```

### Date Comparison Functions

Tilde provides three dedicated functions for comparing dates that return boolean values for use in conditionals and logic:

#### `date-before date1 date2`

Returns `true` if the first date is before (earlier than) the second date.

```tilde
~meeting is date "2024-03-15T14:30:00Z"
~deadline is date "2024-03-20T17:00:00Z"

~is_before_deadline is date-before ~meeting ~deadline
say ~is_before_deadline  # true

# Use in conditionals
if date-before ~meeting ~deadline (
    say "Meeting is scheduled before the deadline"
) else (
    say "Meeting is after the deadline!"
)

# Works with date-only formats too
~yesterday is date "2024-03-14"
~today is date "2024-03-15"
~is_yesterday_before is date-before ~yesterday ~today
say ~is_yesterday_before  # true
```

#### `date-after date1 date2`

Returns `true` if the first date is after (later than) the second date.

```tilde
~current is now
~birthday is date "1990-12-25"

~is_after_birthday is date-after ~current ~birthday
say ~is_after_birthday  # true (assuming current time is after 1990)

# Check if event has passed
~event is date "2024-01-01T00:00:00Z"
if date-after ~current ~event (
    say "The event already happened"
) else (
    say "The event is still coming up"
)
```

#### `date-equal date1 date2`

Returns `true` if two dates represent the exact same moment in time, even if expressed in different timezones.

```tilde
~utc is date "2024-03-15T14:30:00Z"
~paris is date "2024-03-15T15:30:00+01:00"  # Same moment, different timezone

~same_time is date-equal ~utc ~paris
say ~same_time  # true

# Useful for exact time matching
~scheduled is date "2024-03-15T14:30:00Z"
~actual is date "2024-03-15T14:30:00Z"

if date-equal ~scheduled ~actual (
    say "Right on time!"
) else (
    say "Time difference detected"
)
```

### Working with Date Ranges

Date comparison functions are essential for working with date ranges and time periods:

#### Checking if a Date Falls Within a Range

```tilde
~start is date "2024-03-01"
~end is date "2024-03-31"
~check_date is date "2024-03-15"

# Check if date is within range (inclusive)
~in_range is (date-after ~check_date ~start) and (date-before ~check_date ~end)
say ~in_range  # true

# More explicit range checking
if date-after ~check_date ~start and date-before ~check_date ~end (
    say "Date is within March 2024"
) else (
    say "Date is outside the range"
)
```

#### Business Hours and Scheduling Logic

```tilde
~business_start is date "2024-03-15T09:00:00Z"
~business_end is date "2024-03-15T17:00:00Z"
~meeting_time is date "2024-03-15T14:30:00Z"

# Check if meeting is during business hours
if date-after ~meeting_time ~business_start and date-before ~meeting_time ~business_end (
    say "Meeting is during business hours"
) else (
    say "Meeting is outside business hours"
)

# Find the earliest of multiple dates
~option1 is date "2024-03-20T10:00:00Z"
~option2 is date "2024-03-18T14:00:00Z"
~option3 is date "2024-03-22T09:00:00Z"

if date-before ~option2 ~option1 and date-before ~option2 ~option3 (
    ~earliest is ~option2
    say "Option 2 is the earliest"
)
```

#### Deadline and Reminder Systems

```tilde
~deadline is date "2024-12-31T23:59:59Z"
~current is now
~reminder_period is date-add ~deadline -7  # 7 days before deadline

# Check deadline status
if date-after ~current ~deadline (
    ~status is "overdue"
) else if date-after ~current ~reminder_period (
    ~status is "reminder-period"
) else (
    ~status is "plenty-of-time"
)

say "Project status: " ~status

# Days until deadline
~days_left is date-diff ~current ~deadline
if ~days_left > 0 (
    say "Days remaining: " ~days_left
) else (
    say "Days overdue: " (0 - ~days_left)
)
```

#### Filtering Lists by Date

```tilde
# Filter events by date range
~events is [
    {"name": "Meeting A", "date": date "2024-03-10"},
    {"name": "Meeting B", "date": date "2024-03-15"},
    {"name": "Meeting C", "date": date "2024-03-20"},
    {"name": "Meeting D", "date": date "2024-03-25"}
]

~start_filter is date "2024-03-12"
~end_filter is date "2024-03-22"

# Filter events within date range
action in-range ~event (
    give (date-after ~event.date ~start_filter) and (date-before ~event.date ~end_filter)
)

~filtered_events is filter ~events in-range
say "Events in range: " ~filtered_events
# Result: [{"name": "Meeting B", ...}, {"name": "Meeting C", ...}]
```

**Important Notes:**
- All comparison functions return boolean values (`true` or `false`)
- Comparisons work with any combination of date-only and full datetime values
- Timezone differences are automatically handled (everything converted to UTC)
- Use with `and`, `or`, and `not` operators for complex date logic
- Perfect for building scheduling, filtering, and validation systems
### Best Practices

1. **Always use ISO 8601 format:** When creating dates, prefer the full datetime format with timezone information for clarity.

2. **UTC by default:** All dates are stored in UTC internally, eliminating timezone confusion.

3.**String formatting:** Dates automatically format to ISO 8601 strings when displayed or interpolated.

4.**Immutability:** Dates are immutable values that can be safely passed around and stored.

### Date Arithmetic

Tilde provides powerful date arithmetic functions for time calculations and duration analysis:

#### `date-add date days`

Adds a specified number of days to a date, returning a new date.

```tilde
~birthday is date "1990-12-25"
~week_later is date-add ~birthday 7
say ~week_later  # "1991-01-01T00:00:00Z"

# Works with negative numbers (subtract days)
~week_earlier is date-add ~birthday -7
say ~week_earlier  # "1990-12-18T00:00:00Z"

# Handles month/year boundaries automatically
~end_of_month is date "2024-01-31"
~next_month is date-add ~end_of_month 1
say ~next_month  # "2024-02-01T00:00:00Z"
```

#### `date-subtract date days`

Subtracts a specified number of days from a date, returning a new date.

```tilde
~deadline is date "2024-12-31"
~reminder is date-subtract ~deadline 7
say ~reminder  # "2024-12-24T00:00:00Z"

# Calculate past dates
~today is date "2024-03-15"
~last_week is date-subtract ~today 7
say ~last_week  # "2024-03-08T00:00:00Z"
```

#### `date-diff date1 date2`

Calculates the time difference between two dates and returns a comprehensive object with multiple time units. This is the most powerful function for duration analysis.

**Returns an object with:**
- `days` - Difference in days
- `hours` - Total difference in hours
- `minutes` - Total difference in minutes
- `seconds` - Total difference in seconds
- `milliseconds` - Total difference in milliseconds

```tilde
~start is date "2024-03-15T10:30:00Z"
~end is date "2024-03-17T14:45:30Z"

~duration is date-diff ~start ~end
say "Days: " ~duration.days             # 2
say "Hours: " ~duration.hours           # 52
say "Minutes: " ~duration.minutes       # 3135
say "Seconds: " ~duration.seconds       # 188130
say "Milliseconds: " ~duration.milliseconds  # 188130000

# Use specific units as needed
~meeting_start is date "2024-03-15T14:00:00Z"
~meeting_end is date "2024-03-15T16:30:00Z"
~meeting_duration is date-diff ~meeting_start ~meeting_end

say "Meeting duration: " ~meeting_duration.hours " hours"     # 2 hours
say "Meeting duration: " ~meeting_duration.minutes " minutes" # 150 minutes

# Negative differences work too
~past_event is date "2024-01-01"
~current is now
~time_since is date-diff ~past_event ~current
say "Days since New Year: " ~time_since.days
```

#### Real-World Usage Examples

**Project Timeline Management:**
```tilde
~project_start is date "2024-03-01"
~project_end is date "2024-06-30"
~today is now

~total_duration is date-diff ~project_start ~project_end
~elapsed is date-diff ~project_start ~today
~remaining is date-diff ~today ~project_end

say "Project length: " ~total_duration.days " days"
say "Days completed: " ~elapsed.days
say "Days remaining: " ~remaining.days

# Calculate completion percentage
~completion_pct is (~elapsed.days / ~total_duration.days) * 100
say "Project " ~completion_pct "% complete"
```

**Meeting Duration Analysis:**
```tilde
~meetings is [
    {"start": date "2024-03-15T09:00:00Z", "end": date "2024-03-15T10:30:00Z"},
    {"start": date "2024-03-15T11:00:00Z", "end": date "2024-03-15T12:00:00Z"},
    {"start": date "2024-03-15T14:00:00Z", "end": date "2024-03-15T15:30:00Z"}
]

action calc-duration ~meeting (
    ~duration is date-diff ~meeting.start ~meeting.end
    give ~duration.minutes
)

~durations is map ~meetings calc-duration
~total_minutes is reduce ~durations add 0
say "Total meeting time: " ~total_minutes " minutes"
say "Total meeting time: " (~total_minutes / 60) " hours"
```

**Deadline Monitoring:**
```tilde
~deadline is date "2024-12-31T23:59:59Z"
~current is now
~time_left is date-diff ~current ~deadline

if ~time_left.days > 30 (
    say "Plenty of time: " ~time_left.days " days left"
) else if ~time_left.days > 7 (
    say "Getting close: " ~time_left.days " days left"
) else if ~time_left.days > 0 (
    say "URGENT: Only " ~time_left.days " days left!"
) else (
    say "OVERDUE by " (0 - ~time_left.days) " days"
)
```

## HTTP Operations

Tilde provides a comprehensive HTTP client for making web requests with full support for all HTTP verbs, authentication, headers, error handling, and more.

### Basic HTTP Requests

#### `get url [options]`

Make an HTTP GET request.

```tilde
# Simple GET request
~response is get "https://api.example.com/users"

# Access response data
~status is ~response.status
~data is ~response.body
~headers is ~response.headers
```

#### `post url [options]`

Make an HTTP POST request with optional body data.

```tilde
# POST with JSON data
~user is {
    "name": "John Doe",
    "email": "john@example.com"
}

~response is post "https://api.example.com/users" {
    "body": ~user,
    "headers": {
        "Content-Type": "application/json"
    }
}
```

#### `put url [options]`

Make an HTTP PUT request for updates.

```tilde
# Update a resource
~updated_user is {
    "name": "Jane Doe",
    "email": "jane@example.com"
}

~response is put "https://api.example.com/users/123" {
    "body": ~updated_user,
    "headers": {
        "Content-Type": "application/json"
    }
}
```

#### `patch url [options]`

Make an HTTP PATCH request for partial updates.

```tilde
# Partial update
~changes is {"email": "newemail@example.com"}

~response is patch "https://api.example.com/users/123" {
    "body": ~changes,
    "headers": {
        "Content-Type": "application/json"
    }
}
```

#### `delete url [options]`

Make an HTTP DELETE request.

```tilde
# Delete a resource
~response is delete "https://api.example.com/users/123"

if ~response.ok (
    say "User deleted successfully"
)
```

#### `http method url [options]`

Generic HTTP function for custom methods.

```tilde
# Custom HTTP method
~response is http "OPTIONS" "https://api.example.com/users" {
    "headers": {
        "Access-Control-Request-Method": "POST"
    }
}
```

### HTTP Options

All HTTP functions accept an optional configuration object with the following fields:

#### Headers
```tilde
~response is get "https://api.example.com/data" {
    "headers": {
        "User-Agent": "MyApp/1.0",
        "Accept": "application/json",
        "X-API-Key": "your-api-key"
    }
}
```

#### Request Body
```tilde
# String body
~response is post "https://api.example.com/data" {
    "body": "raw string data",
    "headers": {
        "Content-Type": "text/plain"
    }
}

# JSON body (automatically serialized)
~response is post "https://api.example.com/data" {
    "body": {"key": "value"},
    "headers": {
        "Content-Type": "application/json"
    }
}
```

#### Timeout
```tilde
# Set timeout in milliseconds
~response is get "https://slow-api.example.com/data" {
    "timeout": 5000  # 5 seconds
}
```

### Authentication

#### Bearer Token Authentication
```tilde
~response is get "https://api.example.com/protected" {
    "headers": {
        "Authorization": "Bearer your-jwt-token-here"
    }
}

# Or use the bearer_token shorthand
~response is get "https://api.example.com/protected" {
    "bearer_token": "your-jwt-token-here"
}
```

#### Basic Authentication
```tilde
~response is get "https://api.example.com/protected" {
    "basic_auth": {
        "username": "your-username",
        "password": "your-password"
    }
}
```

### Response Object

HTTP responses contain the following fields:

```tilde
~response is get "https://api.example.com/data"

# Status information
~status_code is ~response.status          # 200
~status_text is ~response.status_text     # "OK"
~is_successful is ~response.ok            # true for 2xx
~is_successful2 is ~response.success      # alias for ok

# Response data
~body_data is ~response.body              # Parsed JSON or raw string
~raw_text is ~response.body_text          # Raw response text
~response_headers is ~response.headers    # Header object

# Timing information
~request-time is ~response.response-time-ms  # Response time in milliseconds
~url is ~response.url                     # Final URL (after redirects)
```

### Error Handling with attempt/rescue

HTTP errors (4xx, 5xx status codes, timeouts, network failures) can be caught and handled:

```tilde
~success is false
~error_info is null

attempt (
    ~response is get "https://api.example.com/might-fail"
    ~success is true
    ~data is ~response.body
) rescue ~error (
    say "Request failed:" ~error.message

    # Access error detilde
    if (has "status" ~error.context) (
        say "HTTP Status:" ~error.context.status
    )

    # Different error types
    if (~error.message contains "timeout") (
        say "Request timed out"
    ) else if (~error.context.status == 404) (
        say "Resource not found"
    ) else if (~error.context.status >= 500) (
        say "Server error"
    )
)
```

### Error Context

HTTP errors provide rich context information:

```tilde
attempt (
    ~response is get "https://httpbin.org/status/404"
) rescue ~error (
    # Error message
    say "Error:" ~error.message             # "http status: 404"

    # HTTP status code (for HTTP errors)
    say "Status:" ~error.context.status    # 404

    # Response timing
    say "Response time:" ~error.context.response-time-ms

    # Timeout setting
    say "Timeout was:" ~error.context.timeout_ms

    # Error body (if available)
    if (has "body" ~error.context) (
        say "Error body:" ~error.context.body
    )

    # Response headers (for HTTP errors)
    if (has "headers" ~error.context) (
        say "Response headers:" ~error.context.headers
    )
)

## System Functions

### Environment Variables

Access environment variables with automatic fallback support:

```tilde
# Basic usage
~port is env "PORT"                    # Returns string value or null
~debug is env "DEBUG"                  # Returns "true"/"false" or null

# With fallback values (recommended pattern)
~port is env "PORT" or 8080            # Returns env value or 8080
~db_url is env "DATABASE_URL" or "localhost"
~timeout is env "TIMEOUT" or 30000

# AWS configuration example
~access_key is env "AWS_ACCESS_KEY_ID" or "not-configured"
~secret_key is env "AWS_SECRET_ACCESS_KEY" or "not-configured"
~region is env "AWS_REGION" or "us-east-1"

# Using in server configuration
server listen (env "PORT" or 8080)
```

**Function Signature:**
- `env variable_name` → string | null

**Parameters:**
- `variable_name` (string): Name of the environment variable to retrieve

**Returns:**
- String value if environment variable exists
- `null` if environment variable is not set

**Error Handling:**
- Function argument must be a string
- Missing environment variables return `null` (not an error)
- Use with `or` operator for graceful fallbacks

## See Also

- [SYNTAX.md](SYNTAX.md) - Complete Tilde language reference
- [examples/stdlib_demo.tilde](../examples/stdlib_demo.tilde) - Working examples
- [examples/foreach_advanced.tilde](../examples/foreach_advanced.tilde) - Alternative iteration patterns