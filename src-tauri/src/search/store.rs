use std::path::{Path, PathBuf};

use duckdb::{params, Connection};
use regex::Regex;

use super::types::{ChunkInfo, IndexStatus, SearchResult};

/// Error type for search store operations.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("DuckDB error: {0}")]
    DuckDb(#[from] duckdb::Error),

    #[error("invalid regex pattern: {0}")]
    InvalidRegex(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Manages the DuckDB database for storing and querying code chunks.
pub struct SearchStore {
    conn: Connection,
    index_path: PathBuf,
}

impl SearchStore {
    /// Open or create a DuckDB database at the given path and ensure the schema exists.
    pub fn new(path: &Path) -> Result<Self, StoreError> {
        let conn = Connection::open(path)?;
        let store = Self {
            conn,
            index_path: path.to_path_buf(),
        };
        store.ensure_schema()?;
        Ok(store)
    }

    /// Create the chunks table and index if they do not already exist.
    fn ensure_schema(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE SEQUENCE IF NOT EXISTS seq_chunk_id START 1;
             CREATE TABLE IF NOT EXISTS chunks (
                 id INTEGER DEFAULT nextval('seq_chunk_id') PRIMARY KEY,
                 file_path TEXT NOT NULL,
                 start_line INTEGER NOT NULL,
                 end_line INTEGER NOT NULL,
                 content TEXT NOT NULL,
                 language TEXT,
                 embedding FLOAT[384]
             );
             CREATE INDEX IF NOT EXISTS idx_chunks_file ON chunks(file_path);",
        )?;
        Ok(())
    }

    /// Drop and recreate the chunks table for a fresh index.
    pub fn clear(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "DROP TABLE IF EXISTS chunks;
             DROP SEQUENCE IF EXISTS seq_chunk_id;",
        )?;
        self.ensure_schema()?;
        Ok(())
    }

    /// Batch-insert chunks into the database using prepared statements.
    pub fn insert_chunks(&self, chunks: &[ChunkInfo]) -> Result<(), StoreError> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO chunks (file_path, start_line, end_line, content, language) \
             VALUES (?, ?, ?, ?, ?)",
        )?;

        for chunk in chunks {
            stmt.execute(params![
                chunk.file_path.as_str(),
                chunk.start_line as i32,
                chunk.end_line as i32,
                chunk.content.as_str(),
                chunk.language.as_deref(),
            ])?;
        }

        Ok(())
    }

    /// Search chunks using a Rust regex pattern.
    ///
    /// Fetches chunks from DuckDB (optionally filtered by file path prefix),
    /// then applies the regex in Rust for portable, predictable matching.
    pub fn search_regex(
        &self,
        pattern: &str,
        path_filter: Option<&str>,
        max_results: u32,
    ) -> Result<Vec<SearchResult>, StoreError> {
        let re = Regex::new(pattern).map_err(|e| StoreError::InvalidRegex(e.to_string()))?;

        let (sql, has_path_filter) = if path_filter.is_some() {
            (
                "SELECT file_path, start_line, end_line, content, language \
                 FROM chunks WHERE file_path LIKE ? || '%' \
                 ORDER BY file_path, start_line",
                true,
            )
        } else {
            (
                "SELECT file_path, start_line, end_line, content, language \
                 FROM chunks ORDER BY file_path, start_line",
                false,
            )
        };

        let mut stmt = self.conn.prepare(sql)?;

        let rows = if has_path_filter {
            stmt.query_map(params![path_filter], map_chunk_row)?
        } else {
            stmt.query_map([], map_chunk_row)?
        };

        let mut results = Vec::new();

        for row_result in rows {
            let (file_path, start_line, end_line, content, language): (
                String,
                i32,
                i32,
                String,
                Option<String>,
            ) = row_result?;

            if let Some(mat) = re.find(&content) {
                // Extract a context line around the first match
                let match_context = extract_match_context(&content, mat.start(), mat.end());

                // Score: count of matches in the chunk
                let match_count = re.find_iter(&content).count();

                results.push(SearchResult {
                    file_path,
                    start_line: start_line as u32,
                    end_line: end_line as u32,
                    content: content.clone(),
                    language,
                    score: match_count as f64,
                    match_context,
                });

                if results.len() >= max_results as usize {
                    break;
                }
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(results)
    }

    /// Get the current status of the search index.
    pub fn get_status(&self) -> Result<IndexStatus, StoreError> {
        let chunk_count: u32 = self
            .conn
            .query_row("SELECT COUNT(*) FROM chunks", [], |row| row.get(0))
            .map(|c: i64| c as u32)?;

        let embedding_count: u32 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM chunks WHERE embedding IS NOT NULL",
                [],
                |row| row.get(0),
            )
            .map(|c: i64| c as u32)?;

        Ok(IndexStatus {
            is_indexed: chunk_count > 0,
            chunk_count,
            has_embeddings: embedding_count > 0,
            last_indexed: None,
            index_path: self.index_path.to_string_lossy().to_string(),
        })
    }
}

/// Map a DuckDB row to chunk field tuple.
fn map_chunk_row(
    row: &duckdb::Row<'_>,
) -> duckdb::Result<(String, i32, i32, String, Option<String>)> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
    ))
}

/// Extract a short context string around a regex match within the chunk content.
fn extract_match_context(content: &str, match_start: usize, _match_end: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();

    // Find which line contains the match start
    let mut byte_offset = 0;
    let mut match_line_idx = 0;
    for (i, line) in lines.iter().enumerate() {
        let line_end = byte_offset + line.len() + 1; // +1 for newline
        if match_start < line_end {
            match_line_idx = i;
            break;
        }
        byte_offset = line_end;
    }

    // Return the matching line and one line of context on each side
    let start = if match_line_idx > 0 {
        match_line_idx - 1
    } else {
        0
    };
    let end = (match_line_idx + 2).min(lines.len());

    lines[start..end].join("\n")
}
