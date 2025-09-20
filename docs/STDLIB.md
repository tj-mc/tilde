# Tails Standard Library

The Tails standard library provides powerful built-in functions for common programming tasks. Stdlib functions are called directly by name and take precedence over user-defined actions when resolving function names.

## Quick Reference

```tails
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
~lower is map ~words lowercase

# Math operations and helpers
~positive is absolute -42
~root is square-root 16
~squares is map ~numbers square
~halved is map ~numbers half
```

## Helper Functions

The standard library includes common helper functions designed to work seamlessly with `map`, `filter`, and `reduce`. Helper functions are called directly by name (e.g., `double`, `is-even`).

### Predicate Functions (for filtering)

#### `is-even number`
Returns `true` if the number is even, `false` otherwise.

```tails
~numbers is [1, 2, 3, 4, 5]
~evens is filter ~numbers is-even
say ~evens  # [2, 4]

# Direct usage
~result is is-even 6
say ~result  # true
```

#### `is-odd number` / `is-odd`
Returns `true` if the number is odd, `false` otherwise.

```tails
~numbers is [1, 2, 3, 4, 5]
~odds is filter ~numbers is-odd
say ~odds  # [1, 3, 5]
```

#### `is-positive number` / `is-positive`
Returns `true` if the number is greater than zero.

```tails
~numbers is [-2, -1, 0, 1, 2]
~positives is filter ~numbers is-positive
say ~positives  # [1, 2]
```

#### `is-negative number` / `is-negative`
Returns `true` if the number is less than zero.

```tails
~numbers is [-2, -1, 0, 1, 2]
~negatives is filter ~numbers is-negative
say ~negatives  # [-2, -1]
```

#### `is-zero number` / `is-zero`
Returns `true` if the number equals zero.

```tails
~numbers is [-1, 0, 1, 0, 2]
~zeros is filter ~numbers is-zero
say ~zeros  # [0, 0]
```

### Transformation Functions (for mapping)

#### `double number` / `double`
Multiplies a number by 2.

```tails
~numbers is [1, 2, 3, 4]
~doubled is map ~numbers double
say ~doubled  # [2, 4, 6, 8]
```

#### `triple number` / `triple`
Multiplies a number by 3.

```tails
~numbers is [1, 2, 3]
~tripled is map ~numbers triple
say ~tripled  # [3, 6, 9]
```

#### `quadruple number` / `quadruple`
Multiplies a number by 4.

```tails
~numbers is [1, 2, 3]
~quadrupled is map ~numbers quadruple
say ~quadrupled  # [4, 8, 12]
```

#### `half number` / `half`
Divides a number by 2.

```tails
~numbers is [2, 4, 6, 8]
~halved is map ~numbers half
say ~halved  # [1, 2, 3, 4]
```

#### `square number` / `square`
Multiplies a number by itself.

```tails
~numbers is [1, 2, 3, 4]
~squares is map ~numbers square
say ~squares  # [1, 4, 9, 16]
```

#### `increment number` / `increment`
Adds 1 to a number.

```tails
~numbers is [0, 1, 2]
~incremented is map ~numbers increment
say ~incremented  # [1, 2, 3]
```

#### `decrement number` / `decrement`
Subtracts 1 from a number.

```tails
~numbers is [3, 2, 1]
~decremented is map ~numbers decrement
say ~decremented  # [2, 1, 0]
```

#### `uppercase string` / `uppercase`
Converts a string to uppercase.

```tails
~words is ["hello", "world"]
~upper is map ~words uppercase
say ~upper  # ["HELLO", "WORLD"]
```

#### `lowercase string` / `lowercase`
Converts a string to lowercase.

```tails
~words is ["HELLO", "WORLD"]
~lower is map ~words lowercase
say ~lower  # ["hello", "world"]
```

### Reduction Functions (for reducing)

#### `add number number` / `add`
Adds two numbers together.

```tails
~numbers is [1, 2, 3, 4, 5]
~sum is reduce ~numbers add 0
say ~sum  # 15
```

#### `multiply number number` / `multiply`
Multiplies two numbers together.

```tails
~numbers is [2, 3, 4]
~product is reduce ~numbers multiply 1
say ~product  # 24
```

#### `max number number` / `max`
Returns the larger of two numbers.

```tails
~numbers is [5, 2, 8, 1, 9]
~maximum is reduce ~numbers max 0
say ~maximum  # 9
```

#### `min number number` / `min`
Returns the smaller of two numbers.

```tails
~numbers is [5, 2, 8, 1, 9]
~minimum is reduce ~numbers min 999
say ~minimum  # 1
```

## List Operations

