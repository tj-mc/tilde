# `~tilde` 

*The interpreted programming language for getting things done.*

With english-like syntax, and powerful built-ins for modern tasks like HTTP requests and data processing, Tilde makes scripts that anyone can understand.

## ⚡️ See It In Action

### Built-in HTTP
```tilde
~user is get "https://api.github.com/users/octocat"
say "Found user: `~user.name` with `~user.public_repos` repos"
```

### Functions
```tilde
function greet-user ~name (
    say "Welcome, `~name`!"
)

# Call them with *
*greet-user "Alice"
```

### List Processing 
```tilde
                        👇 built-in helpers 👇
~uppercase-usernames is map ~usernames uppercase
~odd-numbers is filter ~numbers is-odd
~sorted is sort ~odd-numbers
```

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


## Why?
You might wonder why the world needs yet another language, and you're perfectly right to think that.

Javascript 