## ADDED Requirements

### Requirement: Daemon installation documentation

The system SHALL provide documentation for running deploy-bot as a system daemon on Ubuntu/Debian servers, supporting both systemd and SysV init systems.

#### Scenario: systemd installation instructions exist

- **WHEN** a user reads the README on a systemd-based Ubuntu/Debian system
- **THEN** they can find instructions to copy `scripts/deploy-bot.service` to `/etc/systemd/system/`, run `systemctl daemon-reload`, and start the service with `systemctl start deploy-bot`

#### Scenario: SysV init installation instructions exist

- **WHEN** a user reads the README on a SysV-based Ubuntu/Debian system
- **THEN** they can find instructions to copy `scripts/deploy-bot.init` to `/etc/init.d/deploy-bot`, set executable permission, and start the service with `service deploy-bot start`

### Requirement: systemd unit file functionality

The provided systemd unit file SHALL enable deploy-bot to:

- Start automatically on system boot (when `systemctl enable` is run)
- Restart automatically on failure
- Output logs to journald (viewable via `journalctl -u deploy-bot`)

#### Scenario: Service starts successfully

- **WHEN** `systemctl start deploy-bot` is executed
- **THEN** the deploy-bot process is running and responding to webhooks

#### Scenario: Service auto-restarts on failure

- **WHEN** the deploy-bot process crashes
- **THEN** systemd automatically restarts it within 5 seconds

### Requirement: SysV init script functionality

The provided SysV init script SHALL enable deploy-bot to:

- Start in background with output redirected to `/opt/deploy-bot/logs/deploy-bot.log`
- Stop gracefully via PID file
- Restart via `service deploy-bot restart`
- Report status via `service deploy-bot status`

#### Scenario: Service starts and logs to file

- **WHEN** `service deploy-bot start` is executed
- **THEN** deploy-bot runs in background and all stdout/stderr are written to `/opt/deploy-bot/logs/deploy-bot.log`

#### Scenario: Service stops gracefully

- **WHEN** `service deploy-bot stop` is executed
- **THEN** the deploy-bot process is terminated and the PID file is removed

### Requirement: Standard installation layout

The installation documentation SHALL specify a standard layout of:

```
/opt/deploy-bot/
├── deploy-bot        # 二进制文件
├── config.yaml       # 配置文件
└── logs/             # SysV init 日志 (仅 SysV)
```

#### Scenario: Binary and config are in correct location

- **WHEN** a user follows the installation instructions
- **THEN** the binary is at `/opt/deploy-bot/deploy-bot` and `config.yaml` is at `/opt/deploy-bot/config.yaml`
