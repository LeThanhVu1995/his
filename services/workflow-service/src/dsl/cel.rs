use serde_json::Value as Json;

pub fn evaluate_condition(condition: &str, context: &Json) -> anyhow::Result<bool> {
    // TODO: Implement CEL condition evaluation
    // For now, return true as placeholder
    tracing::debug!("Evaluating CEL condition: {}", condition);
    Ok(true)
}
