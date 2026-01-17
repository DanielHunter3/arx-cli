mod app;
mod logger;

use app::ArxApplication;
use logger::runtime_log_level;

use clap::Parser;

fn main() {
    let args = ArxApplication::parse();   
    let level = runtime_log_level(args.verbose);
    
    logger::setup_smart_logger(level);
    
    println!("Args: {:?}", args);
    println!("Log level: {:?}", level);
}