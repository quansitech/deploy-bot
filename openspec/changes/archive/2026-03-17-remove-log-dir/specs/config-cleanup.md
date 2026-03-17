## Summary

This change removes the `log_dir` configuration option and simplifies the logging system to output to stderr only.

No new capabilities are introduced - this is a configuration cleanup task.

## REMOVED Requirements

### Requirement: File-based logging via log_dir
**Reason**: Unnecessary complexity - Docker/systemd can capture stderr directly
**Migration**: Use container log collection or journald for log aggregation
