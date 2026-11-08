use std::fs;
use std::path::Path;

use serde_json::{json, Map, Value};

use crate::finding::{short_hash, Report};

pub fn write_status(root: &Path, report: &Report) -> anyhow::Result<()> {
    let existing = read_status_value(root, "audit")
        .map(|(_, value)| value)
        .unwrap_or(Value::Null);
    let vibecoding = existing.get("vibecoding").cloned().unwrap_or(Value::Null);
    let preserved_report = vibecoding.get("report").cloned().unwrap_or(Value::Null);
    let preserved_metrics = vibecoding.get("metrics").cloned().unwrap_or(json!({}));
    let preserved_read_by = vibecoding.get("read_by").cloned().unwrap_or(Value::Null);
    let metrics = merge_metrics(preserved_metrics, &report.metrics);

    let commit = report.checked_commit.clone().unwrap_or_default();
    let short = short_hash(&commit);
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
            "open_findings": { "BLOCKER": 0, "DOC-WRONG": 0, "PAINFUL": 0, "MINOR": 0 },
            "metrics": metrics,
            "checked_by": format!("vibecheck {}", env!("CARGO_PKG_VERSION")),
            "read_by": preserved_read_by
        }
    });

    let dir = root.join("audit");
    fs::create_dir_all(&dir)?;
    fs::write(
        dir.join("status.json"),
        serde_json::to_string_pretty(&status)? + "\n",
    )?;
    Ok(())
}

fn merge_metrics(
    existing: Value,
    report_metrics: &std::collections::BTreeMap<String, Value>,
) -> Value {
    let mut map = match existing {
        Value::Object(obj) => obj,
        _ => Map::new(),
    };
    for (key, value) in report_metrics {
        map.insert(key.clone(), value.clone());
    }
    Value::Object(map)
}

pub fn read_status_value(root: &Path, dir: &str) -> Option<(std::path::PathBuf, Value)> {
    let path = root.join(dir).join("status.json");
    let text = fs::read_to_string(&path).ok()?;
    let value: Value = serde_json::from_str(&text).ok()?;
    Some((path, value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finding::{Finding, Severity};
    use std::collections::BTreeMap;

    fn report_with_metrics(metrics: BTreeMap<String, Value>) -> Report {
        Report {
            level: 1,
            findings: vec![Finding::info("D1", "ok", "all good", None)],
            checked_commit: Some("abcdef1234567890".to_string()),
            date: chrono::Utc::now().naive_local().date(),
            metrics,
        }
    }

    #[test]
    fn writes_status_and_preserves_metrics() {
        let dir = tempfile::tempdir().unwrap();
        let mut first = BTreeMap::new();
        first.insert("old".to_string(), json!(1));
        write_status(dir.path(), &report_with_metrics(first)).unwrap();

        let mut second = BTreeMap::new();
        second.insert("new".to_string(), json!(2));
        write_status(dir.path(), &report_with_metrics(second)).unwrap();

        let (_, value) = read_status_value(dir.path(), "audit").unwrap();
        let metrics = value.get("vibecoding").unwrap().get("metrics").unwrap();
        assert_eq!(metrics.get("old"), Some(&json!(1)));
        assert_eq!(metrics.get("new"), Some(&json!(2)));
        assert_eq!(
            value.get("vibecoding").unwrap().get("verdict"),
            Some(&json!("pass"))
        );
        assert!(!matches!(
            value.get("vibecoding").unwrap().get("verdict"),
            Some(Value::String(s)) if s == "fail"
        ));
        let _ = Severity::Info;
    }
}
