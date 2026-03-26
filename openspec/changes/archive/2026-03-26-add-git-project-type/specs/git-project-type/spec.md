## ADDED Requirements

### Requirement: Git project type exists
The system SHALL support a new project type called "git" that only executes git pull operations without any default install or build steps.

#### Scenario: Git type is recognized
- **WHEN** a project is configured with `project_type = "git"`
- **THEN** the system accepts the configuration as valid

#### Scenario: Git type executes git pull
- **WHEN** a deployment is triggered for a git type project
- **THEN** the system executes git pull to fetch the latest code

#### Scenario: Git type skips install step
- **WHEN** a git type project has no install_command configured
- **THEN** the system skips the install dependencies step

#### Scenario: Git type skips build step
- **WHEN** a git type project has no build_command configured
- **THEN** the system skips the build step

### Requirement: Git type supports custom commands
The system SHALL execute custom install_command and build_command if configured for git type projects.

#### Scenario: Custom install command is executed
- **WHEN** a git type project has install_command configured
- **THEN** the system executes the custom install command

#### Scenario: Custom build command is executed
- **WHEN** a git type project has build_command configured
- **THEN** the system executes the custom build command

### Requirement: Git type supports optional steps
The system SHALL support extra_command and restart_service for git type projects.

#### Scenario: Extra command is executed
- **WHEN** a git type project has extra_command configured
- **THEN** the system executes the extra command after build step

#### Scenario: Docker services are restarted
- **WHEN** a git type project has restart_service configured
- **THEN** the system restarts the specified Docker services after deployment
