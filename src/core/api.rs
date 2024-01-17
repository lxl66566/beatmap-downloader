use clap::{Args, Parser, Subcommand};
use serde::Serialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    url: Option<String>,
    #[arg(short, long)]
    debug: Option<bool>,
    #[command(subcommand)]
    pub download: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Download(API),
}

#[derive(Args, Debug, Serialize, Clone)]
pub struct API {
    /// Sayobot needs a post value: "cmd": "beatmaplist"
    #[clap(skip)]
    cmd: String,
    #[arg(long, help = t!("cli.limit"))]
    pub limit: u32,
    #[arg(long, help = t!("cli.offset"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[arg(long, help = t!("cli.type"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<u32>,
}

impl API {
    pub fn with_cmd(mut self, cmd: String) -> Self {
        self.cmd = cmd;
        self
    }
    pub fn set_cmd(&mut self, cmd: String) {
        self.cmd = cmd;
    }
}
