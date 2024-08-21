use std::{collections::HashSet, time::Duration};

use anyhow::{Error, Ok};
use reqwest::blocking::Client;
use trust_dns_resolver::{
  config::{ResolverConfig, ResolverOpts},
  Resolver,
};

use super::model::{CrtShEntry, Subdomain};

pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
  let entries = http_client
    .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
    .send()?
    .json::<Vec<CrtShEntry>>()?;

  let subdomains = entries
    .into_iter()
    .flat_map(|entry| {
      entry
        .name_value
        .split("\n")
        .map(|subdomain| subdomain.trim().to_string())
        .collect::<Vec<String>>()
    })
    .filter(|subdomain| subdomain != target)
    .filter(|subdomain| !subdomain.contains("*"))
    .collect::<HashSet<String>>();

  let subdomains = subdomains
    .into_iter()
    .map(|subdomain| Subdomain { domain: subdomain, open_ports: vec![] })
    .filter(resolve)
    .collect::<Vec<Subdomain>>();

  Ok(subdomains)
}

pub fn resolve(domain: &Subdomain) -> bool {
  let mut opts = ResolverOpts::default();
  opts.timeout = Duration::from_secs(4);

  let dns_resolver = Resolver::new(ResolverConfig::default(), opts)
    .expect("subdomain resolver: building DNS client");

  dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
