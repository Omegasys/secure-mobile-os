use std::path::PathBuf;

#[derive(Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub relevance_score: f32,
}

pub struct SearchEngine;

impl SearchEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn search(
        &self,
        query: &str,
    ) -> Vec<SearchResult> {
        println!(
            "Searching for '{}'",
            query
        );

        vec![
            SearchResult {
                path: PathBuf::from(
                    "/documents/report.txt",
                ),
                relevance_score: 0.95,
            },
            SearchResult {
                path: PathBuf::from(
                    "/downloads/report_backup.txt",
                ),
                relevance_score: 0.75,
            },
        ]
    }

    pub fn search_by_extension(
        &self,
        extension: &str,
    ) -> Vec<SearchResult> {
        println!(
            "Searching for *.{} files",
            extension
        );

        Vec::new()
    }
}
