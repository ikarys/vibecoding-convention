use std::fs;
use std::path::Path;

use chrono::Duration;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Declaration {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub processes: Processes,
    #[serde(default)]
    pub convention: Convention,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Processes {
    #[serde(default)]
    pub design: String,
    #[serde(default)]
    pub implementation: String,
    #[serde(default)]
    pub testing: String,
    #[serde(default)]
    pub documentation: String,
    #[serde(default)]
    pub review: String,
    #[serde(default)]
    pub deployment: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Convention {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub level: u8,
    #[serde(default)]
    pub adopted_at: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub tools: Vec<String>,
    /// Trailers this project uses for AI attribution; empty means the common defaults.
    #[serde(default)]
    pub attribution_trailers: Vec<String>,
    /// Extra agent instruction files to check, in addition to the known names.
    #[serde(default)]
    pub agent_instruction_files: Vec<String>,
    #[serde(default)]
    pub maintainers: Vec<String>,
    #[serde(default)]
    pub review: String,
    #[serde(default)]
    pub security_response: String,
    #[serde(default)]
    pub licence: String,
    #[serde(default)]
    pub coverage: Option<Coverage>,
    #[serde(default)]
    pub maintainability: Option<Maintainability>,
    #[serde(default)]
    pub mutation: Option<Mutation>,
    #[serde(default)]
    pub audit_dir: String,
    #[serde(default)]
    pub audit_cadence: String,
    #[serde(default)]
    pub blind_test: String,
    #[serde(default)]
    pub check_interval: Option<CheckInterval>,
    #[serde(default)]
    pub agent_permissions: String,
    #[serde(default)]
    pub commands: Commands,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Coverage {
    #[serde(default)]
    pub threshold: u32,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub tool: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Maintainability {
    #[serde(default)]
    pub tool: String,
    #[serde(default)]
    pub duplication_max: u32,
    #[serde(default)]
    pub complexity_max: u32,
    #[serde(default)]
    pub function_max: u32,
    #[serde(default)]
    pub file_max: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Mutation {
    #[serde(default)]
    pub tool: String,
    #[serde(default)]
    pub threshold: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Commands {
    #[serde(default)]
    pub test: Option<String>,
    #[serde(default)]
    pub coverage: Option<String>,
    #[serde(default)]
    pub lint: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub audit: Option<String>,
    #[serde(default)]
    pub static_analysis: Option<String>,
    #[serde(default)]
    pub secrets: Option<String>,
    #[serde(default)]
    pub maintainability: Option<String>,
    #[serde(default)]
    pub mutation: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct CheckInterval(Duration);

impl CheckInterval {
    pub fn as_duration(&self) -> Duration {
        self.0
    }
}

impl Default for CheckInterval {
    fn default() -> Self {
        Self(Duration::days(180))
    }
}

impl Serialize for CheckInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}d", self.0.num_days()))
    }
}

impl<'de> Deserialize<'de> for CheckInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        parse_interval(&value)
            .map(CheckInterval)
            .map_err(serde::de::Error::custom)
    }
}

pub fn parse_interval(value: &str) -> anyhow::Result<Duration> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(Duration::days(180));
    }

    let (number, unit) = value.split_at(value.len() - 1);
    let days: i64 = number
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid check_interval number: {number}"))?;

    match unit {
        "d" => Ok(Duration::days(days)),
        "m" => Ok(Duration::days(days * 30)),
        "y" => Ok(Duration::days(days * 365)),
        _ => Err(anyhow::anyhow!("invalid check_interval unit: {unit}")),
    }
}

pub fn split_front_matter(text: &str) -> anyhow::Result<(String, String)> {
    let mut first: Option<usize> = None;

    for (index, line) in text.lines().enumerate() {
        if line.trim() == "---" {
            match first {
                None => first = Some(index),
                Some(start) => {
                    let yaml = text
                        .lines()
                        .skip(start + 1)
                        .take(index - start - 1)
                        .collect::<Vec<_>>()
                        .join("\n");
                    let body = text.lines().skip(index + 1).collect::<Vec<_>>().join("\n");
                    return Ok((yaml, body));
                }
            }
        }
    }

    Err(anyhow::anyhow!(
        "missing closing --- front matter delimiter"
    ))
}

