## ADDED Requirements

### Requirement: Git configuration validation
The system SHALL validate that projects requiring git operations have repo_url and branch configured.

#### Scenario: Non-custom type requires repo_url
- **WHEN** a project with type nodejs/rust/python/php/git has empty or missing repo_url
- **THEN** the system rejects the configuration with validation error

#### Scenario: Non-custom type requires branch
- **WHEN** a project with type nodejs/rust/python/php/git has empty or missing branch
- **THEN** the system rejects the configuration with validation error

#### Scenario: Custom type does not require git config
- **WHEN** a project with type custom has no repo_url or branch configured
- **THEN** the system accepts the configuration as valid

### Requirement: Validation timing
The system SHALL validate project configuration when loading .deploy.yaml file during webhook handling.

#### Scenario: Invalid config prevents deployment
- **WHEN** webhook handler loads an invalid project configuration
- **THEN** the system returns error response without queuing deployment

#### Scenario: Valid config allows deployment
- **WHEN** webhook handler loads a valid project configuration
- **THEN** the system queues the deployment task
