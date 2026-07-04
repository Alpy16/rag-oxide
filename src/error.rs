#[derive(Debug, PartialEq, Eq)]
pub enum FileError {
    MissingExtension,
    UnsupportedExtension,
    InvalidPath,
    EmptyFile,
    ReadFailed,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChunkError {
    InvalidChunkSize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExportError {
    WriteFailed,
    SerializeFailed,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IngestError {
    File(FileError),
    Chunk(ChunkError),
    Export(ExportError),
}
