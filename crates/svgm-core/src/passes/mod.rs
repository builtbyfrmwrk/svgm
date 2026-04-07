pub mod cleanup_attrs;
pub mod cleanup_ids;
pub mod cleanup_numeric_values;
pub mod collapse_groups;
pub mod convert_colors;
pub mod convert_path_data;
pub mod convert_shape_to_path;
pub mod convert_transform;
pub mod inline_styles;
pub mod merge_paths;
pub mod minify_styles;
pub mod minify_whitespace;
pub mod remove_comments;
pub mod remove_desc;
pub mod remove_doctype;
pub mod remove_editor_data;
pub mod remove_empty_attrs;
pub mod remove_empty_containers;
pub mod remove_empty_text;
pub mod remove_hidden_elems;
pub mod remove_metadata;
pub mod remove_proc_inst;
pub mod remove_unknowns_and_defaults;
pub mod remove_unused_namespaces;
pub mod sort_attrs;

use crate::ast::Document;

/// Result of running a single optimization pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassResult {
    Changed,
    Unchanged,
}

impl PassResult {
    pub fn changed(self) -> bool {
        self == PassResult::Changed
    }
}

/// An optimization pass that transforms a Document in place.
pub trait Pass {
    fn name(&self) -> &'static str;
    fn run(&self, doc: &mut Document) -> PassResult;
}

/// Returns the default set of passes in execution order (Balanced preset).
pub fn default_passes() -> Vec<Box<dyn Pass>> {
    crate::config::passes_for_config(&crate::config::Config::default())
}
