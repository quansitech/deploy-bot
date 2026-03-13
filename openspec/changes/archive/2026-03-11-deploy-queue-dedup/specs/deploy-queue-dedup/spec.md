## ADDED Requirements

### Requirement: Deploy queue deduplication
The system SHALL prevent duplicate deployment tasks from being queued for the same project and branch.

#### Scenario: First deployment request
- **WHEN** a webhook triggers a deployment for a project/branch with no existing pending or running task
- **THEN** the task is added to the queue and returns a deployment ID

#### Scenario: Duplicate deployment request
- **WHEN** a webhook triggers a deployment for a project/branch that already has a pending or running task
- **THEN** the request is skipped and no new task is added to the queue

#### Scenario: Deployment request after completion
- **WHEN** a webhook triggers a deployment for a project/branch that has a completed (success/failed/cancelled) task
- **THEN** a new task is added to the queue and returns a deployment ID
