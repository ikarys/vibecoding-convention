use std::path::PathBuf;
use std::process::{Command, Stdio};

use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub subject: String,
    pub body: String,
    pub trailers: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct Git {
    root: PathBuf,
}

impl Git {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn is_repo(&self) -> bool {
        self.run_ok(&["rev-parse", "--is-inside-work-tree"])
    }

    pub fn head(&self) -> Option<String> {
        self.output(&["rev-parse", "HEAD"])
            .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
            .filter(|hash| !hash.is_empty())
    }

    pub fn commit_exists(&self, hash: &str) -> bool {
        if hash.is_empty() {
            return false;
        }
        self.run_ok(&["cat-file", "-e", &format!("{hash}^{{commit}}")])
    }

    pub fn commits_since(&self, r: &str) -> Vec<Commit> {
        if !self.is_repo() || r.is_empty() {
            return Vec::new();
        }
        let range = format!("{r}..HEAD");
        self.output(&[
            "log",
            "--format=%H%x00%s%x00%b%x00%(trailers:only,unfold)%x1e",
            &range,
        ])
        .map(parse_log)
        .unwrap_or_default()
    }

    pub fn commits_since_date(&self, date: &NaiveDate) -> Vec<Commit> {
        if !self.is_repo() {
            return Vec::new();
        }
        let since = date.to_string();
        self.output(&[
            "log",
            "--format=%H%x00%s%x00%b%x00%(trailers:only,unfold)%x1e",
            &format!("--since={since}"),
            "HEAD",
        ])
        .map(parse_log)
        .unwrap_or_default()
    }

    pub fn latest_tag(&self) -> Option<(String, DateTime<Utc>)> {
        self.output(&[
            "for-each-ref",
            "--sort=-creatordate",
            "--format=%(refname:short)%x00%(creatordate:iso-strict)",
            "refs/tags",
        ])
        .and_then(|out| {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .next()
                .and_then(parse_tag_line)
        })
    }

    pub fn show_stat(&self, hash: &str) -> String {
        self.output(&["show", "--stat", "--format=", hash])
            .map(|out| String::from_utf8_lossy(&out.stdout).into_owned())
            .unwrap_or_default()
    }

    pub fn show_diff(&self, hash: &str) -> String {
        self.output(&["show", "--diff", "--format=", hash])
            .map(|out| String::from_utf8_lossy(&out.stdout).into_owned())
            .unwrap_or_default()
    }

    fn run_ok(&self, args: &[&str]) -> bool {
        Command::new("git")
            .args(args)
            .current_dir(&self.root)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    fn output(&self, args: &[&str]) -> Option<std::process::Output> {
        self.command(args).output().ok()
    }

    fn command(&self, args: &[&str]) -> Command {
        let mut command = Command::new("git");
        command.current_dir(&self.root);
        for arg in args {
            command.arg(arg);
        }
        command
    }
}

fn parse_log(output: std::process::Output) -> Vec<Commit> {
    let text = String::from_utf8_lossy(&output.stdout);
    text.split('\x1e')
        .filter_map(|record| {
            let record = record.trim();
            if record.is_empty() {
                return None;
            }
            let mut parts = record.split('\x00');
            let hash = parts.next()?.to_string();
            let subject = parts.next().unwrap_or_default().to_string();
            let body = parts.next().unwrap_or_default().to_string();
            let trailers_raw = parts.next().unwrap_or_default();
            let trailers = trailers_raw.lines().filter_map(parse_trailer).collect();
            Some(Commit {
                hash,
                subject,
                body,
                trailers,
            })
        })
        .collect()
}

fn parse_trailer(line: &str) -> Option<(String, String)> {
    let (key, value) = line.split_once(':')?;
    let key = key.trim().to_string();
    let value = value.trim().to_string();
    if key.is_empty() {
        None
    } else {
        Some((key, value))
    }
}

fn parse_tag_line(line: &str) -> Option<(String, DateTime<Utc>)> {
    let (name, date) = line.split_once('\x00')?;
    let date = DateTime::parse_from_rfc3339(date)
        .ok()
        .map(|value| value.with_timezone(&Utc))?;
    Some((name.to_string(), date))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_log_records() {
        let output = std::process::Output {
            status: Default::default(),
            stdout: "abc\x00subject\x00body\x00Assisted-by: tool\x1e"
                .as_bytes()
                .to_vec(),
            stderr: Vec::new(),
        };
        let commits = parse_log(output);
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].hash, "abc");
        assert_eq!(commits[0].trailers[0].0, "Assisted-by");
    }

    #[test]
    fn parses_tag_line() {
        let parsed = parse_tag_line("v1.0.0\x002026-01-02T03:04:05+00:00").unwrap();
        assert_eq!(parsed.0, "v1.0.0");
        assert_eq!(chrono::Timelike::hour(&parsed.1), 3);
    }
}
