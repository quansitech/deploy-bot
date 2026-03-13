## ADDED Requirements

### Requirement: Git pull detects empty directory for clone

The git operations module MUST detect when a project directory exists but contains no files except `.deploy.yaml`, and MUST use `git clone` to initialize the repository.

#### Scenario: Empty directory triggers git clone
- **WHEN** project directory exists and contains only `.deploy.yaml` (or is completely empty)
- **THEN** system MUST execute `git clone --branch <branch> --depth 1 <repo_url> .` to clone into current directory

#### Scenario: Non-empty directory triggers git fetch
- **WHEN** project directory exists and contains other files (excluding `.deploy.yaml`)
- **THEN** system MUST execute `git fetch && git checkout` to update the repository

### Requirement: Git clone uses shallow clone for performance

The git clone operation MUST use `--depth 1` flag to perform a shallow clone, fetching only the latest commit.

#### Scenario: Clone uses shallow clone
- **WHEN** git clone is executed on empty directory
- **THEN** command MUST include `--depth 1` flag

### Requirement: Git clone uses configured branch

The git clone operation MUST clone the branch specified in the project configuration.

#### Scenario: Clone specific branch
- **WHEN** git clone is executed on empty directory
- **THEN** command MUST include `--branch <branch_name>` flag
