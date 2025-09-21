#!/bin/bash

# Tilde Web REPL Development Server
# Serves the web REPL locally for development and testing

PORT=${1:-8000}

echo "🐈‍⬛ Starting Tilde Web REPL development server..."
echo "Server will be available at: http://localhost:$PORT"
echo "Press Ctrl+C to stop the server"
echo ""

cd web

# Try different server options
if command -v python3 &> /dev/null; then
    echo "Using Python 3 HTTP server..."
    python3 -m http.server $PORT
elif command -v python &> /dev/null; then
    echo "Using Python HTTP server..."
    python -m SimpleHTTPServer $PORT
elif command -v node &> /dev/null && command -v npx &> /dev/null; then
    echo "Using Node.js http-server..."
    npx http-server -p $PORT
else
    echo "❌ No suitable HTTP server found."
    echo "Please install Python 3 or Node.js to run the development server."
    exit 1
fi