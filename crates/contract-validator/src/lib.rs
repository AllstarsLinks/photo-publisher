\
use anyhow::{Context, Result};
use jsonschema::{Draft, Validator};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn load_json(path: impl AsRef<Path>) -> Result<Value> {
    let path = path.as_ref();
    let text = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&text)
        .with_context(|| format!("invalid JSON in {}", path.display()))
}

pub fn compile_schema(schema_path: impl AsRef<Path>) -> Result<Validator> {
    let schema = load_json(schema_path)?;
    let validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .build(&schema)
        .context("failed to compile JSON Schema")?;
    Ok(validator)
}

pub fn validate(schema_path: impl AsRef<Path>, document_path: impl AsRef<Path>) -> Result<()> {
    let validator = compile_schema(schema_path)?;
    let document = load_json(document_path)?;

    let errors: Vec<String> = validator
        .iter_errors(&document)
        .map(|error| error.to_string())
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        anyhow::bail!("schema validation failed:\n{}", errors.join("\n"))
    }
}
