use crate::document::FileData;
use crate::error::ChunkError;
use serde::Serialize;
use std::path::PathBuf;
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Chunk {
    pub chunk_id: u64,
    pub source_path: PathBuf,
    pub text: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_line: usize,
    pub end_line: usize,
}

pub fn from_file_data(
    file_data: &FileData,
    chunk_size_lines: usize,
    starting_chunk_id: u64,
) -> Result<Vec<Chunk>, ChunkError> {
    if chunk_size_lines < 1 {
        return Err(ChunkError::InvalidChunkSize);
    }
    let mut current_chunk_text = String::new();
    let mut current_start_line = 1;
    let mut current_line_count = 0;
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut next_chunk_id = starting_chunk_id;
    let mut current_byte_offset = 0;
    let mut current_start_byte = 0;
    for (line_number, line_with_newline) in file_data.text.split_inclusive('\n').enumerate() {
        let actual_line_number = line_number + 1;

        current_chunk_text.push_str(line_with_newline);

        current_line_count += 1;
        current_byte_offset += line_with_newline.len();

        if current_line_count == chunk_size_lines {
            chunks.push(Chunk {
                chunk_id: next_chunk_id,
                source_path: file_data.path.clone(),
                text: std::mem::take(&mut current_chunk_text),
                start_byte: current_start_byte,
                end_byte: current_byte_offset,
                start_line: current_start_line,
                end_line: actual_line_number,
            });
            next_chunk_id += 1;
            current_line_count = 0;
            current_start_line = actual_line_number + 1;
            current_start_byte = current_byte_offset;
        }
    }
    if current_line_count > 0 {
        chunks.push(Chunk {
            chunk_id: next_chunk_id,
            source_path: file_data.path.clone(),
            text: std::mem::take(&mut current_chunk_text),
            start_byte: current_start_byte,
            end_byte: current_byte_offset,
            start_line: current_start_line,
            end_line: current_start_line + current_line_count - 1,
        });
    }
    Ok(chunks)
}
