// Tails Web REPL - JavaScript Interface
class TailsWebREPL {
    constructor() {
        this.wasm = null;
        this.repl = null;
        this.editor = null;
        this.outputEl = document.getElementById('output');
        this.loadingEl = document.getElementById('loadingIndicator');
        this.statusEl = document.getElementById('statusText');
        this.runCount = 0;

        this.initializeElements();
        this.setupEventListeners();
        this.updateShortcutDisplay();
        this.loadWasm();
    }

    initializeElements() {
        // Initialize CodeMirror editor
        const textarea = document.getElementById('codeInput');
        this.editor = CodeMirror.fromTextArea(textarea, {
            mode: 'javascript', // Using JavaScript mode as placeholder
            theme: 'material-darker',
            autofocus: true,
            lineNumbers: true,
            lineWrapping: true,
            autoCloseBrackets: true,
            matchBrackets: true,
            indentUnit: 2,
            tabSize: 2,
            extraKeys: {
                'Ctrl-Enter': () => this.runCode(),
                'Cmd-Enter': () => this.runCode()
            }
        });

        // Set default code
        this.editor.setValue('~name is ask "Your name?"\nsay "Hello, " ~name');

        // Style the editor
        this.editor.setSize(null, '100%');
    }

    setupEventListeners() {
        // Run button
        document.getElementById('runBtn').addEventListener('click', () => this.runCode());

        // Clear code button
        document.getElementById('clearCodeBtn').addEventListener('click', () => this.clearCode());

        // Clear button
        document.getElementById('clearBtn').addEventListener('click', () => this.clearOutput());

        // Reset button
        document.getElementById('resetBtn').addEventListener('click', () => this.resetREPL());

        // Examples button
        document.getElementById('examplesBtn').addEventListener('click', () => this.showExamples());

        // Close examples button
        document.getElementById('closeExamples').addEventListener('click', () => this.hideExamples());

        // Close examples on backdrop click
        document.getElementById('examplesBackdrop').addEventListener('click', (e) => {
            // Only close if clicking on the backdrop itself, not the panel
            if (e.target === document.getElementById('examplesBackdrop')) {
                this.hideExamples();
            }
        });

        // Example items
        document.querySelectorAll('.example-item').forEach(item => {
            item.addEventListener('click', () => {
                const code = item.dataset.code.replace(/\\n/g, '\n');
                this.editor.setValue(code);
                this.resetREPL(); // Auto-reset REPL when changing examples
                this.hideExamples();
                this.editor.focus();
            });
        });

        // Handle keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                e.preventDefault();
                this.runCode();
            }
        });
    }

    updateShortcutDisplay() {
        const shortcutEl = document.querySelector('.shortcut');
        if (shortcutEl) {
            const isMac = navigator.userAgent.includes('Mac') || /iPad|iPhone|iPod/.test(navigator.userAgent);
            const shortcutKey = isMac ? '⌘' : 'Ctrl';
            shortcutEl.textContent = `${shortcutKey}+Enter to run`;
        }
    }

    async loadWasm() {
        try {
            this.updateStatus('Loading WebAssembly module...');

            // Import the actual WASM module
            const wasmModule = await import('./pkg/tails.js');
            const { WasmTailsRepl, init } = wasmModule;

            // Initialize the WASM module first with default function, then call init
            await wasmModule.default();
            init();

            // Create the REPL instance
            this.repl = new WasmTailsRepl();

            // Update version display dynamically
            const version = this.repl.get_version();
            document.getElementById('versionText').textContent = `v${version}`;

            this.hideLoading();
            this.updateStatus('Ready');
            const isMac = navigator.userAgent.includes('Mac') || /iPad|iPhone|iPod/.test(navigator.userAgent);
            const shortcutKey = isMac ? '⌘' : 'Ctrl';
            this.addOutput(`Type your Tails code in the editor and click "Run Code" or press ${shortcutKey}+Enter.`, 'info');

        } catch (error) {
            this.hideLoading();
            this.updateStatus('Error loading WASM');
            this.addOutput(`Failed to load Tails WASM module: ${error.message}`, 'error');
            console.error('WASM loading error:', error);
        }
    }

    async runCode() {
        if (!this.repl) {
            this.addOutput('REPL not initialized yet. Please wait...', 'error');
            return;
        }

        const code = this.editor.getValue().trim();
        if (!code) {
            this.addOutput('Please enter some code to execute.', 'error');
            return;
        }

        this.runCount++;
        const runId = this.runCount;

        this.updateStatus(`Running code... (${runId})`);

        try {
            // Execute the code using real WASM
            const resultStr = this.repl.execute(code);
            const result = JSON.parse(resultStr);

            if (result.success) {
                if (result.output && result.output.length > 0) {
                    result.output.forEach(line => {
                        this.addOutput(line, 'success');
                    });
                }

                if (result.value !== null && result.value !== undefined) {
                    this.addOutput(`→ ${JSON.stringify(result.value)}`, 'result');
                }
            } else {
                this.addOutput(result.error || 'Unknown error occurred', 'error');
            }

            this.updateStatus('Ready');

        } catch (error) {
            this.addOutput(`Runtime error: ${error.message}`, 'error');
            this.updateStatus('Error');
            console.error('Execution error:', error);
        }

        // Auto-scroll to bottom
        this.outputEl.scrollTop = this.outputEl.scrollHeight;
    }

    addOutput(text, type = 'normal') {
        const outputLine = document.createElement('div');
        outputLine.className = `output-${type}`;

        if (type === 'code') {
            outputLine.style.opacity = '0.7';
            outputLine.style.fontStyle = 'italic';
        }

        outputLine.textContent = text;
        this.outputEl.appendChild(outputLine);

        // Auto-scroll to bottom
        this.outputEl.scrollTop = this.outputEl.scrollHeight;
    }

    clearOutput() {
        this.outputEl.innerHTML = '';
        this.updateStatus('Output cleared');
    }

    clearCode() {
        this.editor.setValue('');
        this.editor.focus();
        this.updateStatus('Code cleared');
    }

    resetREPL() {
        if (this.repl) {
            this.repl.reset();
        }
        this.clearOutput();
        this.runCount = 0;
        this.addOutput('🔄 REPL reset. All variables and actions cleared.', 'info');
        this.updateStatus('Reset complete');
    }

    showExamples() {
        const backdrop = document.getElementById('examplesBackdrop');
        if (backdrop) {
            document.documentElement.classList.add('modal-open');
            document.body.classList.add('modal-open');
            backdrop.classList.remove('examples-hidden');
        } else {
            console.error('examplesBackdrop element not found');
        }
    }

    hideExamples() {
        const backdrop = document.getElementById('examplesBackdrop');
        if (backdrop) {
            document.documentElement.classList.remove('modal-open');
            document.body.classList.remove('modal-open');
            backdrop.classList.add('examples-hidden');
        }
    }

    hideLoading() {
        this.loadingEl.classList.add('hidden');
    }

    updateStatus(message) {
        this.statusEl.textContent = message;
    }
}


// Initialize the REPL when the page loads
document.addEventListener('DOMContentLoaded', () => {
    new TailsWebREPL();
});

// Global functions that WASM might call
window.tailsPrompt = function(message) {
    return prompt(message) || '';
};

window.tailsFetch = function(url) {
    // Note: This uses deprecated synchronous XMLHttpRequest for WASM compatibility
    // This will block the main thread but allows HTTP requests to work in WASM
    try {
        const xhr = new XMLHttpRequest();
        xhr.open('GET', url, false); // false = synchronous
        xhr.send();

        if (xhr.status >= 200 && xhr.status < 300) {
            // Try to return as-is first (might be JSON)
            return xhr.responseText;
        } else {
            return JSON.stringify({
                error: `HTTP ${xhr.status}: ${xhr.statusText}`,
                url: url
            });
        }
    } catch (error) {
        return JSON.stringify({
            error: error.message,
            url: url,
            note: "CORS or network error - make sure the URL allows cross-origin requests"
        });
    }
};