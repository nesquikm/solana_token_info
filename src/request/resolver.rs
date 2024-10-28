use anyhow::{anyhow, Result};
use log::info;
use std::str::FromStr;
use url::Url;

use domain::base::name::UncertainName;
use domain::resolv::StubResolver;

#[derive(Debug)]
pub struct DNSRecords {
    pub ipv4count: usize,
    pub ipv6count: usize,
}

pub async fn get_website_dns_records(website: &String) -> Result<DNSRecords> {
    info!("Fetching DNS records for website: {}", website);

    let url = Url::from_str(website).map_err(|e| anyhow!("Error parsing website URL: {}", e))?;

    let host = url
        .host_str()
        .ok_or_else(|| anyhow!("Error fetching host"))?;

    info!("Host: {:?}", host);

    let host =
        UncertainName::<Vec<u8>>::from_str(&host).map_err(|e| anyhow!("Error host: {}", e))?;

    let resolver = StubResolver::new();

    let answer = match host {
        UncertainName::Absolute(ref name) => resolver
            .lookup_host(name)
            .await
            .map_err(|e| anyhow!("Error resolve host: {}", e)),
        UncertainName::Relative(ref name) => resolver
            .search_host(name)
            .await
            .map_err(|e| anyhow!("Error resolve host: {}", e)),
    }?;

    let ipv4count = answer.iter().filter(|addr| addr.is_ipv4()).count();
    let ipv6count = answer.iter().filter(|addr| addr.is_ipv6()).count();

    info!("Answer: ipv4: {} ipv6: {}", ipv4count, ipv6count);

    Ok(DNSRecords { ipv4count, ipv6count })
}
