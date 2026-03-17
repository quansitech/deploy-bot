#!/bin/bash
# update-service.sh
# For systems using SysVinit (older Linux distributions)

set -e

NEW_BINARY="$1"
SERVICE_NAME="deploy-bot"
BINARY_PATH="/usr/local/bin/deploy-bot"
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

echo "Stopping $SERVICE_NAME service..."
service "$SERVICE_NAME" stop || true

# Wait for process to stop
sleep 2

# Force kill if still running
if pgrep -x "$SERVICE_NAME" > /dev/null 2>&1; then
    pkill -9 "$SERVICE_NAME" || true
    sleep 1
fi

echo "Replacing binary..."
cp "$NEW_BINARY" "$BINARY_PATH"
chmod +x "$BINARY_PATH"

echo "Starting $SERVICE_NAME service..."
service "$SERVICE_NAME" start

echo "=== $(date) Update completed successfully ==="
