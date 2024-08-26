use serde_json::Value;

pub fn serialize_json(dto: Vec<Value>) -> anyhow::Result<String> {
  Ok(serde_json::to_string_pretty(&dto)?)
}

pub fn serialize_yaml(dto: Vec<Value>) -> anyhow::Result<String> {
  Ok(serde_yaml::to_string(&dto)?)
}
