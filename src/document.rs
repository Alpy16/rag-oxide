use std::path::{Path, PathBuf};

use crate::error::FileError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    Json,
    Text,
    Yaml,
    Markdown,
    Html,
    Rtf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileData {
    pub file_type: FileType,
    pub text: String,
    pub path: PathBuf,
}

impl FileData {
    pub fn detect_filetype(path: &Path) -> Result<FileType, FileError> {
        let extension = path
            .extension()
            .ok_or(FileError::MissingExtension)?
            .to_str()
            .ok_or(FileError::InvalidPath)?
            .to_lowercase();

        let file_type = match extension.as_str() {
            "md" => FileType::Markdown,
            "json" => FileType::Json,
            "yaml" => FileType::Yaml,
            "yml" => FileType::Yaml,
            "txt" => FileType::Text,
            "rtf" => FileType::Rtf,
            "html" => FileType::Html,
            _ => return Err(FileError::UnsupportedExtension),
        };

        Ok(file_type)
    }

    pub fn from_file(path: &Path) -> Result<FileData, FileError> {
        let file_type = Self::detect_filetype(path)?;

        let text = std::fs::read_to_string(path).map_err(|_| FileError::ReadFailed)?;

        if text.is_empty() {
            return Err(FileError::EmptyFile);
        }

        Ok(FileData {
            file_type,
            text,
            path: path.to_path_buf(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_markdown_filetype() {
        let path = Path::new("notes.md");

        let result = FileData::detect_filetype(path);

        assert_eq!(result.unwrap(), FileType::Markdown);
    }

    #[test]
    fn detects_yaml_filetype() {
        let path = Path::new("text.yaml");
        let result = FileData::detect_filetype(path);
        assert_eq!(result.unwrap(), FileType::Yaml);
    }
}
