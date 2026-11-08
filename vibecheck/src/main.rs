use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Format {
    Text,
    Json,
}

#[derive(Debug, Parser)]
#[command(
    name = "vibecheck",
    about = "Validate a project against the Vibecoding Convention"
)]
struct Args {
    /// Repository root.
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Override the declared level.
    #[arg(long)]
    level: Option<u8>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    format: Format,

    /// Write audit/status.json after the run.
    #[arg(long)]
    write_status: bool,

    /// Treat warnings as failures for the exit code.
    #[arg(long)]
    strict: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if !args.path.is_dir() {
        eprintln!(
            "vibecheck: path is not a directory: {}",
            args.path.display()
        );
        return ExitCode::from(2);
    }

    match vibecheck::run(&args.path, args.level) {
        Ok(report) => {
            if args.write_status {
                if let Err(err) = vibecheck::status::write_status(&args.path, &report) {
                    eprintln!("vibecheck: cannot write status: {err}");
                    return ExitCode::from(2);
                }
            }

            let output = match args.format {
                Format::Text => report.to_text(),
                Format::Json => report.to_json(),
            };
            println!("{output}");

            if report.has_fail() || (args.strict && report.has_warn()) {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(err) => {
            eprintln!("vibecheck: {err}");
            ExitCode::from(2)
        }
    }
}
