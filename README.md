# `~tilde` 

*The easy-to-read programming language*

With friendly syntax, built-in HTTP, and an extensive library of helper functions, Tilde makes scripts that anyone can understand.

## ⚡️ See It In Action

### Hit the web like Spiderman
```tilde
~user is get "https://api.github.com/users/octocat"
say "Found user: `~user.name` with `~user.public_repos` repos"
```

### Break code into Functions
```tilde
function greet-user ~name (
    say "Welcome, `~name`!"
)

# Call them with *
*greet-user "Alice"
```

### Filter and sort like a pro
```tilde
                        👇 built-in helpers 👇
~uppercase-usernames is map ~usernames uppercase
~odd-numbers is filter ~numbers is-odd
~sorted is sort ~odd-numbers
```

_and much more..._ 
Tilde is a batteries-included language. No need to import or install modules to do the most common scripting tasks. 

It's perfect for 
- Learning programming
- Algorithm practice
- Making automations scripts
- Data processing tasks


> ⚠️ Tilde under active development. The API may change until we hit `1.0.0` ⚠️
> 
> Your suggestions & contributions during this time will frame the future direction of Tilde!
## Install & Usage

### [🌐 Try in Browser](https://tj-mc.github.io/tilde/) 👈

The easiest way to try out Tilde! No installation required - runs directly in your browser.




### Install Locally
For the full experience, install Tilde locally.

**Paste into your terminal:**
```bash
curl -sSL https://raw.githubusercontent.com/tj-mc/tilde/main/install.sh | bash
```

**Usage:**
```bash
tilde                 # Start REPL
tilde script.tde      # Run a file
tilde --help          # Show help
```

## 📚 Ready to Learn More?

**[📖 Main Language Reference → SYNTAX.md](docs/SYNTAX.md)**

**📚 [Standard Library Documentation → STDLIB.md](docs/STDLIB.md)**

**✍️ [Examples Directory](./examples)**

**🌐 [Web REPL - Online Tilde Playground](https://tj-mc.github.io/tilde/)**


Discover all of Tilde' features including:
- **Standard Library** (`map`, `filter`, `reduce`) for functional programming
- Advanced object manipulation and property access
- Comprehensive control flow (`loop`, `if`/`else`, `break-loop`)
- HTTP operations (`get`, `wait`) and error handling
- String interpolation patterns and data types
- And much more!


## Motivation
You might wonder why the world needs yet another language. These are my main motivations for creating Tilde.

### We need a language that is easy for a beginner to **learn**.

Take Javascript for example. It is ubiquitous, and flexible. It might seem like an obvious choice for a beginner, but I'd actually consider it quite a confusing introduction to coding.
Due to its web history, there are at least 4 ways to do everything, and a decade of conflicting advice online. I've personally spent years navigating its oddities; I can assure you it is *not* a simple language.

Javascript is incredibly powerful, but if you're interested in learning _CODING_ -- rather than the quirks of a decades-old language -- Tilde could be for you.

In Tilde, there's usually just 1 standard way to do a given thing. 
The basic syntax can be learnt in a day, which is perfect for beginners. It has all the features you'd need to build most scripts, so you can focus on learning programmatic thinking, not setup and tooling.

### We need a language that is easy to **read**.

The syntax is designed to form sentences (where possible) that clearly describe operations.
By prefixing `~variables` and `*custom-functions`, we can allow the standard library to fill the global namespace. The most common operations, like `map`, `write`, `now`, `filter`, `get`, can be composed into tiny, obvious programs - without imports:


Lets compare this same script in **Python**:
```python
 import requests

 result = requests.get("https://api.orderhistory.net/krustykrab").json()

 def get_id(order):
     return order['id']
     
 order_ids = [get_id(order) for order in result]
 sorted_ids = sorted(order_ids)

```

**Javascript** (Node)
```js
import fetch from 'node-fetch';

const result = await fetch("https://api.orderhistory.net/krustykrab").then(r => r.json());
const getId = (order) => order.id
const orderIds = result.map(getId);
const sorted = orderIds.sort();
```

and finally, in **Tilde**:
```tilde
~result is get "https://api.orderhistory.net/krustykrab"
function get-id ~order ( ~order.id )
~order-ids is map ~result get-id
~sorted is sort ~order-ids
```
Shortest in both lines and characters.

As you can see, Tilde is focused on simple and concise syntax that allows you to understand any script easily.

The final reason, is that it's just fun to create a language! I've always been interested in syntax, and how it can be such a huge influence on our software.

## Help
If you need help or found a bug, please raise an issue, and I'll take a look. 
Feature requests and contributions are also welcome. I don't claim to be an expert in Rust, so external input is always appreciated. <3 



## Limitations
All software has limitations and caveats; here's the most notable shortcomings of Tilde.

- Slow
  - The tree-walking interpreter is many times slower than Bun, Node, or Python
- Lack of community/ecosystem
  - A classic chicken/egg problem for new languages - Will you be an early adopter? 
- No package manager for sharing code
  - Coming soon!
- Young
  - We're just getting started, so we haven't found all the cracks and edges yet.
- Struggles with deep recursion (function that calls itself)
  - With Fibonacci @ 35 depth:
    - Bun: ~40ms
    - Tilde: 22s (yes, SECONDS)

## Strengths
It's not _all_ bad.

- English-like syntax
  - Syntax-first design makes code for _humans_
- No legacy issues
  - We got a clean slate here. No need to worry about back-compat!
  - We can design whatever crazy cool API our hearts desire
- Insane startup time
  - Bun (fastest JS engine): ~36ms
  - Tilde: ~6ms
    - This is where Tilde really shines ⭐
    - The simple interpreter design means runtime is a bit slower, but start is instant.

#
 _Born in Melbourne, Australia_