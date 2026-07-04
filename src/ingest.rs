use crate::chunk::from_file_data;
use crate::document::FileData;
use crate::error::IngestError;
use crate::export::export_chunks_to_jsonl;

use std::path::Path;

pub fn ingest_file(
    input_path: &Path,
    output_path: &Path,
    chunk_size_lines: usize,
) -> Result<(), IngestError> {
    let file_data = FileData::from_file(input_path).map_err(IngestError::File)?;

    let chunks = from_file_data(&file_data, chunk_size_lines, 1).map_err(IngestError::Chunk)?;

    export_chunks_to_jsonl(&chunks, output_path).map_err(IngestError::Export)?;

    Ok(())
}
