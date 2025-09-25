use serde_json::Value as Json;
use crate::dsl::model::WorkflowTemplate;

pub fn parse_yaml(yaml_content: &str) -> anyhow::Result<WorkflowTemplate> {
    let template: WorkflowTemplate = serde_yaml::from_str(yaml_content)?;
    Ok(template)
}

pub fn parse_json(json_content: &str) -> anyhow::Result<WorkflowTemplate> {
    let template: WorkflowTemplate = serde_json::from_str(json_content)?;
    Ok(template)
}

pub fn parse_from_value(value: Json) -> anyhow::Result<WorkflowTemplate> {
    let template: WorkflowTemplate = serde_json::from_value(value)?;
    Ok(template)
}
