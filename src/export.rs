use crate::chunk::Chunk;
use crate::error::ExportError;

use std::fs::File;
use std::io::Write;
use std::path::Path;
pub fn export_chunks_to_jsonl(chunks: &[Chunk], output_path: &Path) -> Result<(), ExportError> {
    let output_path = output_path.with_extension("jsonl");
    let mut file = File::create(&output_path).map_err(|_| ExportError::WriteFailed)?;

    for chunk in chunks {
        let json_line = serde_json::to_string(chunk).map_err(|_| ExportError::SerializeFailed)?;
        writeln!(file, "{}", json_line).map_err(|_| ExportError::WriteFailed)?;
    }
    Ok(())
}
