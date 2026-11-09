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
    /// Review only the commits HEAD adds beyond BASE (pull-request mode).
    #[arg(long, value_name = "BASE")]
    pr_review: Option<String>,
}

fn main() -> process::ExitCode {
    let args = Args::parse();

    let result = match &args.pr_review {
        Some(base) => {
            if args.write_status {
                eprintln!("vibecheck: --write-status does not apply to a pull-request review");
                return process::ExitCode::from(2);
            }
            vibecheck::run_review(&args.path, base)
        }
        None => vibecheck::run(
            &args.path,
            vibecheck::Options {
                level_override: args.level,
            },
        ),
    };

    match result {
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
            process::ExitCode::from(2)
        }
    }
}
