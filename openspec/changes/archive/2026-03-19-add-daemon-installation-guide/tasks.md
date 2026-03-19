## 1. Create systemd unit file

- [x] 1.1 Create `scripts/deploy-bot.service` with Type=simple, ExecStart=/opt/deploy-bot/bin/deploy-bot, WorkingDirectory=/opt/deploy-bot, Restart=on-failure
- [x] 1.2 Add Install section with WantedBy=multi-user.target

## 2. Create SysV init script

- [x] 2.1 Create `scripts/deploy-bot.init` with start/stop/restart/status commands using start-stop-daemon
- [x] 2.2 Configure PID file at `/var/run/deploy-bot.pid`
- [x] 2.3 Redirect stdout/stderr to `/opt/deploy-bot/logs/deploy-bot.log`
- [x] 2.4 Make script executable

## 3. Update README.md

- [x] 3.1 Add "安装为系统服务" section under "3. 启动服务"
- [x] 3.2 Document systemd installation steps (Ubuntu 15.04+ / Debian 8+)
- [x] 3.3 Document SysV init installation steps (Ubuntu 14.04- / Debian 7-)
- [x] 3.4 Document standard directory layout (/opt/deploy-bot/)

## 4. Verification

- [x] 4.1 Run `cargo clippy -- -D warnings` to check code
- [x] 4.2 Run `cargo test` to ensure no regressions
