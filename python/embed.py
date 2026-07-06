import json
from sentence_transformers import SentenceTransformer


def load_chunks(path: str) -> list[dict]:
    chunks = []

    with open(path, "r", encoding="utf-8") as file:
        for line in file:
            chunk = json.loads(line)
            chunks.append(chunk)

    return chunks



if __name__ == "__main__":
    chunks = load_chunks("output.jsonl")

    for chunk in chunks:
        print(
            f"Chunk {chunk['chunk_id']} — "
            f"{chunk['source_path']}:{chunk['start_line']}-{chunk['end_line']}"
        )
        print(chunk["text"])

    texts = [chunk["text"] for chunk in chunks]

    print(f"Loaded {len(chunks)} chunks")
    print(f"Extracted {len(texts)} texts")

    model = SentenceTransformer("sentence-transformers/all-MiniLM-L6-v2")
    embeddings = model.encode(texts, normalize_embeddings=True)

    print(embeddings.shape)