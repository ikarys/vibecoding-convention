use crate::checks::support;
use crate::checks::{Check, Context};
use crate::finding::Finding;

pub struct T4;

impl Check for T4 {
    fn id(&self) -> &'static str {
        "T4"
    }

    fn level(&self) -> u8 {
        2
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(_) = ctx.decl.as_ref() else {
            return support::missing_declaration(self.id());
        };

        let needles = [
            "e2e",
            "end-to-end",
            "integration",
            "acceptance",
            "cypress",
            "playwright",
        ];
        if let Some(path) = support::find_path_containing_any(&ctx.root, &needles) {
            return vec![Finding::info(
                self.id(),
                "end-to-end suite found",
                "end-to-end suite found",
                Some(support::relative_path(&ctx.root, &path)),
            )];
        }

        let feature = walkdir::WalkDir::new(&ctx.root)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .find(|entry| {
                entry.file_type().is_file()
                    && entry
                        .path()
                        .extension()
                        .map(|ext| ext == "feature")
                        .unwrap_or(false)
                    && entry
                        .path()
                        .components()
                        .any(|c| c.as_os_str() == "features")
            });

        if let Some(entry) = feature {
            return vec![Finding::info(
                self.id(),
                "end-to-end suite found",
                "end-to-end suite found",
                Some(support::relative_path(&ctx.root, entry.path())),
            )];
        }

        vec![Finding::warn(
            self.id(),
            "no end-to-end suite",
            "no end-to-end test suite found; required for user-facing software, declare an exception otherwise",
            None,
            Some("add an end-to-end suite or declare an exception".to_string()),
        )]
    }
}
