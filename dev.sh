#!/bin/bash
# Custom dev server that bypasses Trunk's buggy wasm-bindgen stage dir handling.
# Usage: ./dev.sh [port]
set -e

PORT=${1:-8081}
PROJ_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$PROJ_DIR"
DIST_DIR="$PROJ_DIR/dist"
CRATE="lichtblick-app"
WASM_TARGET="wasm32-unknown-unknown"
CARGO_TOML="$PROJ_DIR/crates/$CRATE/Cargo.toml"
TARGET_DIR="$PROJ_DIR/target/$WASM_TARGET/debug"
WASM_FILE="$TARGET_DIR/${CRATE//-/_}.wasm"
BINDGEN_OUT="$DIST_DIR"

mkdir -p "$DIST_DIR"

build() {
    echo "🔨 Building..."
    cargo build --target "$WASM_TARGET" -p "$CRATE" 2>&1 | tail -5

    echo "🔗 Running wasm-bindgen..."
    wasm-bindgen "$WASM_FILE" \
        --out-dir "$BINDGEN_OUT" \
        --out-name lichtblick_app \
        --target web \
        --no-typescript

    # Copy static assets
    cp "$PROJ_DIR/web/style.css" "$DIST_DIR/"

    # Generate index.html for the dist
    cat > "$DIST_DIR/index.html" << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Lichtblick</title>
    <meta name="description" content="Lichtblick - Robotics Data Visualization" />
    <link rel="stylesheet" href="style.css" />
</head>
<body>
    <noscript>Lichtblick requires WebAssembly and JavaScript to run.</noscript>
    <script type="module">
        import init from './lichtblick_app.js';
        init();
    </script>
</body>
</html>
EOF

    echo "✅ Build complete"
}

# Initial build
build

# Start simple HTTP server in background
echo "📡 Starting server on http://127.0.0.1:$PORT/"
if command -v python3 &>/dev/null; then
    (cd "$DIST_DIR" && python3 -m http.server "$PORT" --bind 127.0.0.1) &
    SERVER_PID=$!
else
    echo "ERROR: python3 not found for HTTP server"
    exit 1
fi

trap "kill $SERVER_PID 2>/dev/null; exit" INT TERM

# Watch for changes and rebuild
echo "👀 Watching for changes in crates/ and web/..."
if command -v fswatch &>/dev/null; then
    fswatch -o -l 2 "$PROJ_DIR/crates" "$PROJ_DIR/web" | while read -r; do
        echo ""
        echo "🔄 Change detected, rebuilding..."
        build || echo "❌ Build failed"
    done
elif command -v cargo-watch &>/dev/null; then
    cargo watch -w crates -w web -s "$(realpath "$0") --rebuild-only" &
    WATCH_PID=$!
    wait $WATCH_PID
else
    echo "⚠️  No file watcher found. Install fswatch (brew install fswatch) for hot-reload."
    echo "   Running without hot-reload. Press Ctrl+C to stop."
    wait $SERVER_PID
fi
