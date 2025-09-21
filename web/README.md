# Tilde Web REPL 🐈‍⬛

The Tilde Web REPL is a browser-based interactive environment for the Tilde scripting language, powered by WebAssembly.

## 🌐 Access the Published Version

The web REPL is automatically deployed to GitHub Pages and available at:

**https://tj-mc.github.io/tilde/**

## 🚀 Local Development

### Prerequisites

- Rust (with `wasm32-unknown-unknown` target)
- `wasm-pack` tool
- Python 3 (for development server)

### Quick Start

1. **Build the WebAssembly package:**
   ```bash
   ./build-web.sh
   ```

2. **Start the development server:**
   ```bash
   ./serve-web.sh
   ```

   Or with a custom port:
   ```bash
   ./serve-web.sh 8080
   ```

3. **Open your browser:**
   ```
   http://localhost:8000
   ```

### Manual Build Process

If you prefer to build manually:

```bash
# Install wasm-pack (if not already installed)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Build the WASM package
wasm-pack build --target web --out-dir web/pkg --features wasm

# Clean up generated files
rm -f web/pkg/.gitignore web/pkg/package.json web/pkg/README.md

# Serve locally
cd web && python3 -m http.server 8000
```

## 📁 File Structure

```
web/
├── README.md          # This documentation
├── index.html         # Main REPL interface
├── style.css          # Responsive styling
├── repl.js            # JavaScript REPL logic
├── package.json       # Project metadata
└── pkg/               # Generated WASM files (after build)
    ├── tilde.js       # WASM JavaScript bindings
    ├── tilde_bg.wasm  # WebAssembly binary
    └── *.d.ts         # TypeScript definitions
```

## 🎯 Features

- **Real Tilde Interpreter**: Full language support via WebAssembly
- **Interactive Code Editor**: Syntax highlighting with CodeMirror
- **Example Programs**: Built-in examples for learning
- **Responsive Design**: Works on desktop, tablet, and mobile
- **Keyboard Shortcuts**: `Ctrl+Enter` / `Cmd+Enter` to run code
- **Persistent State**: Variables and actions persist between runs

## 🔧 Development Commands

| Command | Description |
|---------|-------------|
| `./build-web.sh` | Build WASM package |
| `./serve-web.sh [port]` | Start development server |
| `npm run build` | Build (from web/ directory) |
| `npm run serve` | Serve (from web/ directory) |
| `npm run dev` | Start dev server (from web/ directory) |

## 🐛 Troubleshooting

### "Failed to load WASM module"
- Ensure you've built the WASM package: `./build-web.sh`
- Check that `web/pkg/` directory exists with `.wasm` files
- Serve from HTTP server (not file://) due to CORS restrictions

### Port Already in Use
- Try a different port: `./serve-web.sh 8081`
- Kill existing processes: `lsof -ti:8000 | xargs kill`

### Build Errors
- Ensure Rust is installed: `rustup --version`
- Add WASM target: `rustup target add wasm32-unknown-unknown`
- Install wasm-pack: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`

## 🚢 Deployment

The web REPL automatically deploys to GitHub Pages via GitHub Actions when you push to the `main` branch.

### Manual Deployment Setup

1. **Enable GitHub Pages** in your repository settings
2. **Set source** to "GitHub Actions"
3. **Push to main branch** to trigger deployment

The workflow file is located at `.github/workflows/web-deploy.yml`.

## 🎨 Customization

- **Styling**: Edit `web/style.css`
- **Examples**: Modify example programs in `web/index.html`
- **Editor**: Configure CodeMirror options in `web/repl.js`
- **Features**: Add new WASM bindings in `src/wasm.rs`

## 📚 Learn More

- [Tilde Language Syntax](../docs/SYNTAX.md)
- [Main Project README](../README.md)
- [Performance Testing](../docs/PERFORMANCE_TESTING.md)