use std::collections::HashMap;

use crate::passes::{self, Pass};

/// Optimization aggressiveness level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Preset {
    /// Zero-risk-to-rendering: removal, normalization, and whitespace passes only.
    Safe,
    /// Structural and transform optimization (default). Matches the full pass set.
    #[default]
    Balanced,
    /// Maximum compression. Same passes as Balanced but with lower numeric precision (2).
    Aggressive,
}

/// Configuration for the optimization pipeline.
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Which preset to use as the base pass set.
    pub preset: Preset,
    /// Numeric precision for rounding passes. If `None`, uses the preset default
    /// (3 for Safe/Balanced, 2 for Aggressive).
    pub precision: Option<u32>,
    /// Per-pass overrides. `true` enables a pass not in the preset, `false` disables one that is.
    pub pass_overrides: HashMap<String, bool>,
}

impl Config {
    /// Returns the effective numeric precision for this configuration.
    pub fn effective_precision(&self) -> u32 {
        if let Some(p) = self.precision {
            return p;
        }
        match self.preset {
            Preset::Aggressive => 2,
            _ => 3,
        }
    }
}

// (name, in_safe, in_balanced, in_aggressive) — execution order matters.
const PASS_CATALOG: &[(&str, bool, bool, bool)] = &[
    ("removeDoctype", true, true, true),
    ("removeProcInst", true, true, true),
    ("removeComments", true, true, true),
    ("removeMetadata", true, true, true),
    ("removeEditorData", true, true, true),
    ("removeEmptyAttrs", true, true, true),
    ("removeEmptyText", true, true, true),
    ("removeEmptyContainers", true, true, true),
    ("removeHiddenElems", true, true, true),
    ("removeUnusedNamespaces", true, true, true),
    ("cleanupAttrs", true, true, true),
    ("inlineStyles", false, true, true),
    ("cleanupNumericValues", true, true, true),
    ("convertColors", true, true, true),
    ("removeUnknownsAndDefaults", true, true, true),
    ("convertShapeToPath", false, true, true),
    ("convertTransform", false, true, true),
    ("collapseGroups", false, true, true),
    ("cleanupIds", false, true, true),
    ("convertPathData", false, true, true),
    ("mergePaths", false, true, true),
    ("sortAttrs", true, true, true),
    ("minifyStyles", true, true, true),
    ("minifyWhitespace", true, true, true),
    // removeDesc is opt-in only — not part of any preset
    ("removeDesc", false, false, false),
];

fn is_in_preset(entry: &(&str, bool, bool, bool), preset: Preset) -> bool {
    match preset {
        Preset::Safe => entry.1,
        Preset::Balanced => entry.2,
        Preset::Aggressive => entry.3,
    }
}

fn create_pass(name: &str, precision: u32) -> Box<dyn Pass> {
    match name {
        "removeDoctype" => Box::new(passes::remove_doctype::RemoveDoctype),
        "removeProcInst" => Box::new(passes::remove_proc_inst::RemoveProcInst),
        "removeComments" => Box::new(passes::remove_comments::RemoveComments),
        "removeMetadata" => Box::new(passes::remove_metadata::RemoveMetadata),
        "removeEditorData" => Box::new(passes::remove_editor_data::RemoveEditorData),
        "removeEmptyAttrs" => Box::new(passes::remove_empty_attrs::RemoveEmptyAttrs),
        "removeEmptyText" => Box::new(passes::remove_empty_text::RemoveEmptyText),
        "removeEmptyContainers" => Box::new(passes::remove_empty_containers::RemoveEmptyContainers),
        "removeHiddenElems" => Box::new(passes::remove_hidden_elems::RemoveHiddenElems),
        "removeUnusedNamespaces" => {
            Box::new(passes::remove_unused_namespaces::RemoveUnusedNamespaces)
        }
        "cleanupAttrs" => Box::new(passes::cleanup_attrs::CleanupAttrs),
        "inlineStyles" => Box::new(passes::inline_styles::InlineStyles),
        "cleanupNumericValues" => {
            Box::new(passes::cleanup_numeric_values::CleanupNumericValues { precision })
        }
        "convertColors" => Box::new(passes::convert_colors::ConvertColors),
        "removeUnknownsAndDefaults" => {
            Box::new(passes::remove_unknowns_and_defaults::RemoveUnknownsAndDefaults)
        }
        "convertShapeToPath" => {
            Box::new(passes::convert_shape_to_path::ConvertShapeToPath { precision })
        }
        "convertTransform" => Box::new(passes::convert_transform::ConvertTransform { precision }),
        "collapseGroups" => Box::new(passes::collapse_groups::CollapseGroups),
        "cleanupIds" => Box::new(passes::cleanup_ids::CleanupIds),
        "convertPathData" => Box::new(passes::convert_path_data::ConvertPathData { precision }),
        "mergePaths" => Box::new(passes::merge_paths::MergePaths),
        "sortAttrs" => Box::new(passes::sort_attrs::SortAttrs),
        "minifyStyles" => Box::new(passes::minify_styles::MinifyStyles),
        "minifyWhitespace" => Box::new(passes::minify_whitespace::MinifyWhitespace),
        "removeDesc" => Box::new(passes::remove_desc::RemoveDesc),
        _ => panic!("unknown pass: {name}"),
    }
}

