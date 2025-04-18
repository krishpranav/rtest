#![allow(dead_code)]

mod assertion;
mod reporter;
mod suite;
mod suite_context;
mod spec;


pub use suite::{describe, Suite, NullState};
pub use suite_context::SuiteContext;
pub use spec::SpecContext;
pub use assertion::{expect, should_panic, should_not_panic};
pub type LabResult = Result<(), String>;