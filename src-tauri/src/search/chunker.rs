use std::path::Path;

use ignore::WalkBuilder;

use super::types::ChunkInfo;

/// Target number of lines per chunk.
const TARGET_CHUNK_LINES: usize = 50;

/// Tolerance window for finding a blank-line split point near the target.
const SPLIT_TOLERANCE: usize = 10;

/// Maximum file size in bytes (1 MB). Files larger than this are skipped.
const MAX_FILE_SIZE: u64 = 1_048_576;

/// File extensions to skip (binary and non-text files).
const BINARY_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "woff", "woff2", "ttf", "eot", "otf",
    "mp3", "mp4", "avi", "mov", "zip", "tar", "gz", "rar", "7z", "exe", "dll", "so", "dylib",
    "wasm", "pdf", "lock",
];

/// Error type for chunking operations.
#[derive(Debug, thiserror::Error)]
pub enum ChunkError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("walk error: {0}")]
    Walk(String),
}

/// Walk a codebase rooted at `root`, respecting `.gitignore`, and split
/// every eligible text file into chunks of approximately `TARGET_CHUNK_LINES` lines.
///
/// `excluded_paths` contains path prefixes (relative to root) that should be skipped.
pub fn chunk_codebase(root: &Path, excluded_paths: &[String]) -> Result<Vec<ChunkInfo>, ChunkError> {
    let mut chunks = Vec::new();

    let walker = WalkBuilder::new(root)
        .hidden(true) // respect hidden-file rules
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry_result in walker {
        let entry = entry_result.map_err(|e| ChunkError::Walk(e.to_string()))?;

        // Skip directories
        if entry.file_type().is_none_or(|ft| !ft.is_file()) {
            continue;
        }

        let abs_path = entry.path();

        // Compute relative path for storage and filtering
        let rel_path = abs_path
            .strip_prefix(root)
            .unwrap_or(abs_path)
            .to_string_lossy()
            .replace('\\', "/");

        // Check excluded paths
        if excluded_paths
            .iter()
            .any(|excluded| rel_path.starts_with(excluded.as_str()))
        {
            continue;
        }

        // Skip binary files by extension
        if is_binary_extension(abs_path) {
            continue;
        }

        // Skip files larger than MAX_FILE_SIZE
        let metadata = std::fs::metadata(abs_path)?;
        if metadata.len() > MAX_FILE_SIZE {
            continue;
        }

        // Read file content; skip files that are not valid UTF-8
        let content = match std::fs::read_to_string(abs_path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::InvalidData => continue,
            Err(e) => return Err(ChunkError::Io(e)),
        };

        if content.is_empty() {
            continue;
        }

        let language = detect_language(abs_path);
        let file_chunks = split_into_chunks(&content, &rel_path, language.as_deref());
        chunks.extend(file_chunks);
    }

    Ok(chunks)
}

/// Check whether a file's extension is in the binary skip list.
fn is_binary_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| BINARY_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Detect the programming language from a file's extension.
fn detect_language(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "rs" => Some("rust".to_string()),
        "ts" | "tsx" => Some("typescript".to_string()),
        "js" | "jsx" => Some("javascript".to_string()),
        "svelte" => Some("svelte".to_string()),
        "py" => Some("python".to_string()),
        "go" => Some("go".to_string()),
        "toml" => Some("toml".to_string()),
        "json" => Some("json".to_string()),
        "yaml" | "yml" => Some("yaml".to_string()),
        "md" => Some("markdown".to_string()),
        "html" => Some("html".to_string()),
        "css" => Some("css".to_string()),
        "sql" => Some("sql".to_string()),
        "sh" | "bash" => Some("shell".to_string()),
        _ => None,
    }
}

