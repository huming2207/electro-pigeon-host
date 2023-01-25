use std::time::{UNIX_EPOCH, SystemTime};

use clap::Parser;
use fern::colors::{ColoredLevelConfig, Color};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    port: String,

    #[arg(short, long, default_value_t = 9600)]
    baud: u32,

    #[arg(long, default_value_t = false)]
    quiet: bool,

    #[arg(long, default_value_t = true)]
    color_log: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let log_colors = ColoredLevelConfig::new()
        .debug(Color::Cyan)
        .info(Color::Green);

    if !args.quiet {
        fern::Dispatch::new()
            .format(move |out, msg, record| {
                out.finish(format_args!(
                    "[{}][{} - {}] {}",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                    log_colors.color(record.level()),
                    record.target(),
                    msg,
                ))
            })
            .chain(std::io::stdout())
            .apply().unwrap();
    }


    
    Ok(())
}
