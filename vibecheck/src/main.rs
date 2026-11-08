use std::path::PathBuf;
use std::process;

use clap::Parser;

#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq)]
enum Format {
    Text,
    Json,
}

#[derive(Parser)]
#[command(version, about = "Validator for the Vibecoding Convention")]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,
    #[arg(long)]
    level: Option<u8>,
    #[arg(long, value_enum, default_value_t = Format::Text)]
    format: Format,
    #[arg(long)]
    write_status: bool,
    #[arg(long)]
    strict: bool,
}

fn main() -> process::ExitCode {
    let args = Args::parse();
    let opts = vibecheck::Options {
        level_override: args.level,
    };

    match vibecheck::run(&args.path, opts) {
        Ok(report) => {
            match args.format {
                Format::Text => println!("{}", report.to_text()),
                Format::Json => println!("{}", report.to_json()),
            }

            if args.write_status {
                if let Err(err) = vibecheck::status::write_status(&args.path, &report) {
                    eprintln!("failed to write status: {err}");
                    return process::ExitCode::FAILURE;
                }
            }

            if report.has_fail() || (args.strict && report.has_warn()) {
                process::ExitCode::FAILURE
            } else {
                process::ExitCode::SUCCESS
            }
        }
        Err(err) => {
            eprintln!("vibecheck: {err}");
            process::ExitCode::FAILURE
        }
    }
}
