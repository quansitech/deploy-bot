## Context

deploy-bot is a webhook-based deployment service written in Rust. The README currently only documents direct execution (`cargo run` or `./target/release/deploy-bot`). For production deployment on Ubuntu/Debian servers, users need to run deploy-bot as a system daemon (background service) with proper startup/shutdown management.

Two init systems are relevant:
- **systemd**: Ubuntu 15.04+ and Debian 8+ (default modern init)
- **SysV init**: Ubuntu 14.04- and Debian 7- (legacy, uses `service` command)

The `scripts/update-systemd.sh` already assumes the binary is at `/usr/local/bin/deploy-bot`, but no unit file or init script is provided in the repository.

## Goals / Non-Goals

**Goals:**
- Provide a systemd unit file for modern Ubuntu/Debian
- Provide a SysV init script for legacy Ubuntu/Debian
- Document installation steps in README.md
- Use consistent directory layout: `/opt/deploy-bot/`
- Support SysV init logging to file

**Non-Goals:**
- Creating a dedicated `deploy-bot` system user (run as root for simplicity)
- Auto-installation script (manual file placement is sufficient)
- Supporting other init systems (SysV/systemd only)
- Modifying deploy-bot binary or application behavior

## Decisions

### 1. Directory layout: `/opt/deploy-bot/`

**Decision:** Place all deploy-bot files under `/opt/deploy-bot/`

```
/opt/deploy-bot/
├── deploy-bot        # 二进制
├── config.yaml       # 配置
└── logs/             # SysV init 日志
```

**Rationale:** FHS-compliant (`/opt` for third-party software), self-contained, easy to backup/migrate.

**Alternatives considered:**
- `/usr/local/deploy-bot`: Not self-contained, mixed with system files
- `/home/deploy-bot`: Not appropriate for a server tool

### 2. systemd unit: `/etc/systemd/system/deploy-bot.service`

**Decision:** Provide a simple systemd unit file.

```ini
[Unit]
Description=Deploy Bot Webhook Service
After=network.target

[Service]
Type=simple
ExecStart=/opt/deploy-bot/deploy-bot
WorkingDirectory=/opt/deploy-bot
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

**Rationale:**
- `Type=simple`: deploy-bot runs in foreground, no forking
- `After=network.target`: ensures network is up before starting
- `Restart=on-failure`: auto-restart on crash
- `WorkingDirectory=/opt/deploy-bot`: binary finds `config.yaml` relative to working dir

### 3. SysV init script: `/etc/init.d/deploy-bot`

**Decision:** Provide a bash init script using `start-stop-daemon`.

**Key features:**
- `start`: Check if already running, then start with output redirected
- `stop`: Graceful shutdown via PID file
- `restart`: stop + start
- `status`: Check PID file and process existence
- Logging: Redirect stdout/stderr to `/opt/deploy-bot/logs/deploy-bot.log`

**Rationale:** `start-stop-daemon` is the standard SysV tool for managing daemons on Debian/Ubuntu.

### 4. Config file location

**Decision:** `config.yaml` at `/opt/deploy-bot/config.yaml`

**Rationale:** Relative to binary location, matches current README convention. Binary should find config relative to working directory (`/opt/deploy-bot`).

### 5. SysV init log file: `/opt/deploy-bot/logs/deploy-bot.log`

**Decision:** Create `/opt/deploy-bot/logs/` directory, redirect all output to `deploy-bot.log`.

**Rationale:** SysV init does not auto-manage logs. Manual file redirection is required. systemd uses journald (`journalctl -u deploy-bot`).

## Risks / Trade-offs

| Risk | Mitigation |
|------|-----------|
| Config file not found | Ensure `WorkingDirectory=/opt/deploy-bot` is set; document that `config.yaml` must be in that directory |
| SysV init log grows unbounded | Document log rotation (e.g., `logrotate`) or manual cleanup; not automated |
| Binary permissions | Binary must be executable (`chmod +x`); documented in README |
| Running as root | Document security implication; suggest creating dedicated user for production |

## Migration Plan

**For systemd users:**
1. Copy `scripts/deploy-bot.service` to `/etc/systemd/system/`
2. Run `systemctl daemon-reload`
3. Run `systemctl start deploy-bot`
4. Optional: `systemctl enable deploy-bot` for boot start

**For SysV users:**
1. Copy `scripts/deploy-bot.init` to `/etc/init.d/deploy-bot`
2. `chmod +x /etc/init.d/deploy-bot`
3. Create `/opt/deploy-bot/logs/` directory
4. Run `service deploy-bot start`

**Rollback:** Simply `systemctl stop deploy-bot` or `service deploy-bot stop` and remove the unit/init files.

## Open Questions

- Should the README document how to create a non-root user (`deploy-bot`)? (Currently running as root)
- Should `workspace/` and `logs/` directories be created by the init scripts or documented as manual steps?
