use std::fs;
use std::path::Path;

use serde_json::{json, Value};

use crate::finding::{short_hash, Report};

pub fn write_status(root: &Path, report: &Report) -> anyhow::Result<()> {
    let existing = read_status_value(root, "audit")
        .map(|(_, value)| value)
        .unwrap_or(Value::Null);
    let vibecoding = existing.get("vibecoding").cloned().unwrap_or(Value::Null);
    let preserved_report = vibecoding.get("report").cloned().unwrap_or(Value::Null);
    let preserved_metrics = vibecoding.get("metrics").cloned().unwrap_or(json!({}));
    let preserved_read_by = vibecoding.get("read_by").cloned().unwrap_or(Value::Null);

    let commit = report.checked_commit.clone().unwrap_or_default();
    let short = short_hash(&commit);
    // The file written here is what D5 checks, so its own findings about the
    // previous file (missing, stale) do not decide the verdict.
    let counts = |severity: crate::finding::Severity| {
        report
            .findings
            .iter()
            .filter(|f| f.id != "D5" && f.severity == severity)
            .count()
    };
    let (verdict, color) = if counts(crate::finding::Severity::Fail) > 0 {
        ("fail", "red")
    } else if counts(crate::finding::Severity::Warn) > 0 {
        ("pass with findings", "yellow")
    } else {
        ("pass", "blue")
    };

    let status = json!({
        "schemaVersion": 1,
        "label": "vibecoding",
        "message": format!("level {} · checked {} · {short}", report.level, report.date),
        "color": color,
        "vibecoding": {
            "level": report.level,
            "verdict": verdict,
            "date": report.date.to_string(),
            "commit": short,
            "report": preserved_report,
            "open_findings": {
                "BLOCKER": 0,
                "DOC-WRONG": 0,
                "PAINFUL": 0,
                "MINOR": 0
            },
            "metrics": preserved_metrics,
            "checked_by": format!("vibecheck {}", env!("CARGO_PKG_VERSION")),
            "read_by": preserved_read_by
        }
    });

    let dir = root.join("audit");
    fs::create_dir_all(&dir)?;
    let path = dir.join("status.json");
    fs::write(path, serde_json::to_string_pretty(&status)?)?;
    Ok(())
}

pub fn read_status_value(root: &Path, audit_dir: &str) -> Option<(std::path::PathBuf, Value)> {
    let dir = if audit_dir.is_empty() {
        "audit"
    } else {
        audit_dir
    };
    let path = root.join(dir).join("status.json");
    let text = fs::read_to_string(&path).ok()?;
    let value: Value = serde_json::from_str(&text).ok()?;
    Some((path, value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finding::{Finding, Severity};

    #[test]
    fn writes_status_and_preserves_metrics() {
        let dir = tempfile::tempdir().unwrap();
        let audit = dir.path().join("audit");
        fs::create_dir_all(&audit).unwrap();
        fs::write(
            audit.join("status.json"),
            r#"{ "vibecoding": { "metrics": { "x": 1 }, "report": "old" } }"#,
        )
        .unwrap();

        let report = Report {
            level: 2,
            findings: vec![Finding::new("D1", Severity::Info, "t", "p", None, None)],
            checked_commit: Some("abcdef1234567890".into()),
            date: chrono::NaiveDate::from_ymd_opt(2026, 1, 2).unwrap(),
        };

        write_status(dir.path(), &report).unwrap();
        let value: Value =
            serde_json::from_str(&fs::read_to_string(audit.join("status.json")).unwrap()).unwrap();
        assert_eq!(value["vibecoding"]["level"], 2);
        assert_eq!(value["vibecoding"]["metrics"]["x"], 1);
        assert_eq!(value["vibecoding"]["report"], "old");
        assert_eq!(value["vibecoding"]["commit"], "abcdef1");
    }
}
