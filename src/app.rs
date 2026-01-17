use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "arx", 
    version = "0.1.0", 
    about = "CLI realisation of packet manager ARX"
)]
pub struct ArxApplication {
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}