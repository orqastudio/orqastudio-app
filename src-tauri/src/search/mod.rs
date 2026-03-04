pub mod chunker;
pub mod store;
pub mod types;

use std::path::Path;

use store::SearchStore;
use types::{IndexStatus, SearchResult};

/// The main search engine that coordinates indexing and searching.
pub struct SearchEngine {
    store: SearchStore,
    project_root: Option<std::path::PathBuf>,
}

impl SearchEngine {
    /// Create a new search engine backed by a DuckDB database at `db_path`.
    pub fn new(db_path: &Path) -> Result<Self, String> {
        let store = SearchStore::new(db_path).map_err(|e| e.to_string())?;
        Ok(Self {
            store,
            project_root: None,
        })
    }

    /// Index a codebase rooted at `root`, storing chunks in DuckDB.
    ///
    /// This clears any existing index before re-indexing.
    pub fn index(
        &mut self,
        root: &Path,
        excluded_paths: &[String],
    ) -> Result<IndexStatus, String> {
        self.project_root = Some(root.to_path_buf());
        let chunks =
            chunker::chunk_codebase(root, excluded_paths).map_err(|e| e.to_string())?;
        self.store.clear().map_err(|e| e.to_string())?;
        self.store
            .insert_chunks(&chunks)
            .map_err(|e| e.to_string())?;
        self.store.get_status().map_err(|e| e.to_string())
    }

    /// Search the indexed codebase with a regex pattern.
    pub fn search_regex(
        &self,
        pattern: &str,
        path_filter: Option<&str>,
        max_results: u32,
    ) -> Result<Vec<SearchResult>, String> {
        self.store
            .search_regex(pattern, path_filter, max_results)
            .map_err(|e| e.to_string())
    }

    /// Get the current status of the search index.
    pub fn get_status(&self) -> Result<IndexStatus, String> {
        self.store.get_status().map_err(|e| e.to_string())
    }
}
