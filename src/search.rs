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
use log::error;

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
    fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            path: path.as_ref().to_owned(),
        }
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
            let entry = match result {
                Ok(entry) => entry,
                Err(error) => {
                    error!("Failed to walk directory entry: {}", error);
                    return None;
                }
            };
            let path = entry.path();
            if path.is_file() {
                if let Some(stem) = path.file_stem() {
                    if let Some(str) = stem.to_str() {
                        if let Ok(true) = matcher.is_match(str.as_bytes()) {
                            return Some(SearchResult::new(path));
                        }
                    }
                }
                let mut sink = SingleMatchSink::new();
                match searcher.search_path(&matcher, path, &mut sink) {
                    Ok(_) => {
                        if sink.has_match() {
                            Some(SearchResult::new(path))
                        } else {
                            None
                        }
                    }
                    Err(error) => {
                        error!("Failed to search in path '{:?}': {}", path, error);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect();
    Ok(matched_paths)
}
