use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Stock ticker symbol
    #[arg(short, long, required = true)]
    pub ticker: String,
}

pub fn get_args() -> Args {
    Args::parse()
}
