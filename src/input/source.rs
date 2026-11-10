use crate::cli;
use crate::input::MAIN_SEPARATOR;
use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// The direction used to order entries in a sort source.
#[derive(PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Where the files to rename come from.
pub enum Source {
    Regex(Regex, usize, Option<usize>),
    Map(Vec<(String, String)>),
    Sort(SortOrder),
}

impl Source {
    /// Build a regex source from a pattern and optional depth bounds.
    pub fn new_regex(
        pattern: &str,
        depth: Option<usize>,
        max_depth: Option<usize>,
    ) -> Result<Self> {
        Ok(Self::Regex(
            Regex::new(pattern)?,
            depth.unwrap_or(pattern.matches(MAIN_SEPARATOR).count() + 1),
            max_depth,
        ))
    }

    /// Load a precomputed `{output: input}` map from a JSON file.
    pub fn new_map(filename: &Path) -> Result<Self> {
        let contents = fs::read_to_string(filename)?;
        Ok(Self::Map(serde_json::from_str(contents.as_str()).map(
            |map: HashMap<String, String>| map.into_iter().collect(),
        )?))
    }

    /// Build a sort source from the requested ordering.
    pub fn new_sort(order: cli::Order) -> Result<Self> {
        Ok(Self::Sort(match order {
            cli::Order::Asc => SortOrder::Asc,
            cli::Order::Desc => SortOrder::Desc,
        }))
    }
}
