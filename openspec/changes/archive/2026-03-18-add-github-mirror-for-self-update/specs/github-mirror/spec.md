## ADDED Requirements

### Requirement: GitHub mirror configuration
The system SHALL support configuring a GitHub mirror URL in the config.yaml file under the `[server]` section.

#### Scenario: GitHub mirror configured
- **WHEN** `github_mirror` is set to `"https://ghproxy.com/"` in config.yaml
- **THEN** the configuration SHALL be loaded and available for use during self-update

#### Scenario: GitHub mirror not configured
- **WHEN** `github_mirror` is not set in config.yaml
- **THEN** the system SHALL treat it as no mirror configured and use direct GitHub URLs

### Requirement: Mirror URL applied to GitHub downloads
The system SHALL prepend the configured mirror URL to GitHub download URLs during self-update.

#### Scenario: Apply mirror to GitHub URL
- **WHEN** downloading from `https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot` and `github_mirror` is configured as `"https://ghproxy.com/"`
- **THEN** the system SHALL download from `https://ghproxy.com/https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot`

#### Scenario: Non-GitHub URL bypasses mirror
- **WHEN** downloading from `https://other-cdn.com/file.tar.gz` and `github_mirror` is configured
- **THEN** the system SHALL download directly from the original URL without modification
