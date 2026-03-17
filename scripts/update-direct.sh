#!/bin/bash
# update-direct.sh
# For systems without systemd (or running deploy-bot directly)

set -e

NEW_BINARY="$1"
BINARY_PATH="/path/to/deploy-bot"  # 修改为你的 deploy-bot 路径
PID_FILE="/var/run/deploy-bot.pid"  # 修改为你的 PID 文件路径
LOG_FILE="/var/log/deploy-bot-update.log"

# Redirect all output to log file
exec > "$LOG_FILE" 2>&1

echo "=== $(date) Starting update ==="

if [ -z "$NEW_BINARY" ]; then
    echo "Usage: $0 <path-to-new-binary>"
    exit 1
fi

if [ ! -f "$NEW_BINARY" ]; then
    echo "Error: New binary not found: $NEW_BINARY"
    exit 1
fi

# Stop running process
if [ -f "$PID_FILE" ]; then
    PID=$(cat "$PID_FILE")
    if kill -0 "$PID" 2>/dev/null; then
        echo "Stopping deploy-bot (PID: $PID)..."
        kill "$PID"
        # Wait for process to stop
        for i in {1..10}; do
            if ! kill -0 "$PID" 2>/dev/null; then
                break
            fi
            sleep 1
        done
        # Force kill if still running
        if kill -0 "$PID" 2>/dev/null; then
            echo "Force killing process..."
            kill -9 "$PID" 2>/dev/null || true
        fi
    fi
    rm -f "$PID_FILE"
fi

echo "Replacing binary..."
cp "$NEW_BINARY" "$BINARY_PATH"
chmod +x "$BINARY_PATH"

echo "Starting deploy-bot..."
# Start deploy-bot in background
nohup "$BINARY_PATH" > /var/log/deploy-bot.log 2>&1 &
echo $! > "$PID_FILE"

echo "=== $(date) Update completed successfully ==="
