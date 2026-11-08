use std::path::Path;
use std::process::Command;

pub struct Outcome {
    pub success: bool,
    pub output: String,
}

pub fn run_command(root: &Path, command: &str) -> anyhow::Result<Outcome> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(root)
        .output()?;
    let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(Outcome {
        success: output.status.success(),
        output: combined,
    })
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
}
