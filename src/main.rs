use rag_oxide::ingest::ingest_file;
fn main() {
    let input_path = std::path::Path::new("input.txt");
    let output_path = std::path::Path::new("output.jsonl");
    let chunk_size_lines = 2;

    if let Err(error) = ingest_file(input_path, output_path, chunk_size_lines) {
        eprintln!("ingest failed: {:?}", error);
        std::process::exit(1);
    }
}
