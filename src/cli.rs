use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(default_value_t = 8000)]
    pub port: u32,

    #[arg(long, short, help = "Path to serve from [default: cwd]")]
    pub path: Option<String>,
}
