//! The audit index (`audit/README.md`): one row per report, columns found
//! by header name so projects may order and add columns freely.

use crate::audit::table_cells;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IndexRow {
    pub date: String,
    pub commit: String,
    pub kind: String,
    pub read_by: String,
}

pub fn parse_index(text: &str) -> Vec<IndexRow> {
    let mut rows = Vec::new();
    let mut columns = None;
    for line in text.lines() {
        let cells = table_cells(line);
        if cells.is_empty() {
            continue;
        }
        if columns.is_none() {
            if let Some(found) = index_columns(&cells) {
                columns = Some(found);
                continue;
            }
        }
        // Rows before any header keep the template's column order.
        let columns = columns.unwrap_or(IndexColumns::TEMPLATE);
        let date = optional_cell(&cells, Some(columns.date));
        if !is_date_cell(&date) {
            continue;
        }
        rows.push(IndexRow {
            date,
            commit: optional_cell(&cells, Some(columns.commit)),
            kind: optional_cell(&cells, columns.kind),
            read_by: optional_cell(&cells, columns.read_by),
        });
    }
    rows
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

#[derive(Debug, Clone, Copy)]
struct IndexColumns {
    date: usize,
    commit: usize,
    kind: Option<usize>,
    read_by: Option<usize>,
}

impl IndexColumns {
    const TEMPLATE: Self = Self {
        date: 0,
        commit: 1,
        kind: Some(2),
        read_by: Some(4),
    };
}

/// The header row of the index, matched by column names so projects may order
/// and add columns freely.
fn index_columns(cells: &[String]) -> Option<IndexColumns> {
    let position = |pred: fn(&str) -> bool| {
        cells
            .iter()
            .position(|cell| pred(cell.to_lowercase().as_str()))
    };
    Some(IndexColumns {
        date: position(|c| c.starts_with("date"))?,
        commit: position(|c| c.starts_with("commit"))?,
        kind: position(|c| c.starts_with("kind")),
        read_by: position(|c| matches!(c, "read by" | "read_by" | "reader")),
    })
}

fn optional_cell(cells: &[String], index: Option<usize>) -> String {
    index
        .and_then(|index| cells.get(index).cloned())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn parse_index_finds_columns_by_name() {
        let text = "\
| Kind | Date | Reader | Commit | Verdict |
|---|---|---|---|---|
| architecture | 2026-11-08 | ikarys | abc1234 | usable |
";
        let rows = parse_index(text);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].date, "2026-11-08");
        assert_eq!(rows[0].commit, "abc1234");
        assert_eq!(rows[0].kind, "architecture");
        assert_eq!(rows[0].read_by, "ikarys");
    }

    #[test]
    fn parse_index_without_read_by_column() {
        let text = "\
| Date | Commit | Kind |
|---|---|---|
| 2026-11-08 | abc1234 | architecture |
";
        let rows = parse_index(text);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].date, "2026-11-08");
        assert_eq!(rows[0].commit, "abc1234");
        assert_eq!(rows[0].kind, "architecture");
        assert_eq!(rows[0].read_by, "");
    }
}
