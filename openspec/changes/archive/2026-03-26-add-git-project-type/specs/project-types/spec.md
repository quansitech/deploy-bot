## MODIFIED Requirements

### Requirement: Custom type behavior
The system SHALL skip git pull operations for projects with project_type = "custom", allowing fully custom deployment workflows.

#### Scenario: Custom type skips git pull
- **WHEN** a deployment is triggered for a custom type project
- **THEN** the system skips the git pull step and logs the skip

#### Scenario: Custom type skips install by default
- **WHEN** a custom type project has no install_command configured
- **THEN** the system skips the install dependencies step

#### Scenario: Custom type skips build by default
- **WHEN** a custom type project has no build_command configured
- **THEN** the system skips the build step

#### Scenario: Custom type executes custom commands
- **WHEN** a custom type project has install_command or build_command configured
- **THEN** the system executes the configured commands

#### Scenario: Custom type executes extra command
- **WHEN** a custom type project has extra_command configured
- **THEN** the system executes the extra command

#### Scenario: Custom type restarts services
- **WHEN** a custom type project has restart_service configured
- **THEN** the system restarts the specified Docker services
