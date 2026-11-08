use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Severity {
    Fail,
    Warn,
    Info,
    NotRun,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Fail => write!(f, "FAIL"),
            Severity::Warn => write!(f, "WARN"),
            Severity::Info => write!(f, "INFO"),
            Severity::NotRun => write!(f, "NOT RUN"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub id: String,
    pub severity: Severity,
    pub title: String,
    pub where_: Option<String>,
    pub problem: String,
    pub fix: Option<String>,
}

impl Finding {
    pub fn new(
        id: &str,
        severity: Severity,
        title: &str,
        problem: impl Into<String>,
        where_: Option<String>,
        fix: Option<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            severity,
            title: title.to_string(),
            where_,
            problem: problem.into(),
            fix,
        }
    }

    pub fn fail(
        id: &str,
        title: &str,
        problem: impl Into<String>,
        where_: Option<String>,
        fix: Option<String>,
    ) -> Self {
        Self::new(id, Severity::Fail, title, problem, where_, fix)
    }

    pub fn warn(
        id: &str,
        title: &str,
        problem: impl Into<String>,
        where_: Option<String>,
        fix: Option<String>,
    ) -> Self {
        Self::new(id, Severity::Warn, title, problem, where_, fix)
    }

    pub fn info(id: &str, title: &str, problem: impl Into<String>, where_: Option<String>) -> Self {
        Self::new(id, Severity::Info, title, problem, where_, None)
    }

    pub fn not_run(
        id: &str,
        title: &str,
        problem: impl Into<String>,
        where_: Option<String>,
    ) -> Self {
        Self::new(id, Severity::NotRun, title, problem, where_, None)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub level: u8,
    pub findings: Vec<Finding>,
    pub checked_commit: Option<String>,
    pub date: chrono::NaiveDate,
    pub metrics: BTreeMap<String, Value>,
    /// Declared evidence keys and their paths, recorded in `status.json`.
    pub evidence: BTreeMap<String, String>,
}

impl Report {
    pub fn has_fail(&self) -> bool {
        self.findings.iter().any(|f| f.severity == Severity::Fail)
    }

    pub fn has_warn(&self) -> bool {
        self.findings.iter().any(|f| f.severity == Severity::Warn)
    }

    pub fn counts(&self) -> (usize, usize, usize, usize) {
        let mut fail = 0;
        let mut warn = 0;
        let mut info = 0;
        let mut not_run = 0;
        for finding in &self.findings {
            match finding.severity {
                Severity::Fail => fail += 1,
                Severity::Warn => warn += 1,
                Severity::Info => info += 1,
                Severity::NotRun => not_run += 1,
            }
        }
        (fail, warn, info, not_run)
    }

    pub fn to_text(&self) -> String {
        let mut out = String::new();
        let mut last_rank = None;
        for finding in &self.findings {
            let rank = requirement_rank(&finding.id);
            if last_rank.as_ref() != Some(&rank) {
                if last_rank.is_some() {
                    out.push('\n');
                }
                last_rank = Some(rank);
            }
            let where_part = finding
                .where_
                .as_ref()
                .map(|w| format!("  ({w})"))
                .unwrap_or_default();
            out.push_str(&format!(
                "{}  {}  {}{}\n",
                finding.id, finding.severity, finding.problem, where_part
            ));
            if let Some(fix) = &finding.fix {
                out.push_str(&format!("    fix: {fix}\n"));
            }
        }

        let (fail, warn, info, not_run) = self.counts();
        let commit = self.checked_commit.clone().unwrap_or_default();
        let short = if commit.is_empty() {
            "none".to_string()
        } else {
            short_hash(&commit).to_string()
        };
        out.push_str(&format!(
            "{fail} fail, {warn} warn, {info} info, {not_run} not run · level {} · commit {short}\n",
            self.level
        ));
        out
    }

    pub fn to_json(&self) -> String {
        let (fail, warn, info, not_run) = self.counts();
        let commit = self.checked_commit.clone().unwrap_or_default();
        let value = json!({
            "level": self.level,
            "date": self.date,
            "commit": short_hash(&commit),
            "findings": self.findings,
            "summary": {
                "fail": fail,
                "warn": warn,
                "info": info,
                "not_run": not_run
            },
            "metrics": self.metrics
        });
        serde_json::to_string_pretty(&value).unwrap_or_default()
    }
}

pub fn short_hash(hash: &str) -> &str {
    hash.get(..7).unwrap_or(hash)
}

pub fn requirement_rank(id: &str) -> (u8, u32, String) {
    let mut chars = id.chars();
    let letter = chars.next().unwrap_or('D');
    let number: u32 = chars
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    let rank = match letter {
        'D' => 0,
        'T' => 1,
        'A' => 2,
        _ => 255,
    };
    (rank, number, id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_hash_truncates() {
        assert_eq!(short_hash("abcdef123456"), "abcdef1");
        assert_eq!(short_hash("abc"), "abc");
    }

    #[test]
    fn requirement_rank_orders_d_t_a() {
        assert!(requirement_rank("D1") < requirement_rank("T1"));
        assert!(requirement_rank("T1") < requirement_rank("A1"));
        assert!(requirement_rank("D2") < requirement_rank("D10"));
    }
}
