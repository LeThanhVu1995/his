use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    pub name: String,
    pub vars: Option<serde_json::Value>,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub http: Option<HttpStep>,
    pub task: Option<TaskStep>,
    pub timer: Option<TimerStep>,
    pub kafka_publish: Option<KafkaStep>,
    pub switch: Option<SwitchStep>,
    pub parallel: Option<ParallelStep>,
    pub map: Option<MapStep>,
    pub assign: Option<AssignStep>,
    pub compensate: Option<CompensateStep>,
    pub save_as: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpStep {
    pub method: String,
    pub url: String,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskStep {
    pub name: String,
    pub candidate_roles: Option<Vec<String>>,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimerStep {
    pub seconds: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KafkaStep {
    pub topic: String,
    pub key: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchStep {
    pub condition: String,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchCase {
    pub condition: String,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParallelStep {
    pub branches: Vec<Vec<WorkflowStep>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapStep {
    pub input: String,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignStep {
    pub variables: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompensateStep {
    pub http: Option<HttpStep>,
}
