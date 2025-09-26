# Advanced Workflow Features Guide

## Priority 1 Features (Production Ready)

### 1. Per-Step Timeouts
```json
{
  "id": "api-call",
  "http": {
    "method": "POST",
    "url": "http://service/api/endpoint",
    "timeout_secs": 10
  }
}
```

### 2. Retry Policies with Exponential Backoff
```json
{
  "id": "reliable-call",
  "http": {
    "method": "POST",
    "url": "http://service/api/endpoint"
  },
  "retry": {
    "max_attempts": 3,
    "backoff": {
      "type": "exponential",
      "initial_secs": 1,
      "max_secs": 30
    }
  }
}
```

### 3. Error Boundaries (Try/Catch/Finally)
```json
{
  "id": "robust-operation",
  "try": {
    "steps": [
      {
        "id": "risky-call",
        "http": {
          "method": "POST",
          "url": "http://unreliable-service/api/endpoint"
        }
      }
    ]
  },
  "catch": [
    {
      "on": "http_error",
      "steps": [
        {
          "id": "fallback",
          "http": {
            "method": "POST",
            "url": "http://backup-service/api/endpoint"
          }
        }
      ]
    },
    {
      "on": "timeout",
      "steps": [
        {
          "id": "notify-timeout",
          "kafka-publish": {
            "topic": "alerts",
            "payload": {"type": "timeout", "service": "unreliable-service"}
          }
        }
      ]
    }
  ],
  "finally": [
    {
      "id": "cleanup",
      "kafka-publish": {
        "topic": "audit",
        "payload": {"event": "operation_completed"}
      }
    }
  ]
}
```

## Priority 2 Features (Versioning)

### 4. Workflow Versioning
```json
{
  "code": "patient-admission",
  "name": "Patient Admission Workflow",
  "version": 2,
  "spec": {
    "steps": [...]
  }
}
```

**API Usage:**
```bash
# Create new version (auto-increments)
curl -X POST /api/v1/wf/templates:upsert -d '{"code": "admission", "version": 1, ...}'
curl -X POST /api/v1/wf/templates:upsert -d '{"code": "admission", "version": 1, ...}'  # Creates v2

# Start instance with specific version
curl -X POST /api/v1/wf/instances:start/admission?version=1 -d '{"patient_id": "123"}'

# Start instance with latest version
curl -X POST /api/v1/wf/instances:start/admission -d '{"patient_id": "123"}'
```

## Priority 3 Features (Advanced Control Flow)

### 5. Dynamic Parallel Execution (parallel_for)
```json
{
  "id": "process-multiple-items",
  "parallel_for": {
    "items": "{{ctx.item_list}}",
    "as": "item",
    "merge_key": "results",
    "steps": [
      {
        "id": "process-item",
        "http": {
          "method": "POST",
          "url": "http://service/api/process",
          "body": {
            "item_id": "{{vars.item.id}}",
            "data": "{{vars.item.data}}"
          },
          "save_as": "item_result"
        }
      }
    ]
  }
}
```

**Features:**
- Processes variable number of items in parallel
- Each item gets its own context (`vars.item`)
- Results merged back as `ctx.results_0`, `ctx.results_1`, etc.
- Automatic error handling per item

### 6. While Loop
```json
{
  "id": "retry-until-success",
  "loop": {
    "while": "ctx.success == false && ctx.retry_count < 5",
    "max_iter": 10,
    "steps": [
      {
        "id": "attempt-operation",
        "http": {
          "method": "POST",
          "url": "http://service/api/attempt"
        }
      },
      {
        "id": "check-result",
        "http": {
          "method": "GET",
          "url": "http://service/api/status",
          "save_as": "operation_status"
        }
      }
    ]
  }
}
```

**Features:**
- Continues while condition is true
- Maximum iteration limit to prevent infinite loops
- State preserved between iterations
- Cursor tracks current iteration

### 7. For-Each Loop
```json
{
  "id": "process-documents",
  "foreach": {
    "items": "{{ctx.document_list}}",
    "as": "document",
    "steps": [
      {
        "id": "process-document",
        "http": {
          "method": "POST",
          "url": "http://service/api/process",
          "body": {
            "document_id": "{{vars.document.id}}",
            "type": "{{vars.document.type}}"
          }
        }
      }
    ]
  }
}
```