pub fn read_declaration(root: &Path) -> (Option<Declaration>, Option<String>) {
    let path = root.join("VIBECODING.md");
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            return (
                None,
                Some(format!("VIBECODING.md not found or unreadable: {err}")),
            );
        }
    };

    let (yaml, body) = match split_front_matter(&text) {
        Ok(parts) => parts,
        Err(err) => return (None, Some(format!("VIBECODING.md front matter: {err}"))),
    };

    let mut declaration: Declaration = match serde_yaml_ng::from_str(&yaml) {
        Ok(value) => value,
        Err(err) => return (None, Some(format!("VIBECODING.md YAML: {err}"))),
    };
    declaration.body = body;

    (Some(declaration), None)
}

pub fn missing_required_keys(text: &str) -> Vec<String> {
    let (yaml, _) = match split_front_matter(text) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    let value: serde_yaml_ng::Value = match serde_yaml_ng::from_str(&yaml) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };

    let mut missing = Vec::new();
    for key in ["version", "level", "processes", "convention"] {
        if value.get(key).is_none() {
            missing.push(key.to_string());
        }
    }

    if let Some(convention) = value.get("convention") {
        if convention.is_mapping() {
            for key in [
                "version",
                "level",
                "adopted_at",
                "status",
                "tools",
                "maintainers",
                "review",
                "security_response",
                "licence",
            ] {
                if convention.get(key).is_none() {
                    missing.push(format!("convention.{key}"));
                }
            }

            if let Some(level) = convention.get("level").and_then(|v| v.as_u64()) {
                if level >= 2 {
                    for key in ["coverage", "maintainability"] {
                        if convention.get(key).is_none() {
                            missing.push(format!("convention.{key}"));
                        }
                    }
                }
                if level >= 3 {
                    for key in [
                        "audit_dir",
                        "audit_cadence",
                        "blind_test",
                        "check_interval",
                        "agent_permissions",
                    ] {
                        if convention.get(key).is_none() {
                            missing.push(format!("convention.{key}"));
                        }
                    }
                }
            }
        } else {
            missing.push("convention".to_string());
        }
    }

    missing
}

/// Attribution trailers accepted when the project declares none.
pub const DEFAULT_ATTRIBUTION_TRAILERS: &[&str] =
    &["Assisted-by", "Generated-by", "Co-Authored-By", "AI-Tool"];

impl Convention {
    /// The attribution trailers to accept, lower-cased.
    pub fn attribution_trailers(&self) -> Vec<String> {
        if self.attribution_trailers.is_empty() {
            DEFAULT_ATTRIBUTION_TRAILERS
                .iter()
                .map(|t| t.to_lowercase())
                .collect()
        } else {
            self.attribution_trailers
                .iter()
                .map(|t| t.to_lowercase())
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_intervals() {
        assert_eq!(parse_interval("90d").unwrap(), Duration::days(90));
        assert_eq!(parse_interval("6m").unwrap(), Duration::days(180));
        assert_eq!(parse_interval("1y").unwrap(), Duration::days(365));
        assert!(parse_interval("6x").is_err());
    }

    #[test]
    fn splits_front_matter() {
        let text = "---\nlevel: 1\n---\n# Body\n";
        let (yaml, body) = split_front_matter(text).unwrap();
        assert_eq!(yaml, "level: 1");
        assert_eq!(body, "# Body");
    }

    #[test]
    fn reports_missing_required_keys() {
        let text = "---\nversion: \"0.1\"\n---\n";
        let missing = missing_required_keys(text);
        assert!(missing.contains(&"level".to_string()));
        assert!(missing.contains(&"convention".to_string()));
    }
}
