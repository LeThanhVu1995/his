use serde_json::Value as Json;
use std::collections::HashMap;

pub async fn evaluate_condition(
    condition: &str,
    context: &Json,
) -> anyhow::Result<bool> {
    tracing::debug!("Evaluating condition: {} with context: {:?}", condition, context);

    // Simple expression evaluator for basic conditions
    // Supports: ==, !=, >, <, >=, <=, &&, ||, !, in, contains
    let result = evaluate_expression(condition, context)?;

    // Convert result to boolean
    match result {
        Json::Bool(b) => Ok(b),
        Json::Number(n) => Ok(n.as_f64().unwrap_or(0.0) != 0.0),
        Json::String(s) => Ok(!s.is_empty() && s != "false" && s != "0"),
        Json::Null => Ok(false),
        _ => Ok(true), // Non-empty arrays/objects are truthy
    }
}

fn evaluate_expression(expr: &str, context: &Json) -> anyhow::Result<Json> {
    let expr = expr.trim();

    // Handle parentheses
    if expr.starts_with('(') && expr.ends_with(')') {
        return evaluate_expression(&expr[1..expr.len()-1], context);
    }

    // Handle logical operators (&&, ||)
    if let Some(pos) = find_logical_operator(expr, "&&") {
        let left = evaluate_expression(&expr[..pos], context)?;
        let right = evaluate_expression(&expr[pos+2..], context)?;
        return Ok(Json::Bool(to_bool(&left) && to_bool(&right)));
    }

    if let Some(pos) = find_logical_operator(expr, "||") {
        let left = evaluate_expression(&expr[..pos], context)?;
        let right = evaluate_expression(&expr[pos+2..], context)?;
        return Ok(Json::Bool(to_bool(&left) || to_bool(&right)));
    }

    // Handle comparison operators
    for op in ["==", "!=", ">=", "<=", ">", "<"] {
        if let Some(pos) = expr.find(op) {
            let left = evaluate_expression(&expr[..pos], context)?;
            let right = evaluate_expression(&expr[pos+op.len()..], context)?;
            return Ok(Json::Bool(compare_values(&left, &right, op)));
        }
    }

    // Handle 'in' operator
    if let Some(pos) = expr.find(" in ") {
        let left = evaluate_expression(&expr[..pos], context)?;
        let right = evaluate_expression(&expr[pos+4..], context)?;
        return Ok(Json::Bool(contains_value(&right, &left)));
    }

    // Handle 'contains' operator
    if let Some(pos) = expr.find(" contains ") {
        let left = evaluate_expression(&expr[..pos], context)?;
        let right = evaluate_expression(&expr[pos+10..], context)?;
        return Ok(Json::Bool(contains_value(&left, &right)));
    }

    // Handle negation
    if expr.starts_with('!') {
        let inner = evaluate_expression(&expr[1..], context)?;
        return Ok(Json::Bool(!to_bool(&inner)));
    }

    // Handle variable references
    if expr.starts_with("ctx.") || expr.starts_with("vars.") {
        return Ok(resolve_path(context, expr));
    }

    // Handle literals
    parse_literal(expr)
}

fn find_logical_operator(expr: &str, op: &str) -> Option<usize> {
    let mut depth = 0;
    let mut i = 0;
    while i < expr.len() - op.len() + 1 {
        let c = expr.chars().nth(i).unwrap();
        if c == '(' { depth += 1; }
        else if c == ')' { depth -= 1; }
        else if depth == 0 && expr[i..].starts_with(op) {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn compare_values(left: &Json, right: &Json, op: &str) -> bool {
    match (left, right) {
        (Json::Number(a), Json::Number(b)) => {
            let a_val = a.as_f64().unwrap_or(0.0);
            let b_val = b.as_f64().unwrap_or(0.0);
            match op {
                "==" => a_val == b_val,
                "!=" => a_val != b_val,
                ">" => a_val > b_val,
                "<" => a_val < b_val,
                ">=" => a_val >= b_val,
                "<=" => a_val <= b_val,
                _ => false,
            }
        },
        (Json::String(a), Json::String(b)) => {
            match op {
                "==" => a == b,
                "!=" => a != b,
                _ => false,
            }
        },
        (Json::Bool(a), Json::Bool(b)) => {
            match op {
                "==" => a == b,
                "!=" => a != b,
                _ => false,
            }
        },
        _ => false,
    }
}

fn contains_value(container: &Json, value: &Json) -> bool {
    match container {
        Json::Array(arr) => arr.contains(value),
        Json::String(s) => {
            if let Json::String(v) = value {
                s.contains(v)
            } else {
                false
            }
        },
        _ => false,
    }
}

fn to_bool(value: &Json) -> bool {
    match value {
        Json::Bool(b) => *b,
        Json::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
        Json::String(s) => !s.is_empty() && s != "false" && s != "0",
        Json::Null => false,
        _ => true,
    }
}

fn resolve_path(context: &Json, path: &str) -> Json {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = context;

    for part in parts {
        match current {
            Json::Object(map) => {
                if let Some(value) = map.get(part) {
                    current = value;
                } else {
                    return Json::Null;
                }
            },
            _ => return Json::Null,
        }
    }

    current.clone()
}

fn parse_literal(expr: &str) -> anyhow::Result<Json> {
    let expr = expr.trim();

    // Boolean literals
    if expr == "true" { return Ok(Json::Bool(true)); }
    if expr == "false" { return Ok(Json::Bool(false)); }

    // Null literal
    if expr == "null" { return Ok(Json::Null); }

    // String literals
    if expr.starts_with('"') && expr.ends_with('"') {
        return Ok(Json::String(expr[1..expr.len()-1].to_string()));
    }

    // Number literals
    if let Ok(num) = expr.parse::<f64>() {
        return Ok(Json::Number(serde_json::Number::from_f64(num).unwrap()));
    }

    // Integer literals
    if let Ok(num) = expr.parse::<i64>() {
        return Ok(Json::Number(serde_json::Number::from(num)));
    }

    Err(anyhow::anyhow!("Unable to parse literal: {}", expr))
}
