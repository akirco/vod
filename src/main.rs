use anyhow::{Context, Result};
use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use std::time::Duration;

mod handler;
mod models;

use handler::*;
use models::*;

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36";
const REQUEST_TIMEOUT_SECS: u64 = 30;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Apple CMS API URL
    #[arg(short, long, env = "VOD_API_URL")]
    url: String,

    /// action [class,detail,videolist]
    #[arg(short = 'a', long, default_value = "")]
    action: String,

    /// type ID
    #[arg(short, long)]
    t: Option<String>,

    /// page number
    #[arg(short, long, default_value = "1")]
    pg: String,

    /// search keyword
    #[arg(short, long)]
    wd: Option<String>,

    /// IDs for detail  [id1,id2,...]
    #[arg(short = 'i', long)]
    ids: Option<String>,

    /// Output in JSON format , default is false
    #[arg(short, long, default_value = "false")]
    json: bool,
}

fn build_url(
    base_url: &str,
    action: &str,
    t: Option<&str>,
    pg: &str,
    wd: Option<&str>,
    ids: Option<&str>,
) -> Result<String> {
    let mut url = reqwest::Url::parse(base_url).context("Failed to parse base URL")?;

    let mut query = vec![];
    if !action.is_empty() {
        query.push(("ac", action));
    }
    if let Some(t) = t {
        query.push(("t", t));
    }
    query.push(("pg", pg));
    if let Some(wd) = wd {
        query.push(("wd", wd));
    }
    if let Some(ids) = ids {
        query.push(("ids", ids));
    }

    url.query_pairs_mut().extend_pairs(&query);

    Ok(url.to_string())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let url = build_url(
        &cli.url,
        &cli.action,
        cli.t.as_deref(),
        &cli.pg,
        cli.wd.as_deref(),
        cli.ids.as_deref(),
    )?;

    let client = Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()
        .context("Failed to create HTTP client")?;

    let response_text = client
        .get(&url)
        .header(USER_AGENT, DEFAULT_USER_AGENT)
        .send()
        .with_context(|| format!("Failed to send request to {}", url))?
        .text()
        .context("Failed to read response body")?;

    let wrapper =
        Handler::parse_response(&response_text).context("Failed to parse API response")?;

    let output_data = Handler::extract_data(&wrapper, &cli.action);

    if cli.json {
        let json_output = Handler::format_output(output_data, OutputFormat::Json)?;
        println!("{}", json_output);
    } else {
        let tabular_output = Handler::format_output(output_data, OutputFormat::Tabular)?;
        print!("{}", tabular_output);
    }

    Ok(())
}
