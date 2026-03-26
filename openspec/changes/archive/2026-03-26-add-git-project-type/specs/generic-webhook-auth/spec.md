## ADDED Requirements

### Requirement: Generic webhook token authentication
The system SHALL support authentication via a generic webhook token for triggering deployments from any source.

#### Scenario: Valid token authenticates successfully
- **WHEN** a webhook request includes X-Webhook-Token header matching the configured webhook_token
- **THEN** the system accepts the request and proceeds with deployment

#### Scenario: Invalid token is rejected
- **WHEN** a webhook request includes X-Webhook-Token header that does not match the configured webhook_token
- **THEN** the system rejects the request with authentication error

#### Scenario: Missing token is rejected
- **WHEN** a webhook request has no recognized authentication header (GitHub/GitLab/Codeup/Generic)
- **THEN** the system rejects the request with authentication error

### Requirement: Token configuration
The system SHALL allow configuring a global webhook_token in config.yaml.

#### Scenario: Token is configured in config file
- **WHEN** webhook_token is set in config.yaml server section
- **THEN** the system loads and uses the token for authentication

#### Scenario: Token is optional
- **WHEN** webhook_token is not configured in config.yaml
- **THEN** the system still accepts requests authenticated via GitHub/GitLab/Codeup tokens

### Requirement: Authentication priority
The system SHALL check authentication methods in order: GitHub, GitLab, Codeup, then generic token.

#### Scenario: First matching authentication succeeds
- **WHEN** a request has multiple authentication headers
- **THEN** the system accepts the request if any one authentication method succeeds
