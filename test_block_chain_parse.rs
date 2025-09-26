# Test chain operations with if statements
~numbers is [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Test function calls in conditions with chaining
~filtered is filter ~numbers is-even
~count is length ~filtered

if ~count > 3 (
    ~message is "Many evens found"
) else (
    ~message is "Few evens found"
)

# Test function calls with parentheses in complex expressions
~obj1 is {"items": [1, 2, 3]}
~obj2 is {"items": [4, 5]}

if length(~obj1.items) > length(~obj2.items) (
    ~winner is "obj1"
) else (
    ~winner is "obj2"
)

# Test deeply nested conditions with stdlib functions
~text is "Hello World"
~words is split ~text " "

if length(~words) == 2 (
    if contains(~text, "Hello") (
        ~analysis is "greeting found"
    ) else (
        ~analysis is "two words but no greeting"
    )
) else (
    ~analysis is "not two words"
)

say "Message: " ~message
say "Winner: " ~winner  
say "Analysis: " ~analysis
