use crate::document::FileData;
use std::io;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub chunk_id: u64,       // global chunk identity
    pub source_path: String, // original file path
    pub text: String,        // exact chunk content
    pub start_byte: usize,   // where the chunk begins in the source text
    pub end_byte: usize,     // where it ends
    pub start_line: usize,   // first line number in the chunk
    pub end_line: usize,     // last line number in the chunk
}

impl Chunk {
    pub fn chunk_from_file_data(
        file_data: &FileData,
        chunk_size_lines: usize,
        starting_chunk_id: u64,
    ) -> Result<Vec<Chunk>, io::Error> {
        if chunk_size_lines == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "chunk_size_lines cannot be zero",
            ));
        }
        let mut chunks = Vec::new();
        let mut current_chunk_id = starting_chunk_id;
        let mut current_lines = Vec::new();
        let mut chunk_start_line: usize = 1;
        let mut current_line_number: usize = 1;
        let mut chunk_start_byte: usize = 0;
        let mut lines_in_current_chunk: usize = 0;
        for line in file_data.text.lines() {
            current_lines.push(line);
            lines_in_current_chunk += 1;
            if lines_in_current_chunk == chunk_size_lines {
                let chunk_text = current_lines.join("\n");
                let chunk_end_byte = chunk_start_byte + chunk_text.len();
                let chunk_end_line = chunk_start_line + lines_in_current_chunk - 1;
                chunks.push(Chunk {
                    chunk_id: current_chunk_id,
                    source_path: file_data.path.clone(),
                    text: chunk_text,
                    start_byte: chunk_start_byte,
                    end_byte: chunk_end_byte,
                    start_line: chunk_start_line,
                    end_line: chunk_end_line,
                });
                current_chunk_id += 1;
                current_lines.clear();
                chunk_start_line = current_line_number + 1;
                chunk_start_byte = chunk_end_byte + 1; // +1 for the newline, however this assumes exactly one newline character between lines
                lines_in_current_chunk = 0;
            }
            current_line_number += 1;
        }

        if !current_lines.is_empty() {
            let chunk_text = current_lines.join("\n");
            let chunk_end_byte = chunk_start_byte + chunk_text.len();
            let chunk_end_line = chunk_start_line + lines_in_current_chunk - 1;

            chunks.push(Chunk {
                chunk_id: current_chunk_id,
                source_path: file_data.path.clone(),
                text: chunk_text,
                start_byte: chunk_start_byte,
                end_byte: chunk_end_byte,
                start_line: chunk_start_line,
                end_line: chunk_end_line,
            });
        }

        Ok(chunks)
    }
}
