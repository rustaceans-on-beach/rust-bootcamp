use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
  pub username: String,
  pub email: String,
  pub full_name: String,
  pub address: String,
  #[serde(rename = "phone_number")]
  pub phone: Option<String>,
}
