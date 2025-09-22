# Phase 2: Standard Library Expansion Plan

## Objective
Implement the top 90% of foundational utilities missing from the current standard library to make Tilde production-ready for real-world automation and data processing tasks.

## Current State Analysis
- **Strengths**: Excellent functional programming primitives (map, filter, reduce), comprehensive list operations, basic I/O
- **Critical Gap**: No date/time operations, limited string processing, no JSON handling, basic math only
- **Blocker**: No structured error handling mechanism

## Implementation Phases

### Phase 2.1 - Critical Foundations (Tier 1)
**Target: Essential operations for 90% coverage**

#### Date/Time Operations
```
now() -> timestamp
date(year, month, day) -> date_object
time(hour, minute, second) -> time_object
timestamp() -> unix_timestamp
date-add(date, amount, unit) -> date
date-subtract(date, amount, unit) -> date
date-diff(date1, date2, unit) -> number
format-date(date, format_string) -> string
parse-date(string, format_string) -> date
year(date) -> number
month(date) -> number
day(date) -> number
hour(time) -> number
minute(time) -> number
second(time) -> number
```

#### Advanced String Processing
```
regex-match(string, pattern) -> list|null
regex-replace(string, pattern, replacement) -> string
regex-split(string, pattern) -> list
starts-with(string, prefix) -> boolean
ends-with(string, suffix) -> boolean
contains(string, substring) -> boolean
substring(string, start, [length]) -> string
replace(string, old, new) -> string
repeat(string, count) -> string
pad-left(string, length, [char]) -> string
pad-right(string, length, [char]) -> string
```

#### Core Math Functions
```
sin(radians) -> number
cos(radians) -> number
tan(radians) -> number
asin(number) -> radians
acos(number) -> radians
atan(number) -> radians
atan2(y, x) -> radians
log(number, [base]) -> number
log10(number) -> number
exp(number) -> number
pow(base, exponent) -> number
round(number, [precision]) -> number
floor(number) -> number
ceil(number) -> number
pi -> constant
e -> constant
```

#### JSON Serialization
```
to-json(value) -> string
from-json(string) -> value
```

#### Type Checking System
```
is-number(value) -> boolean
is-string(value) -> boolean
is-boolean(value) -> boolean
is-list(value) -> boolean
is-object(value) -> boolean
is-null(value) -> boolean
is-empty(value) -> boolean
is-defined(value) -> boolean
```

#### Error Handling Framework
```
try(action_name, ...args) -> {success: boolean, value: any, error: string}
throw(message) -> error
assert(condition, message) -> void|error
```

### Phase 2.2 - System Integration (Tier 2)

#### File System Operations
```
list-files(directory) -> list
list-directories(directory) -> list
file-exists(path) -> boolean
directory-exists(path) -> boolean
create-directory(path) -> result_object
remove-file(path) -> result_object
remove-directory(path) -> result_object
copy-file(source, destination) -> result_object
move-file(source, destination) -> result_object
file-size(path) -> number
file-modified(path) -> timestamp
```

#### Extended HTTP Operations
```
post(url, data, [headers]) -> response_object
put(url, data, [headers]) -> response_object
delete(url, [headers]) -> response_object
patch(url, data, [headers]) -> response_object
http-headers(response) -> object
http-status(response) -> number
```

#### Statistical Functions
```
sum(list) -> number
mean(list) -> number
median(list) -> number
mode(list) -> value
variance(list) -> number
standard-deviation(list) -> number
percentile(list, percent) -> number
quartile(list, quarter) -> number
```

#### Object Manipulation
```
merge(object1, object2, ...) -> object
deep-merge(object1, object2) -> object
pick(object, keys_list) -> object
omit(object, keys_list) -> object
invert(object) -> object
flatten-object(object, [separator]) -> object
object-to-list(object) -> list
list-to-object(list, key_func, value_func) -> object
```

#### Environment Access
```
env-get(variable_name) -> string|null
env-set(variable_name, value) -> boolean
args() -> list
current-directory() -> string
change-directory(path) -> boolean
platform() -> string
exit([status_code]) -> void
```

### Phase 2.3 - Advanced Utilities (Tier 3)

#### Data Encoding
```
encode-base64(string) -> string
decode-base64(string) -> string
encode-url(string) -> string
decode-url(string) -> string
to-csv(list_of_objects) -> string
from-csv(string) -> list_of_objects
```

#### Basic Cryptography
```
hash-md5(string) -> string
hash-sha256(string) -> string
hash-sha512(string) -> string
uuid-generate() -> string
random-string(length, [charset]) -> string
random-bytes(length) -> list
hmac(message, key, algorithm) -> string
```

#### Data Validation
```
validate-email(string) -> boolean
validate-url(string) -> boolean
validate-phone(string, [format]) -> boolean
validate-json(string) -> boolean
```

#### Advanced Math
```
factorial(number) -> number
gcd(a, b) -> number
lcm(a, b) -> number
mod-inverse(a, m) -> number
clamp(value, min, max) -> number
lerp(start, end, t) -> number
```

#### Performance Utilities
```
benchmark(action_name, ...args) -> {time: number, result: any}
cache(action_name, ...args) -> any
debounce(action_name, delay) -> function
throttle(action_name, interval) -> function
```

## Implementation Notes

### Error Handling Strategy
- All new functions return result objects with `{success, value, error}` pattern
- Backward compatibility: existing functions unchanged
- New `try` built-in wraps any function call with error catching

### Memory & Performance
- All list operations remain immutable
- String operations return new strings
- Date objects are immutable value types
- Regex operations use compiled patterns internally

### Testing Requirements
- Unit tests for all functions
- Integration tests for complex workflows
- Performance benchmarks vs. equivalent scripts in other languages
- Cross-platform compatibility tests

### Documentation Requirements
- STDLIB.md expansion with examples for each function
- Migration guide from Phase 1 to Phase 2
- Performance characteristics documentation
- Best practices for each utility category

## Success Metrics
- Can implement 90% of common automation scripts without external dependencies
- Performance within 2x of equivalent Python/Node.js scripts
- Zero breaking changes to existing code
- Complete test coverage for all new functions

## Estimated Implementation Effort
- **Phase 2.1**: ~6-8 weeks (critical path: error handling framework)
- **Phase 2.2**: ~4-6 weeks
- **Phase 2.3**: ~3-4 weeks
- **Total**: ~13-18 weeks for complete Phase 2