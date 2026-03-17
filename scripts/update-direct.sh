#!/bin/bash
# update-direct.sh
# For systems without systemd (or running deploy-bot directly)

set -e

NEW_BINARY="$1"
BINARY_PATH="/path/to/deploy-bot"  # 修改为你的 deploy-bot 路径
PID_FILE="/var/run/deploy-bot.pid"  # 修改为你的 PID 文件路径

if [ -z "$NEW_BINARY" ]; then
    echo "Usage: $0 <path-to-new-binary>"
    exit 1
fi

if [ ! -f "$NEW_BINARY" ]; then
    echo "Error: New binary not found: $NEW_BINARY"
    exit 1
fi

# 停止运行中的进程
if [ -f "$PID_FILE" ]; then
    PID=$(cat "$PID_FILE")
    if kill -0 "$PID" 2>/dev/null; then
        echo "Stopping deploy-bot (PID: $PID)..."
        kill "$PID"
        # 等待进程停止
        for i in {1..10}; do
            if ! kill -0 "$PID" 2>/dev/null; then
                break
            fi
            sleep 1
        done
        # 如果进程仍未停止，强制终止
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
# 后台启动 deploy-bot
nohup "$BINARY_PATH" > /var/log/deploy-bot.log 2>&1 &
echo $! > "$PID_FILE"

echo "Update completed successfully!"
