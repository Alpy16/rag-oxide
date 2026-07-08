# RAG-Oxide

RAG-Oxide is a small learning project for building a document ingestion and retrieval pipeline.

The project is split into two parts:

```text
Rust   → document ingestion, chunking, JSONL export
Python → embeddings, saved vector index, similarity search
```

The goal is to understand the full RAG retrieval path from first principles before adding larger frameworks or LLM answer generation.

## Current Status

The first end-to-end prototype works.

```text
input.txt
   ↓
Rust ingestion pipeline
   ↓
output.jsonl
   ↓
Python embedding pipeline
   ↓
embeddings.npy
   ↓
Python search pipeline
   ↓
ranked chunks
```

The project currently supports:

* Loading a single text-like document
* Detecting supported file types
* Reading file contents into memory
* Splitting documents into fixed-size line chunks
* Preserving chunk metadata
* Exporting chunks as JSONL
* Embedding chunks with `sentence-transformers`
* Saving embeddings to `embeddings.npy`
* Searching chunks by vector similarity

## Project Structure

```text
rag-oxide/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── error.rs
│   ├── document.rs
│   ├── chunk.rs
│   ├── export.rs
│   └── ingest.rs
├── python/
│   ├── embed.py
│   ├── search.py
│   └── requirements.txt
├── input.txt
├── output.jsonl
├── embeddings.npy
└── README.md
```

## Rust Modules

### `document.rs`

Handles document loading.

```text
Path → FileData
```

Responsibilities:

* Detect file extension
* Map extension to `FileType`
* Read file text
* Reject empty files
* Preserve source path

### `chunk.rs`

Handles chunk creation.

```text
FileData → Vec<Chunk>
```

Responsibilities:

* Split text into fixed-size line chunks
* Assign chunk IDs
* Preserve source path
* Track line ranges
* Track byte ranges

### `export.rs`

Handles JSONL export.

```text
Vec<Chunk> → output.jsonl
```

Responsibilities:

* Serialize each chunk as JSON
* Write one JSON object per line
* Produce data that Python can read

### `ingest.rs`

Coordinates the Rust pipeline.

```text
input_path → FileData → chunks → output.jsonl
```

Responsibilities:

* Call document loading
* Call chunking
* Call export
* Convert lower-level errors into `IngestError`

### `main.rs`

Temporary entry point with hardcoded paths.

Current flow:

```text
input.txt → output.jsonl
```

## Python Modules

### `embed.py`

Creates the vector index.

```text
output.jsonl → embeddings.npy
```

Responsibilities:

* Load chunks from JSONL
* Extract chunk text
* Load embedding model
* Encode chunk text into vectors
* Save vectors to disk

### `search.py`

Runs semantic search.

```text
query + output.jsonl + embeddings.npy → ranked chunks
```

Responsibilities:

* Load chunks
* Load saved embeddings
* Embed user query
* Compute similarity scores
* Return top matching chunks

## Setup

Create and activate a Python virtual environment:

```bash
python3 -m venv .venv
source .venv/bin/activate
```

Install Python dependencies:

```bash
pip install -r python/requirements.txt
```

If `venv` is missing on Ubuntu/WSL:

```bash
sudo apt update
sudo apt install python3.12-venv
```

## Usage

### 1. Generate chunks with Rust

Put text into `input.txt`, then run:

```bash
cargo run
```

This creates:

```text
output.jsonl
```

Example JSONL output:

```json
{"chunk_id":1,"source_path":"input.txt","text":"...","start_byte":0,"end_byte":120,"start_line":1,"end_line":2}
```

### 2. Generate embeddings with Python

```bash
python python/embed.py
```

This creates:

```text
embeddings.npy
```

### 3. Search chunks

```bash
python python/search.py
```

Enter a query when prompted.

Example:

```text
Query: how does ownership work in rust
```

Example output:

```text
Score: 0.7784
Chunk 1 — input.txt:1-2
Rust ownership means each value has a single owner.
Borrowing allows code to access data without taking ownership.
```

## Current Limitations

This is still a prototype.

Current limitations:

* Only one input file
* Hardcoded paths
* No CLI arguments
* No folder ingestion
* No tests yet
* Simple fixed-line chunking
* No LLM answer generation
* Chunk embeddings must be regenerated manually after `output.jsonl` changes

## Next Planned Improvements

Possible next steps:

1. Add Rust tests for `document.rs`, `chunk.rs`, and `export.rs`
2. Add CLI arguments for input path, output path, and chunk size
3. Add folder ingestion
4. Move shared Python loading logic into a utility module
5. Add better error handling in Python
6. Add optional LLM answer generation after retrieval
7. Add a README example using a real document

## Learning Goal

The purpose of RAG-Oxide is not to hide the retrieval pipeline behind a framework.

The purpose is to learn the pipeline directly:

```text
documents
   ↓
chunks
   ↓
embeddings
   ↓
similarity search
   ↓
retrieved context
```

Rust is used for the structured ingestion side.

Python is used for the machine learning and retrieval side.
