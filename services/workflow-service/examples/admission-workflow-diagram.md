# Patient Admission Workflow Diagram

```mermaid
flowchart TD
    A[Start: patient-admission] --> B[Validate Patient]
    B -->|success| C[Check Insurance]
    B -.->|failure| BX[Compensate: Revert validation]
    
    C --> D[Parallel: Assign Room]
    D --> D1[Check Room Availability]
    D --> D2[Check Equipment]
    
    D1 --> E{Decision}
    D2 --> E
    
    E -->|approved && rooms > 0| F[Approve Admission]
    E -->|else| G[Reject Admission]
    
    F --> H[Publish: admission_approved]
    G --> I[Publish: admission_rejected]
    
    F --> J[Wait: doctor_confirmation]
    J --> K[Finalize Admission]
    
    K --> L[Parallel: Notifications]
    L --> L1[Notify Patient]
    L --> L2[Notify Staff]
    
    L1 --> Z[End]
    L2 --> Z
```

## Step Details

1. **Validate Patient** - HTTP POST to patient-service
2. **Check Insurance** - HTTP POST to insurance-service  
3. **Parallel Room Assignment**:
   - Check Room Availability (HTTP GET)
   - Check Equipment (HTTP GET)
4. **Decision Switch**:
   - If approved: Approve Admission + Publish success event
   - Else: Reject Admission + Publish rejection event
5. **Event Trigger** - Wait for doctor confirmation
6. **Finalize** - HTTP PUT to finalize admission
7. **Parallel Notifications**:
   - Notify Patient (Kafka)
   - Notify Staff (Kafka)
