use std::{
    io,
    path::{Path, PathBuf},
};

use grep::{
    matcher::Matcher,
    regex::RegexMatcherBuilder,
    searcher::{Searcher, SearcherBuilder, Sink, SinkMatch},
};
use ignore::Walk;

#[derive(Debug)]
struct SingleMatchSink {
    has_match: bool,
}

impl SingleMatchSink {
    fn new() -> Self {
        Self { has_match: false }
    }
    fn has_match(&self) -> bool {
        self.has_match
    }
}

impl Sink for SingleMatchSink {
    type Error = io::Error;

    fn matched(
        &mut self,
        _searcher: &Searcher,
        _match: &SinkMatch<'_>,
    ) -> Result<bool, Self::Error> {
        self.has_match = true;
        Ok(false)
    }
}

pub struct SearchResult {
    path: PathBuf,
}

impl SearchResult {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn file_stem(&self) -> Option<&str> {
        self.path.file_stem()?.to_str()
    }

    pub fn path(self) -> PathBuf {
        self.path
    }
}

pub fn search_for_files<P>(from_directory: P, pattern: &str) -> anyhow::Result<Vec<SearchResult>>
where
    P: AsRef<Path>,
{
    let matcher = RegexMatcherBuilder::new().case_smart(true).build(pattern)?;
    let mut searcher = SearcherBuilder::new().build();
    let walker = Walk::new(from_directory);
    let matched_paths = walker
        .filter_map(|result| {
            let entry = result.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }
            if let Some(stem) = path.file_stem() {
                if let Some(str) = stem.to_str() {
                    if let Ok(true) = matcher.is_match(str.as_bytes()) {
                        return Some(SearchResult::new(path.to_path_buf()));
                    }
                }
            }
            let mut sink = SingleMatchSink::new();
            searcher.search_path(&matcher, path, &mut sink).ok()?;
            if sink.has_match() {
                Some(SearchResult::new(path.to_path_buf()))
            } else {
                None
            }
        })
        .collect();
    Ok(matched_paths)
}
