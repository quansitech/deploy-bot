## Why

README.md currently only shows how to run deploy-bot directly (`cargo run` or `./target/release/deploy-bot`) but does not explain how to run it as a system daemon. Users deploying to production servers need clear instructions for both modern Ubuntu (systemd) and legacy Ubuntu (SysV init).

## What Changes

- **New**: `scripts/deploy-bot.service` — systemd unit file for modern Ubuntu/Debian
- **New**: `scripts/deploy-bot.init` — SysV init script for legacy Ubuntu/Debian
- **Modified**: README.md — add "安装为系统服务" section under "启动服务"

## Capabilities

### New Capabilities

- `daemon-installation`: Documentation and scripts for running deploy-bot as a system service on Ubuntu/Debian, supporting both systemd (15.04+, Debian 8+) and SysV init (legacy) deployment methods.

### Modified Capabilities

- (none — no existing spec behavior changes)

## Impact

**New files:**
- `scripts/deploy-bot.service` — systemd unit file
- `scripts/deploy-bot.init` — SysV init script

**Modified files:**
- `README.md` — new installation section

**Installation layout:**
```
/opt/deploy-bot/
├── deploy-bot        # 二进制
├── config.yaml       # 配置
└── logs/             # SysV init 日志
```