**Features:**
- Processes each item sequentially
- Current item available as `vars.document`
- State preserved between iterations
- Cursor tracks current index

## Complete Example: Advanced Patient Processing

```json
{
  "code": "advanced-patient-processing",
  "name": "Advanced Patient Processing",
  "version": 1,
  "spec": {
    "steps": [
      {
        "id": "get-patient-batch",
        "name": "Get Patient Batch",
        "http": {
          "method": "GET",
          "url": "http://patient-service/api/v1/batch",
          "save_as": "patient_batch",
          "timeout_secs": 30
        },
        "retry": {
          "max_attempts": 3,
          "backoff": {
            "type": "exponential",
            "initial_secs": 2,
            "max_secs": 60
          }
        }
      },
      {
        "id": "process-patients-parallel",
        "name": "Process Patients in Parallel",
        "parallel_for": {
          "items": "{{ctx.patient_batch.patients}}",
          "as": "patient",
          "merge_key": "patient_results",
          "steps": [
            {
              "id": "validate-patient",
              "try": {
                "steps": [
                  {
                    "http": {
                      "method": "POST",
                      "url": "http://patient-service/api/v1/validate",
                      "body": {
                        "patient_id": "{{vars.patient.id}}"
                      },
                      "save_as": "validation_result",
                      "timeout_secs": 10
                    }
                  }
                ]
              },
              "catch": [
                {
                  "on": "http_error",
                  "steps": [
                    {
                      "kafka-publish": {
                        "topic": "patient-errors",
                        "payload": {
                          "patient_id": "{{vars.patient.id}}",
                          "error": "validation_failed"
                        }
                      }
                    }
                  ]
                }
              ]
            },
            {
              "id": "retry-insurance-check",
              "loop": {
                "while": "ctx.insurance_checked == false && ctx.retry_count < 3",
                "max_iter": 5,
                "steps": [
                  {
                    "http": {
                      "method": "POST",
                      "url": "http://insurance-service/api/v1/check",
                      "body": {
                        "patient_id": "{{vars.patient.id}}"
                      },
                      "save_as": "insurance_result",
                      "timeout_secs": 15
                    }
                  }
                ]
              }
            }
          ]
        }
      },
      {
        "id": "generate-report",
        "name": "Generate Final Report",
        "http": {
          "method": "POST",
          "url": "http://reporting-service/api/v1/report",
          "body": {
            "results": "{{ctx.patient_results}}",
            "total_processed": "{{ctx.patient_results.length}}"
          },
          "save_as": "final_report"
        }
      }
    ]
  }
}
```

## Usage Examples

### Start Advanced Workflow
```bash
curl -X POST http://localhost:8080/api/v1/wf/instances:start/advanced-patient-processing \
  -H "Content-Type: application/json" \
  -d '{
    "batch_size": 100,
    "include_medical_history": true
  }'
```

### Monitor Progress
```bash
curl -X GET http://localhost:8080/api/v1/wf/instances/{instance_id}
```

### Handle Events
```bash
curl -X POST http://localhost:8080/api/v1/wf/events \
  -H "Content-Type: application/json" \
  -d '{
    "event": "patient_validation_complete",
    "payload": {"patient_id": "123", "status": "valid"},
    "correlation_id": "workflow-instance-id"
  }'
```

## Best Practices

1. **Use timeouts** for all external calls
2. **Implement retry policies** for transient failures
3. **Use try/catch/finally** for robust error handling
4. **Version your workflows** for safe deployments
5. **Use parallel_for** for processing collections
6. **Use loops** for retry logic and polling
7. **Monitor workflow execution** with health checks
8. **Log important events** to Kafka for audit trails

## Performance Considerations

- **Parallel execution** scales with available CPU cores
- **Dynamic parallel** is limited by memory and connection pools
- **Loops** should have reasonable max_iter limits
- **Timeouts** prevent resource exhaustion
- **Retry policies** should have exponential backoff
- **Versioning** allows safe rollbacks
