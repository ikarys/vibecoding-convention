use std::io::Read;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Set in the environment of every command vibecheck runs. A nested
/// vibecheck that finds it refuses to run commands: a declared `test`
/// command that runs the project's suite, whose tests call vibecheck with
/// `--run`, would otherwise recurse without bound.
pub const NESTED_MARKER: &str = "VIBECHECK_RUNNING";

/// Wall-clock limit for one declared command. Test suites and coverage runs
/// are slow; anything beyond this is treated as hung and killed.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(900);

/// Variables that a parent build tool leaks into our environment and that
/// would redirect or instrument a nested build (cargo, cargo-llvm-cov).
const INHERITED_BUILD_ENV: &[&str] = &[
    "CARGO_TARGET_DIR",
    "CARGO_BUILD_TARGET_DIR",
    "CARGO_INCREMENTAL",
    "CARGO_MANIFEST_DIR",
    "RUSTFLAGS",
    "RUSTDOCFLAGS",
    "CARGO_ENCODED_RUSTFLAGS",
    "CARGO_BUILD_RUSTFLAGS",
    "LLVM_PROFILE_FILE",
    "CARGO_LLVM_COV",
    "CARGO_LLVM_COV_TARGET_DIR",
    "CARGO_LLVM_COV_SHOW_ENV",
    "RUST_TEST_THREADS",
];

#[derive(Debug)]
pub struct Outcome {
    pub success: bool,
    pub output: String,
}

pub fn run_command(root: &Path, command: &str) -> anyhow::Result<Outcome> {
    let nested = std::env::var_os(NESTED_MARKER).is_some();
    run_command_with(root, command, DEFAULT_TIMEOUT, nested)
}

fn run_command_with(
    root: &Path,
    command: &str,
    timeout: Duration,
    nested: bool,
) -> anyhow::Result<Outcome> {
    if nested {
        anyhow::bail!(
            "refusing to run `{command}`: a parent vibecheck is already running commands ({NESTED_MARKER} is set)"
        );
    }
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(command)
        .current_dir(root)
        .env(NESTED_MARKER, "1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for name in INHERITED_BUILD_ENV {
        cmd.env_remove(name);
    }
    // Own process group, so a timeout kills the shell and everything it
    // started; killing the shell alone leaves its children running.
    cmd.process_group(0);
    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().expect("stdout is piped");
    let stderr = child.stderr.take().expect("stderr is piped");
    let out_reader = std::thread::spawn(move || read_all(stdout));
    let err_reader = std::thread::spawn(move || read_all(stderr));

    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if started.elapsed() >= timeout {
            let _ = Command::new("sh")
                .arg("-c")
                .arg(format!("kill -9 -{}", child.id()))
                .status();
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(50));
    };

    let mut combined = out_reader.join().unwrap_or_default();
    combined.push_str(&err_reader.join().unwrap_or_default());
    match status {
        Some(status) => Ok(Outcome {
            success: status.success(),
            output: combined,
        }),
        None => {
            combined.push_str(&format!(
                "\nvibecheck: command killed after {} s (timeout)\n",
                timeout.as_secs()
            ));
            Ok(Outcome {
                success: false,
                output: combined,
            })
        }
    }
}

fn read_all(mut source: impl Read) -> String {
    let mut bytes = Vec::new();
    let _ = source.read_to_end(&mut bytes);
    String::from_utf8_lossy(&bytes).into_owned()
}

pub fn last_percent(text: &str) -> Option<f64> {
    let re = regex::Regex::new(r"(\d+(?:\.\d+)?)\s*%").ok()?;
    re.captures_iter(text)
        .last()
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_percent_reads_single_value() {
        assert_eq!(last_percent("lines: 83.21%"), Some(83.21));
    }

    #[test]
    fn last_percent_none() {
        assert_eq!(last_percent("no figure"), None);
    }

    #[test]
    fn last_percent_reads_last_value() {
        assert_eq!(last_percent("first 10% then 42.5%"), Some(42.5));
    }

    #[test]
    fn command_output_and_status_are_captured() {
        let dir = std::env::temp_dir();
        let outcome = run_command_with(
            &dir,
            "echo out; echo err >&2; exit 3",
            DEFAULT_TIMEOUT,
            false,
        )
        .unwrap();
        assert!(!outcome.success);
        assert!(outcome.output.contains("out"));
        assert!(outcome.output.contains("err"));
    }

    #[test]
    fn nested_run_is_refused() {
        let dir = std::env::temp_dir();
        let err = run_command_with(&dir, "true", DEFAULT_TIMEOUT, true).unwrap_err();
        assert!(err.to_string().contains(NESTED_MARKER));
    }

    #[test]
    fn child_sees_marker_and_no_build_env() {
        let dir = std::env::temp_dir();
        let outcome = run_command_with(
            &dir,
            "test \"$VIBECHECK_RUNNING\" = 1 && test -z \"$CARGO_TARGET_DIR\" && test -z \"$RUSTFLAGS\"",
            DEFAULT_TIMEOUT,
            false,
        )
        .unwrap();
        assert!(outcome.success, "{}", outcome.output);
    }

    #[test]
    fn hung_command_is_killed_after_timeout() {
        let dir = std::env::temp_dir();
        let started = Instant::now();
        let outcome =
            run_command_with(&dir, "sleep 30", Duration::from_millis(300), false).unwrap();
        assert!(!outcome.success);
        assert!(outcome.output.contains("timeout"));
        assert!(started.elapsed() < Duration::from_secs(10));
    }
}
