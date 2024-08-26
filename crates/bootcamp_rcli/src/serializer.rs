use crate::dto::Record;

pub fn serialize_json(dto: Vec<Record>) -> anyhow::Result<String> {
  Ok(serde_json::to_string_pretty(&dto)?)
}