### `map list function_name`

Transforms each element in a list using the provided function.

**Example:**
```tails
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
```tails
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
```tails
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
```tails
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
```tails
~numbers is [1, 2, 3, 4, 5]
~backwards is reverse ~numbers
say ~backwards  # [5, 4, 3, 2, 1]
```

### `append list value`

Adds a new element to the end of a list, returning a new list with the element appended. This operation is functional - it does not modify the original list.

**Example:**
```tails
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
```tails
action is-big ~n (give ~n > 5)
~numbers is [1, 3, 7, 2, 9]
~first-big is find ~numbers is-big
say ~first-big  # 7
```

#### `find-index list function_name`

Finds the index of the first item that matches a predicate function. Returns the index as a number or null if no match found.

**Example:**
```tails
action is-negative ~n (give ~n < 0)
~numbers is [5, -2, 3, -1]
~neg-index is find-index ~numbers is-negative
say ~neg-index  # 1
```

#### `find-last list function_name`

Finds the last item in a list that matches a predicate function. Returns the item or null if no match found.

**Example:**
```tails
action is-even ~n (give ~n % 2 == 0)
~numbers is [1, 2, 3, 4, 5, 6]
~last-even is find-last ~numbers is-even
say ~last-even  # 6
```

#### `every list function_name`

Tests if all items in a list match a predicate function. Returns true if all match, false otherwise.

**Example:**
```tails
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
```tails
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
```tails
action is-negative ~n (give ~n < 0)
~numbers is [1, -2, 3, -4, 5]
~positives is remove-if ~numbers is-negative
say ~positives  # [1, 3, 5]
```

#### `count-if list function_name`

Counts items in a list that match a predicate function. Returns the count as a number.

**Example:**
```tails
action is-long ~word (give (length ~word) > 4)
~words is ["cat", "elephant", "dog", "hippopotamus"]
~long-count is count-if ~words is-long
say ~long-count  # 2
```

#### `take-while list function_name`

Takes items from the beginning of a list while a predicate is true. Stops at the first item that doesn't match.

**Example:**
```tails
action is-small ~n (give ~n < 5)
~numbers is [1, 2, 3, 7, 4, 1]
~small-prefix is take-while ~numbers is-small
say ~small-prefix  # [1, 2, 3]
```

#### `drop-while list function_name`

Drops items from the beginning of a list while a predicate is true. Returns the remaining items after the first non-match.

**Example:**
```tails
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
```tails
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
```tails
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
```tails
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
```tails
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
```tails
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
```tails
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
```tails
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
```tails
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
```tails
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
```tails
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
```tails
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
```tails
~numbers is [1, 2, 3, 4]
~result is shift ~numbers
say ~result.value  # 1
say ~result.list   # [2, 3, 4]
```

#### `unshift list value`

Adds an element to the beginning of a list.

**Example:**
```tails
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
```tails
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

#### `index-of list value`

Finds the index of the first occurrence of a value in a list. Returns the index as a number or null if not found.

**Example:**
```tails
~fruits is ["apple", "banana", "cherry", "banana"]
~index is index-of ~fruits "banana"
say ~index  # 1

~index is index-of ~fruits "grape"
say ~index  # null
```

#### `contains list value`

Checks if a list contains a specific value. Returns true or false.

**Example:**
```tails
~numbers is [1, 2, 3, 4, 5]
~has-three is contains ~numbers 3
say ~has-three  # true

~has-ten is contains ~numbers 10
say ~has-ten  # false
```

#### `slice list start [end]`

Extracts a portion of a list from start index to end index (exclusive). If end is not provided, slices to the end.

**Example:**
```tails
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
```tails
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
```tails
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
```tails
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
```tails
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
```tails
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
```tails
~numbers is [1, 2, 3]
~letters is ["a", "b", "c", "d"]
~pairs is zip ~numbers ~letters
say ~pairs  # [[1, "a"], [2, "b"], [3, "c"]]
```

#### `chunk list size`

Partitions a list into chunks of the specified size.

**Example:**
```tails
~numbers is [1, 2, 3, 4, 5, 6, 7]
~chunks is chunk ~numbers 3
say ~chunks  # [[1, 2, 3], [4, 5, 6], [7]]

~pairs is chunk ~numbers 2
say ~pairs  # [[1, 2], [3, 4], [5, 6], [7]]
```

#### `transpose matrix`

Transposes a matrix (list of lists) where rows become columns.

**Example:**
```tails
~matrix is [[1, 2, 3], [4, 5, 6]]
~transposed is transpose ~matrix
say ~transposed  # [[1, 4], [2, 5], [3, 6]]

