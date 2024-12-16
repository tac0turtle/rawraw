#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IXCLanguage {}

include!(concat!(env!("OUT_DIR"), "/syntax_kind.rs"));
