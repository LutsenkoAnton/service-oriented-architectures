use clap::Parser;
/// User service
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Port to run the app on
    #[arg(short, long)]
    pub port: u16,
}
