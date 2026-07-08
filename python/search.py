import json
import numpy as np
from sentence_transformers import SentenceTransformer


def load_chunks(path: str) -> list[dict]:
    chunks = []

    with open(path, "r", encoding="utf-8") as file:
        for line in file:
            chunk = json.loads(line)
            chunks.append(chunk)

    return chunks


def search_chunks(
    query: str,
    chunks: list[dict],
    embeddings: np.ndarray,
    model: SentenceTransformer,
    top_k: int = 3,
) -> list[tuple[float, dict]]:
    query_embedding = model.encode(query, normalize_embeddings=True)

    scores = embeddings @ query_embedding

    top_indices = np.argsort(scores)[::-1][:top_k]

    results = []

    for index in top_indices:
        score = float(scores[index])
        chunk = chunks[index]
        results.append((score, chunk))

    return results


if __name__ == "__main__":
    chunks = load_chunks("output.jsonl")

    embeddings = np.load("embeddings.npy")

    print(f"Loaded {len(chunks)} chunks")
    print(f"Loaded embeddings with shape: {embeddings.shape}")

    print("Loading embedding model...")
    model = SentenceTransformer("sentence-transformers/all-MiniLM-L6-v2")

    query = input("\nQuery: ")

    results = search_chunks(query, chunks, embeddings, model, top_k=3)

    print(f"\nResults for: {query}\n")

    for score, chunk in results:
        print(f"Score: {score:.4f}")
        print(
            f"Chunk {chunk['chunk_id']} — "
            f"{chunk['source_path']}:{chunk['start_line']}-{chunk['end_line']}"
        )
        print(chunk["text"])