use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    Text,
    Markdown,
    Rust,
    Python,
    Json,
    Toml,
    Yaml,
}

#[derive(Debug, Clone)]
pub struct FileData {
    pub file_type: FileType,
    pub text: String,
    pub path: String,
}

impl FileData {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file_path = path.as_ref();
        let file_type = detect_file_type(file_path)?;
        let file_text = fs::read_to_string(file_path)?;
        Ok(Self {
            file_type,
            text: file_text,
            path: file_path.to_string_lossy().into_owned(),
        })
    }
}
fn detect_file_type(path: &Path) -> Result<FileType, io::Error> {
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase());

    match extension.as_deref() {
        Some("txt") => Ok(FileType::Text),
        Some("md") => Ok(FileType::Markdown),
        Some("rs") => Ok(FileType::Rust),
        Some("py") => Ok(FileType::Python),
        Some("json") => Ok(FileType::Json),
        Some("toml") => Ok(FileType::Toml),
        Some("yml") | Some("yaml") => Ok(FileType::Yaml),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unsupported file type",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_data_from_file() {
        let file_data = super::FileData::from_file("src/document.rs").expect("Failed to read file");
        assert_eq!(file_data.file_type, super::FileType::Rust);
        assert!(file_data.text.contains("pub struct FileData"));
        assert_eq!(file_data.path, "src/document.rs");
    }
}
