use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Fail,
    Warn,
    Info,
    NotRun,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            problem: problem.into(),
            where_,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub level: u8,
    pub findings: Vec<Finding>,
    pub checked_commit: Option<String>,
    pub date: chrono::NaiveDate,
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
        let mut output = String::new();
        let mut groups: std::collections::BTreeMap<(u8, u32, String), Vec<&Finding>> =
            std::collections::BTreeMap::new();

        for finding in &self.findings {
            groups
                .entry(requirement_rank(&finding.id))
                .or_default()
                .push(finding);
        }

        for ((_, _, id), findings) in groups {
            for finding in findings {
                let severity = match finding.severity {
                    Severity::Fail => "FAIL",
                    Severity::Warn => "WARN",
                    Severity::Info => "INFO",
                    Severity::NotRun => "NOT RUN",
                };
                let where_part = finding
                    .where_
                    .as_ref()
                    .map(|value| format!("  ({value})"))
                    .unwrap_or_default();
                output.push_str(&format!(
                    "{id}  {severity}  {}{}\n",
                    finding.problem, where_part
                ));
                if let Some(fix) = &finding.fix {
                    output.push_str(&format!("    fix: {fix}\n"));
                }
            }
        }

        let (fail, warn, info, not_run) = self.counts();
        let commit = self
            .checked_commit
            .as_deref()
            .map(short_hash)
            .unwrap_or("none");
        output.push_str(&format!(
            "{fail} fail, {warn} warn, {info} info, {not_run} not run · level {} · commit {commit}\n",
            self.level
        ));

        output
    }

    pub fn to_json(&self) -> String {
        let summary = serde_json::json!({
            "fail": self.counts().0,
            "warn": self.counts().1,
            "info": self.counts().2,
            "not_run": self.counts().3,
            "level": self.level,
            "date": self.date.to_string(),
            "checked_commit": self.checked_commit.clone(),
        });

        serde_json::json!({
            "findings": self.findings,
            "summary": summary,
        })
        .to_string()
    }
}

pub fn short_hash(hash: &str) -> &str {
    hash.get(..7).unwrap_or(hash)
}

/// Sort key for requirement identifiers: level letter (D, T, A), then number.
pub fn requirement_rank(id: &str) -> (u8, u32, String) {
    let letter = match id.chars().next() {
        Some('D') => 0,
        Some('T') => 1,
        Some('A') => 2,
        _ => 3,
    };
    let number = id
        .trim_start_matches(|c: char| c.is_ascii_alphabetic())
        .parse::<u32>()
        .unwrap_or(0);
    (letter, number, id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_output_contains_summary() {
        let report = Report {
            level: 1,
            findings: vec![Finding::fail(
                "D1",
                "t",
                "problem",
                Some("where".into()),
                None,
            )],
            checked_commit: Some("0123456789".into()),
            date: chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        };
        let text = report.to_text();
        assert!(text.contains("D1  FAIL  problem  (where)"));
        assert!(text.contains("1 fail, 0 warn, 0 info, 0 not run"));
        assert!(text.contains("commit 0123456"));
    }

    #[test]
    fn json_output_contains_findings() {
        let report = Report {
            level: 1,
            findings: vec![Finding::info("D1", "t", "problem", None)],
            checked_commit: None,
            date: chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        };
        let json: serde_json::Value = serde_json::from_str(&report.to_json()).unwrap();
        assert_eq!(json["findings"][0]["id"], "D1");
        assert_eq!(json["summary"]["info"], 1);
    }
}
