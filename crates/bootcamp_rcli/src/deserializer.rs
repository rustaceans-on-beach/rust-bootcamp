use crate::dto::Record;

pub fn deserialize_csv(file_path: String) -> anyhow::Result<Vec<Record>> {
  Ok(csv::Reader::from_path(file_path)?.deserialize().map(|r| r.unwrap()).collect::<Vec<Record>>())
}