/// Build the pass list for a given configuration.
pub fn passes_for_config(config: &Config) -> Vec<Box<dyn Pass>> {
    let precision = config.effective_precision();
    let mut result = Vec::new();

    for entry in PASS_CATALOG {
        let name = entry.0;
        let enabled = if let Some(&override_val) = config.pass_overrides.get(name) {
            override_val
        } else {
            is_in_preset(entry, config.preset)
        };

        if enabled {
            result.push(create_pass(name, precision));
        }
    }

    result
}

/// Returns the list of all known pass names in execution order.
pub fn all_pass_names() -> Vec<&'static str> {
    PASS_CATALOG.iter().map(|e| e.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pass_names(config: &Config) -> Vec<&'static str> {
        passes_for_config(config).iter().map(|p| p.name()).collect()
    }

    #[test]
    fn safe_preset_passes() {
        let config = Config {
            preset: Preset::Safe,
            ..Config::default()
        };
        let names = pass_names(&config);
        assert_eq!(
            names,
            vec![
                "removeDoctype",
                "removeProcInst",
                "removeComments",
                "removeMetadata",
                "removeEditorData",
                "removeEmptyAttrs",
                "removeEmptyText",
                "removeEmptyContainers",
                "removeHiddenElems",
                "removeUnusedNamespaces",
                "cleanupAttrs",
                "cleanupNumericValues",
                "convertColors",
                "removeUnknownsAndDefaults",
                "sortAttrs",
                "minifyStyles",
                "minifyWhitespace",
            ]
        );
        assert_eq!(names.len(), 17);
    }

    #[test]
    fn balanced_preset_passes() {
        let config = Config::default();
        assert_eq!(config.preset, Preset::Balanced);
        let names = pass_names(&config);
        assert_eq!(
            names,
            vec![
                "removeDoctype",
                "removeProcInst",
                "removeComments",
                "removeMetadata",
                "removeEditorData",
                "removeEmptyAttrs",
                "removeEmptyText",
                "removeEmptyContainers",
                "removeHiddenElems",
                "removeUnusedNamespaces",
                "cleanupAttrs",
                "inlineStyles",
                "cleanupNumericValues",
                "convertColors",
                "removeUnknownsAndDefaults",
                "convertShapeToPath",
                "convertTransform",
                "collapseGroups",
                "cleanupIds",
                "convertPathData",
                "mergePaths",
                "sortAttrs",
                "minifyStyles",
                "minifyWhitespace",
            ]
        );
        assert_eq!(names.len(), 24);
    }

    #[test]
    fn aggressive_preset_passes() {
        let config = Config {
            preset: Preset::Aggressive,
            ..Config::default()
        };
        let names = pass_names(&config);
        // Same passes as balanced
        assert_eq!(names.len(), 24);
        assert_eq!(names, pass_names(&Config::default()));
    }

    #[test]
    fn remove_desc_not_in_any_preset() {
        for preset in [Preset::Safe, Preset::Balanced, Preset::Aggressive] {
            let config = Config {
                preset,
                ..Config::default()
            };
            let names = pass_names(&config);
            assert!(
                !names.contains(&"removeDesc"),
                "removeDesc should not be in {preset:?}"
            );
        }
    }

    #[test]
    fn override_enables_opt_in_pass() {
        let config = Config {
            preset: Preset::Safe,
            pass_overrides: HashMap::from([("removeDesc".to_string(), true)]),
            ..Config::default()
        };
        let names = pass_names(&config);
        assert!(names.contains(&"removeDesc"));
        // Should be at the end (catalog order)
        assert_eq!(names.last(), Some(&"removeDesc"));
    }

    #[test]
    fn override_disables_preset_pass() {
        let config = Config {
            preset: Preset::Balanced,
            pass_overrides: HashMap::from([("collapseGroups".to_string(), false)]),
            ..Config::default()
        };
        let names = pass_names(&config);
        assert!(!names.contains(&"collapseGroups"));
    }

    #[test]
    fn effective_precision_defaults() {
        assert_eq!(
            Config {
                preset: Preset::Safe,
                ..Config::default()
            }
            .effective_precision(),
            3
        );
        assert_eq!(Config::default().effective_precision(), 3);
        assert_eq!(
            Config {
                preset: Preset::Aggressive,
                ..Config::default()
            }
            .effective_precision(),
            2
        );
    }

    #[test]
    fn explicit_precision_overrides_preset() {
        let config = Config {
            preset: Preset::Aggressive,
            precision: Some(4),
            ..Config::default()
        };
        assert_eq!(config.effective_precision(), 4);
    }

    #[test]
    fn balanced_matches_old_default_passes() {
        // Balanced with default config must produce the exact same pass list
        // as the old hardcoded default_passes() — same names, same order.
        let config = Config::default();
        let new_names: Vec<&str> = passes_for_config(&config)
            .iter()
            .map(|p| p.name())
            .collect();

        let old_names: Vec<&str> = vec![
            "removeDoctype",
            "removeProcInst",
            "removeComments",
            "removeMetadata",
            "removeEditorData",
            "removeEmptyAttrs",
            "removeEmptyText",
            "removeEmptyContainers",
            "removeHiddenElems",
            "removeUnusedNamespaces",
            "cleanupAttrs",
            "inlineStyles",
            "cleanupNumericValues",
            "convertColors",
            "removeUnknownsAndDefaults",
            "convertShapeToPath",
            "convertTransform",
            "collapseGroups",
            "cleanupIds",
            "convertPathData",
            "mergePaths",
            "sortAttrs",
            "minifyStyles",
            "minifyWhitespace",
        ];

        assert_eq!(new_names, old_names);
    }
}
