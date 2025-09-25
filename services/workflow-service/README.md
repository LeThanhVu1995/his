# Workflow Service

A comprehensive workflow engine for the HIS (Hospital Information System) that supports complex business processes with error handling, compensation, and event-driven execution.

## Features

### Core Workflow Engine
- **Template Management**: Create, update, and version workflow templates
- **Instance Execution**: Start and manage workflow instances
- **Task Management**: Human tasks with assignment and completion tracking
- **State Management**: Persistent workflow state with checkpoints

### Step Types
- **HTTP Steps**: Call external APIs with error handling and compensation
- **Kafka Steps**: Publish events to message queues
- **Task Steps**: Human tasks with role-based assignment
- **Timer Steps**: Delays and timeouts
- **Switch Steps**: Conditional branching with CEL expressions
- **Parallel Steps**: Execute multiple branches concurrently
- **Sub-process Steps**: Call other workflows as sub-processes
- **Event Trigger Steps**: Wait for external events

### Advanced Features
- **Saga Pattern**: Automatic compensation for failed operations
- **Error Handling**: Comprehensive error recovery and rollback
- **Event-Driven**: React to external events and resume workflows
- **Parallel Execution**: Concurrent branch execution with result merging
- **Rule Engine**: CEL-based condition evaluation
- **Observability**: Health checks, metrics, and logging

## API Endpoints

### Templates
- `POST /api/v1/wf/templates:upsert` - Create or update workflow template
- `GET /api/v1/wf/templates/{code}` - Get workflow template
- `GET /api/v1/wf/templates` - List all templates

### Instances
- `POST /api/v1/wf/instances:start/{code}` - Start workflow instance
- `GET /api/v1/wf/instances/{id}` - Get workflow instance

### Tasks
- `POST /api/v1/wf/tasks/{id}:claim` - Claim a task
- `POST /api/v1/wf/tasks/{id}:complete` - Complete a task
- `GET /api/v1/wf/tasks/{id}` - Get task details

### Events
- `POST /api/v1/wf/events` - Handle workflow events

### Observability
- `GET /api/v1/wf/observability/health` - Health check
- `GET /api/v1/wf/observability/metrics` - Metrics

## Workflow Template Format

```json
{
  "code": "workflow-name",
  "name": "Human Readable Name",
  "version": 1,
  "spec": {
    "steps": [
      {
        "id": "step-id",
        "name": "Step Name",
        "http": {
          "method": "POST",
          "url": "http://service/api/endpoint",
          "body": {"key": "{{ctx.variable}}"},
          "save_as": "result_variable"
        },
        "compensate": {
          "http": {
            "method": "DELETE",
            "url": "http://service/api/cleanup"
          }
        }
      }
    ]
  }
}
```

## Step Types

### HTTP Step
```json
{
  "id": "api-call",
  "http": {
    "method": "POST",
    "url": "http://service/api/endpoint",
    "body": {"data": "{{ctx.input}}"},
    "save_as": "response"
  }
}
```

### Kafka Step
```json
{
  "id": "publish-event",
  "kafka-publish": {
    "topic": "events",
    "key": "{{ctx.id}}",
    "payload": {"event": "created", "data": "{{ctx.data}}"}
  }
}
```

### Task Step
```json
{
  "id": "human-task",
  "task": {
    "name": "Review Document",
    "candidate_roles": ["doctor", "nurse"],
    "payload": {"document_id": "{{ctx.doc_id}}"}
  }
}
```

### Timer Step
```json
{
  "id": "wait",
  "timer": {
    "seconds": 300
  }
}
```

### Switch Step
```json
{
  "id": "conditional",
  "switch": {
    "condition": "ctx.status == 'approved'",
    "cases": [
      {
        "condition": "true",
        "steps": [{"id": "approve", "http": {...}}]
      },
      {
        "condition": "false", 
        "steps": [{"id": "reject", "http": {...}}]
      }
    ]
  }
}
```

### Parallel Step
```json
{
  "id": "parallel-work",
  "parallel": {
    "branches": [
      {"steps": [{"id": "branch1", "http": {...}}]},
      {"steps": [{"id": "branch2", "http": {...}}]}
    ],
    "merge_key": "results"
  }
}
```

### Sub-process Step
```json
{
  "id": "sub-workflow",
  "subprocess": {
    "template": "sub-workflow-name",
    "input": {"data": "{{ctx.data}}"},
    "save_as": "sub_result"
  }
}
```

### Event Trigger Step
```json
{
  "id": "wait-for-event",
  "event-trigger": {
    "event": "external_event",
    "payload": {"id": "{{ctx.id}}"},
    "wait_for": {
      "timeout": 300,
      "save_as": "event_response"
    }
  }
}
```

## CEL Expression Examples

The rule engine supports CEL (Common Expression Language) expressions:

```javascript
// Basic comparisons
ctx.status == "approved"
ctx.amount > 1000
ctx.priority in ["high", "urgent"]

// Logical operators
ctx.approved == true && ctx.amount < 5000
ctx.status == "pending" || ctx.retry_count < 3

// String operations
ctx.name contains "John"
ctx.email startsWith "admin@"

// Array operations
ctx.roles contains "doctor"
ctx.permissions in ["read", "write"]

// Complex conditions
(ctx.status == "approved" && ctx.amount > 1000) || ctx.override == true
```

## Configuration

Environment variables:

```bash
SERVICE_NAME=workflow-service
SERVICE_PORT=8080
DATABASE_URL=postgresql://user:pass@localhost/workflow
KAFKA_BROKERS=localhost:9092
KAFKA_GROUP_ID=workflow-service
WORKFLOW_MAX_PARALLEL_BRANCHES=10
WORKFLOW_DEFAULT_TIMEOUT_SECS=300
WORKFLOW_RETRY_ATTEMPTS=3
```

## Database Schema

The service uses the following tables:
- `wf_templates` - Workflow template definitions
- `wf_instances` - Active workflow instances
- `wf_tasks` - Human tasks
- `wf_saga_log` - Saga compensation logs
- `outbox_events` - Event publishing

## Example Workflow

See `examples/patient-admission-workflow.json` for a comprehensive example that demonstrates:
- Patient validation
- Insurance checking
- Parallel room and equipment availability checks
- Conditional admission approval
- Event-driven doctor confirmation
- Parallel notifications

## Security

The service integrates with the HIS IAM system and requires the following permissions:
- `his.workflow.template.*` - Template management
- `his.workflow.instance.*` - Instance management  
- `his.workflow.task.*` - Task management
- `his.workflow.event.*` - Event handling
- `his.workflow.observability.*` - Monitoring

## Monitoring

The service provides:
- Health checks with database and IAM status
- Metrics for workflow execution
- Comprehensive logging with tracing
- Saga compensation tracking
- Error monitoring and alerting
