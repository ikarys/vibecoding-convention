//! Readers for the audit directory: the index and the report headers
//! and decisions. Pure functions over text; no IO.

use std::collections::BTreeMap;

use crate::declaration::split_front_matter;
use serde_yaml_ng::Value;

pub const SEVERITIES: [&str; 4] = ["BLOCKER", "DOC-WRONG", "PAINFUL", "MINOR"];

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReportHeader {
    pub date: String,
    pub commit: String,
    pub kind: String,
    pub read_by: String,
}

#[derive(Debug, Clone, Default)]
pub struct AuditReport {
    pub header: Option<ReportHeader>,
    pub open: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IndexRow {
    pub date: String,
    pub commit: String,
    pub kind: String,
    pub read_by: String,
}

pub fn parse_report(text: &str) -> AuditReport {
    let mut report = AuditReport::default();
    // A report without front matter still has findings worth counting.
    let (yaml, body) =
        split_front_matter(text).unwrap_or_else(|_| (String::new(), text.to_string()));
    report.header = serde_yaml_ng::from_str::<Value>(&yaml)
        .ok()
        .and_then(|front_matter| parse_header(&front_matter));

    let findings = finding_severities(&body);
    let decisions = decision_cells(&body);
    for (number, severity) in findings.iter().enumerate() {
        let decision = decisions
            .get(&(number + 1))
            .map(|value| value.trim().to_lowercase())
            .unwrap_or_default();
        let open = decision.is_empty() || decision == "open" || decision.starts_with('<');
        if open {
            *report.open.entry(severity.to_string()).or_insert(0) += 1;
        }
    }
    report
}

pub fn parse_index(text: &str) -> Vec<IndexRow> {
    let mut rows = Vec::new();
    for line in text.lines() {
        let cells = table_cells(line);
        if cells.len() < 5 || !is_date_cell(&cells[0]) {
            continue;
        }
        rows.push(IndexRow {
            date: cells[0].clone(),
            commit: cells[1].clone(),
            kind: cells[2].clone(),
            read_by: cells[4].clone(),
        });
    }
    rows
}

pub fn is_report_name(file_name: &str) -> bool {
    file_name.to_lowercase().ends_with(".md") && !file_name.eq_ignore_ascii_case("README.md")
}

pub fn same_commit(a: &str, b: &str) -> bool {
    a.len() >= 7 && b.len() >= 7 && (a.starts_with(b) || b.starts_with(a))
}

/// None when `date`, `commit` or `kind` is missing: the index cannot match
/// such a report.
fn parse_header(front_matter: &Value) -> Option<ReportHeader> {
    let header = ReportHeader {
        date: front_matter_field(front_matter, "date").unwrap_or_default(),
        commit: front_matter_field(front_matter, "commit").unwrap_or_default(),
        kind: front_matter_field(front_matter, "kind").unwrap_or_default(),
        read_by: front_matter_field(front_matter, "read_by").unwrap_or_default(),
    };
    let complete = !header.date.is_empty() && !header.commit.is_empty() && !header.kind.is_empty();
    complete.then_some(header)
}

fn front_matter_field(front_matter: &Value, key: &str) -> Option<String> {
    front_matter
        .as_mapping()
        .and_then(|mapping| mapping.get(key))
        .and_then(scalar_to_string)
}

fn scalar_to_string(value: &Value) -> Option<String> {
    if let Some(text) = value.as_str() {
        return Some(text.to_string());
    }
    if value.is_null() {
        return None;
    }
    serde_yaml_ng::to_string(value)
        .ok()
        .map(|text| text.trim().to_string())
}

fn finding_severities(body: &str) -> Vec<&'static str> {
    let mut severities = Vec::new();
    for line in body.lines() {
        if let Some(severity) = list_severity(line) {
            severities.push(severity);
        } else if let Some(severity) = table_severity(line) {
            severities.push(severity);
        }
    }
    severities
}

