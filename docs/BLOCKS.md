# Block Syntax in Tilde

Tilde supports **block syntax** for organizing and accessing functions from different modules/namespaces. This allows you to explicitly specify which implementation of a function you want to use, and enables future module imports.

## Syntax

Block syntax uses colons on both sides: `:block_name:function_name`

```tilde
:core:is-even 4
:core:double 5
:math:sqrt 16
```

## Available Blocks

### `:core:` Block

The `:core:` block provides access to the standard library functions, bypassing any user-defined functions with the same name.

**Available functions:**
- **Predicates:** `is-even`, `is-odd`, `is-positive`, `is-negative`, `is-zero`
- **Math:** `double`, `triple`, `quadruple`, `half`, `square`, `increment`, `decrement`, `absolute`, `square-root`
- **List operations:** `map`, `filter`, `reduce`, `sort`, `reverse`, `length`, `append`
- **String operations:** `split`, `join`, `trim`, `uppercase`, `lowercase`
- **Object operations:** `keys`, `values`, `has`
- **And many more...**

## Priority Resolution

Tilde follows this priority order when resolving function names:

1. **Block syntax** (`:core:function`) - Always uses the specified block
2. **User-defined functions** - Functions you define with `function`
3. **Standard library** - Built-in functions without block prefix

## Examples

### Basic Usage

```tilde
# Using core functions directly
~result is :core:is-even 4
say ~result  # true

# Using in calculations
~doubled is :core:double 5
say ~doubled  # 10
```

### Priority Resolution Example

```tilde
# Define a custom is-even that always returns false
function is-even ~x (
    give false
)

~user_result is is-even 4        # false (uses user function)
~core_result is :core:is-even 4  # true (uses core function)

say "User result: " ~user_result  # User result: false
say "Core result: " ~core_result  # Core result: true
```

### Higher-Order Functions

Block syntax works seamlessly with higher-order functions like `map`:

```tilde
~numbers is [1, 2, 3, 4, 5]

# Using core function in map
~even_checks is map ~numbers :core:is-even
say ~even_checks  # [false, true, false, true, false]

# Using core function for transformation
~doubled is map ~numbers :core:double
say ~doubled  # [2, 4, 6, 8, 10]
```

### Mixed Usage

```tilde
# You can mix user functions and core functions
function custom-transform ~x (
    give ~x * 10
)

~numbers is [1, 2, 3]
~custom_result is map ~numbers custom-transform  # [10, 20, 30]
~core_result is map ~numbers :core:square        # [1, 4, 9]
```

## Error Handling

### Unknown Block

```tilde
~result is :unknown:function 1
# Error: Unknown block: unknown
```

### Unknown Function in Block

```tilde
~result is :core:unknown-function 1
# Error: Unknown core function: unknown-function
```

## Future Extensions

The block syntax is designed to support future module imports:

```tilde
# Future syntax (not yet implemented)
import :math: from "math-lib"
import :string: from "string-utils"

~result is :math:complex-calculation 5
~text is :string:advanced-format "Hello {}"
```

## Best Practices

1. **Use `:core:` when you need guaranteed standard behavior**
   ```tilde
   # Ensure standard is-even logic even if user overrides it
   ~filtered is filter ~numbers :core:is-even
   ```

2. **Prefer bare names for user-defined functions**
   ```tilde
   function my-custom-logic ~x ( ... )
   ~result is map ~data my-custom-logic
   ```

3. **Use block syntax in libraries to avoid conflicts**
   ```tilde
   # In a library, use :core: to ensure predictable behavior
   function validate-positive ~x (
       if :core:is-positive ~x (
           give true
       ) else (
           give false
       )
   )
   ```

The block syntax provides a clean way to organize code and avoid naming conflicts while maintaining backward compatibility with existing Tilde scripts.