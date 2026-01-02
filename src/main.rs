use anyhow::Result;
use clap::Parser;
use reqwest::Url;
use reqwest::blocking::Client;

mod handler;
mod models;

use handler::*;
use models::*;

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 构建 URL 和参数
    let mut url_obj = Url::parse(&cli.url)?;

    {
        let mut query_pairs = url_obj.query_pairs_mut();

        if !cli.action.is_empty() {
            query_pairs.append_pair("ac", &cli.action);
        }
        if let Some(ref t) = cli.t {
            query_pairs.append_pair("t", t);
        }
        query_pairs.append_pair("pg", &cli.pg);
        if let Some(ref wd) = cli.wd {
            query_pairs.append_pair("wd", wd);
        }
        if let Some(ref ids) = cli.ids {
            query_pairs.append_pair("ids", ids);
        }
    }

    let client = Client::new();
    let response_text = client.get(url_obj).send()?.text()?;

    // 使用新的响应处理器
    let wrapper = Handler::parse_response(&response_text)?;
    let output_data = Handler::extract_data(&wrapper, &cli.action);

    if cli.json {
        // 输出 JSON 格式
        let json_output = Handler::format_output(output_data, OutputFormat::Json)?;
        println!("{}", json_output);
    } else {
        // 输出制表符格式
        let tabular_output = Handler::format_output(output_data, OutputFormat::Tabular)?;
        print!("{}", tabular_output);
    }

    Ok(())
}
