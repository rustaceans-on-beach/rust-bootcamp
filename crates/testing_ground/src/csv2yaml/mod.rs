#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Record {
  a: String,
  d: String,
  #[serde(rename = "zh-CN")]
  zh_cn: String,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut rdr = csv::Reader::from_path("./crates/testing_ground/src/csv2yaml/test.csv")?;

  let records = rdr.deserialize().collect::<Result<Vec<Record>, _>>()?;

  let yaml = serde_yaml::to_string(&records)?;

  std::fs::write("./crates/testing_ground/src/csv2yaml/temp-test.yaml", yaml)?;

  Ok(())
}
