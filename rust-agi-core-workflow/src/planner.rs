use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use jsonschema::JSONSchema;
use crate::llm::LLMClient;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanStep {
    pub id: String,
    pub action: String,
    pub params: Value,
    pub confidence: f64,
}

pub type Plan = Vec<PlanStep>;

const PLAN_SCHEMA: &str = r#"
{
  "type":"array",
  "minItems":1,
  "maxItems":12,
  "items":{
    "type":"object",
    "required":["id","action","params","confidence"],
    "properties":{
      "id":{"type":"string"},
      "action":{"type":"string"},
      "params":{"type":"object"},
      "confidence":{"type":"number","minimum":0,"maximum":1}
    },
    "additionalProperties": false
  }
}
"#;

pub async fn create_plan(llm: &LLMClient, goal: &str, context: &str, max_steps: usize) -> Result<Plan> {
    let system = "You are a precise planner. Output STRICT JSON only, matching the schema provided. No explanation.";
    let user = format!(
        "Goal: {}\nContext: {}\nReturn a JSON array of up to {} steps. Each step: id, action, params (object), confidence (0.0-1.0).",
        goal, context, max_steps
    );
    let raw = llm.send_chat(system, &user).await?;
    // try to extract JSON array substring
    let json_text = {
        if let Some(start) = raw.find('[') {
            if let Some(end) = raw.rfind(']') {
                raw[start..=end].to_string()
            } else {
                raw.clone()
            }
        } else {
            raw.clone()
        }
    };

    let parsed: Value = serde_json::from_str(&json_text)
        .map_err(|e| anyhow!("Failed to parse JSON from LLM: {}\nRaw: {}", e, raw))?;

    let schema: Value = serde_json::from_str(PLAN_SCHEMA)?;
    let compiled = JSONSchema::compile(&schema)?;
    if let Err(errors) = compiled.validate(&parsed).err() {
        // collect messages
        let msgs: Vec<String> = errors.map(|e| e.to_string()).collect();
        return Err(anyhow!("Plan JSON failed schema validation: {:?}", msgs));
    }

    // parse into Plan
    let plan: Plan = serde_json::from_value(parsed)?;
    Ok(plan.into_iter().take(max_steps).collect())
}
