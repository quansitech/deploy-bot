## ADDED Requirements

### Requirement: Default install command for Node.js projects

When a project with project_type "nodejs" does not specify install_command, the system SHALL automatically execute `npm install`.

#### Scenario: Node.js project without install_command
- **WHEN** project_type is "nodejs" and install_command is not configured
- **THEN** system MUST execute "npm install"

### Requirement: Default install command for PHP projects

When a project with project_type "php" does not specify install_command, the system SHALL automatically execute `composer install`.

#### Scenario: PHP project without install_command
- **WHEN** project_type is "php" and install_command is not configured
- **THEN** system MUST execute "composer install"

### Requirement: Default install command for Python projects

When a project with project_type "python" does not specify install_command, the system SHALL automatically execute `pip install -r requirements.txt`.

#### Scenario: Python project without install_command
- **WHEN** project_type is "python" and install_command is not configured
- **THEN** system MUST execute "pip install -r requirements.txt"

### Requirement: Rust projects skip default install

When a project with project_type "rust" does not specify install_command, the system SHALL skip the install step since `cargo build` already handles dependency installation.

#### Scenario: Rust project without install_command
- **WHEN** project_type is "rust" and install_command is not configured
- **THEN** system MUST skip install step (cargo build handles dependencies in build phase)

### Requirement: Custom project type skips default install

When a project with project_type "custom" does not specify install_command, the system SHALL NOT execute any default install command.

#### Scenario: Custom project without install_command
- **WHEN** project_type is "custom" and install_command is not configured
- **THEN** system MUST skip install step (no error)

### Requirement: Custom install_command takes precedence

When a project specifies a custom install_command, the system SHALL always use the custom command, ignoring default commands.

#### Scenario: Custom install_command configured
- **WHEN** install_command is explicitly configured
- **THEN** system MUST use the configured command regardless of project_type