/// Split file content into chunks of approximately `TARGET_CHUNK_LINES` lines,
/// preferring to split at blank-line boundaries.
fn split_into_chunks(content: &str, file_path: &str, language: Option<&str>) -> Vec<ChunkInfo> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Vec::new();
    }

    let mut chunks = Vec::new();
    let mut start = 0;

    while start < lines.len() {
        let ideal_end = (start + TARGET_CHUNK_LINES).min(lines.len());

        let end = if ideal_end >= lines.len() {
            // Last chunk — take everything remaining
            lines.len()
        } else {
            // Try to find a blank line within the tolerance window
            find_blank_line_boundary(&lines, ideal_end)
        };

        let chunk_content = lines[start..end].join("\n");

        chunks.push(ChunkInfo {
            file_path: file_path.to_string(),
            start_line: (start + 1) as u32,     // 1-indexed
            end_line: end as u32,                 // inclusive of last line
            content: chunk_content,
            language: language.map(String::from),
        });

        start = end;
    }

    chunks
}

/// Search for a blank-line split point near `target_line`.
///
/// Scans within `+/- SPLIT_TOLERANCE` lines of the target and returns the
/// position of the first blank line found (searching outward from the target).
/// If no blank line is found, returns the target.
fn find_blank_line_boundary(lines: &[&str], target: usize) -> usize {
    let lower = target.saturating_sub(SPLIT_TOLERANCE);
    let upper = (target + SPLIT_TOLERANCE).min(lines.len());

    // Search outward from the target for a blank line
    for offset in 0..=SPLIT_TOLERANCE {
        // Check after target first
        let after = target + offset;
        if after < upper && lines[after].trim().is_empty() {
            return after + 1; // split after the blank line
        }

        // Then check before target
        if offset <= target {
            let before = target - offset;
            if before >= lower && lines[before].trim().is_empty() {
                return before + 1; // split after the blank line
            }
        }
    }

    // No blank line found — split at the target
    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_language_known_extensions() {
        let cases = vec![
            ("foo.rs", Some("rust")),
            ("bar.ts", Some("typescript")),
            ("baz.py", Some("python")),
            ("qux.go", Some("go")),
            ("file.svelte", Some("svelte")),
            ("unknown.xyz", None),
        ];

        for (filename, expected) in cases {
            let path = Path::new(filename);
            let result = detect_language(path);
            assert_eq!(
                result.as_deref(),
                expected,
                "detect_language({filename})"
            );
        }
    }

    #[test]
    fn is_binary_extension_detects_binaries() {
        assert!(is_binary_extension(Path::new("image.png")));
        assert!(is_binary_extension(Path::new("font.woff2")));
        assert!(is_binary_extension(Path::new("archive.zip")));
        assert!(!is_binary_extension(Path::new("code.rs")));
        assert!(!is_binary_extension(Path::new("readme.md")));
    }

    #[test]
    fn split_short_file_produces_single_chunk() {
        let content = "line 1\nline 2\nline 3";
        let chunks = split_into_chunks(content, "test.rs", Some("rust"));
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].start_line, 1);
        assert_eq!(chunks[0].end_line, 3);
        assert_eq!(chunks[0].language.as_deref(), Some("rust"));
    }

    #[test]
    fn split_long_file_produces_multiple_chunks() {
        let lines: Vec<String> = (1..=120).map(|i| format!("line {i}")).collect();
        let content = lines.join("\n");
        let chunks = split_into_chunks(&content, "big.rs", Some("rust"));
        assert!(chunks.len() >= 2, "expected at least 2 chunks, got {}", chunks.len());
        // Verify no gaps — each chunk starts where the previous ended
        for pair in chunks.windows(2) {
            assert_eq!(pair[0].end_line + 1, pair[1].start_line);
        }
    }

    #[test]
    fn split_prefers_blank_line_boundary() {
        // Create content with a blank line at line 48 (0-indexed: index 47)
        let mut lines: Vec<String> = (1..=100).map(|i| format!("line {i}")).collect();
        lines[47] = String::new(); // blank line at position 48

        let content = lines.join("\n");
        let chunks = split_into_chunks(&content, "test.rs", None);

        // The first chunk should split at or near the blank line
        assert!(
            chunks[0].end_line >= 45 && chunks[0].end_line <= 55,
            "expected first chunk to end near line 48, got {}",
            chunks[0].end_line
        );
    }

    #[test]
    fn empty_content_produces_no_chunks() {
        let chunks = split_into_chunks("", "empty.rs", None);
        assert!(chunks.is_empty());
    }
}