fn decision_cells(body: &str) -> BTreeMap<usize, String> {
    let mut decisions = BTreeMap::new();
    let mut in_decisions = false;
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            in_decisions = trimmed.to_lowercase().contains("decisions");
            continue;
        }
        if !in_decisions {
            continue;
        }
        let cells = table_cells(trimmed);
        if cells.len() < 2 || !is_number(&cells[0]) {
            continue;
        }
        let number: usize = cells[0].parse().unwrap_or(0);
        if number > 0 {
            decisions.insert(number, cells[1].clone());
        }
    }
    decisions
}

fn list_severity(line: &str) -> Option<&'static str> {
    let rest = line.trim().strip_prefix('-')?.trim_start();
    let (key, value) = rest.split_once(':')?;
    if key.trim().eq_ignore_ascii_case("severity") {
        normalize_severity(value)
    } else {
        None
    }
}

fn table_severity(line: &str) -> Option<&'static str> {
    let cells = table_cells(line);
    if cells.len() >= 3 && is_number(&cells[0]) {
        normalize_severity(&cells[2])
    } else {
        None
    }
}

fn normalize_severity(text: &str) -> Option<&'static str> {
    let upper = text.trim().to_uppercase();
    SEVERITIES
        .iter()
        .copied()
        .find(|severity| *severity == upper.as_str())
}

fn table_cells(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') {
        return Vec::new();
    }
    let inner = trimmed.trim_start_matches('|').trim_end_matches('|');
    inner
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

fn is_number(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_date_cell(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && (0..4).all(|index| bytes[index].is_ascii_digit())
        && (5..7).all(|index| bytes[index].is_ascii_digit())
        && (8..10).all(|index| bytes[index].is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_form_report_counts_open_findings() {
        let text = "\
---
date: 2026-11-08
commit: abc1234
kind: architecture
read_by: ikarys
---
- Severity: BLOCKER
- Severity: PAINFUL
- Severity: MINOR

## Decisions

| 1 | fixed | abc1234 |
| 2 | open |  |
| 3 | <fixed | ...> |  |
";
        let report = parse_report(text);
        assert_eq!(report.header.as_ref().unwrap().date, "2026-11-08");
        assert_eq!(report.open.get("BLOCKER"), None);
        assert_eq!(report.open.get("PAINFUL"), Some(&1));
        assert_eq!(report.open.get("MINOR"), Some(&1));
    }

    #[test]
    fn friction_log_report_counts_open_findings() {
        let text = "\
---
date: 2026-11-08
commit: abc1234
kind: blind-test
read_by: ikarys
---
| 1 | install | BLOCKER | broken |
| 2 | run | MINOR | slow |

## Decisions

| 1 | fixed | abc1234 |
";
        let report = parse_report(text);
        assert_eq!(report.open.get("BLOCKER"), None);
        assert_eq!(report.open.get("MINOR"), Some(&1));
    }

    #[test]
    fn report_without_front_matter_has_no_header() {
        let report = parse_report("- Severity: MINOR\n");
        assert_eq!(report.header, None);
        assert_eq!(report.open.get("MINOR"), Some(&1));
    }

    #[test]
    fn parse_index_skips_non_data_rows() {
        let text = "\
| Date | Commit | Kind | Tool / model | Read by | Verdict | Open findings |
|---|---|---|---|---|---|---|
| <YYYY-MM-DD> | <commit> | <kind> | <tool> | <read_by> | <verdict> | 0 |
| 2026-11-08 | abc1234 | architecture | Claude Code:claude-fable-5-1 | ikarys | usable | 2 |
";
        let rows = parse_index(text);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].date, "2026-11-08");
        assert_eq!(rows[0].commit, "abc1234");
        assert_eq!(rows[0].kind, "architecture");
        assert_eq!(rows[0].read_by, "ikarys");
    }

    #[test]
    fn same_commit_accepts_prefixes_and_rejects_short_hashes() {
        assert!(same_commit("abc1234", "abc1234567890"));
        assert!(same_commit("abc1234567890", "abc1234"));
        assert!(!same_commit("abc123", "abc1234"));
        assert!(!same_commit("abc1234", "def1234"));
    }
}
