use std::time::Duration;

use anyhow::Ok;
use model::Subdomain;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest::{blocking::Client, redirect};

mod common_ports;
mod error;
mod model;
mod ports;
mod subdomains;

pub fn main(target: &str) -> Result<(), anyhow::Error> {
  let http_timeout = Duration::from_secs(5);
  let http_client =
    Client::builder().redirect(redirect::Policy::limited(4)).timeout(http_timeout).build()?;

  // use a thread pool
  let pool = rayon::ThreadPoolBuilder::new().num_threads(256).build().unwrap();

  pool.install(|| {
    let scan_result = subdomains::enumerate(&http_client, target)
      .unwrap()
      .into_par_iter()
      .map(ports::scan_ports)
      .collect::<Vec<Subdomain>>();

    for subdomain in scan_result {
      for port in &subdomain.open_ports {
        println!("{} is open", port.port);
      }
      println!("=====================");
    }
  });

  Ok(())
}
