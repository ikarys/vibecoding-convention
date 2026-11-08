pub mod d1;
pub mod d10;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;
pub mod d9;

use std::path::PathBuf;

use crate::declaration::Declaration;
use crate::finding::Finding;
use crate::git::Git;

pub struct Context {
    pub root: PathBuf,
    pub decl: Option<Declaration>,
    pub decl_error: Option<String>,
    pub git: Git,
    pub level: u8,
}

pub trait Check {
    fn id(&self) -> &'static str;
    fn level(&self) -> u8;
    fn run(&self, ctx: &Context) -> Vec<Finding>;
}

pub fn all() -> Vec<Box<dyn Check>> {
    vec![
        Box::new(d1::D1),
        Box::new(d2::D2),
        Box::new(d3::D3),
        Box::new(d4::D4),
        Box::new(d5::D5),
        Box::new(d6::D6),
        Box::new(d7::D7),
        Box::new(d8::D8),
        Box::new(d9::D9),
        Box::new(d10::D10),
    ]
}

pub fn run(ctx: &Context) -> Vec<Finding> {
    all()
        .into_iter()
        .filter(|check| check.level() <= ctx.level)
        .flat_map(|check| check.run(ctx))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_contains_level_one_checks() {
        let checks = all();
        assert_eq!(checks.len(), 10);
        assert!(checks.iter().all(|check| check.level() == 1));
    }
}
