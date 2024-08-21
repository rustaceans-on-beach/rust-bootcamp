use std::time::Duration;

use anyhow::Ok;
use model::Subdomain;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest::{blocking::Client, redirect};
use spinners::Spinner;

mod common_ports;
mod error;
mod model;
mod ports;
mod subdomains;

pub fn main(target: &str, write_path: &Option<String>) -> Result<(), anyhow::Error> {
  let http_timeout = Duration::from_secs(5);
  let http_client =
    Client::builder().redirect(redirect::Policy::limited(4)).timeout(http_timeout).build()?;

  // use a thread pool
  let pool = rayon::ThreadPoolBuilder::new().num_threads(256).build().unwrap();

  let mut console_buffer: Vec<String> = Vec::new();
  let mut spinner =
    Spinner::new(spinners::Spinners::Dots, "Starting tricoder scan for {} 🔭...".to_string());

  pool.install(|| {
    let scan_result = subdomains::enumerate(&http_client, target)
      .unwrap()
      .into_par_iter()
      .map(ports::scan_ports)
      .collect::<Vec<Subdomain>>();

    for subdomain in scan_result {
      console_buffer.push(format!("subdomain: {}", subdomain.domain));
      for port in &subdomain.open_ports {
        console_buffer.push(format!("{} is open", port.port));
      }
      console_buffer.push("=====================".to_string());
    }
  });

  spinner.stop();
  println!();
  println!("Scan complete! ✅");

  if let Some(path) = write_path {
    std::fs::write(path, console_buffer.join("\n"))?;
  } else {
    for line in console_buffer {
      println!("{}", line);
    }
  }

  Ok(())
}