# Works with jagged arrays
~jagged is [[1, 2], [3, 4, 5], [6]]
~result is transpose ~jagged
say ~result  # [[1, 3, 6], [2, 4, null], [null, 5, null]]
```

## Functional Programming Patterns with New Operations

### Complex Data Processing Pipelines

```tails
~data is [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Pipeline: get evens → square them → take first 3 → chunk into pairs
~evens is filter ~data is-even       # [2, 4, 6, 8, 10]
~squared is map ~evens square        # [4, 16, 36, 64, 100]
~first-three is take ~squared 3      # [4, 16, 36]
~pairs is chunk ~first-three 2       # [[4, 16], [36]]

say ~pairs
```

### Set Operations for Data Comparison

```tails
# Compare different datasets
~last-week-users is ["alice", "bob", "charlie", "david"]
~this-week-users is ["bob", "charlie", "eve", "frank"]

# Find all users across both weeks
~all-users is union ~last-week-users ~this-week-users
say "All users:" ~all-users  # ["alice", "bob", "charlie", "david", "eve", "frank"]

# Find returning users
~returning is intersection ~last-week-users ~this-week-users
say "Returning users:" ~returning  # ["bob", "charlie"]

# Find users who stopped using the service
~churned is difference ~last-week-users ~this-week-users
say "Churned users:" ~churned  # ["alice", "david"]

# Find new users
~new-users is difference ~this-week-users ~last-week-users
say "New users:" ~new-users  # ["eve", "frank"]
```

### Working with Nested Data

```tails
~nested-data is [[1, 2], [3, [4, 5]], [6, 7, 8]]

# Flatten and process
~flat is flatten ~nested-data        # [1, 2, 3, 4, 5, 6, 7, 8]
~unique is unique ~flat              # [1, 2, 3, 4, 5, 6, 7, 8]
~chunks is chunk ~unique 3           # [[1, 2, 3], [4, 5, 6], [7, 8]]

say ~chunks
```

### Matrix Operations

```tails
~matrix-a is [[1, 2], [3, 4], [5, 6]]
~matrix-b is [[7, 8], [9, 10], [11, 12]]

# Zip matrices row-wise
~combined is zip ~matrix-a ~matrix-b
say ~combined  # [[[1, 2], [7, 8]], [[3, 4], [9, 10]], [[5, 6], [11, 12]]]

# Transpose for column operations
~cols-a is transpose ~matrix-a
~cols-b is transpose ~matrix-b
say ~cols-a  # [[1, 3, 5], [2, 4, 6]]
```

### List Building and Mutation Chains

```tails
~base is [1, 2, 3]

# Build complex list through chaining
~step1 is insert ~base 1 99         # [1, 99, 2, 3]
~step2 is unshift ~step1 0          # [0, 1, 99, 2, 3]
~step3 is remove ~step2 99          # [0, 1, 2, 3]
~step4 is set-at ~step3 0 10        # [10, 1, 2, 3]

say ~step4
```

## String Operations

### `split string delimiter`

Splits a string into a list using the given delimiter.

**Example:**
```tails
~csv is "apple,banana,cherry"
~fruits is split ~csv ","
say ~fruits  # ["apple", "banana", "cherry"]

~sentence is "hello world from tails"
~words is split ~sentence " "
say ~words  # ["hello", "world", "from", "tails"]
```

### `join list delimiter`

Joins a list of values into a single string using the given delimiter.

**Example:**
```tails
~words is ["hello", "world", "from", "tails"]
~sentence is join ~words " "
say ~sentence  # "hello world from tails"

~numbers is [1, 2, 3, 4]
~csv is join ~numbers ","
say ~csv  # "1,2,3,4"
```

### `trim string`

Removes whitespace from the beginning and end of a string.

**Example:**
```tails
~messy is "  hello world  "
~clean is trim ~messy
say "'" ~clean "'"  # 'hello world'
```

## Math Operations

### `absolute number`

Returns the absolute value of a number.

**Example:**
```tails
~positive is absolute -42
say ~positive  # 42

~already_positive is absolute 15
say ~already_positive  # 15
```

### `square-root number`

Returns the square root of a number. The number must be non-negative.

**Example:**
```tails
~root is square-root 16
say ~root  # 4

~decimal is square-root 2
say ~decimal  # 1.4142135623730951
```

**Error:** Throws an error if the number is negative.


## See Also

- [SYNTAX.md](SYNTAX.md) - Complete Tails language reference
- [examples/stdlib_demo.tails](../examples/stdlib_demo.tails) - Working examples
- [examples/foreach_advanced.tails](../examples/foreach_advanced.tails) - Alternative iteration patterns