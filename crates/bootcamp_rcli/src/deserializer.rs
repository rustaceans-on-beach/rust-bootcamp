use serde_json::Value;

pub fn deserialize_csv(file_path: String) -> anyhow::Result<Vec<Value>> {
  let mut csv_content = csv::Reader::from_path(file_path)?;

  let headers = csv_content.headers()?.clone();

  let records = csv_content
    .records()
    .map(|r| {
      headers
        .iter()
        .zip(r.unwrap().iter().map(|r| if r.is_empty() { Value::Null } else { r.into() }))
        .collect()
    })
    .collect::<Vec<Value>>();

  Ok(records)
}
