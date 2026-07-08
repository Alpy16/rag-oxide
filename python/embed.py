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


if __name__ == "__main__":
    chunks = load_chunks("output.jsonl")

    texts = [chunk["text"] for chunk in chunks]

    print(f"Loaded {len(chunks)} chunks")
    print(f"Extracted {len(texts)} texts")

    print("Loading embedding model...")
    model = SentenceTransformer("sentence-transformers/all-MiniLM-L6-v2")

    print("Encoding chunks...")
    embeddings = model.encode(texts, normalize_embeddings=True)

    print(f"Embeddings shape: {embeddings.shape}")

    np.save("embeddings.npy", embeddings)

    print("Saved embeddings to embeddings.npy")